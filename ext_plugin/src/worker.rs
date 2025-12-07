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

//! Main JavaScript execution environment - MainWorker creation

use anyhow::Result;
use deno_runtime::FeatureChecker;
use deno_runtime::deno_broadcast_channel::InMemoryBroadcastChannel;
use deno_runtime::deno_core::ModuleSpecifier;
use deno_runtime::deno_fs::RealFs;
use deno_runtime::deno_permissions::{Permissions, PermissionsContainer};
use deno_runtime::deno_web::BlobStore;
use deno_runtime::permissions::RuntimePermissionDescriptorParser;
use deno_runtime::worker::{MainWorker, WorkerOptions, WorkerServiceOptions};
use deno_tls::RootCertStoreProvider;
use std::rc::Rc;
use std::sync::Arc;

use crate::cert_store::SapphillonRootCertStoreProvider;
use crate::module_loader::NoopModuleLoader;
use crate::npm::{NoopExtNodeSys, NoopInNpmPackageChecker, NoopNpmPackageFolderResolver};

/// The runtime snapshot generated at build time.
/// This contains the pre-compiled Deno runtime JavaScript/TypeScript code.
static RUNTIME_SNAPSHOT: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/EXT_PLUGIN_SNAPSHOT.bin"));

/// Creates a MainWorker configured for simple JavaScript execution.
///
/// This worker has access to Deno's built-in APIs like:
/// - `console.log`, `console.error`, etc.
/// - `fetch` (for HTTP requests)
/// - `Deno.readTextFile`, `Deno.writeTextFile` (filesystem operations)
/// - `setTimeout`, `setInterval`
/// - And other Deno runtime APIs
///
/// Note: ES module imports are NOT supported. Only inline script execution works.
pub fn create_main_worker() -> Result<MainWorker> {
    // Initialize rustls crypto provider for TLS/HTTPS support (required for fetch)
    // Use ring as the crypto backend (ignore error if already installed)
    let _ = deno_runtime::deno_tls::rustls::crypto::ring::default_provider().install_default();

    // Create a dummy main module URL (required but not used for execute_script)
    let main_module = ModuleSpecifier::parse("file:///main.js")?;

    let root_cert_store_provider = Arc::new(SapphillonRootCertStoreProvider::new());

    // Create services with minimal configuration
    let services = WorkerServiceOptions::<
        NoopInNpmPackageChecker,
        NoopNpmPackageFolderResolver,
        NoopExtNodeSys,
    > {
        blob_store: Arc::new(BlobStore::default()),
        broadcast_channel: InMemoryBroadcastChannel::default(),
        deno_rt_native_addon_loader: None,
        feature_checker: Arc::new(FeatureChecker::default()),
        fs: Arc::new(RealFs),
        module_loader: Rc::new(NoopModuleLoader), // No module loading support
        node_services: None,                      // No Node.js compatibility
        npm_process_state_provider: None,
        // Create permission descriptor parser and permissions container
        permissions: PermissionsContainer::new(
            Arc::new(RuntimePermissionDescriptorParser::new(
                sys_traits::impls::RealSys,
            )),
            Permissions::allow_all(),
        ),
        root_cert_store_provider: Some(root_cert_store_provider as Arc<dyn RootCertStoreProvider>),
        fetch_dns_resolver: Default::default(),
        shared_array_buffer_store: None,
        compiled_wasm_module_store: None,
        v8_code_cache: None,
        bundle_provider: None,
    };

    // Create worker options with the pre-generated snapshot
    let options = WorkerOptions {
        startup_snapshot: Some(RUNTIME_SNAPSHOT),
        ..Default::default()
    };

    // Bootstrap the worker with the snapshot
    let worker = MainWorker::bootstrap_from_options(&main_module, services, options);

    Ok(worker)
}
