// Sapphillon-Core
// SPDX-FileCopyrightText: 2025 Yuta Takahashi
// SPDX-License-Identifier: MPL-2.0 OR GPL-3.0-or-later

//! Rs-Js Bridge for external plugin execution.
//!
//! This module provides the bridge between Rust and JavaScript for executing
//! external plugin functions. It handles serialization/deserialization of
//! arguments and return values between the two runtimes.

use deno_core::{OpState, op2};
use ext_plugin::{RsJsBridgeArgs, SapphillonPackage};
use proto::sapphillon::v1::Permission;
/// Core implementation of the Rs-Js bridge.
///
/// This function:
/// 1. Retrieves the external package from `OpState` using the package ID
/// 2. Parses `RsJsBridgeArgs` from the input JSON string
/// 3. Creates a `SapphillonPackage` from the package JavaScript code
/// 4. Executes the specified function synchronously
/// 5. Returns `RsJsBridgeReturns` as a JSON string
///
/// # Arguments
///
/// * `state` - Mutable reference to the Deno `OpState`
/// * `args_json` - JSON string containing serialized `RsJsBridgeArgs`
/// * `package_id` - ID of the external plugin package to execute
///
/// # Returns
///
/// A `Result` containing either:
/// - `Ok(String)` - JSON string of `RsJsBridgeReturns` on success
/// - `Err(anyhow::Error)` - Error details on failure
///
/// # Example
///
/// ```ignore
/// let args_json = r#"{"func_name":"greet","args":{"arg0":"World"}}"#;
/// let package_id = "com.test.test-plugin";
/// let result = rsjs_bridge_core(&mut state, args_json, package_id)?;
/// ```
pub fn rsjs_bridge_core(
    state: &mut OpState,
    args_json: &str,
    package_id: &str,
) -> anyhow::Result<String> {
    use crate::runtime::OpStateWorkflowData;
    use ext_plugin::extplugin_client;
    use std::sync::{Arc, Mutex};

    // Step 1: Retrieve the external package from OpState
    let workflow_data = state
        .borrow::<Arc<Mutex<OpStateWorkflowData>>>()
        .lock()
        .map_err(|e| anyhow::anyhow!("Failed to lock OpStateWorkflowData: {e}"))?;

    let package = workflow_data
        .external_package
        .iter()
        .find(|pkg| pkg.id == package_id)
        .ok_or_else(|| anyhow::anyhow!("Package not found: {package_id}"))?;

    // Clone the data we need before dropping the lock
    // We need the SapphillonPackage itself, but it's wrapped in Arc<CorePluginExternalPackage>.
    // CorePluginExternalPackage contains the package_js string.
    // We need to construct a SapphillonPackage from it to pass to extplugin_client.
    // Wait, extplugin_client takes &SapphillonPackage.
    // CorePluginExternalPackage is NOT SapphillonPackage.
    // CorePluginExternalPackage has package_js.

    let package_js = package.package_js.clone();

    // Step 2: Parse RsJsBridgeArgs from JSON to get the function name
    let args = RsJsBridgeArgs::new_from_str(args_json)?;

    // Step 3: Get allowed permissions for this function
    // Build the plugin_function_id from package_id and func_name
    let plugin_function_id = format!("{}.{}", package_id, args.func_name);

    let sapphillon_permissions: Vec<Permission> = workflow_data
        .get_allowed_permissions()
        .as_ref()
        .and_then(|allowed_list| {
            allowed_list
                .iter()
                .find(|pf| {
                    pf.plugin_function_id == plugin_function_id || pf.plugin_function_id == "*"
                })
                .map(|pf| pf.permissions.permissions.clone())
        })
        .unwrap_or_default();

    // Step 4: Get external package runner path and args from OpStateWorkflowData
    // If provided, use them; otherwise use default values
    let runner_path_from_data = workflow_data.get_external_package_runner_path().clone();
    let runner_args_from_data = workflow_data.get_external_package_runner_args().clone();

    // Drop the lock
    drop(workflow_data);

    // Step 5: Create SapphillonPackage (lightweight, just parsing JS)
    // We need this because extplugin_client expects it.
    // Note: SapphillonPackage::new parses the JS.
    let sapphillon_package = SapphillonPackage::new(&package_js)?;

    // Step 6: Locate the runner process
    // If external_package_runner_path is set in OpStateWorkflowData, use it.
    // Otherwise, use environment variable to specify the server binary path explicitly
    // Fall back to a default path for development/testing
    let server_path = runner_path_from_data.unwrap_or_else(|| {
        std::env::var("EXTPLUGIN_SERVER_PATH").unwrap_or_else(|_| {
            std::env::current_exe()
                .ok()
                .and_then(|mut p| {
                    p.pop(); // Remove binary name
                    if p.file_name().and_then(|s| s.to_str()) == Some("deps") {
                        p.pop(); // Remove "deps" if in test
                    }
                    p.push("extplugin_test_server");
                    p.to_str().map(|s| s.to_string())
                })
                .unwrap_or_else(|| "extplugin_test_server".to_string())
        })
    });

    // If external_package_runner_args is set in OpStateWorkflowData, use it.
    // Otherwise, use empty args
    let server_args = runner_args_from_data.unwrap_or_else(Vec::new);
    let server_args_refs: Vec<&str> = server_args.iter().map(|s| s.as_str()).collect();

    // Step 7: Execute via IPC
    let returns = extplugin_client(
        &sapphillon_package,
        &args.func_name,
        &args,
        &server_path,
        server_args_refs,
        sapphillon_permissions,
    )?;

    // Step 6: Serialize RsJsBridgeReturns to JSON
    returns.to_string()
}

/// Deno op wrapper for the Rs-Js bridge.
///
/// This operation is called from JavaScript workflow code to execute external
/// plugin functions. It wraps `rsjs_bridge_core` and converts any errors to
/// JavaScript-compatible error format.
///
/// # Arguments
///
/// * `state` - Mutable reference to the Deno `OpState`
/// * `args_json` - JSON string containing serialized `RsJsBridgeArgs`
/// * `package_id` - ID of the external plugin package to execute
///
/// # Returns
///
/// JSON string of `RsJsBridgeReturns` on success, or throws a JavaScript error
/// on failure.
#[op2(reentrant)]
#[string]
pub fn rsjs_bridge_opdecl(
    state: &mut OpState,
    #[string] args_json: String,
    #[string] package_id: String,
) -> Result<String, deno_error::JsErrorBox> {
    rsjs_bridge_core(state, &args_json, &package_id)
        .map_err(|e| deno_error::JsErrorBox::generic(format!("Rs-Js Bridge Error: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plugin::CorePluginExternalPackage;
    use crate::runtime::OpStateWorkflowData;
    use ext_plugin::RsJsBridgeReturns;
    use serde_json::json;
    use std::sync::{Arc, Mutex};

    fn get_test_package_js() -> String {
        r#"
        globalThis.Sapphillon = {
            Package: {
                meta: {
                    name: "test-plugin",
                    version: "1.0.0",
                    description: "A test plugin",
                    author_id: "com.test",
                    package_id: "com.test.test-plugin"
                },
                functions: {
                    greet: {
                        description: "Greets a user",
                        permissions: [],
                        parameters: [{
                            idx: 0,
                            name: "arg0",
                            type: "string",
                            description: "Name to greet"
                        }],
                        returns: [{
                            idx: 0,
                            type: "string",
                            description: "Greeting message"
                        }],
                        handler: (name) => `Hello, ${name}!`
                    },
                    add: {
                        description: "Adds two numbers",
                        permissions: [],
                        parameters: [
                            { idx: 0, name: "arg0", type: "number", description: "First number" },
                            { idx: 1, name: "arg1", type: "number", description: "Second number" }
                        ],
                        returns: [{
                            idx: 0,
                            type: "number",
                            description: "Sum of the two numbers"
                        }],
                        handler: (a, b) => a + b
                    }
                }
            }
        };
        "#
        .to_string()
    }

    /// Creates OpState with workflow data containing the test external package
    /// Returns the OpState and the tokio Runtime to keep the runtime alive
    fn create_opstate_with_package() -> (deno_core::OpState, tokio::runtime::Runtime) {
        use deno_core::OpState;

        let mut op_state = OpState::new(None);

        // Create the external package
        let package = CorePluginExternalPackage::new(
            "com.test.test-plugin".to_string(),
            "test-plugin".to_string(),
            vec![], // functions list not needed for this test
            get_test_package_js(),
        );

        // Create OpStateWorkflowData with the external package
        let tokio_runtime = tokio::runtime::Runtime::new().unwrap();
        let workflow_data = OpStateWorkflowData::new(
            "test_workflow",
            false,
            None,
            None,
            tokio_runtime.handle().clone(),
            vec![Arc::new(package)],
            None,
            None,
        );

        // Put workflow data into OpState
        op_state.put(Arc::new(Mutex::new(workflow_data)));

        (op_state, tokio_runtime)
    }

    #[test]
    fn test_rsjs_bridge_core_greet() {
        let (mut op_state, _tokio_rt) = create_opstate_with_package();

        let args = RsJsBridgeArgs {
            func_name: "greet".to_string(),
            args: vec![("arg0".to_string(), json!("World"))]
                .into_iter()
                .collect(),
        };
        let args_json = args.to_string().unwrap();

        let result = rsjs_bridge_core(&mut op_state, &args_json, "com.test.test-plugin");
        assert!(result.is_ok(), "Expected Ok, got {result:?}");

        let result_json = result.unwrap();
        let returns = RsJsBridgeReturns::new_from_str(&result_json).unwrap();
        assert_eq!(returns.args.get("result"), Some(&json!("Hello, World!")));
    }

    #[test]
    fn test_rsjs_bridge_core_add() {
        let (mut op_state, _tokio_rt) = create_opstate_with_package();

        let args = RsJsBridgeArgs {
            func_name: "add".to_string(),
            args: vec![
                ("arg0".to_string(), json!(5)),
                ("arg1".to_string(), json!(3)),
            ]
            .into_iter()
            .collect(),
        };
        let args_json = args.to_string().unwrap();

        let result = rsjs_bridge_core(&mut op_state, &args_json, "com.test.test-plugin");
        assert!(result.is_ok(), "Expected Ok, got {result:?}");

        let result_json = result.unwrap();
        let returns = RsJsBridgeReturns::new_from_str(&result_json).unwrap();
        assert_eq!(returns.args.get("result"), Some(&json!(8)));
    }

    #[test]
    fn test_rsjs_bridge_core_invalid_json() {
        let (mut op_state, _tokio_rt) = create_opstate_with_package();

        let result = rsjs_bridge_core(&mut op_state, "invalid json", "com.test.test-plugin");
        assert!(result.is_err());
    }

    #[test]
    fn test_rsjs_bridge_core_invalid_package() {
        let (mut op_state, _tokio_rt) = create_opstate_with_package();

        let args = RsJsBridgeArgs {
            func_name: "test".to_string(),
            args: Default::default(),
        };
        let args_json = args.to_string().unwrap();

        // Try to use a package name that doesn't exist
        let result = rsjs_bridge_core(&mut op_state, &args_json, "non-existent-package");
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Package not found"));
    }
}
