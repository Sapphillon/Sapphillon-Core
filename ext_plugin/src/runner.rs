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
use deno_runtime::{FeatureChecker, deno_permissions::PermissionsContainer};
use deno_web::BlobStore;
use std::sync::Arc;
use deno_lib::worker::{CreateModuleLoaderResult, LibMainWorkerFactory, ModuleLoaderFactory};

// Dummy ModuleLoaderFactory implementation
struct NoopModuleLoaderFactory;
impl ModuleLoaderFactory for NoopModuleLoaderFactory {
    fn create_for_main(&self, _root_permissions: PermissionsContainer) -> CreateModuleLoaderResult{
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

// Dummy NodeResolver
use std::sync::Arc;

// --------------------------- Adjust these imports ---------------------------
// The module paths below reflect the structure used in recent Deno trees but may
// require minor edits for the exact crate versions you depend on.
use node_resolver::{NodeResolver, NodeResolverOptions, DenoIsBuiltInNodeModuleChecker};
use deno_resolver::npm::NpmResolver; // may be under deno_resolver::npm
use deno_resolver::pkg_json::PackageJsonResolver;
use deno_resolver::in_npm::DenoInNpmPackageChecker;

// TSys is the "sys" abstraction type used across deno crates (DenoLibSys / CliSys).
// Provide your real sys type when calling the function.
pub fn build_dummy_node_resolver<TSys>(
  sys: TSys,
) -> Arc<
  NodeResolver<
    DenoInNpmPackageChecker,
    NpmResolver<TSys>,
    TSys,
  >,
>
where
  // These trait bounds are common; if your TSys type uses different bounds,
  // adapt these bounds accordingly.
  TSys: Clone + Send + Sync + 'static,
{
  // 1) in-npm package checker
  //
  // We want "no managed npm" behavior so that resolver will not try to resolve
  // npm packages. Many Deno-internal helpers provide a "Byonm" (bring-your-own-node-modules)
  // option or similar. If there is a simple constructor, use it. Otherwise use the
  // default that indicates "not in managed npm".
  //
  // TODO: Replace the unimplemented!() with the appropriate constructor for
  // DenoInNpmPackageChecker in your deno_resolver version. Example:
  //   let in_npm_checker = DenoInNpmPackageChecker::new(CreateInNpmPkgCheckerOptions::Byonm);
  let in_npm_checker: DenoInNpmPackageChecker = {
    // If DenoInNpmPackageChecker has a simple default or new() that fits your
    // use-case, call it here. Otherwise construct the variant that means "no managed npm".
    //
    // Placeholder:
    unimplemented!("construct a DenoInNpmPackageChecker (Byonm / default) for your deno version")
  };

  // 2) npm_resolver: a minimal/dummy NpmResolver<TSys>
  //
  // NodeResolver expects an NpmResolver value. For the dummy case we create an
  // NpmResolver minimal instance (or a very lightweight wrapper) that will return
  // errors when asked to resolve npm artifacts. How to construct it depends on
  // deno_resolver::npm::NpmResolver API.
  //
  // TODO: Replace the unimplemented!() with a real NpmResolver constructor or a
  // thin wrapper type that implements the same API. If creating a wrapper is easier,
  // implement `struct DummyNpmResolver<TSys>(...)` and expose `Arc<DummyNpmResolver>` here.
  let npm_resolver: Arc<NpmResolver<TSys>> = {
    // Example placeholder:
    unimplemented!("construct a minimal NpmResolver<TSys> or wrapper for your deno version")
  };

  // 3) package.json resolver: minimal PackageJsonResolver<TSys>
  //
  // Many constructors expect a pkg_json_resolver. Provide a simple implementation
  // or reuse a default from deno_resolver.
  //
  // TODO: Replace with actual constructor for PackageJsonResolver in your deno version.
  let pkg_json_resolver: Arc<PackageJsonResolver<TSys>> = {
    unimplemented!("construct a minimal PackageJsonResolver<TSys> for your deno version")
  };

  // 4) NodeResolverOptions: choose conservative default values.
  //
  // Use Default::default() if available, otherwise fill required fields.
  let node_options = {
    // TODO: Either use NodeResolverOptions::default() or populate fields such as:
    // - conditions
    // - typescript_version
    // - bundle_mode: false
    // - is_browser_platform: true/false (for embedding probably false)
    //
    // Example:
    // NodeResolverOptions { conditions: vec![], typescript_version: None, bundle_mode: false, is_browser_platform: false }
    Default::default()
  };

  // 5) Construct NodeResolver
  //
  // The NodeResolver::new() signature in the Deno codebase usually looks like:
  // NodeResolver::new(in_npm_checker, DenoIsBuiltInNodeModuleChecker, npm_resolver, pkg_json_resolver, sys, node_options)
  //
  // TODO: Call the actual constructor for your version.
  let node_resolver = NodeResolver::new(
    in_npm_checker,
    DenoIsBuiltInNodeModuleChecker,
    npm_resolver,
    pkg_json_resolver,
    sys,
    node_options,
  );

  Arc::new(node_resolver)
}


async fn run() -> Result<()> {
    let blob_store = BlobStore::default();
    let feature_checker = Arc::new({
        let mut checker = FeatureChecker::default();
    });
    let fs = Arc::new(sys_traits::impls::RealSys);
    let module_loader_factory = Box::new(NoopModuleLoaderFactory);
    let node_resolver = build_dummy_node_resolver(fs.clone());
    
    

    Ok(())
}
