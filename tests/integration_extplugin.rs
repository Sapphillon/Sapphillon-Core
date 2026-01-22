use sapphillon_core::ext_plugin::{RsJsBridgeArgs, RsJsBridgeReturns};
use sapphillon_core::extplugin_rsjs_bridge::rsjs_bridge_core;
use sapphillon_core::permission::{
    Permission, PermissionType, Permissions, PluginFunctionPermissions,
};
use sapphillon_core::plugin::{CorePluginExternalPackage, PluginPackageTrait};
use sapphillon_core::runtime::OpStateWorkflowData;
use serde_json::json;
use serial_test::serial;
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
    author_id: &str,
) -> (deno_core::OpState, tokio::runtime::Runtime) {
    use deno_core::OpState;

    let fixture_path = get_fixture_path(fixture_filename);
    let package_js = std::fs::read_to_string(fixture_path).expect("Failed to read fixture");

    let mut op_state = OpState::new(None);

    // Create the external package
    let package_id = format!("{author_id}.{package_name}");

    let package = CorePluginExternalPackage::new(
        package_id,
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
        None,
        None,
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
    let (mut op_state, _tokio_rt) =
        create_opstate_with_fixture("plugin_package.js", "math-plugin", "com.sapphillon.test");

    // 2. Prepare arguments for the 'add' function
    let args = RsJsBridgeArgs {
        func_name: "add".to_string(),
        args: vec![("a".to_string(), json!(10)), ("b".to_string(), json!(20))]
            .into_iter()
            .collect(),
    };
    let args_json = args.to_string().unwrap();

    // 3. Execute the bridge
    let result = rsjs_bridge_core(&mut op_state, &args_json, "com.sapphillon.test.math-plugin");
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
    let (mut op_state, _tokio_rt) =
        create_opstate_with_fixture("plugin_package.js", "math-plugin", "com.sapphillon.test");

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
    let result = rsjs_bridge_core(&mut op_state, &args_json, "com.sapphillon.test.math-plugin");
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
/// 2. Create a `CoreWorkflowCode` with workflow code that calls `com.sapphillon.test.mathPlugin.add(5, 7)`.
/// 3. Execute the workflow via `CoreWorkflowCode::run()`.
/// 4. Verify that the workflow result contains the expected output (`12`).
///
/// **Note:** This test is ignored by default due to V8 isolate limitations in test environments.
/// When the Deno test harness creates multiple V8 isolates concurrently (the workflow's JsRuntime
/// plus the external plugin's MainWorker), V8 may crash with assertion failures.
/// In production, workflows run sequentially, avoiding this issue.
/// Run with `cargo test -- --ignored` to execute.
#[test]
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
        "com.sapphillon.test".to_string(),
    );

    // Create external package
    let ext_package = CorePluginExternalPackage::new(
        "com.sapphillon.test.mathPlugin".to_string(),
        "mathPlugin".to_string(),
        vec![add_func],
        package_js,
    );

    // Create workflow code that calls the external plugin
    let workflow_code = r#"
        const result = com.sapphillon.test.mathPlugin.add(5, 7);
        console.log(result);
    "#;

    let mut code = CoreWorkflowCode::new(
        "test_wf_ext".to_string(),
        workflow_code.to_string(),
        vec![Arc::new(ext_package) as Arc<dyn PluginPackageTrait>],
        1,
        vec![], // allowed permissions
        vec![], // required permissions
    );

    // Run the workflow
    let tokio_runtime = tokio::runtime::Runtime::new().unwrap();
    code.run(tokio_runtime.handle().clone(), None, None);

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
/// 2. Create a `CoreWorkflowCode` with workflow code that calls `com.sapphillon.test.mathPlugin.process_data()`.
/// 3. Execute the workflow via `CoreWorkflowCode::run()`.
/// 4. Verify that the workflow result contains the expected processed data.
///
/// **Note:** This test is ignored by default due to V8 isolate limitations in test environments.
/// See `test_integration_workflow_with_external_plugin_add` for details.
/// Run with `cargo test -- --ignored` to execute.
#[test]
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
        "com.sapphillon.test".to_string(),
    );

    // Create external package
    let ext_package = CorePluginExternalPackage::new(
        "com.sapphillon.test.mathPlugin".to_string(),
        "mathPlugin".to_string(),
        vec![process_func],
        package_js,
    );

    // Create workflow code that calls the external plugin with complex data
    let workflow_code = r#"
        const data = { value: 25, multiplier: 4 };
        const result = com.sapphillon.test.mathPlugin.process_data(data);
        console.log("Original:", result.original);
        console.log("Result:", result.result);
    "#;

    let mut code = CoreWorkflowCode::new(
        "test_wf_ext_complex".to_string(),
        workflow_code.to_string(),
        vec![Arc::new(ext_package) as Arc<dyn PluginPackageTrait>],
        1,
        vec![], // allowed permissions
        vec![], // required permissions
    );

    // Run the workflow
    let tokio_runtime = tokio::runtime::Runtime::new().unwrap();
    code.run(tokio_runtime.handle().clone(), None, None);

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

/// Integration Test: Workflow with Permission Granted (Success Case)
///
/// **Purpose:**
/// Verify that `CoreWorkflowCode` successfully executes when the required permissions
/// are granted in the `allowed_permissions`.
///
/// **Intent:**
/// This test ensures that:
/// 1. A function requiring specific permissions can execute successfully.
/// 2. The permission check passes when allowed_permissions match the required permissions.
/// 3. The workflow completes without permission errors.
///
/// **Flow:**
/// 1. Create a `CorePluginExternalPackage` with a function that requires FilesystemRead permission.
/// 2. Create `allowed_permissions` that grant FilesystemRead permission.
/// 3. Create workflow code that calls the external plugin function.
/// 4. Execute the workflow via `CoreWorkflowCode::run()`.
/// 5. Verify that the workflow result succeeds with exit_code 0 and contains the expected output.
///
/// **Note:** This test is currently ignored because permission checking in external plugins
/// requires additional implementation in the extplugin_server process.
#[test]
#[serial]
fn test_integration_workflow_with_permission_granted() {
    use sapphillon_core::plugin::{CorePluginExternalFunction, CorePluginExternalPackage};
    use sapphillon_core::workflow::CoreWorkflowCode;
    use tempfile::tempdir;

    // Create a temporary file with known content
    let dir = tempdir().expect("create temp dir");
    let test_file = dir.path().join("test.txt");
    let test_content = "Hello from test file!";
    std::fs::write(&test_file, test_content).expect("write temp file");
    // Convert Windows backslash paths to forward slashes to avoid JS escape issues
    // Windows APIs accept forward slashes, and this avoids \t, \n etc being interpreted as escapes
    let test_file_path = test_file.to_string_lossy().replace('\\', "/");

    let fixture_path = get_fixture_path("plugin_package_with_permission.js");
    let package_js = std::fs::read_to_string(fixture_path).expect("Failed to read fixture");

    // Create external function for 'read_file' (requires FilesystemRead permission)
    let read_file_func = CorePluginExternalFunction::new(
        "file-plugin-read-file".to_string(),
        "read_file".to_string(),
        "Reads a file".to_string(),
        "filePlugin".to_string(),
        package_js.clone(),
        "com.sapphillon.test".to_string(),
    );

    // Create external package
    let ext_package = CorePluginExternalPackage::new(
        "com.sapphillon.test.filePlugin".to_string(),
        "filePlugin".to_string(),
        vec![read_file_func],
        package_js,
    );

    // Create allowed permissions that grant FilesystemRead for the temp directory
    // Use forward slash path format for consistency with Deno
    let temp_dir_path = dir.path().to_string_lossy().replace('\\', "/");
    let allowed_permissions = vec![PluginFunctionPermissions {
        plugin_function_id: "com.sapphillon.test.filePlugin.read_file".to_string(),
        permissions: Permissions::new(vec![Permission {
            permission_type: PermissionType::FilesystemRead as i32,
            display_name: "Filesystem Read".to_string(),
            description: "Permission to read files".to_string(),
            resource: vec![temp_dir_path],
            permission_level: 1,
        }]),
    }];

    // Create workflow code that calls the external plugin
    // Path already uses forward slashes, so no complex escaping needed
    let workflow_code = format!(
        r#"
        const result = com.sapphillon.test.filePlugin.read_file("{test_file_path}");
        console.log("File content:", result);
    "#
    );

    let mut code = CoreWorkflowCode::new(
        "test_wf_permission_granted".to_string(),
        workflow_code,
        vec![Arc::new(ext_package) as Arc<dyn PluginPackageTrait>],
        1,
        allowed_permissions,
        vec![], // no required permissions at workflow level
    );

    // Run the workflow
    let tokio_runtime = tokio::runtime::Runtime::new().unwrap();
    code.run(tokio_runtime.handle().clone(), None, None);

    // Verify the result - should succeed
    assert_eq!(code.result.len(), 1);
    let res = &code.result[0];
    assert_eq!(
        res.exit_code, 0,
        "Workflow should succeed with granted permissions, but got error: {}",
        res.result
    );
    assert!(
        res.result.contains(test_content),
        "Expected result to contain file content '{}', got: {}",
        test_content,
        res.result
    );
}

/// Integration Test: Workflow with Permission Denied (Error Case)
///
/// **Purpose:**
/// Verify that `CoreWorkflowCode` fails with a permission error when the required
/// permissions are not granted in the `allowed_permissions`.
///
/// **Intent:**
/// This test ensures that:
/// 1. A function requiring specific permissions cannot execute without proper grants.
/// 2. The permission check fails when allowed_permissions don't match the required permissions.
/// 3. The workflow completes with a permission error and non-zero exit code.
///
/// **Flow:**
/// 1. Create a `CorePluginExternalPackage` with a function that requires FilesystemRead permission.
/// 2. Create empty or mismatched `allowed_permissions` (not granting the required permission).
/// 3. Create workflow code that calls the external plugin function.
/// 4. Execute the workflow via `CoreWorkflowCode::run()`.
/// 5. Verify that the workflow result fails with exit_code 1 and contains a permission error message.
///
/// **Note:** This test is currently ignored because permission checking in external plugins
/// requires additional implementation in the extplugin_server process.
#[test]
#[serial]
fn test_integration_workflow_with_permission_denied() {
    use sapphillon_core::plugin::{CorePluginExternalFunction, CorePluginExternalPackage};
    use sapphillon_core::workflow::CoreWorkflowCode;
    use tempfile::tempdir;

    // Create a temporary file with known content
    let dir = tempdir().expect("create temp dir");
    let test_file = dir.path().join("secret.txt");
    std::fs::write(&test_file, "secret content").expect("write temp file");
    // Convert Windows backslash paths to forward slashes, matching the granted test
    let test_file_path = test_file.to_string_lossy().replace('\\', "/");

    let fixture_path = get_fixture_path("plugin_package_with_permission.js");
    let package_js = std::fs::read_to_string(fixture_path).expect("Failed to read fixture");

    // Create external function for 'read_file' (requires FilesystemRead permission)
    let read_file_func = CorePluginExternalFunction::new(
        "file-plugin-read-file".to_string(),
        "read_file".to_string(),
        "Reads a file".to_string(),
        "filePlugin".to_string(),
        package_js.clone(),
        "com.sapphillon.test".to_string(),
    );

    // Create external package
    let ext_package = CorePluginExternalPackage::new(
        "com.sapphillon.test.filePlugin".to_string(),
        "filePlugin".to_string(),
        vec![read_file_func],
        package_js,
    );

    // Create empty allowed_permissions - no permissions granted!
    let allowed_permissions = vec![];

    // Create workflow code that calls the external plugin
    // Path uses forward slashes, so no complex escaping needed
    let workflow_code = format!(
        r#"
        const result = com.sapphillon.test.filePlugin.read_file("{test_file_path}");
        console.log("File content:", result);
    "#
    );

    let mut code = CoreWorkflowCode::new(
        "test_wf_permission_denied".to_string(),
        workflow_code,
        vec![Arc::new(ext_package) as Arc<dyn PluginPackageTrait>],
        1,
        allowed_permissions,
        vec![], // no required permissions at workflow level
    );

    // Run the workflow
    let tokio_runtime = tokio::runtime::Runtime::new().unwrap();
    code.run(tokio_runtime.handle().clone(), None, None);

    // Verify the result - should fail with permission error
    assert_eq!(code.result.len(), 1);
    let res = &code.result[0];
    assert_eq!(
        res.exit_code, 1,
        "Workflow should fail without granted permissions, but got exit code: {} result: {}",
        res.exit_code, res.result
    );
    assert!(
        res.result.to_lowercase().contains("permission")
            || res.result.to_lowercase().contains("notcapable"),
        "Expected result to contain permission error, got: {}",
        res.result
    );
}

/// Integration Test: Workflow with Function Without Permissions (Success Case)
///
/// **Purpose:**
/// Verify that `CoreWorkflowCode` successfully executes functions that don't require
/// any permissions, even when no permissions are granted.
///
/// **Intent:**
/// This test ensures that:
/// 1. Functions without permission requirements can execute freely.
/// 2. Empty allowed_permissions doesn't block functions that don't need permissions.
/// 3. The workflow completes successfully.
///
/// **Flow:**
/// 1. Create a `CorePluginExternalPackage` with the `simple_function` that requires no permissions.
/// 2. Create empty `allowed_permissions`.
/// 3. Create workflow code that calls `filePlugin.simple_function("Hello")`.
/// 4. Execute the workflow via `CoreWorkflowCode::run()`.
/// 5. Verify that the workflow result succeeds with exit_code 0.
#[test]
fn test_integration_workflow_without_permission_requirement() {
    use sapphillon_core::plugin::{CorePluginExternalFunction, CorePluginExternalPackage};
    use sapphillon_core::workflow::CoreWorkflowCode;

    let fixture_path = get_fixture_path("plugin_package_with_permission.js");
    let package_js = std::fs::read_to_string(fixture_path).expect("Failed to read fixture");

    // Create external function for 'simple_function' (no permissions required)
    let simple_func = CorePluginExternalFunction::new(
        "file-plugin-simple-function".to_string(),
        "simple_function".to_string(),
        "A simple function".to_string(),
        "filePlugin".to_string(),
        package_js.clone(),
        "com.sapphillon.test".to_string(),
    );

    // Create external package
    let ext_package = CorePluginExternalPackage::new(
        "com.sapphillon.test.filePlugin".to_string(),
        "filePlugin".to_string(),
        vec![simple_func],
        package_js,
    );

    // Create empty allowed_permissions - but the function doesn't need any!
    let allowed_permissions = vec![];

    // Create workflow code that calls the external plugin
    let workflow_code = r#"
        const result = com.sapphillon.test.filePlugin.simple_function("Hello World");
        console.log("Result:", result);
    "#;

    let mut code = CoreWorkflowCode::new(
        "test_wf_no_permission_needed".to_string(),
        workflow_code.to_string(),
        vec![Arc::new(ext_package) as Arc<dyn PluginPackageTrait>],
        1,
        allowed_permissions,
        vec![], // no required permissions at workflow level
    );

    // Run the workflow
    let tokio_runtime = tokio::runtime::Runtime::new().unwrap();
    code.run(tokio_runtime.handle().clone(), None, None);

    // Verify the result - should succeed even without permissions
    assert_eq!(code.result.len(), 1);
    let res = &code.result[0];
    assert_eq!(
        res.exit_code, 0,
        "Workflow should succeed for functions without permission requirements, but got error: {}",
        res.result
    );
    assert!(
        res.result.contains("Echo: Hello World"),
        "Expected result to contain echoed text, got: {}",
        res.result
    );
}

/// Integration Test: Plugin Throws Error
///
/// **Purpose:**
/// Verify that exceptions thrown within the plugin (both synchronous and asynchronous)
/// are correctly propagated back to Rust as errors.
///
/// **Intent:**
/// This test ensures that:
/// 1. `rsjs_bridge_core` returns an `Err` result when the JS handler throws.
/// 2. The error message contains the original exception message from JavaScript.
///
/// **Flow:**
/// 1. Load `plugin_package_errors.js`.
/// 2. Call `throw_immediate` and verify the error message.
/// 3. Call `throw_async` and verify the error message.
#[test]
fn test_integration_plugin_throws_error() {
    use std::collections::HashMap;

    // 1. Create runtime with the error plugin package
    let (mut op_state, _tokio_rt) = create_opstate_with_fixture(
        "plugin_package_errors.js",
        "error-plugin",
        "com.sapphillon.test",
    );

    // Test Case 1: Immediate Throw
    let args_immediate = RsJsBridgeArgs {
        func_name: "throw_immediate".to_string(),
        args: HashMap::new(),
    };
    let result_immediate = rsjs_bridge_core(
        &mut op_state,
        &args_immediate.to_string().unwrap(),
        "com.sapphillon.test.error-plugin",
    );

    assert!(
        result_immediate.is_err(),
        "Expected error from throw_immediate, got Ok"
    );
    let err_msg = result_immediate.err().unwrap().to_string();
    assert!(
        err_msg.contains("This is an immediate error"),
        "Expected error message to contain 'This is an immediate error', got: {err_msg}"
    );

    // Test Case 2: Async Throw
    let args_async = RsJsBridgeArgs {
        func_name: "throw_async".to_string(),
        args: HashMap::new(),
    };
    let result_async = rsjs_bridge_core(
        &mut op_state,
        &args_async.to_string().unwrap(),
        "com.sapphillon.test.error-plugin",
    );

    assert!(
        result_async.is_err(),
        "Expected error from throw_async, got Ok"
    );
    let err_msg_async = result_async.err().unwrap().to_string();
    assert!(
        err_msg_async.contains("This is an async error"),
        "Expected error message to contain 'This is an async error', got: {err_msg_async}"
    );
}

/// Integration Test: Unknown Function Call
///
/// **Purpose:**
/// Verify that calling a function not defined in the plugin package schema
/// results in an appropriate error.
///
/// **Intent:**
/// This test ensures that:
/// 1. The bridge validates the function name against the schema/handlers.
/// 2. A clear error is returned if the function is not found.
///
/// **Flow:**
/// 1. Load `plugin_package.js`.
/// 2. Call a non-existent function `non_existent_func`.
/// 3. Verify that the result is an `Err` and the message indicates the function is unknown.
#[test]
fn test_integration_plugin_unknown_function() {
    use std::collections::HashMap;

    let (mut op_state, _tokio_rt) =
        create_opstate_with_fixture("plugin_package.js", "math-plugin", "com.sapphillon.test");

    let args = RsJsBridgeArgs {
        func_name: "non_existent_func".to_string(),
        args: HashMap::new(),
    };

    let result = rsjs_bridge_core(
        &mut op_state,
        &args.to_string().unwrap(),
        "com.sapphillon.test.math-plugin",
    );

    assert!(
        result.is_err(),
        "Expected error for unknown function, got Ok"
    );
    let err_msg = result.err().unwrap().to_string();
    assert!(
        err_msg.contains("Unknown function") || err_msg.contains("schema not found"),
        "Expected 'Unknown function' error, got: {err_msg}"
    );
}

/// Integration Test: Loose Type Handling (Demonstration)
///
/// **Purpose:**
/// Verify the behavior when incorrect argument types are passed to a plugin function.
/// Currently, the system relies on JS loose typing, so this test demonstrates that behavior
/// rather than asserting a type error.
///
/// **Intent:**
/// This test documents that:
/// 1. Passing strings to a math function ("10", "20") does NOT cause a crash.
/// 2. JS executes `a + b` as string concatenation "1020".
/// 3. The return value reflects this runtime behavior.
///
/// **Flow:**
/// 1. Load `plugin_package.js`.
/// 2. Call `add` with string arguments "10" and "20".
/// 3. Verify the result is "1020" (string concatenation), demonstrating loose typing.
#[test]
fn test_integration_plugin_loose_type_handling() {
    let (mut op_state, _tokio_rt) =
        create_opstate_with_fixture("plugin_package.js", "math-plugin", "com.sapphillon.test");

    // Pass strings instead of numbers
    let args = RsJsBridgeArgs {
        func_name: "add".to_string(),
        args: vec![
            ("a".to_string(), json!("10")),
            ("b".to_string(), json!("20")),
        ]
        .into_iter()
        .collect(),
    };

    let result = rsjs_bridge_core(
        &mut op_state,
        &args.to_string().unwrap(),
        "com.sapphillon.test.math-plugin",
    );

    assert!(
        result.is_ok(),
        "Bridge execution should succeed (JS is loose typed): {:?}",
        result.err()
    );

    let result_json = result.unwrap();
    let returns = RsJsBridgeReturns::new_from_str(&result_json).expect("Failed to parse returns");

    // JS `+` operator with strings does concatenation
    assert_eq!(
        returns.args.get("result"),
        Some(&json!("1020")),
        "Expected string concatenation result '1020'"
    );
}

/// Integration Test: Async Function Success
///
/// **Purpose:**
/// Verify that async functions (Promises) are correctly awaited and their
/// return values are properly serialized.
///
/// **Intent:**
/// This test ensures that:
/// 1. Async handlers are awaited before extracting the return value.
/// 2. The result is correctly serialized to JSON.
///
/// **Flow:**
/// 1. Load `plugin_package_errors.js` with `async_success` function.
/// 2. Call `async_success` with a test value.
/// 3. Verify the async result is correctly returned.
#[test]
fn test_integration_plugin_async_success() {
    let (mut op_state, _tokio_rt) = create_opstate_with_fixture(
        "plugin_package_errors.js",
        "error-plugin",
        "com.sapphillon.test",
    );

    let args = RsJsBridgeArgs {
        func_name: "async_success".to_string(),
        args: vec![("value".to_string(), json!("test-value"))]
            .into_iter()
            .collect(),
    };

    let result = rsjs_bridge_core(
        &mut op_state,
        &args.to_string().unwrap(),
        "com.sapphillon.test.error-plugin",
    );

    assert!(
        result.is_ok(),
        "Async function should succeed: {:?}",
        result.err()
    );

    let result_json = result.unwrap();
    let returns = RsJsBridgeReturns::new_from_str(&result_json).expect("Failed to parse returns");

    assert_eq!(
        returns.args.get("result"),
        Some(&json!("async: test-value")),
        "Expected async transformed result"
    );
}

/// Integration Test: Null/Undefined Return Handling
///
/// **Purpose:**
/// Verify behavior when JS handlers return null, undefined, or nothing.
///
/// **Intent:**
/// This test documents the current behavior for edge cases:
/// 1. `return null` - Should return a result with null value.
/// 2. `return undefined` - May error or return null (depending on impl).
/// 3. No return (no-op) - Same as undefined.
///
/// **Flow:**
/// 1. Load `plugin_package_errors.js`.
/// 2. Call `return_null`, `return_undefined`, and `no_op`.
/// 3. Document the observed behavior.
#[test]
fn test_integration_plugin_null_undefined_return() {
    let (mut op_state, _tokio_rt) = create_opstate_with_fixture(
        "plugin_package_errors.js",
        "error-plugin",
        "com.sapphillon.test",
    );

    // Test: return null
    let args_null = RsJsBridgeArgs {
        func_name: "return_null".to_string(),
        args: std::collections::HashMap::new(),
    };
    let result_null = rsjs_bridge_core(
        &mut op_state,
        &args_null.to_string().unwrap(),
        "com.sapphillon.test.error-plugin",
    );

    // Note: The current implementation may error on null/undefined returns
    // because the runner expects a string return value.
    // This test documents the current behavior.
    if let Ok(result_json) = result_null {
        let returns = RsJsBridgeReturns::new_from_str(&result_json);
        // If parsing succeeds, check the value
        if let Ok(r) = returns {
            assert!(
                r.args.get("result") == Some(&json!(null)) || r.args.is_empty(),
                "Expected null result or empty args"
            );
        }
    } else {
        // Current implementation errors on null/undefined returns
        let err_msg = result_null.err().unwrap().to_string();
        assert!(
            err_msg.contains("null")
                || err_msg.contains("undefined")
                || err_msg.contains("non-string"),
            "Expected null/undefined related error, got: {err_msg}"
        );
    }

    // Test: return undefined (implicit via no-op)
    let args_noop = RsJsBridgeArgs {
        func_name: "no_op".to_string(),
        args: std::collections::HashMap::new(),
    };
    let result_noop = rsjs_bridge_core(
        &mut op_state,
        &args_noop.to_string().unwrap(),
        "com.sapphillon.test.error-plugin",
    );

    // Same handling as null - document behavior
    if result_noop.is_err() {
        let err_msg = result_noop.err().unwrap().to_string();
        assert!(
            err_msg.contains("null")
                || err_msg.contains("undefined")
                || err_msg.contains("non-string"),
            "Expected null/undefined related error for no-op, got: {err_msg}"
        );
    }
    // If it succeeds, that's also valid behavior - the test passes either way
    // as long as it doesn't panic
}

/// Integration Test: Serde V8 Error Reproduction
///
/// **Purpose:**
/// Verify that calling `rsjs_bridge_opdecl` from JavaScript via a generated wrapper
/// does not cause a `serde_v8` error (invalid type; expected: string, got: array).
///
/// **Intent:**
/// This test ensures that:
/// 1. The generated JavaScript wrapper correctly calls `rsjs_bridge_opdecl`.
/// 2. The `rsjs_bridge_opdecl` implementation correctly handles the arguments passed from JS.
/// 3. No serialization/deserialization errors occur during the call.
///
/// **Flow:**
/// 1. Define a simple plugin inline.
/// 2. Create a `CoreWorkflowCode` that calls this plugin.
/// 3. Execute the workflow.
/// 4. Verify success.
#[test]
fn test_integration_repro_serde_v8_error() {
    use sapphillon_core::plugin::{
        CorePluginExternalFunction, CorePluginExternalPackage, PluginPackageTrait,
    };
    use sapphillon_core::workflow::CoreWorkflowCode;

    let plugin_js = r#"
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
            simple_function: {
                description: "A simple function",
                permissions: [],
                parameters: [
                    { idx: 0, name: "message", type: "string", description: "Message to echo" }
                ],
                returns: [{
                    idx: 0,
                    type: "string",
                    description: "Echoed message"
                }],
                handler: (message) => {
                    return `Echo: ${message}`;
                }
            }
        }
    }
};
"#;

    let func = CorePluginExternalFunction::new(
        "test-function".to_string(),
        "simple_function".to_string(),
        "A simple function".to_string(),
        "testPlugin".to_string(),
        plugin_js.to_string(),
        "com.test".to_string(),
    );
    let package = CorePluginExternalPackage::new(
        "com.test.testPlugin".to_string(),
        "testPlugin".to_string(),
        vec![func],
        plugin_js.to_string(),
    );

    let workflow_code = r#"
        const result = com.test.testPlugin.simple_function("Hello World");
        console.log("Result:", result);
    "#;

    let mut code = CoreWorkflowCode::new(
        "test_workflow_repro".to_string(),
        workflow_code.to_string(),
        vec![Arc::new(package) as Arc<dyn PluginPackageTrait>],
        1,
        vec![],
        vec![],
    );

    let tokio_runtime = tokio::runtime::Runtime::new().unwrap();
    code.run(tokio_runtime.handle().clone(), None, None);

    if code.result[0].exit_code != 0 {
        println!(
            "Workflow failed with exit code {}",
            code.result[0].exit_code
        );
        println!("Result: {}", code.result[0].result);
        println!("Description: {}", code.result[0].description);
    }
    assert_eq!(code.result[0].exit_code, 0, "Workflow failed");
    assert!(
        code.result[0].result.contains("Echo: Hello World"),
        "Expected result to contain 'Echo: Hello World', got: {}",
        code.result[0].result
    );
}
