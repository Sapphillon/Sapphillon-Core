// Sapphillon-Core
// SPDX-FileCopyrightText: 2025 Yuta Takahashi
// SPDX-License-Identifier: MPL-2.0 OR GPL-3.0-or-later

use crate::permission::PluginFunctionPermissions;
use crate::plugin::CorePluginPackage;
use crate::proto::google::protobuf::Timestamp;
use crate::proto::sapphillon;
use crate::proto::sapphillon::v1::{WorkflowResult, WorkflowResultType};
use crate::runtime::{OpStateWorkflowData, run_script};
use regex::Regex;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::runtime::Handle;

#[derive(Debug, Clone)]
pub struct CoreWorkflowCode {
    /// Unique ID of the workflow code
    pub id: String,
    /// The JavaScript code of the workflow.
    pub code: String,
    /// List of plugin packages used in the workflow
    pub plugin_packages: Vec<CorePluginPackage>,

    pub code_revision: i32,
    pub result: Vec<sapphillon::v1::WorkflowResult>,

    pub allowed_permissions: Vec<PluginFunctionPermissions>,
    pub required_permissions: Vec<PluginFunctionPermissions>,
}

impl CoreWorkflowCode {
    /// Creates a new `CoreWorkflowCode`.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier for the workflow code.
    /// * `code` - The JavaScript code for the workflow.
    /// * `plugin_packages` - A vector of `CorePluginPackage`s used in this workflow.
    /// * `code_revision` - The revision number of the code.
    pub fn new(
        id: String,
        code: String,
        plugin_packages: Vec<CorePluginPackage>,
        code_revision: i32,
        allowed_permissions: Vec<PluginFunctionPermissions>,
        required_permissions: Vec<PluginFunctionPermissions>,
    ) -> Self {
        Self {
            id,
            code: unescaper::unescape(&code).unwrap(),
            plugin_packages,
            code_revision,
            result: Vec::new(),
            allowed_permissions,
            required_permissions,
        }
    }

    /// Executes the workflow code and appends a WorkflowResult to the result list.
    ///
    /// This method collects all OpDecls from the associated plugin packages, executes the workflow code
    /// using these operations, and records the execution result. The result includes metadata such as
    /// execution time, revision, exit code, and result type (success or failure). The result is appended
    /// to the `result` field of the struct.
    ///
    /// # Execution Flow
    /// 1. Collect OpDecls from all plugin packages.
    /// 2. Generate execution metadata (ID, display name, timestamp, revision).
    /// 3. Execute the workflow code using `run_script`.
    /// 4. Construct a `WorkflowResult` based on the execution outcome.
    /// 5. Append the result to the `result` vector.
    ///
    /// # Side Effects
    /// - Modifies the `result` field by adding a new `WorkflowResult`.
    pub fn run(&mut self, handle: Handle) {
        // Collect OpDecls from plugin packages
        let mut ops = Vec::new();
        for pkg in &self.plugin_packages {
            for func in &pkg.functions {
                ops.push(func.func.clone().into_owned());
            }
        }

        // Execute the workflow code and record the result
        let now = SystemTime::now();
        let epoch = now.duration_since(UNIX_EPOCH).unwrap();
        let id = format!("{}-{}", self.id, epoch.as_nanos());
        let display_name = format!("Run {}", epoch.as_secs());
        let ran_at = Some(Timestamp {
            seconds: epoch.as_secs() as i64,
            nanos: epoch.subsec_nanos() as i32,
        });
        let workflow_result_revision = self
            .result
            .last()
            .map(|r| r.workflow_result_revision + 1)
            .unwrap_or(1);

        // Create pre-run script state

        let pre_run_js: Option<Vec<String>> = {
            let v: Vec<String> = self
                .plugin_packages
                .iter()
                .flat_map(|pkg| {
                    pkg.functions
                        .iter()
                        .filter_map(|func| func.pre_run_js.clone())
                })
                .collect();
            if v.is_empty() { None } else { Some(v) }
        };

        let opstate_workflow_data = OpStateWorkflowData::new(
            &self.id,
            true,
            // Convert Vec<PluginFunctionPermissions> fields into Option<Vec<PluginFunctionPermissions>>
            // by mapping empty vectors to None and non-empty vectors to Some(vec).
            if self.allowed_permissions.is_empty() {
                None
            } else {
                Some(self.allowed_permissions.clone())
            },
            if self.required_permissions.is_empty() {
                None
            } else {
                Some(self.required_permissions.clone())
            },
            handle.clone(),
        );
        let result = run_script(
            &self.code,
            ops,
            Some(Arc::new(Mutex::new(opstate_workflow_data))),
            pre_run_js,
        );

        let (description, result, result_type, exit_code) = match result {
            Ok(data) => (
                "Success".to_string(),
                data.lock().unwrap().stdout_to_string(),
                WorkflowResultType::SuccessUnspecified as i32,
                0,
            ),
            Err(e) => (
                format!("Error: {e}"),
                format!("{e}"),
                WorkflowResultType::Failure as i32,
                1,
            ),
        };

        let result_obj = WorkflowResult {
            id,
            display_name,
            description,
            result,
            ran_at,
            result_type,
            exit_code,
            workflow_result_revision,
        };
        self.result.push(result_obj);
    }

    /// Creates a `CoreWorkflowCode` from a protobuf `WorkflowCode` message.
    ///
    /// # Arguments
    ///
    /// * `workflow_code` - The protobuf `WorkflowCode` message.
    /// * `plugin_packages` - A vector of `CorePluginPackage`s used in this workflow.
    pub fn new_from_proto(
        workflow_code: &sapphillon::v1::WorkflowCode,
        plugin_packages: Vec<CorePluginPackage>,
        required_permissions: Vec<PluginFunctionPermissions>,
        allowed_permissions: Vec<PluginFunctionPermissions>,
    ) -> Self {
        Self {
            id: workflow_code.id.clone(),
            code: workflow_code.code.clone(),
            plugin_packages,
            code_revision: workflow_code.code_revision,
            result: Vec::new(),
            required_permissions,
            allowed_permissions,
        }
    }

    /// Extracts the list of plugins used in the workflow code.
    ///
    /// This function parses the JavaScript code and extracts plugin function calls
    /// in the format `package_name.function_name(...)`, then filters them against
    /// the provided list of available plugin packages.
    ///
    /// # Arguments
    ///
    /// * `available_plugins` - A slice of `CorePluginPackage` representing all available plugins.
    ///
    /// # Returns
    ///
    /// A `Vec<PluginIdentifier>` containing all unique plugin calls found in the code
    /// that match the available plugins.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use sapphillon_core::plugin::CorePluginPackage;
    ///
    /// let available_plugins = vec![
    ///     CorePluginPackage::new(
    ///         "id1".to_string(),
    ///         "myPlugin".to_string(),
    ///         vec![/* functions */],
    ///     ),
    /// ];
    /// // Assuming we have a CoreWorkflowCode instance `workflow`
    /// let plugins = workflow.extract_used_plugins(&available_plugins);
    /// // plugins will only contain myPlugin.doSomething() if doSomething is defined
    /// ```
    pub fn extract_used_plugins(
        &self,
        available_plugins: &[CorePluginPackage],
    ) -> Vec<PluginIdentifier> {
        extract_used_plugins_from_code(&self.code, available_plugins)
    }
}

/// Extracts the list of plugins used in the given JavaScript code.
///
/// This function parses the JavaScript code and extracts plugin function calls
/// in the format `package_name.function_name(...)`, then filters them against
/// the provided list of available plugin packages.
///
/// Only plugins that are registered in `available_plugins` with matching
/// package name and function name will be included in the result.
///
/// # Arguments
///
/// * `code` - The JavaScript code to analyze.
/// * `available_plugins` - A slice of `CorePluginPackage` representing all available plugins.
///
/// # Returns
///
/// A `Vec<PluginIdentifier>` containing all unique plugin calls found in the code
/// that match the available plugins.
///
/// # Example
///
/// ```ignore
/// use sapphillon_core::plugin::{CorePluginPackage, CorePluginFunction};
/// use sapphillon_core::workflow::extract_used_plugins_from_code;
///
/// // Create available plugins
/// let available_plugins = vec![
///     CorePluginPackage::new(
///         "pkg1".to_string(),
///         "filePlugin".to_string(),
///         vec![
///             CorePluginFunction::new(
///                 "f1".to_string(),
///                 "read".to_string(),
///                 "Read a file".to_string(),
///                 dummy_op(),
///                 None,
///             ),
///         ],
///     ),
/// ];
///
/// let code = "filePlugin.read('/path'); unknownPlugin.call();";
/// let plugins = extract_used_plugins_from_code(code, &available_plugins);
/// // plugins will contain only filePlugin.read
/// ```
pub fn extract_used_plugins_from_code(
    code: &str,
    available_plugins: &[CorePluginPackage],
) -> Vec<PluginIdentifier> {
    // Build a HashSet of valid "package_name.function_name" combinations
    let valid_plugin_calls: HashSet<(String, String)> = available_plugins
        .iter()
        .flat_map(|pkg| {
            pkg.functions
                .iter()
                .map(move |func| (pkg.name.clone(), func.name.clone()))
        })
        .collect();

    // Regex pattern to match: identifier.identifier( (function call pattern)
    // This matches patterns like: packageName.functionName(
    // where packageName and functionName are valid JavaScript identifiers
    //
    // Updated to support:
    // 1. Namespaced packages (e.g. `sapphillon.core.exec.exec`)
    // 2. Optional chaining (e.g. `plugin?.func()`)
    static RE: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
    let pattern = r"\b((?:[a-zA-Z_$][a-zA-Z0-9_$]*\.)*[a-zA-Z_$][a-zA-Z0-9_$]*)\??\.([a-zA-Z_$][a-zA-Z0-9_$]*)\s*\(";

    let re = RE.get_or_init(|| {
        Regex::new(pattern).unwrap_or_else(|e| {
            panic!(
                "Failed to compile plugin call regex pattern {:?}: {}",
                pattern, e
            )
        })
    });

    let mut seen = HashSet::new();
    let mut result = Vec::new();

    for cap in re.captures_iter(code) {
        let package_name = cap[1].to_string();
        let function_name = cap[2].to_string();

        // Only include if this is a registered plugin function
        if !valid_plugin_calls.contains(&(package_name.clone(), function_name.clone())) {
            continue;
        }

        let key = format!("{}.{}", package_name, function_name);
        if seen.insert(key) {
            result.push(PluginIdentifier {
                package_name,
                function_name,
            });
        }
    }

    result
}

/// Represents a plugin function call identifier.
///
/// Contains the package name and function name extracted from a workflow code.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PluginIdentifier {
    /// The name of the plugin package
    pub package_name: String,
    /// The name of the function within the package
    pub function_name: String,
}

impl PluginIdentifier {
    /// Creates a new PluginIdentifier.
    pub fn new(package_name: String, function_name: String) -> Self {
        Self {
            package_name,
            function_name,
        }
    }

    /// Returns the full identifier in the format "package_name.function_name".
    pub fn full_name(&self) -> String {
        format!("{}.{}", self.package_name, self.function_name)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::plugin::{CorePluginFunction, CorePluginPackage};
    use crate::proto::sapphillon::v1::WorkflowCode;

    // Generate a dummy CorePluginFunction for testing
    fn dummy_plugin_function() -> CorePluginFunction {
        // Use a dummy OpDecl that returns u32, same as in plugin.rs tests
        use deno_core::op2;
        #[op2(fast)]
        fn dummy_op() -> u32 {
            42
        }
        CorePluginFunction::new(
            "fid".to_string(),
            "fname".to_string(),
            "desc".to_string(),
            dummy_op(),
            None,
        )
    }

    // Generate a dummy CorePluginPackage for testing
    fn dummy_plugin_package() -> CorePluginPackage {
        CorePluginPackage::new(
            "pid".to_string(),
            "pname".to_string(),
            vec![dummy_plugin_function()],
        )
    }

    fn dummy_plugin_function_with_pre_script() -> CorePluginFunction {
        // Use a dummy OpDecl that returns u32, same as in plugin.rs tests
        use deno_core::op2;
        #[op2(fast)]
        fn dummy_op() -> u32 {
            42
        }
        CorePluginFunction::new(
            "fid".to_string(),
            "fname".to_string(),
            "desc".to_string(),
            dummy_op(),
            Some(r"console.log('pre-run script');".to_string()),
        )
    }

    fn dummy_plugin_package_with_pre_script() -> CorePluginPackage {
        CorePluginPackage::new(
            "pid".to_string(),
            "pname".to_string(),
            vec![dummy_plugin_function_with_pre_script()],
        )
    }

    #[test]
    fn test_core_workflow_code_run_success() {
        let pkg = dummy_plugin_package();
        let mut code = CoreWorkflowCode::new(
            "wid".to_string(),
            "console.log(1 + 1);".to_string(),
            vec![pkg],
            1,
            vec![],
            vec![],
        );
        let tokio_runtime = tokio::runtime::Runtime::new().unwrap();
        code.run(tokio_runtime.handle().clone());
        assert_eq!(code.result.len(), 1);
        let res = &code.result[0];
        assert_eq!(res.exit_code, 0);
        assert_eq!(
            res.result_type,
            sapphillon::v1::WorkflowResultType::SuccessUnspecified as i32
        );
        assert_eq!(res.result, "2\n");
    }

    #[test]
    fn test_core_workflow_code_run_with_pre_script() {
        let pkg = dummy_plugin_package_with_pre_script();
        let mut code = CoreWorkflowCode::new(
            "wid".to_string(),
            "console.log(1 + 1);".to_string(),
            vec![pkg],
            1,
            vec![],
            vec![],
        );
        let tokio_runtime = tokio::runtime::Runtime::new().unwrap();
        code.run(tokio_runtime.handle().clone());
        assert_eq!(code.result.len(), 1);
        let res = &code.result[0];
        assert_eq!(res.exit_code, 0);
        assert_eq!(
            res.result_type,
            sapphillon::v1::WorkflowResultType::SuccessUnspecified as i32
        );
        assert_eq!(res.result, "pre-run script\n\n2\n");
    }

    #[test]
    fn test_core_workflow_code_run_failure() {
        let pkg = dummy_plugin_package();
        let mut code = CoreWorkflowCode::new(
            "wid".to_string(),
            "throw new Error('fail');".to_string(),
            vec![pkg],
            1,
            vec![],
            vec![],
        );
        let tokio_runtime = tokio::runtime::Runtime::new().unwrap();
        code.run(tokio_runtime.handle().clone());
        assert_eq!(code.result.len(), 1);
        let res = &code.result[0];
        assert_eq!(res.exit_code, 1);
        assert_eq!(
            res.result_type,
            sapphillon::v1::WorkflowResultType::Failure as i32
        );
        assert!(res.result.contains("fail"));
    }
    // Generate a dummy WorkflowCode (proto) for testing
    fn dummy_proto_workflow_code() -> WorkflowCode {
        WorkflowCode {
            id: "wid".to_string(),
            code: "console.log('hello');".to_string(),
            code_revision: 1,
            ..Default::default()
        }
    }

    #[test]
    fn test_core_workflow_code_new() {
        let pkg = dummy_plugin_package();
        let code = CoreWorkflowCode::new(
            "wid".to_string(),
            r"\nconsole.log('test');".to_string(),
            vec![pkg],
            2,
            vec![],
            vec![],
        );
        assert_eq!(code.id, "wid");
        assert_eq!(code.code, "\nconsole.log('test');");
        assert_eq!(code.plugin_packages.len(), 1);
        assert_eq!(code.code_revision, 2);
        assert!(code.result.is_empty());
    }

    #[test]
    fn test_core_workflow_code_new_from_proto() {
        let proto = dummy_proto_workflow_code();
        let pkg = dummy_plugin_package();
        let code = CoreWorkflowCode::new_from_proto(&proto, vec![pkg], vec![], vec![]);
        assert_eq!(code.id, proto.id);
        assert_eq!(code.code, proto.code);
        assert_eq!(code.plugin_packages.len(), 1);
        assert_eq!(code.code_revision, proto.code_revision);
        assert!(code.result.is_empty());
    }

    #[test]
    fn test_workflow_result_initial_state() {
        let pkg = dummy_plugin_package();
        let code = CoreWorkflowCode::new(
            "wid".to_string(),
            "console.log('test');".to_string(),
            vec![pkg],
            1,
            vec![],
            vec![],
        );
        assert!(code.result.is_empty(), "Initial results should be empty");
    }

    // Helper to create a plugin function for testing
    fn make_test_plugin_function(name: &str) -> CorePluginFunction {
        use deno_core::op2;
        #[op2(fast)]
        fn test_op() -> u32 {
            0
        }
        CorePluginFunction::new(
            format!("{}_id", name),
            name.to_string(),
            "test".to_string(),
            test_op(),
            None,
        )
    }

    // Helper to create a plugin package for testing
    fn make_test_plugin_package(pkg_name: &str, func_names: &[&str]) -> CorePluginPackage {
        let functions = func_names
            .iter()
            .map(|n| make_test_plugin_function(n))
            .collect();
        CorePluginPackage::new(format!("{}_id", pkg_name), pkg_name.to_string(), functions)
    }

    #[test]
    fn test_extract_used_plugins_basic() {
        let available_plugins = vec![
            make_test_plugin_package("myPlugin", &["doSomething"]),
            make_test_plugin_package("anotherPlugin", &["run"]),
        ];

        let code = CoreWorkflowCode::new(
            "wid".to_string(),
            "myPlugin.doSomething(); anotherPlugin.run();".to_string(),
            vec![],
            1,
            vec![],
            vec![],
        );
        let plugins = code.extract_used_plugins(&available_plugins);

        assert_eq!(plugins.len(), 2);
        assert!(
            plugins
                .iter()
                .any(|p| p.package_name == "myPlugin" && p.function_name == "doSomething")
        );
        assert!(
            plugins
                .iter()
                .any(|p| p.package_name == "anotherPlugin" && p.function_name == "run")
        );
    }

    #[test]
    fn test_extract_used_plugins_duplicates_removed() {
        let available_plugins = vec![make_test_plugin_package("plugin", &["func", "other"])];

        let code = CoreWorkflowCode::new(
            "wid".to_string(),
            "plugin.func(); plugin.func(); plugin.other();".to_string(),
            vec![],
            1,
            vec![],
            vec![],
        );
        let plugins = code.extract_used_plugins(&available_plugins);

        assert_eq!(plugins.len(), 2);
        assert!(
            plugins
                .iter()
                .any(|p| p.package_name == "plugin" && p.function_name == "func")
        );
        assert!(
            plugins
                .iter()
                .any(|p| p.package_name == "plugin" && p.function_name == "other")
        );
    }

    #[test]
    fn test_extract_used_plugins_filters_non_plugins() {
        // Only myPlugin.action is a registered plugin
        let available_plugins = vec![make_test_plugin_package("myPlugin", &["action"])];

        let code = CoreWorkflowCode::new(
            "wid".to_string(),
            "console.log('test'); Math.random(); myPlugin.action(); someObj.method();".to_string(),
            vec![],
            1,
            vec![],
            vec![],
        );
        let plugins = code.extract_used_plugins(&available_plugins);

        // Only myPlugin.action should be detected
        assert_eq!(plugins.len(), 1);
        assert_eq!(plugins[0].package_name, "myPlugin");
        assert_eq!(plugins[0].function_name, "action");
    }

    #[test]
    fn test_extract_used_plugins_empty_code() {
        let available_plugins = vec![make_test_plugin_package("plugin", &["func"])];

        let code =
            CoreWorkflowCode::new("wid".to_string(), "".to_string(), vec![], 1, vec![], vec![]);
        let plugins = code.extract_used_plugins(&available_plugins);

        assert!(plugins.is_empty());
    }

    #[test]
    fn test_extract_used_plugins_no_available_plugins() {
        // No plugins are registered, so nothing should be detected
        let available_plugins: Vec<CorePluginPackage> = vec![];

        let code = CoreWorkflowCode::new(
            "wid".to_string(),
            "myPlugin.doSomething(); anotherPlugin.run();".to_string(),
            vec![],
            1,
            vec![],
            vec![],
        );
        let plugins = code.extract_used_plugins(&available_plugins);

        assert!(plugins.is_empty());
    }

    #[test]
    fn test_extract_used_plugins_complex_code() {
        let available_plugins = vec![
            make_test_plugin_package("filePlugin", &["read"]),
            make_test_plugin_package("networkPlugin", &["send"]),
            make_test_plugin_package("dbPlugin", &["save"]),
        ];

        let code = CoreWorkflowCode::new(
            "wid".to_string(),
            r#"
                const result = filePlugin.read('/path/to/file');
                if (result) {
                    networkPlugin.send(result);
                }
                console.log(JSON.stringify(result));
                dbPlugin.save('key', result);
                unknownPlugin.call(); // This should NOT be detected
            "#
            .to_string(),
            vec![],
            1,
            vec![],
            vec![],
        );
        let plugins = code.extract_used_plugins(&available_plugins);

        assert_eq!(plugins.len(), 3);
        assert!(
            plugins
                .iter()
                .any(|p| p.package_name == "filePlugin" && p.function_name == "read")
        );
        assert!(
            plugins
                .iter()
                .any(|p| p.package_name == "networkPlugin" && p.function_name == "send")
        );
        assert!(
            plugins
                .iter()
                .any(|p| p.package_name == "dbPlugin" && p.function_name == "save")
        );
    }

    #[test]
    fn test_extract_used_plugins_function_not_registered() {
        // Plugin package exists but function is not registered
        let available_plugins = vec![make_test_plugin_package("myPlugin", &["registeredFunc"])];

        let code = CoreWorkflowCode::new(
            "wid".to_string(),
            "myPlugin.registeredFunc(); myPlugin.unregisteredFunc();".to_string(),
            vec![],
            1,
            vec![],
            vec![],
        );
        let plugins = code.extract_used_plugins(&available_plugins);

        // Only registeredFunc should be detected
        assert_eq!(plugins.len(), 1);
        assert_eq!(plugins[0].package_name, "myPlugin");
        assert_eq!(plugins[0].function_name, "registeredFunc");
    }

    #[test]
    fn test_plugin_identifier_full_name() {
        let plugin = PluginIdentifier::new("myPackage".to_string(), "myFunction".to_string());
        assert_eq!(plugin.full_name(), "myPackage.myFunction");
    }

    #[test]
    fn test_extract_used_plugins_optional_chaining() {
        let available_plugins = vec![make_test_plugin_package("plugin", &["func"])];
        let code = CoreWorkflowCode::new(
            "wid".to_string(),
            "plugin?.func();".to_string(),
            vec![],
            1,
            vec![],
            vec![],
        );
        let plugins = code.extract_used_plugins(&available_plugins);
        assert_eq!(plugins.len(), 1);
        assert_eq!(plugins[0].package_name, "plugin");
        assert_eq!(plugins[0].function_name, "func");
    }

    #[test]
    fn test_extract_used_plugins_namespaced() {
        let available_plugins = vec![make_test_plugin_package("sapphillon.core.exec", &["exec"])];
        let code = CoreWorkflowCode::new(
            "wid".to_string(),
            "sapphillon.core.exec.exec();".to_string(),
            vec![],
            1,
            vec![],
            vec![],
        );
        let plugins = code.extract_used_plugins(&available_plugins);
        assert_eq!(plugins.len(), 1);
        assert_eq!(plugins[0].package_name, "sapphillon.core.exec");
        assert_eq!(plugins[0].function_name, "exec");
    }
}

#[cfg(test)]
mod permission_tests {
    //! Comprehensive permission validation test suite.
    //!
    //! Goals:
    //! - Verify that [`CoreWorkflowCode::run()`] routes through `runtime::run_script()`
    //!   which invokes `permission::check_permission`, and that missing required
    //!   permissions yield a `PermissionDeniedError` producing a Failure `WorkflowResult`.
    //! - FilesystemRead / FilesystemWrite: ancestor directory coverage logic works.
    //! - NetAccess: origin (scheme, host, effective port) + normalized path segment
    //!   prefix coverage works.
    //! - Execute: presence of the permission type alone (no resources) is sufficient.
    //! - Duplicate allowed permissions of the same type are merged but still cover
    //!   required resources (merge semantics).
    //! - Multiple simultaneously missing types surface each `PermissionType` name
    //!   in the error message.
    //!
    //! Categories:
    //! 1. Single success per PermissionType
    //! 2. Single failure per PermissionType
    //! 3. Composite success (all types satisfied)
    //! 4. Composite failure (multiple missing types)
    //! 5. Merge behavior (duplicate allowed entries)
    //! 6. Error detail (checks "Permission denied" + Requested / Granted fragments)
    //!
    //! These tests collectively ensure logical correctness, merge behavior, and
    //! diagnostic clarity of the permission system.
    use super::CoreWorkflowCode;
    use crate::permission::{Permissions, PluginFunctionPermissions};
    use crate::proto::sapphillon;
    use crate::proto::sapphillon::v1::{Permission, PermissionType};

    // ----------------------------
    // Helper constructors
    // ----------------------------
    fn perm(permission_type: PermissionType, resources: &[&str]) -> Permission {
        Permission {
            permission_type: permission_type as i32,
            resource: resources.iter().map(|s| s.to_string()).collect(),
            ..Default::default()
        }
    }

    fn run_with_permissions(
        allowed: Vec<Permission>,
        required: Vec<Permission>,
        script: &str,
    ) -> CoreWorkflowCode {
        let mut code = CoreWorkflowCode::new(
            "wid".to_string(),
            script.to_string(),
            vec![], // no plugin packages needed
            1,
            vec![PluginFunctionPermissions {
                plugin_function_id: "id".to_string(),
                permissions: Permissions::new(allowed),
            }],
            vec![PluginFunctionPermissions {
                plugin_function_id: "id".to_string(),
                permissions: Permissions::new(required),
            }],
        );
        let tokio_runtime = tokio::runtime::Runtime::new().unwrap();
        code.run(tokio_runtime.handle().clone());
        code
    }

    fn run_with_multi_plugin_permissions(
        allowed: Vec<PluginFunctionPermissions>,
        required: Vec<PluginFunctionPermissions>,
        script: &str,
    ) -> CoreWorkflowCode {
        let mut code = CoreWorkflowCode::new(
            "wid".to_string(),
            script.to_string(),
            vec![], // no plugin packages needed
            1,
            allowed,
            required,
        );
        let tokio_runtime = tokio::runtime::Runtime::new().unwrap();
        code.run(tokio_runtime.handle().clone());
        code
    }

    // ---------------
    // Single success cases
    // ---------------
    #[test]
    fn test_workflow_permissions_fs_read_success() {
        let allowed = vec![perm(PermissionType::FilesystemRead, &["/project"])];
        let required = vec![perm(
            PermissionType::FilesystemRead,
            &["/project/src/main.rs"],
        )];
        let code = run_with_permissions(allowed, required, "console.log(1);");
        assert_eq!(code.result.len(), 1);
        let res = &code.result[0];
        assert_eq!(res.exit_code, 0);
        assert_eq!(
            res.result_type,
            sapphillon::v1::WorkflowResultType::SuccessUnspecified as i32
        );
    }

    #[test]
    fn test_workflow_permissions_fs_write_success() {
        let allowed = vec![perm(PermissionType::FilesystemWrite, &["/data"])];
        let required = vec![perm(
            PermissionType::FilesystemWrite,
            &["/data/output/result.txt"],
        )];
        let code = run_with_permissions(allowed, required, "console.log('ok');");
        let res = &code.result[0];
        assert_eq!(res.exit_code, 0);
    }

    #[test]
    fn test_workflow_permissions_net_access_success() {
        let allowed = vec![perm(
            PermissionType::NetAccess,
            &["https://example.com/api"],
        )];
        let required = vec![perm(
            PermissionType::NetAccess,
            &["https://example.com/api/v1/resource"],
        )];
        let code = run_with_permissions(allowed, required, "console.log('net');");
        let res = &code.result[0];
        assert_eq!(res.exit_code, 0);
    }

    #[test]
    fn test_workflow_permissions_execute_success() {
        let allowed = vec![perm(PermissionType::Execute, &[])];
        let required = vec![perm(PermissionType::Execute, &[])];
        let code = run_with_permissions(allowed, required, "console.log('exec');");
        let res = &code.result[0];
        assert_eq!(res.exit_code, 0);
    }

    // ---------------
    // Single failure cases
    // ---------------
    #[test]
    fn test_workflow_permissions_fs_read_failure() {
        let allowed = vec![perm(PermissionType::FilesystemRead, &["/other"])];
        let required = vec![perm(
            PermissionType::FilesystemRead,
            &["/project/src/main.rs"],
        )];
        let code = run_with_permissions(allowed, required, "console.log('x');");
        let res = &code.result[0];
        assert_eq!(res.exit_code, 1);
        assert_eq!(
            res.result_type,
            sapphillon::v1::WorkflowResultType::Failure as i32
        );
        assert!(res.result.contains("Permission denied"));
        assert!(res.result.contains("PERMISSION_TYPE_FILESYSTEM_READ"));
    }

    #[test]
    fn test_workflow_permissions_fs_write_failure() {
        let allowed = vec![perm(PermissionType::FilesystemWrite, &["/base"])];
        let required = vec![perm(
            PermissionType::FilesystemWrite,
            &["/data/output/result.txt"],
        )];
        let code = run_with_permissions(allowed, required, "console.log('x');");
        let res = &code.result[0];
        assert_eq!(res.exit_code, 1);
        assert!(res.result.contains("PERMISSION_TYPE_FILESYSTEM_WRITE"));
    }

    #[test]
    fn test_workflow_permissions_net_access_failure() {
        let allowed = vec![perm(
            PermissionType::NetAccess,
            &["https://api.example.com/"],
        )];
        let required = vec![perm(
            PermissionType::NetAccess,
            &["https://example.com/api/v1/resource"],
        )];
        let code = run_with_permissions(allowed, required, "console.log('net');");
        let res = &code.result[0];
        assert_eq!(res.exit_code, 1);
        assert!(res.result.contains("PERMISSION_TYPE_NET_ACCESS"));
    }

    #[test]
    fn test_workflow_permissions_execute_failure() {
        let allowed = vec![];
        let required = vec![perm(PermissionType::Execute, &[])];
        let code = run_with_permissions(allowed, required, "console.log('exec');");
        let res = &code.result[0];
        assert_eq!(res.exit_code, 1);
        assert!(res.result.contains("PERMISSION_TYPE_EXECUTE"));
    }

    // ---------------
    // Composite success (all types)
    // ---------------
    #[test]
    fn test_workflow_permissions_composite_success() {
        let allowed = vec![
            perm(PermissionType::FilesystemRead, &["/workspace"]),
            perm(PermissionType::FilesystemWrite, &["/workspace/tmp"]),
            perm(PermissionType::NetAccess, &["https://example.com/api"]),
            perm(PermissionType::Execute, &[]),
        ];
        let required = vec![
            perm(PermissionType::FilesystemRead, &["/workspace/src/lib.rs"]),
            perm(PermissionType::FilesystemWrite, &["/workspace/tmp/out.log"]),
            perm(
                PermissionType::NetAccess,
                &["https://example.com/api/v1/users"],
            ),
            perm(PermissionType::Execute, &[]),
        ];
        let code = run_with_permissions(allowed, required, "console.log('all');");
        let res = &code.result[0];
        assert_eq!(res.exit_code, 0);
    }

    // ---------------
    // Composite failure (multiple missing)
    // ---------------
    #[test]
    fn test_workflow_permissions_composite_multiple_missing() {
        let allowed = vec![perm(PermissionType::FilesystemRead, &["/workspace"])];
        let required = vec![
            perm(PermissionType::FilesystemWrite, &["/workspace/tmp/out.txt"]),
            perm(PermissionType::Execute, &[]),
        ];
        let code = run_with_permissions(allowed, required, "console.log('x');");
        let res = &code.result[0];
        assert_eq!(res.exit_code, 1);
        assert!(res.result.contains("PERMISSION_TYPE_FILESYSTEM_WRITE"));
        assert!(res.result.contains("PERMISSION_TYPE_EXECUTE"));
    }

    // ---------------
    // Merge duplication success
    // ---------------
    #[test]
    fn test_workflow_permissions_merge_duplicate_allowed() {
        // Two read bases; required path covered by second
        let allowed = vec![
            perm(PermissionType::FilesystemRead, &["/data/common"]),
            perm(PermissionType::FilesystemRead, &["/data/project"]),
        ];
        let required = vec![perm(
            PermissionType::FilesystemRead,
            &["/data/project/src/main.rs"],
        )];
        let code = run_with_permissions(allowed, required, "console.log('dup');");
        let res = &code.result[0];
        assert_eq!(res.exit_code, 0);
    }

    // ---------------
    // Error message detail check
    // ---------------
    #[test]
    fn test_workflow_permissions_failure_message_detail() {
        let allowed = vec![perm(PermissionType::FilesystemRead, &["/a"])];
        let required = vec![
            perm(PermissionType::FilesystemRead, &["/b/file.txt"]),
            perm(PermissionType::Execute, &[]),
        ];
        let code = run_with_permissions(allowed, required, "console.log('x');");
        let res = &code.result[0];
        assert_eq!(res.exit_code, 1);
        // Basic markers
        assert!(res.result.contains("Permission denied"));
        assert!(res.result.contains("PERMISSION_TYPE_FILESYSTEM_READ"));
        assert!(res.result.contains("PERMISSION_TYPE_EXECUTE"));
        // Check Requested / Granted fragments present
        assert!(res.result.contains("Requested Permissions"));
        assert!(res.result.contains("Granted Permissions"));
    }

    // ---------------
    // Multiple PluginFunctionPermissions tests
    // ---------------
    #[test]
    fn test_workflow_multiple_plugin_functions_all_satisfied() {
        // Test with two different plugin function IDs, both with permissions satisfied
        let allowed = vec![
            PluginFunctionPermissions {
                plugin_function_id: "plugin.func1".to_string(),
                permissions: Permissions::new(vec![perm(
                    PermissionType::FilesystemRead,
                    &["/data"],
                )]),
            },
            PluginFunctionPermissions {
                plugin_function_id: "plugin.func2".to_string(),
                permissions: Permissions::new(vec![perm(PermissionType::Execute, &[])]),
            },
        ];
        let required = vec![
            PluginFunctionPermissions {
                plugin_function_id: "plugin.func1".to_string(),
                permissions: Permissions::new(vec![perm(
                    PermissionType::FilesystemRead,
                    &["/data/file.txt"],
                )]),
            },
            PluginFunctionPermissions {
                plugin_function_id: "plugin.func2".to_string(),
                permissions: Permissions::new(vec![perm(PermissionType::Execute, &[])]),
            },
        ];
        let code = run_with_multi_plugin_permissions(allowed, required, "console.log('multi');");
        let res = &code.result[0];
        assert_eq!(res.exit_code, 0);
        assert_eq!(
            res.result_type,
            sapphillon::v1::WorkflowResultType::SuccessUnspecified as i32
        );
    }

    #[test]
    fn test_workflow_multiple_plugin_functions_one_missing() {
        // Test with two plugin function IDs, one has missing permissions
        let allowed = vec![
            PluginFunctionPermissions {
                plugin_function_id: "plugin.func1".to_string(),
                permissions: Permissions::new(vec![perm(
                    PermissionType::FilesystemRead,
                    &["/data"],
                )]),
            },
            PluginFunctionPermissions {
                plugin_function_id: "plugin.func2".to_string(),
                permissions: Permissions::new(vec![]), // No Execute permission granted
            },
        ];
        let required = vec![
            PluginFunctionPermissions {
                plugin_function_id: "plugin.func1".to_string(),
                permissions: Permissions::new(vec![perm(
                    PermissionType::FilesystemRead,
                    &["/data/file.txt"],
                )]),
            },
            PluginFunctionPermissions {
                plugin_function_id: "plugin.func2".to_string(),
                permissions: Permissions::new(vec![perm(PermissionType::Execute, &[])]),
            },
        ];
        let code = run_with_multi_plugin_permissions(allowed, required, "console.log('x');");
        let res = &code.result[0];
        assert_eq!(res.exit_code, 1);
        assert_eq!(
            res.result_type,
            sapphillon::v1::WorkflowResultType::Failure as i32
        );
        assert!(res.result.contains("Permission denied"));
        assert!(res.result.contains("PERMISSION_TYPE_EXECUTE"));
    }

    #[test]
    fn test_workflow_multiple_plugin_functions_id_not_in_allowed() {
        // Test where required has a plugin_function_id not present in allowed list
        let allowed = vec![PluginFunctionPermissions {
            plugin_function_id: "plugin.funcA".to_string(),
            permissions: Permissions::new(vec![perm(PermissionType::FilesystemRead, &["/data"])]),
        }];
        let required = vec![
            PluginFunctionPermissions {
                plugin_function_id: "plugin.funcA".to_string(),
                permissions: Permissions::new(vec![perm(
                    PermissionType::FilesystemRead,
                    &["/data/file.txt"],
                )]),
            },
            PluginFunctionPermissions {
                plugin_function_id: "plugin.funcB".to_string(),
                permissions: Permissions::new(vec![perm(PermissionType::Execute, &[])]),
            },
        ];
        let code = run_with_multi_plugin_permissions(allowed, required, "console.log('x');");
        let res = &code.result[0];
        assert_eq!(res.exit_code, 1);
        assert_eq!(
            res.result_type,
            sapphillon::v1::WorkflowResultType::Failure as i32
        );
        assert!(res.result.contains("Permission denied"));
    }

    #[test]
    fn test_workflow_multiple_plugin_functions_same_id_merge() {
        // Test with multiple allowed entries for the same plugin_function_id (merging behavior)
        let allowed = vec![
            PluginFunctionPermissions {
                plugin_function_id: "plugin.main".to_string(),
                permissions: Permissions::new(vec![perm(
                    PermissionType::FilesystemRead,
                    &["/workspace"],
                )]),
            },
            PluginFunctionPermissions {
                plugin_function_id: "plugin.main".to_string(),
                permissions: Permissions::new(vec![perm(
                    PermissionType::FilesystemWrite,
                    &["/output"],
                )]),
            },
        ];
        let required = vec![PluginFunctionPermissions {
            plugin_function_id: "plugin.main".to_string(),
            permissions: Permissions::new(vec![
                perm(PermissionType::FilesystemRead, &["/workspace/src/lib.rs"]),
                perm(PermissionType::FilesystemWrite, &["/output/result.txt"]),
            ]),
        }];
        let code = run_with_multi_plugin_permissions(allowed, required, "console.log('merged');");
        let res = &code.result[0];
        assert_eq!(res.exit_code, 0);
        assert_eq!(
            res.result_type,
            sapphillon::v1::WorkflowResultType::SuccessUnspecified as i32
        );
    }

    #[test]
    fn test_workflow_multiple_plugin_functions_composite() {
        // Test with three different plugin functions with various permission types
        let allowed = vec![
            PluginFunctionPermissions {
                plugin_function_id: "plugin.reader".to_string(),
                permissions: Permissions::new(vec![perm(
                    PermissionType::FilesystemRead,
                    &["/project"],
                )]),
            },
            PluginFunctionPermissions {
                plugin_function_id: "plugin.writer".to_string(),
                permissions: Permissions::new(vec![perm(
                    PermissionType::FilesystemWrite,
                    &["/logs"],
                )]),
            },
            PluginFunctionPermissions {
                plugin_function_id: "plugin.networker".to_string(),
                permissions: Permissions::new(vec![perm(
                    PermissionType::NetAccess,
                    &["https://api.example.com"],
                )]),
            },
        ];
        let required = vec![
            PluginFunctionPermissions {
                plugin_function_id: "plugin.reader".to_string(),
                permissions: Permissions::new(vec![perm(
                    PermissionType::FilesystemRead,
                    &["/project/data.json"],
                )]),
            },
            PluginFunctionPermissions {
                plugin_function_id: "plugin.writer".to_string(),
                permissions: Permissions::new(vec![perm(
                    PermissionType::FilesystemWrite,
                    &["/logs/output.log"],
                )]),
            },
            PluginFunctionPermissions {
                plugin_function_id: "plugin.networker".to_string(),
                permissions: Permissions::new(vec![perm(
                    PermissionType::NetAccess,
                    &["https://api.example.com/v1/data"],
                )]),
            },
        ];
        let code =
            run_with_multi_plugin_permissions(allowed, required, "console.log('composite');");
        let res = &code.result[0];
        assert_eq!(res.exit_code, 0);
        assert_eq!(
            res.result_type,
            sapphillon::v1::WorkflowResultType::SuccessUnspecified as i32
        );
    }
}
