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

/// Core implementation of the Rs-Js bridge.
///
/// This function:
/// 1. Retrieves the external package from `OpState` using the package name
/// 2. Parses `RsJsBridgeArgs` from the input JSON string
/// 3. Creates a `SapphillonPackage` from the package JavaScript code
/// 4. Executes the specified function synchronously
/// 5. Returns `RsJsBridgeReturns` as a JSON string
///
/// # Arguments
///
/// * `state` - Mutable reference to the Deno `OpState`
/// * `args_json` - JSON string containing serialized `RsJsBridgeArgs`
/// * `package_name` - Name of the external plugin package to execute
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
/// let package_name = "test-plugin";
/// let result = rsjs_bridge_core(&mut state, args_json, package_name)?;
/// ```
pub fn rsjs_bridge_core(
    state: &mut OpState,
    args_json: &str,
    package_name: &str,
) -> anyhow::Result<String> {
    use crate::runtime::OpStateWorkflowData;
    use std::sync::{Arc, Mutex};

    // Step 1: Retrieve the external package from OpState
    let workflow_data = state
        .borrow::<Arc<Mutex<OpStateWorkflowData>>>()
        .lock()
        .map_err(|e| anyhow::anyhow!("Failed to lock OpStateWorkflowData: {}", e))?;

    let package = workflow_data
        .external_package
        .iter()
        .find(|pkg| pkg.name == package_name)
        .ok_or_else(|| anyhow::anyhow!("Package not found: {}", package_name))?;

    // Clone the data we need before dropping the lock
    let package_js = package.package_js.clone();
    let tokio_handle = workflow_data.tokio_runtime_handle.clone();
    
    // Drop the lock before doing async work
    drop(workflow_data);

    // Step 2: Parse RsJsBridgeArgs from JSON
    let args = RsJsBridgeArgs::new_from_str(args_json)?;

    // Step 3 & 4: Parse the package and execute within a separate thread
    // We must use a separate thread because we cannot create a new V8 isolate
    // while already inside a V8 context (this function is called from within a Deno op).
    let handle = std::thread::spawn(move || {
        tokio_handle.block_on(async {
            let sapphillon_package = SapphillonPackage::new_async(&package_js).await?;
            sapphillon_package.execute(args, &None).await
        })
    });

    let returns = handle
        .join()
        .map_err(|e| anyhow::anyhow!("Thread panicked: {:?}", e))??;

    // Step 5: Serialize RsJsBridgeReturns to JSON
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
/// * `package_name` - Name of the external plugin package to execute
///
/// # Returns
///
/// JSON string of `RsJsBridgeReturns` on success, or throws a JavaScript error
/// on failure.
#[op2]
#[string]
pub fn rsjs_bridge_opdecl(
    state: &mut OpState,
    #[string] args_json: String,
    #[string] package_name: String,
) -> Result<String, deno_error::JsErrorBox> {
    rsjs_bridge_core(state, &args_json, &package_name)
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
                    package_id: "com.test.plugin"
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
            "com.test.plugin".to_string(),
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

        let result = rsjs_bridge_core(&mut op_state, &args_json, "test-plugin");
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

        let result = rsjs_bridge_core(&mut op_state, &args_json, "test-plugin");
        assert!(result.is_ok(), "Expected Ok, got {result:?}");

        let result_json = result.unwrap();
        let returns = RsJsBridgeReturns::new_from_str(&result_json).unwrap();
        assert_eq!(returns.args.get("result"), Some(&json!(8)));
    }

    #[test]
    fn test_rsjs_bridge_core_invalid_json() {
        let (mut op_state, _tokio_rt) = create_opstate_with_package();

        let result = rsjs_bridge_core(&mut op_state, "invalid json", "test-plugin");
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
