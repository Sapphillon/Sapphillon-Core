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
use deno_error::JsErrorBox;
use deno_runtime::FeatureChecker;
use deno_runtime::deno_broadcast_channel::InMemoryBroadcastChannel;
use deno_runtime::deno_core::{
    ModuleLoadResponse, ModuleLoader, ModuleSpecifier, RequestedModuleType, ResolutionKind,
};
use deno_runtime::deno_fs::RealFs;
use deno_runtime::deno_permissions::{Permissions, PermissionsContainer};
use deno_runtime::deno_web::BlobStore;
use deno_runtime::permissions::RuntimePermissionDescriptorParser;
use deno_runtime::worker::{MainWorker, WorkerOptions, WorkerServiceOptions};
use node_resolver::errors::{PackageFolderResolveError, PackageNotFoundError};
use node_resolver::{InNpmPackageChecker, NpmPackageFolderResolver, UrlOrPathRef};
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;
use url::Url;

// ==============================================================================
// Dummy implementations for npm-related traits (not used in this environment)
// ==============================================================================

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
    ) -> std::result::Result<PathBuf, PackageFolderResolveError> {
        Err(PackageNotFoundError {
            package_name: specifier.to_string(),
            referrer: referrer.display(),
            referrer_extra: None,
        }
        .into())
    }
}

/// A dummy ExtNodeSys implementation using the real filesystem.
/// This is needed for MainWorker but we don't support Node.js features.
pub type NoopExtNodeSys = sys_traits::impls::RealSys;

// ==============================================================================
// Dummy ModuleLoader that doesn't support ES module imports
// ==============================================================================

/// A module loader that doesn't support loading any modules.
/// Only inline script execution via `execute_script` is supported.
struct NoopModuleLoader;

impl ModuleLoader for NoopModuleLoader {
    fn resolve(
        &self,
        specifier: &str,
        _referrer: &str,
        _kind: ResolutionKind,
    ) -> std::result::Result<ModuleSpecifier, JsErrorBox> {
        // Return the specifier as-is if it's a valid URL
        ModuleSpecifier::parse(specifier)
            .map_err(|e| JsErrorBox::generic(format!("Module resolution not supported: {e}")))
    }

    fn load(
        &self,
        module_specifier: &ModuleSpecifier,
        _maybe_referrer: Option<&deno_runtime::deno_core::ModuleLoadReferrer>,
        _is_dyn_import: bool,
        _requested_module_type: RequestedModuleType,
    ) -> ModuleLoadResponse {
        let specifier = module_specifier.clone();
        ModuleLoadResponse::Sync(Err(JsErrorBox::generic(format!(
            "Module loading not supported: {specifier}"
        ))))
    }
}

// ==============================================================================
// Main JavaScript execution environment
// ==============================================================================

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

/// The runtime snapshot generated at build time.
/// This contains the pre-compiled Deno runtime JavaScript/TypeScript code.
static RUNTIME_SNAPSHOT: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/EXT_PLUGIN_SNAPSHOT.bin"));

pub fn create_main_worker() -> Result<MainWorker> {
    // Initialize rustls crypto provider for TLS/HTTPS support (required for fetch)
    // Use ring as the crypto backend (ignore error if already installed)
    let _ = deno_runtime::deno_tls::rustls::crypto::ring::default_provider().install_default();

    // Create a dummy main module URL (required but not used for execute_script)
    let main_module = ModuleSpecifier::parse("file:///main.js")?;

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
        root_cert_store_provider: None,
        fetch_dns_resolver: Default::default(),
        shared_array_buffer_store: None,
        compiled_wasm_module_store: None,
        v8_code_cache: None,
        bundle_provider: None,
    };

    // Create worker options with the pre-generated snapshot
    let mut options = WorkerOptions::default();
    options.startup_snapshot = Some(RUNTIME_SNAPSHOT);

    // Bootstrap the worker with the snapshot
    let worker = MainWorker::bootstrap_from_options(&main_module, services, options);

    Ok(worker)
}

/// Executes JavaScript code using Deno's MainWorker.
///
/// This provides access to Deno's built-in APIs like `console`, `fetch`,
/// filesystem operations, etc.
///
/// # Arguments
/// * `script` - The JavaScript code to execute
///
/// # Returns
/// * `Ok(())` on successful execution
/// * `Err(...)` if the script fails to execute
///
/// # Example
/// ```rust,ignore
/// let result = run_js("console.log('Hello from Deno!')").await;
/// ```
pub async fn run_js(script: &str) -> Result<()> {
    let mut worker = create_main_worker()?;

    // Execute the script
    worker.execute_script("[ext_plugin]", script.to_string().into())?;

    // Run the event loop to completion (handles async operations like fetch)
    worker.run_event_loop(false).await?;

    Ok(())
}

/// Executes JavaScript code and returns the exit code.
///
/// Similar to `run_js` but also handles the full worker lifecycle including
/// dispatching load/unload events.
pub async fn run_js_with_events(script: &str) -> Result<i32> {
    let mut worker = create_main_worker()?;

    // Execute the script
    worker.execute_script("[ext_plugin]", script.to_string().into())?;

    // Dispatch load event
    worker.dispatch_load_event()?;

    // Run event loop
    loop {
        worker.run_event_loop(false).await?;

        let web_continue = worker.dispatch_beforeunload_event()?;
        if !web_continue {
            break;
        }
    }

    // Dispatch unload event
    worker.dispatch_unload_event()?;

    Ok(worker.exit_code())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_run_js_console_log() {
        let result = run_js("console.log('Hello from Deno MainWorker!')").await;
        assert!(result.is_ok(), "Should be able to run console.log");
    }

    #[tokio::test]
    async fn test_run_js_simple_calculation() {
        let result = run_js("const x = 1 + 1; console.log('1 + 1 =', x);").await;
        assert!(result.is_ok(), "Should be able to run simple calculations");
    }

    #[tokio::test]
    async fn test_run_js_with_events() {
        let result = run_js_with_events("console.log('With events!')").await;
        assert!(result.is_ok(), "Should be able to run with events");
        assert_eq!(result.unwrap(), 0, "Exit code should be 0");
    }

    #[tokio::test]
    async fn test_run_js_fetch() {
        let result = run_js(
            r#"
            (async () => {
                const response = await fetch('https://httpbin.org/get');
                console.log('Fetch status:', response.status);
                const data = await response.json();
                console.log('Fetch origin:', data.origin);
            })();
            "#,
        )
        .await;
        assert!(
            result.is_ok(),
            "Should be able to run fetch: {:?}",
            result.err()
        );
    }
}
