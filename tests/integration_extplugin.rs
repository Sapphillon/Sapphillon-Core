use sapphillon_core::ext_plugin::{RsJsBridgeArgs, RsJsBridgeReturns};
use sapphillon_core::extplugin_rsjs_bridge::rsjs_bridge_core;
use sapphillon_core::plugin::CorePluginExternalPackage;
use sapphillon_core::runtime::OpStateWorkflowData;
use serde_json::json;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

fn get_fixture_path(filename: &str) -> PathBuf {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("tests");
    d.push("fixtures");
    d.push(filename);
    d
}

/// Creates OpState with workflow data containing the test external package from fixture
/// Returns the OpState and the tokio Runtime to keep the runtime alive
fn create_opstate_with_fixture(
    fixture_filename: &str,
    package_name: &str,
) -> (deno_core::OpState, tokio::runtime::Runtime) {
    use deno_core::OpState;

    let fixture_path = get_fixture_path(fixture_filename);
    let package_js = std::fs::read_to_string(fixture_path).expect("Failed to read fixture");

    let mut op_state = OpState::new(None);

    // Create the external package
    let package = CorePluginExternalPackage::new(
        format!("test.{}", package_name),
        package_name.to_string(),
        vec![], // functions list not needed for this test
        package_js,
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
/// 1. Load the `plugin_package.js` fixture into OpState.
/// 2. Construct `RsJsBridgeArgs` with the function name "add" and arguments `a=10`, `b=20`.
/// 3. Execute the bridge via `rsjs_bridge_core`.
/// 4. Parse the result and assert that the returned "result" is `30`.
#[test]
fn test_integration_math_plugin_add() {
    // 1. Create runtime with the plugin package
    let (mut op_state, _tokio_rt) = create_opstate_with_fixture("plugin_package.js", "math-plugin");

    // 2. Prepare arguments for the 'add' function
    let args = RsJsBridgeArgs {
        func_name: "add".to_string(),
        args: vec![("a".to_string(), json!(10)), ("b".to_string(), json!(20))]
            .into_iter()
            .collect(),
    };
    let args_json = args.to_string().unwrap();

    // 3. Execute the bridge
    let result = rsjs_bridge_core(&mut op_state, &args_json, "math-plugin");
    assert!(
        result.is_ok(),
        "Bridge execution failed: {:?}",
        result.err()
    );

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
/// 1. Load the `plugin_package.js` fixture into OpState.
/// 2. Construct `RsJsBridgeArgs` for "process_data" with a complex input object.
/// 3. Execute the bridge.
/// 4. Verify that the returned object contains the expected fields ("original", "result", "timestamp").
#[test]
fn test_integration_math_plugin_process_data() {
    // 1. Create runtime with the plugin package
    let (mut op_state, _tokio_rt) = create_opstate_with_fixture("plugin_package.js", "math-plugin");

    // 2. Prepare complex input data
    let input_data = json!({
        "value": 50,
        "multiplier": 2
    });

    let args = RsJsBridgeArgs {
        func_name: "process_data".to_string(),
        args: vec![("data".to_string(), input_data)].into_iter().collect(),
    };
    let args_json = args.to_string().unwrap();

    // 3. Execute the bridge
    let result = rsjs_bridge_core(&mut op_state, &args_json, "math-plugin");
    assert!(
        result.is_ok(),
        "Bridge execution failed: {:?}",
        result.err()
    );

    // 4. Verify the result structure and values
    let result_json = result.unwrap();
    let returns = RsJsBridgeReturns::new_from_str(&result_json).expect("Failed to parse returns");

    let result_obj = returns.args.get("result").expect("No result returned");

    assert_eq!(result_obj.get("original"), Some(&json!(50)));
    assert_eq!(result_obj.get("result"), Some(&json!(100)));
    // Verify timestamp exists (dynamic value, so we just check existence)
    assert!(result_obj.get("timestamp").is_some());
}

/// Integration Test: External Plugin Execution via CoreWorkflowCode
///
/// **Purpose:**
/// Verify that `CoreWorkflowCode` can correctly execute external plugin functions
/// by going through the full workflow execution flow.
///
/// **Intent:**
/// This test ensures that:
/// 1. `CoreWorkflowCode` correctly collects OpDecls from external plugin packages.
/// 2. Pre-run scripts (generated call wrappers) are executed before the main code.
/// 3. The JavaScript workflow code can call external plugin functions.
/// 4. The result is correctly captured in the workflow results.
///
/// **Flow:**
/// 1. Create a `CorePluginExternalPackage` with the `add` function from the fixture.
/// 2. Create a `CoreWorkflowCode` with workflow code that calls `mathPlugin.add(5, 7)`.
/// 3. Execute the workflow via `CoreWorkflowCode::run()`.
/// 4. Verify that the workflow result contains the expected output (`12`).
///
/// **Note:** This test is ignored by default due to V8 isolate limitations in test environments.
/// When the Deno test harness creates multiple V8 isolates concurrently (the workflow's JsRuntime
/// plus the external plugin's MainWorker), V8 may crash with assertion failures.
/// In production, workflows run sequentially, avoiding this issue.
/// Run with `cargo test -- --ignored` to execute.
#[test]
#[ignore]
fn test_integration_workflow_with_external_plugin_add() {
    use sapphillon_core::plugin::{CorePluginExternalFunction, CorePluginExternalPackage};
    use sapphillon_core::workflow::CoreWorkflowCode;

    let fixture_path = get_fixture_path("plugin_package.js");
    let package_js = std::fs::read_to_string(fixture_path).expect("Failed to read fixture");

    // Create external function for 'add'
    let add_func = CorePluginExternalFunction::new(
        "math-plugin-add".to_string(),
        "add".to_string(),
        "Adds two numbers".to_string(),
        "mathPlugin".to_string(),
        package_js.clone(),
    );

    // Create external package
    let ext_package = CorePluginExternalPackage::new(
        "test.math-plugin".to_string(),
        "mathPlugin".to_string(),
        vec![add_func],
        package_js,
    );

    // Create workflow code that calls the external plugin
    let workflow_code = r#"
        const result = mathPlugin.add(5, 7);
        console.log(result);
    "#;

    let mut code = CoreWorkflowCode::new(
        "test_wf_ext".to_string(),
        workflow_code.to_string(),
        vec![], // no internal plugins
        vec![ext_package],
        1,
        vec![], // allowed permissions
        vec![], // required permissions
    );

    // Run the workflow
    let tokio_runtime = tokio::runtime::Runtime::new().unwrap();
    code.run(tokio_runtime.handle().clone());

    // Verify the result
    assert_eq!(code.result.len(), 1);
    let res = &code.result[0];
    assert_eq!(
        res.exit_code, 0,
        "Workflow should succeed, but got error: {}",
        res.result
    );
    assert!(
        res.result.contains("12"),
        "Expected result to contain '12', got: {}",
        res.result
    );
}

/// Integration Test: External Plugin Complex Object via CoreWorkflowCode
///
/// **Purpose:**
/// Verify that `CoreWorkflowCode` can correctly handle complex object arguments
/// and return values when executing external plugin functions.
///
/// **Intent:**
/// This test ensures that:
/// 1. Complex JSON objects can be passed to external plugin functions.
/// 2. The plugin processes the data and returns a complex result.
/// 3. The workflow code can access and log the returned data.
///
/// **Flow:**
/// 1. Create a `CorePluginExternalPackage` with the `process_data` function.
/// 2. Create a `CoreWorkflowCode` with workflow code that calls `mathPlugin.process_data()`.
/// 3. Execute the workflow via `CoreWorkflowCode::run()`.
/// 4. Verify that the workflow result contains the expected processed data.
///
/// **Note:** This test is ignored by default due to V8 isolate limitations in test environments.
/// See `test_integration_workflow_with_external_plugin_add` for details.
/// Run with `cargo test -- --ignored` to execute.
#[test]
#[ignore]
fn test_integration_workflow_with_external_plugin_process_data() {
    use sapphillon_core::plugin::{CorePluginExternalFunction, CorePluginExternalPackage};
    use sapphillon_core::workflow::CoreWorkflowCode;

    let fixture_path = get_fixture_path("plugin_package.js");
    let package_js = std::fs::read_to_string(fixture_path).expect("Failed to read fixture");

    // Create external function for 'process_data'
    let process_func = CorePluginExternalFunction::new(
        "math-plugin-process-data".to_string(),
        "process_data".to_string(),
        "Processes data object".to_string(),
        "mathPlugin".to_string(),
        package_js.clone(),
    );

    // Create external package
    let ext_package = CorePluginExternalPackage::new(
        "test.math-plugin".to_string(),
        "mathPlugin".to_string(),
        vec![process_func],
        package_js,
    );

    // Create workflow code that calls the external plugin with complex data
    let workflow_code = r#"
        const data = { value: 25, multiplier: 4 };
        const result = mathPlugin.process_data(data);
        console.log("Original:", result.original);
        console.log("Result:", result.result);
    "#;

    let mut code = CoreWorkflowCode::new(
        "test_wf_ext_complex".to_string(),
        workflow_code.to_string(),
        vec![], // no internal plugins
        vec![ext_package],
        1,
        vec![], // allowed permissions
        vec![], // required permissions
    );

    // Run the workflow
    let tokio_runtime = tokio::runtime::Runtime::new().unwrap();
    code.run(tokio_runtime.handle().clone());

    // Verify the result
    assert_eq!(code.result.len(), 1);
    let res = &code.result[0];
    assert_eq!(
        res.exit_code, 0,
        "Workflow should succeed, but got error: {}",
        res.result
    );
    assert!(
        res.result.contains("Original: 25"),
        "Expected result to contain 'Original: 25', got: {}",
        res.result
    );
    assert!(
        res.result.contains("Result: 100"),
        "Expected result to contain 'Result: 100', got: {}",
        res.result
    );
}
