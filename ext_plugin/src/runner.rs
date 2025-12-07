// Sapphillon-Core
// Copyright 2025 Yuta Takahashi
//
// This file is part of Sapphillon-Core
//
// Sapphillon-Core is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
//

use anyhow::Result;
use deno_lib::worker::{CreateModuleLoaderResult, ModuleLoaderFactory};
use deno_runtime::{FeatureChecker, deno_permissions::PermissionsContainer};
use deno_web::BlobStore;
use std::sync::Arc;

// Dummy ModuleLoaderFactory implementation
struct NoopModuleLoaderFactory;
impl ModuleLoaderFactory for NoopModuleLoaderFactory {
    fn create_for_main(&self, _root_permissions: PermissionsContainer) -> CreateModuleLoaderResult {
        // TODO: implement a proper module loader
        unimplemented!()
    }

    fn create_for_worker(
        &self,
        _parent_permissions: PermissionsContainer,
        _permissions: PermissionsContainer,
    ) -> CreateModuleLoaderResult {
        unimplemented!()
    }
}

// Dummy NodeResolver implementation using node_resolver's native traits
use node_resolver::{
    DenoIsBuiltInNodeModuleChecker, InNpmPackageChecker, NodeResolver, NodeResolverOptions,
    NodeResolverSys, NpmPackageFolderResolver, PackageJsonResolver, PackageJsonResolverRc,
    UrlOrPathRef,
    cache::NodeResolutionSys,
    errors::{PackageFolderResolveError, PackageNotFoundError},
};
use std::path::PathBuf;
use url::Url;

/// A dummy InNpmPackageChecker that always returns false
/// (indicating no specifier is inside an npm package).
#[derive(Debug, Clone, Copy)]
pub struct NoopInNpmPackageChecker;

impl InNpmPackageChecker for NoopInNpmPackageChecker {
    fn in_npm_package(&self, _specifier: &Url) -> bool {
        false
    }
}

/// A dummy NpmPackageFolderResolver that always returns an error
/// (npm packages are not supported in this dummy implementation).
#[derive(Debug, Clone, Copy)]
pub struct NoopNpmPackageFolderResolver;

impl NpmPackageFolderResolver for NoopNpmPackageFolderResolver {
    fn resolve_package_folder_from_package(
        &self,
        specifier: &str,
        referrer: &UrlOrPathRef,
    ) -> Result<PathBuf, PackageFolderResolveError> {
        Err(PackageNotFoundError {
            package_name: specifier.to_string(),
            referrer: referrer.display(),
            referrer_extra: None,
        }
        .into())
    }
}

/// Build a dummy NodeResolver that doesn't support npm packages.
/// This is useful for testing or when npm resolution is not needed.
pub fn build_dummy_node_resolver<TSys>(
    sys: TSys,
) -> Arc<
    NodeResolver<
        NoopInNpmPackageChecker,
        DenoIsBuiltInNodeModuleChecker,
        NoopNpmPackageFolderResolver,
        TSys,
    >,
>
where
    TSys: NodeResolverSys + Clone + Send + Sync + 'static,
{
    // 1) In-npm package checker - always returns false
    let in_npm_checker = NoopInNpmPackageChecker;

    // 2) Built-in node module checker - use Deno's default implementation
    let is_built_in_checker = DenoIsBuiltInNodeModuleChecker;

    // 3) Npm package folder resolver - returns errors for all npm requests
    let npm_pkg_folder_resolver = NoopNpmPackageFolderResolver;

    // 4) Package.json resolver
    let pkg_json_resolver: PackageJsonResolverRc<TSys> =
        Arc::new(PackageJsonResolver::new(sys.clone(), None));

    // 5) Node resolution sys wrapper
    let node_resolution_sys = NodeResolutionSys::new(sys, None);

    // 6) Node resolver options - conservative defaults
    let node_options = NodeResolverOptions::default();

    // 7) Construct NodeResolver
    let node_resolver = NodeResolver::new(
        in_npm_checker,
        is_built_in_checker,
        npm_pkg_folder_resolver,
        pkg_json_resolver,
        node_resolution_sys,
        node_options,
    );

    Arc::new(node_resolver)
}

#[allow(dead_code)]
async fn run() -> Result<()> {
    let blob_store = BlobStore::default();
    let _feature_checker = Arc::new(FeatureChecker::default());
    // Use RealSys directly, not Arc<RealSys>, since NodeResolverSys is implemented for RealSys
    let fs = sys_traits::impls::RealSys;
    let _module_loader_factory = Box::new(NoopModuleLoaderFactory);
    let _node_resolver = build_dummy_node_resolver(fs);
    let _ = blob_store;

    Ok(())
}
