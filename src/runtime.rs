// Sapphillon-Core
// SPDX-FileCopyrightText: 2025 Yuta Takahashi
// SPDX-License-Identifier: MPL-2.0 OR GPL-3.0-or-later

#![warn(clippy::field_reassign_with_default)]

use crate::core::op_print_wrapper;
use crate::error::{
    Error as SapphillonError, PermissionDeniedError, WorkflowRuntimeError, WorkflowRuntimeErrorType,
};
use crate::permission::{
    CheckPermissionResult, Permissions, PluginFunctionPermissions, check_permission,
};

use deno_core::{Extension, JsRuntime, OpDecl, RuntimeOptions, error::JsError};
use std::boxed::Box;
use std::sync::{Arc, Mutex};

/// Represents the standard output (stdout) of a workflow execution.
/// Each variant holds the output as a string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkflowStdout {
    Stdout(String),
    //    Stderr(String),
}

/// Stores workflow-related state for operations within the runtime.
/// Includes workflow ID, captured stdout results, and a flag for capturing stdout.
#[derive(Debug, Clone)]
pub struct OpStateWorkflowData {
    workflow_id: String,
    result: Vec<WorkflowStdout>,
    capture_stdout: bool,
    // Support multiple plugin-function permission entries for allowed/required.
    allowed_permissions: Option<Vec<PluginFunctionPermissions>>,
    required_permissions: Option<Vec<PluginFunctionPermissions>>,
}

impl OpStateWorkflowData {
    /// Creates a new `OpStateWorkflowData` instance with the specified workflow ID and stdout capture flag.
    pub fn new(
        workflow_id: &str,
        capture_stdout: bool,
        allowed_permissions: Option<Vec<PluginFunctionPermissions>>,
        required_permissions: Option<Vec<PluginFunctionPermissions>>,
    ) -> Self {
        Self {
            workflow_id: workflow_id.to_string(),
            result: Vec::new(),
            capture_stdout,
            allowed_permissions,
            required_permissions,
        }
    }

    /// Returns a reference to the workflow ID.
    pub fn get_workflow_id(&self) -> &str {
        &self.workflow_id
    }

    /// Adds a `WorkflowStdout` result to the results vector if capturing stdout is enabled.
    pub fn add_result(&mut self, stdout: WorkflowStdout) {
        if self.capture_stdout {
            self.result.push(stdout);
        }
    }

    /// Returns a reference to the vector of captured `WorkflowStdout` results.
    pub fn get_results(&self) -> &Vec<WorkflowStdout> {
        &self.result
    }

    /// Returns true if capturing stdout is enabled.
    pub fn is_capture_stdout(&self) -> bool {
        self.capture_stdout
    }

    pub fn stdout_to_string(&self) -> String {
        self.result
            .iter()
            .map(|r| match r {
                WorkflowStdout::Stdout(s) => s.clone(),
                // WorkflowStdout::Stderr(s) => s.clone(),
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
    pub fn get_allowed_permissions(&self) -> &Option<Vec<PluginFunctionPermissions>> {
        &self.allowed_permissions
    }

    pub fn get_required_permissions(&self) -> &Option<Vec<PluginFunctionPermissions>> {
        &self.required_permissions
    }
}

/// Executes a JavaScript script in a Deno `JsRuntime`.
///
/// This function sets up a `JsRuntime` with a custom extension that provides the core functionalities
/// for the workflow execution environment. It can also execute pre-run scripts and manage workflow-specific
/// state data.
///
/// # Arguments
///
/// * `script` - The main JavaScript code to execute.
/// * `ext_func` - A vector of `OpDecl`s that define the native functions available to the JavaScript runtime.
/// * `workflow_data` - An optional, shareable `OpStateWorkflowData` that holds state across `op` calls.
/// * `pre_script` - An optional vector of JavaScript code snippets to execute before the main script.
///
/// # Returns
///
/// * `Ok(Arc<Mutex<OpStateWorkflowData>>)` - On successful execution, returns the (potentially modified)
///   workflow data.
/// * `Err(Box<JsError>)` - If any JavaScript error occurs during execution.
#[allow(unused)]
pub(crate) fn run_script(
    script: &str,
    ext_func: Vec<OpDecl>,
    workflow_data: Option<Arc<Mutex<OpStateWorkflowData>>>,
    pre_script: Option<Vec<String>>,
) -> Result<Arc<Mutex<OpStateWorkflowData>>, Box<SapphillonError>> {
    // Register the extension with the provided operations
    let extension = Extension {
        name: "ext",
        ops: std::borrow::Cow::Owned(ext_func),
        middleware_fn: Some(Box::new(|op| match op.name {
            "op_print" => op_print_wrapper(),
            _ => op,
        })),
        ..Default::default()
    };

    let mut extensions = vec![extension];

    // Create a new JsRuntime with the extension
    let mut runtime = JsRuntime::new(RuntimeOptions {
        extensions,
        ..Default::default()
    });

    let mut data: Arc<Mutex<OpStateWorkflowData>>;
    match workflow_data {
        Some(d) => data = d,
        None => {
            // If no workflow data is provided, create a default one
            data = Arc::new(Mutex::new(OpStateWorkflowData::new(
                "default_workflow",
                false,
                None,
                None,
            )));
        }
    }
    runtime.op_state().borrow_mut().put(data.clone());

    // Check Permission
    // Use the workflow data that was placed into `op_state` above (`data`) to determine
    // what permissions are allowed for this runtime. Avoid using the original
    // `workflow_data` variable because it was moved into `data`.
    // Build allowed and required lists (per plugin-function) from opstate.
    let allowed_list: Vec<PluginFunctionPermissions> = {
        let guard = data.lock().unwrap();
        guard.get_allowed_permissions().clone().unwrap_or_default()
    };

    let required_list: Vec<PluginFunctionPermissions> = {
        let guard = data.lock().unwrap();
        guard.get_required_permissions().clone().unwrap_or_default()
    };

    // For each required plugin-function, merge all allowed entries with the same id
    // and perform a permission check. If no allowed entries exist for a required id,
    // treat it as missing permission (no fallback).
    for req in required_list.iter() {
        // collect allowed entries matching the plugin_function_id
        let matched_allowed: Vec<PluginFunctionPermissions> = allowed_list
            .iter()
            .filter(|a| a.plugin_function_id == req.plugin_function_id)
            .cloned()
            .collect();

        if matched_allowed.is_empty() {
            return Err(Box::new(SapphillonError::PermissionDeniedError(
                PermissionDeniedError {
                    requested: req.permissions.clone(),
                    granted: Permissions::new(vec![]),
                },
            )));
        }

        // Merge the permissions from matched_allowed into a single Permissions value
        let mut merged_vec: Vec<_> = Vec::new();
        for a in matched_allowed {
            merged_vec.extend(a.permissions.permissions.clone());
        }
        let merged_allowed = Permissions::new(merged_vec);

        let perm_check_result = check_permission(&merged_allowed, &req.permissions);
        match perm_check_result {
            CheckPermissionResult::Ok => {}
            CheckPermissionResult::MissingPermission(_missing) => {
                return Err(Box::new(SapphillonError::PermissionDeniedError(
                    PermissionDeniedError {
                        requested: req.permissions.clone(),
                        granted: merged_allowed,
                    },
                )));
            }
        }
    }

    // Execute pre-run scripts if provided from core plugins
    if let Some(scripts) = pre_script {
        let pre_run_script = scripts.join("\n");
        runtime
            .execute_script("pre_script.js", pre_run_script)
            .map_err(|e: Box<JsError>| {
                Box::new(SapphillonError::WorkflowRuntimeError(
                    WorkflowRuntimeError {
                        message: "Failed to execute pre_script".to_string(),
                        error_type: WorkflowRuntimeErrorType::CorePluginPrepareError,
                        js_error: e,
                    },
                ))
            })?;
    }

    // Execute the provided script in the runtime
    runtime
        .execute_script("workflow.js", script.to_string())
        .map_err(|e: Box<JsError>| {
            Box::new(SapphillonError::WorkflowRuntimeError(
                WorkflowRuntimeError {
                    message: "Failed to execute workflow script".to_string(),
                    error_type: WorkflowRuntimeErrorType::WorkflowScriptExecuteError,
                    js_error: e,
                },
            ))
        })?;

    Ok(data)
}

#[cfg(test)]
mod tests {

    use super::*;
    use deno_core::{OpState, op2};

    #[test]
    fn test_extension() {
        #[op2]
        fn test_op(#[serde] a: Vec<i32>) -> i32 {
            a.iter().sum()
        }

        let script = r#"
        console.log("Hello World! From Sapphillon Runtime! with JavaScript and Deno!");
        console.log("Sum of [1, 2, 3, 4, 5]", Deno.core.ops.test_op([1, 2, 3, 4, 5]));
        "#;

        let result = run_script(script, vec![test_op()], None, None);
        println!("[test_extension] result: {result:?}");
    }

    #[test]
    fn test_run_script() {
        let script = "1 + 1;";

        let result = run_script(script, vec![], None, None);
        assert!(result.is_ok(), "Script should run successfully");
    }
    #[test]
    fn test_run_script_hello() {
        let script = "a = 1 + 1; console.log('Hello, world!');console.log(a);";

        let result = run_script(script, vec![], None, None);
        assert!(result.is_ok(), "Script should run successfully");
    }

    #[test]
    fn test_run_script_opstate_workflow_data() {
        // テスト用op: opstateからworkflow_idを取得
        #[op2]
        #[string]
        fn get_workflow_id(state: &mut OpState) -> String {
            let data = state
                .borrow::<Arc<Mutex<OpStateWorkflowData>>>()
                .lock()
                .unwrap();
            data.workflow_id.clone()
        }
        use std::sync::{Arc, Mutex};

        // テスト用workflow_dataを生成
        let workflow_data = OpStateWorkflowData {
            workflow_id: "test_id_123".to_string(),
            result: vec![],
            capture_stdout: false,
            allowed_permissions: None,
            required_permissions: None,
        };
        let workflow_data_arc = Arc::new(Mutex::new(workflow_data.clone()));

        // JSスクリプトでopを呼び出し
        let script = r#"
            let id = Deno.core.ops.get_workflow_id();
            console.log("Workflow ID:", id);
            if (id !== "test_id_123") {
                throw new Error("workflow_id not injected into opstate!");
            }
        "#;

        let result = run_script(
            script,
            vec![get_workflow_id()],
            Some(workflow_data_arc),
            None,
        );
        assert!(
            result.is_ok(),
            "workflow_id should be accessible from opstate"
        );
    }

    #[test]
    fn test_run_script_change_opstate_workflow_data() {
        // テスト用op: opstateからworkflow_idを取得
        #[op2]
        #[string]
        fn add_stdout(state: &mut OpState) -> String {
            let mut data = state
                .borrow_mut::<Arc<Mutex<OpStateWorkflowData>>>()
                .lock()
                .unwrap();
            data.add_result(WorkflowStdout::Stdout("Test stdout".to_string()));
            data.workflow_id.clone()
        }
        use std::sync::{Arc, Mutex};

        // テスト用workflow_dataを生成
        let workflow_data = OpStateWorkflowData {
            workflow_id: "test_id_123".to_string(),
            result: vec![WorkflowStdout::Stdout("Initial stdout".to_string())],
            capture_stdout: true,
            allowed_permissions: None,
            required_permissions: None,
        };
        let workflow_data_arc = Arc::new(Mutex::new(workflow_data.clone()));

        // JSスクリプトでopを呼び出し
        let script = r#"
            Deno.core.ops.add_stdout();
        "#;

        let result = run_script(
            script,
            vec![add_stdout()],
            Some(workflow_data_arc.clone()),
            None,
        );
        assert!(
            result.is_ok(),
            "workflow_id should be accessible from opstate"
        );

        let expected = vec![
            WorkflowStdout::Stdout("Initial stdout".to_string()),
            WorkflowStdout::Stdout("Test stdout".to_string()),
        ];

        // Check if the result was added to the workflow_data
        let data = workflow_data_arc.lock().unwrap();
        assert_eq!(
            data.get_results(),
            &expected,
            "Results should match expected output"
        );
    }

    #[test]
    fn test_run_script_capture_stdout() {
        use std::sync::{Arc, Mutex};

        // テスト用workflow_dataを生成
        let workflow_data = OpStateWorkflowData {
            workflow_id: "test_id_123".to_string(),
            result: vec![],
            capture_stdout: true,
            allowed_permissions: None,
            required_permissions: None,
        };
        let workflow_data_arc = Arc::new(Mutex::new(workflow_data.clone()));

        // JSスクリプトでopを呼び出し
        let script = r#"
            console.log("Initial stdout");
            console.log("Test stdout");
        "#;

        let result = run_script(script, vec![], Some(workflow_data_arc.clone()), None);
        assert!(
            result.is_ok(),
            "workflow_id should be accessible from opstate"
        );

        let expected = vec![
            WorkflowStdout::Stdout("Initial stdout\n".to_string()),
            WorkflowStdout::Stdout("Test stdout\n".to_string()),
        ];

        // Check if the result was added to the workflow_data
        let data = workflow_data_arc.lock().unwrap();
        assert_eq!(
            data.get_results(),
            &expected,
            "Results should match expected output"
        );
    }

    // New unit tests for stdout_to_string()
    #[test]
    fn test_stdout_to_string_empty() {
        let data = OpStateWorkflowData {
            workflow_id: "w".to_string(),
            result: vec![],
            capture_stdout: true,
            allowed_permissions: None,
            required_permissions: None,
        };
        assert_eq!(data.stdout_to_string(), "");
    }

    #[test]
    fn test_stdout_to_string_single() {
        let data = OpStateWorkflowData {
            workflow_id: "w".to_string(),
            result: vec![WorkflowStdout::Stdout("Hello".to_string())],
            capture_stdout: true,
            allowed_permissions: None,
            required_permissions: None,
        };
        assert_eq!(data.stdout_to_string(), "Hello");
    }

    #[test]
    fn test_stdout_to_string_multiple() {
        let data = OpStateWorkflowData {
            workflow_id: "w".to_string(),
            result: vec![
                WorkflowStdout::Stdout("One".to_string()),
                WorkflowStdout::Stdout("Two".to_string()),
                WorkflowStdout::Stdout("Three".to_string()),
            ],
            capture_stdout: true,
            allowed_permissions: None,
            required_permissions: None,
        };
        assert_eq!(data.stdout_to_string(), "One\nTwo\nThree");
    }
    #[test]
    fn test_run_script_capture_stdout_from_return() {
        use std::sync::{Arc, Mutex};

        // テスト用workflow_dataを生成
        let workflow_data = OpStateWorkflowData {
            workflow_id: "test_id_123".to_string(),
            result: vec![],
            capture_stdout: true,
            allowed_permissions: None,
            required_permissions: None,
        };
        let workflow_data_arc = Arc::new(Mutex::new(workflow_data.clone()));

        // JSスクリプトでopを呼び出し
        let script = r#"
            console.log("Initial stdout");
            console.log("Test stdout");
        "#;

        let result = run_script(script, vec![], Some(workflow_data_arc.clone()), None);
        assert!(
            result.is_ok(),
            "workflow_id should be accessible from opstate"
        );

        let expected = vec![
            WorkflowStdout::Stdout("Initial stdout\n".to_string()),
            WorkflowStdout::Stdout("Test stdout\n".to_string()),
        ];

        // Check if the result was added to the workflow_data
        assert_eq!(
            result.unwrap().lock().unwrap().get_results(),
            &expected,
            "Results should match expected output"
        );
    }
    #[test]
    fn test_run_pre_script() {
        use std::sync::{Arc, Mutex};

        // テスト用workflow_dataを生成
        let workflow_data = OpStateWorkflowData {
            workflow_id: "test_id_123".to_string(),
            result: vec![],
            capture_stdout: true,
            allowed_permissions: None,
            required_permissions: None,
        };
        let workflow_data_arc = Arc::new(Mutex::new(workflow_data.clone()));

        let pre_script_1 = "console.log('Pre script 1 executed');".to_string();
        let pre_script_2 = r#"
            function test_38() {
                return 38;
            }
            globalThis.test_38 = test_38;
        "#
        .to_string();

        // JSスクリプトでopを呼び出し
        let script = r#"
            console.log(test_38());
        "#;

        let result = run_script(
            script,
            vec![],
            Some(workflow_data_arc.clone()),
            Some(vec![pre_script_1, pre_script_2]),
        );
        assert!(
            result.is_ok(),
            "workflow_id should be accessible from opstate"
        );

        let expected = vec![
            WorkflowStdout::Stdout("Pre script 1 executed\n".to_string()),
            WorkflowStdout::Stdout("38\n".to_string()),
        ];

        // Check if the result was added to the workflow_data
        assert_eq!(
            result.unwrap().lock().unwrap().get_results(),
            &expected,
            "Results should match expected output"
        );
    }
    #[test]
    fn test_run_script_with_pre_and_workflow_success_simple() {
        use std::sync::{Arc, Mutex};

        // Prepare workflow_data that captures stdout
        let workflow_data = OpStateWorkflowData::new("wid_simple", true, None, None);
        let workflow_data_arc = Arc::new(Mutex::new(workflow_data));

        // Pre-script lines (will be joined with "\n")
        let pre1 = "console.log('pre-run');".to_string();
        let pre2 = "globalThis._val = 123;".to_string();

        // Workflow script uses value set by pre-script
        let script = r#"
            console.log(globalThis._val);
        "#;

        let res = run_script(
            script,
            vec![],
            Some(workflow_data_arc.clone()),
            Some(vec![pre1, pre2]),
        );
        assert!(
            res.is_ok(),
            "Expected run_script to succeed when pre and workflow are valid"
        );

        let results = res.unwrap().lock().unwrap().get_results().clone();
        // Expect pre-run output first, then workflow output
        assert_eq!(results.len(), 2);
        assert_eq!(results[0], WorkflowStdout::Stdout("pre-run\n".to_string()));
        assert_eq!(results[1], WorkflowStdout::Stdout("123\n".to_string()));
    }

    #[test]
    fn test_run_script_no_pre_script_simple() {
        use std::sync::{Arc, Mutex};

        let workflow_data = OpStateWorkflowData::new("wid_no_pre", true, None, None);
        let workflow_data_arc = Arc::new(Mutex::new(workflow_data));

        let script = r#"console.log('only workflow');"#;

        let res = run_script(script, vec![], Some(workflow_data_arc.clone()), None);
        assert!(
            res.is_ok(),
            "Expected run_script to succeed when only workflow runs"
        );

        let results = res.unwrap().lock().unwrap().get_results().clone();
        assert_eq!(results.len(), 1);
        assert_eq!(
            results[0],
            WorkflowStdout::Stdout("only workflow\n".to_string())
        );
    }

    #[test]
    fn test_run_script_pre_script_failure_maps_error() {
        // Invalid JS in pre_script to force a JsError (syntax error)
        let bad_pre = "function() {".to_string();
        let script = r#"console.log('should not run');"#;

        let res = run_script(script, vec![], None, Some(vec![bad_pre]));
        match res {
            Err(e) => match *e {
                SapphillonError::WorkflowRuntimeError(wr) => {
                    assert_eq!(wr.message, "Failed to execute pre_script");
                    match wr.error_type {
                        WorkflowRuntimeErrorType::CorePluginPrepareError => {}
                        _ => panic!("unexpected error_type for pre_script failure"),
                    }
                    // js_error should contain a syntax error message
                    let s = format!("{}", wr.js_error);
                    assert!(
                        s.to_lowercase().contains("syntax")
                            || s.to_lowercase().contains("unexpected"),
                        "js_error should indicate a syntax/unexpected token error, got: {s}"
                    );
                }
                SapphillonError::PermissionDeniedError(_) => {
                    panic!("unexpected PermissionDeniedError when testing pre_script failure")
                }
            },
            Ok(_) => panic!("expected an error when pre_script is invalid"),
        }
    }

    #[test]
    fn test_run_script_workflow_failure_maps_error() {
        // Valid pre-script
        let pre = "console.log('pre ok');".to_string();
        // Invalid workflow script (syntax error)
        let bad_workflow = "var = ;".to_string();

        let res = run_script(&bad_workflow, vec![], None, Some(vec![pre]));
        match res {
            Err(e) => match *e {
                SapphillonError::WorkflowRuntimeError(wr) => {
                    assert_eq!(wr.message, "Failed to execute workflow script");
                    match wr.error_type {
                        WorkflowRuntimeErrorType::WorkflowScriptExecuteError => {}
                        _ => panic!("unexpected error_type for workflow failure"),
                    }
                    let s = format!("{}", wr.js_error);
                    assert!(
                        s.to_lowercase().contains("syntax")
                            || s.to_lowercase().contains("unexpected"),
                        "js_error should indicate a syntax/unexpected token error, got: {s}"
                    );
                }
                SapphillonError::PermissionDeniedError(_) => {
                    panic!("unexpected PermissionDeniedError when testing workflow failure")
                }
            },
            Ok(_) => panic!("expected an error when workflow script is invalid"),
        }
    }
}

#[cfg(test)]
mod per_plugin_permission_tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_run_script_per_plugin_merge_allowed_success() {
        use crate::permission::{Permissions, PluginFunctionPermissions};
        use crate::proto::sapphillon::v1 as sapphillon_v1;

        // Two allowed entries for the same plugin_function_id ("pf.id"),
        // one grants /data/a and the other grants /data/project — they should be merged.
        let allowed1 = sapphillon_v1::Permission {
            permission_type: sapphillon_v1::PermissionType::FilesystemRead as i32,
            resource: vec!["/data/a".to_string()],
            ..Default::default()
        };
        let allowed2 = sapphillon_v1::Permission {
            permission_type: sapphillon_v1::PermissionType::FilesystemRead as i32,
            resource: vec!["/data/project".to_string()],
            ..Default::default()
        };

        // Required path is under /data/project, so the merged allowed should satisfy it.
        let required = sapphillon_v1::Permission {
            permission_type: sapphillon_v1::PermissionType::FilesystemRead as i32,
            resource: vec!["/data/project/src/main.rs".to_string()],
            ..Default::default()
        };

        let allowed_pf1 = PluginFunctionPermissions {
            plugin_function_id: "pf.id".to_string(),
            permissions: Permissions::new(vec![allowed1]),
        };
        let allowed_pf2 = PluginFunctionPermissions {
            plugin_function_id: "pf.id".to_string(),
            permissions: Permissions::new(vec![allowed2]),
        };
        let required_pf = PluginFunctionPermissions {
            plugin_function_id: "pf.id".to_string(),
            permissions: Permissions::new(vec![required]),
        };

        let workflow_data = OpStateWorkflowData::new(
            "wid_merge",
            false,
            Some(vec![allowed_pf1, allowed_pf2]),
            Some(vec![required_pf]),
        );
        let workflow_data_arc = Arc::new(Mutex::new(workflow_data));

        let res = run_script("console.log('ok');", vec![], Some(workflow_data_arc), None);
        assert!(
            res.is_ok(),
            "Expected merged allowed permissions to satisfy required"
        );
    }

    #[test]
    fn test_run_script_per_plugin_allowed_missing() {
        use crate::permission::{Permissions, PluginFunctionPermissions};
        use crate::proto::sapphillon::v1 as sapphillon_v1;

        // Required permission targets plugin function id "pf.id"
        let required = sapphillon_v1::Permission {
            permission_type: sapphillon_v1::PermissionType::Execute as i32,
            resource: vec![],
            ..Default::default()
        };
        let required_pf = PluginFunctionPermissions {
            plugin_function_id: "pf.id".to_string(),
            permissions: Permissions::new(vec![required]),
        };

        // Allowed contains only an entry for a different plugin_function_id -> should fail
        let allowed_pf_other = PluginFunctionPermissions {
            plugin_function_id: "other.id".to_string(),
            permissions: Permissions::new(vec![]),
        };

        let workflow_data = OpStateWorkflowData::new(
            "wid_missing",
            false,
            Some(vec![allowed_pf_other]),
            Some(vec![required_pf]),
        );
        let workflow_data_arc = Arc::new(Mutex::new(workflow_data));

        let res = run_script("console.log('x');", vec![], Some(workflow_data_arc), None);
        assert!(
            res.is_err(),
            "Expected PermissionDeniedError when allowed entry missing for plugin_function_id"
        );

        let err = res.err().unwrap();
        match *err {
            SapphillonError::PermissionDeniedError(_) => {}
            _ => panic!("expected PermissionDeniedError"),
        }
    }
}
