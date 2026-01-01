// Sapphillon-Core
// SPDX-FileCopyrightText: 2025 Yuta Takahashi
// SPDX-License-Identifier: MPL-2.0 OR GPL-3.0-or-later

//! Rs-Js Bridge for external plugin execution.
//!
//! This module provides the bridge between Rust and JavaScript for executing
//! external plugin functions. It handles serialization/deserialization of
//! arguments and return values between the two runtimes.

use deno_core::op2;
use ext_plugin::{RsJsBridgeArgs, SapphillonPackage};

/// Core implementation of the Rs-Js bridge.
///
/// This function:
/// 1. Parses `RsJsBridgeArgs` from the input JSON string
/// 2. Creates a `SapphillonPackage` from the package JavaScript code
/// 3. Executes the specified function synchronously
/// 4. Returns `RsJsBridgeReturns` as a JSON string
///
/// # Arguments
///
/// * `args_json` - JSON string containing serialized `RsJsBridgeArgs`
/// * `package_js` - JavaScript code defining the external plugin package
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
/// let package_js = r#"globalThis.Sapphillon = { Package: { ... } };"#;
/// let result = rsjs_bridge_core(args_json, package_js)?;
/// ```
pub fn rsjs_bridge_core(args_json: &str, package_js: &str) -> anyhow::Result<String> {
    // Step 1: Parse RsJsBridgeArgs from JSON
    let args = RsJsBridgeArgs::new_from_str(args_json)?;

    // Step 2: Parse the package
    // Note: SapphillonPackage::new() internally creates its own tokio runtime
    let package = SapphillonPackage::new(package_js)?;

    // Step 3: Execute the plugin function synchronously
    // Create a new runtime for the execute call since the package parsing
    // already used its own runtime
    let rt = tokio::runtime::Runtime::new()?;
    let returns = rt.block_on(package.execute(args, &None))?;

    // Step 4: Serialize RsJsBridgeReturns to JSON
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
/// * `args_json` - JSON string containing serialized `RsJsBridgeArgs`
/// * `package_js` - JavaScript code defining the external plugin package
///
/// # Returns
///
/// JSON string of `RsJsBridgeReturns` on success, or throws a JavaScript error
/// on failure.
#[op2]
#[string]
pub fn rsjs_bridge_opdecl(
    #[string] args_json: String,
    #[string] package_js: String,
) -> Result<String, deno_error::JsErrorBox> {
    rsjs_bridge_core(&args_json, &package_js)
        .map_err(|e| deno_error::JsErrorBox::generic(format!("Rs-Js Bridge Error: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ext_plugin::RsJsBridgeReturns;
    use serde_json::json;

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

    #[test]
    fn test_rsjs_bridge_core_greet() {
        let package_js = get_test_package_js();
        let args = RsJsBridgeArgs {
            func_name: "greet".to_string(),
            args: vec![("arg0".to_string(), json!("World"))]
                .into_iter()
                .collect(),
        };
        let args_json = args.to_string().unwrap();

        let result = rsjs_bridge_core(&args_json, &package_js);
        assert!(result.is_ok(), "Expected Ok, got {result:?}");

        let result_json = result.unwrap();
        let returns = RsJsBridgeReturns::new_from_str(&result_json).unwrap();
        assert_eq!(returns.args.get("result"), Some(&json!("Hello, World!")));
    }

    #[test]
    fn test_rsjs_bridge_core_add() {
        let package_js = get_test_package_js();
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

        let result = rsjs_bridge_core(&args_json, &package_js);
        assert!(result.is_ok(), "Expected Ok, got {result:?}");

        let result_json = result.unwrap();
        let returns = RsJsBridgeReturns::new_from_str(&result_json).unwrap();
        assert_eq!(returns.args.get("result"), Some(&json!(8)));
    }

    #[test]
    fn test_rsjs_bridge_core_invalid_json() {
        let package_js = get_test_package_js();
        let result = rsjs_bridge_core("invalid json", &package_js);
        assert!(result.is_err());
    }

    #[test]
    fn test_rsjs_bridge_core_invalid_package() {
        let args = RsJsBridgeArgs {
            func_name: "test".to_string(),
            args: Default::default(),
        };
        let args_json = args.to_string().unwrap();
        let result = rsjs_bridge_core(&args_json, "invalid package js");
        assert!(result.is_err());
    }
}
