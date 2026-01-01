
use sapphillon_core::ext_plugin::{RsJsBridgeArgs, RsJsBridgeReturns};
use sapphillon_core::extplugin_rsjs_bridge::rsjs_bridge_core;
use serde_json::json;
use std::path::PathBuf;

fn get_fixture_path(filename: &str) -> PathBuf {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("tests");
    d.push("fixtures");
    d.push(filename);
    d
}

/// Integration Test: Basic Function Execution
///
/// **Purpose:**
/// Verify that the `rsjs_bridge_core` can correctly execute a simple function (`add`)
/// from an external plugin package.
///
/// **Intent:**
/// This test ensures that:
/// 1. The bridge correctly loads the plugin package script.
/// 2. Arguments are correctly serialized and passed to the JavaScript function.
/// 3. The JavaScript function executes successfully.
/// 4. The return value is correctly serialized and returned to Rust.
///
/// **Flow:**
/// 1. Load the `plugin_package.js` fixture.
/// 2. Construct `RsJsBridgeArgs` with the function name "add" and arguments `a=10`, `b=20`.
/// 3. Execute the bridge via `rsjs_bridge_core`.
/// 4. Parse the result and assert that the returned "result" is `30`.
#[test]
fn test_integration_math_plugin_add() {
    // 1. Load the plugin package fixture
    let fixture_path = get_fixture_path("plugin_package.js");
    let package_js = std::fs::read_to_string(fixture_path).expect("Failed to read fixture");

    // 2. Prepare arguments for the 'add' function
    let args = RsJsBridgeArgs {
        func_name: "add".to_string(),
        args: vec![
            ("a".to_string(), json!(10)),
            ("b".to_string(), json!(20)),
        ]
        .into_iter()
        .collect(),
    };
    let args_json = args.to_string().unwrap();

    // 3. Execute the bridge
    let result = rsjs_bridge_core(&args_json, &package_js);
    assert!(result.is_ok(), "Bridge execution failed: {:?}", result.err());

    // 4. Verify the result
    let result_json = result.unwrap();
    let returns = RsJsBridgeReturns::new_from_str(&result_json).expect("Failed to parse returns");

    assert_eq!(returns.args.get("result"), Some(&json!(30)));
}

/// Integration Test: Complex Object Handling
///
/// **Purpose:**
/// Verify that the bridge can handle complex object types for both arguments and return values.
///
/// **Intent:**
/// This test ensures that:
/// 1. JSON objects can be passed as arguments to the plugin.
/// 2. The plugin can process these objects and return a new complex object.
/// 3. The bridge correctly preserves the structure of nested JSON data.
///
/// **Flow:**
/// 1. Load the `plugin_package.js` fixture.
/// 2. Construct `RsJsBridgeArgs` for "process_data" with a complex input object.
/// 3. Execute the bridge.
/// 4. Verify that the returned object contains the expected fields ("original", "result", "timestamp").
#[test]
fn test_integration_math_plugin_process_data() {
    // 1. Load the plugin package fixture
    let fixture_path = get_fixture_path("plugin_package.js");
    let package_js = std::fs::read_to_string(fixture_path).expect("Failed to read fixture");

    // 2. Prepare complex input data
    let input_data = json!({
        "value": 50,
        "multiplier": 2
    });

    let args = RsJsBridgeArgs {
        func_name: "process_data".to_string(),
        args: vec![
            ("data".to_string(), input_data),
        ]
        .into_iter()
        .collect(),
    };
    let args_json = args.to_string().unwrap();

    // 3. Execute the bridge
    let result = rsjs_bridge_core(&args_json, &package_js);
    assert!(result.is_ok(), "Bridge execution failed: {:?}", result.err());

    // 4. Verify the result structure and values
    let result_json = result.unwrap();
    let returns = RsJsBridgeReturns::new_from_str(&result_json).expect("Failed to parse returns");

    let result_obj = returns.args.get("result").expect("No result returned");
    
    assert_eq!(result_obj.get("original"), Some(&json!(50)));
    assert_eq!(result_obj.get("result"), Some(&json!(100)));
    // Verify timestamp exists (dynamic value, so we just check existence)
    assert!(result_obj.get("timestamp").is_some());
}
