// Sapphillon-Core
// SPDX-FileCopyrightText: 2025 Yuta Takahashi
// SPDX-License-Identifier: MPL-2.0 OR GPL-3.0-or-later

use crate::proto::sapphillon::v1::{PluginFunction, PluginPackage};
use crate::extplugin_rsjs_bridge;
use deno_core::OpDecl;
use std::borrow::Cow;

/// Trait representing a plugin function.
pub trait PluginFunctionTrait {
    /// Returns true if the function is external.
    fn is_external(&self) -> bool;
    /// Returns the unique identifier of the function.
    fn get_function_id(&self) -> String;
    /// Returns the name of the function.
    fn get_function_name(&self) -> String;
    /// Returns the Deno operation declaration for the function.
    fn get_opdecl(&self) -> Cow<'static, OpDecl>;
    /// Returns an optional JavaScript snippet to be executed before the main function.
    fn get_pre_run_js(&self) -> Option<String>;
}
/// Trait representing a plugin package.
pub trait PluginPackageTrait {
    /// The type of function contained in this package.
    type Function: PluginFunctionTrait;

    /// Returns true if the package is external.
    fn is_external(&self) -> bool;
    /// Returns the unique identifier of the package.
    fn get_package_id(&self) -> String;
    /// Returns the name of the package.
    fn get_package_name(&self) -> String;
    /// Returns a list of functions included in the package.
    fn get_functions(&self) -> Vec<Self::Function>;
}

/// Core representation of a plugin function.
/// Holds the function's ID, name, and Deno operation.
#[derive(Clone)]
pub struct CorePluginFunction {
    /// Unique ID of the function
    pub id: String,
    /// Function name
    pub name: String,
    /// Deno OpDecl (function body)
    pub func: Cow<'static, OpDecl>,
    /// Description of the function
    pub description: String,
    /// Optional: Pre Run Script
    pub pre_run_js: Option<String>,
    /// The plugin is external
    pub external_plugin: bool,
}

impl std::fmt::Debug for CorePluginFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CorePluginFunction")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("func", &"<OpDecl>")
            .field("description", &self.description)
            .field("pre_run_js", &self.pre_run_js)
            .finish()
    }
}

impl CorePluginFunction {
    /// Creates a new `CorePluginFunction`.
    /// This Function must be internal plugins
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier for the function.
    /// * `name` - The name of the function.
    /// * `description` - A description of what the function does.
    /// * `func` - The `OpDecl` representing the function's implementation.
    /// * `pre_run_js` - An optional JavaScript snippet to be executed before the main function.
    pub fn new(
        id: String,
        name: String,
        description: String,
        func: OpDecl,
        pre_run_js: Option<String>,
    ) -> Self {
        Self {
            id,
            name,
            func: Cow::Owned(func),
            pre_run_js,
            description,
            external_plugin: false,
        }
    }

    /// Creates a `CorePluginFunction` from a protobuf `PluginFunction` and an `OpDecl`.
    /// This function must be internal plugin function
    ///
    /// # Arguments
    ///
    /// * `plugin_function` - The protobuf `PluginFunction` message.
    /// * `function` - The `OpDecl` representing the function's implementation.
    pub fn new_from_plugin_function(plugin_function: &PluginFunction, function: OpDecl) -> Self {
        Self {
            id: plugin_function.function_id.clone(),
            name: plugin_function.function_name.clone(),
            func: Cow::Owned(function),
            description: plugin_function.description.clone(),
            pre_run_js: None,
            external_plugin: false,
        }
    }
}

impl PluginFunctionTrait for CorePluginFunction {
    fn is_external(&self) -> bool {
        self.external_plugin
    }

    fn get_function_id(&self) -> String {
        self.id.clone()
    }

    fn get_function_name(&self) -> String {
        self.name.clone()
    }

    fn get_opdecl(&self) -> Cow<'static, OpDecl> {
        self.func.clone()
    }

    fn get_pre_run_js(&self) -> Option<String> {
        self.pre_run_js.clone()
    }
}

/// Core representation of a plugin package.
/// Holds the package ID, name, and a list of functions.
#[derive(Debug, Clone)]
pub struct CorePluginPackage {
    /// Unique ID of the package
    pub id: String,
    /// Package name
    pub name: String,
    /// List of functions included in the package
    pub functions: Vec<CorePluginFunction>,
}

impl CorePluginPackage {
    /// Creates a new `CorePluginPackage`.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier for the package.
    /// * `name` - The name of the package.
    /// * `functions` - A vector of `CorePluginFunction` instances included in this package.
    pub fn new(id: String, name: String, functions: Vec<CorePluginFunction>) -> Self {
        Self {
            id,
            name,
            functions,
        }
    }

    /// Creates a `CorePluginPackage` from a protobuf `PluginPackage` and a list of functions.
    ///
    /// # Arguments
    ///
    /// * `plugin_package` - The protobuf `PluginPackage` message.
    /// * `functions` - A vector of `CorePluginFunction` instances to be included in this package.
    pub fn new_from_plugin_package(
        plugin_package: &PluginPackage,
        functions: Vec<CorePluginFunction>,
    ) -> Self {
        Self {
            id: plugin_package.package_id.clone(),
            name: plugin_package.package_name.clone(),
            functions,
        }
    }
}

impl PluginPackageTrait for CorePluginPackage {
    type Function = CorePluginFunction;

    fn is_external(&self) -> bool {
        // A package is external if all of its functions are external
        // If there are no functions, we consider it internal
        !self.functions.is_empty() && self.functions.iter().all(|f| f.is_external())
    }

    fn get_package_id(&self) -> String {
        self.id.clone()
    }

    fn get_package_name(&self) -> String {
        self.name.clone()
    }

    fn get_functions(&self) -> Vec<Self::Function> {
        self.functions.clone()
    }
}

/// Core representation of an external plugin function.
/// Holds the function's ID, name, package JavaScript code, and external flag.
#[derive(Debug, Clone)]
pub struct CorePluginExternalFunction {
    /// Unique ID of the function
    pub id: String,
    /// Function name
    pub name: String,
    /// Description of the function
    pub description: String,
    /// Package JavaScript code to be executed
    pub package_js: String,
    /// The plugin is external (always true for this type)
    pub external: bool,
}

impl CorePluginExternalFunction {
    /// Creates a new `CorePluginExternalFunction`.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier for the function.
    /// * `name` - The name of the function.
    /// * `description` - A description of what the function does.
    /// * `package_js` - The JavaScript code for the package.
    pub fn new(id: String, name: String, description: String, package_js: String) -> Self {
        Self {
            id,
            name,
            description,
            package_js,
            external: true,
        }
    }
}

impl PluginFunctionTrait for CorePluginExternalFunction {
    fn is_external(&self) -> bool {
        self.external
    }

    fn get_function_id(&self) -> String {
        self.id.clone()
    }

    fn get_function_name(&self) -> String {
        self.name.clone()
    }

    fn get_opdecl(&self) -> Cow<'static, OpDecl> {
        Cow::Owned(extplugin_rsjs_bridge::rsjs_bridge_opdecl())
    }

    fn get_pre_run_js(&self) -> Option<String> {
        None
    }
}

/// Core representation of an external plugin package.
/// Holds the package ID, name, functions list, package JavaScript code, and external flag.
#[derive(Debug, Clone)]
pub struct CorePluginExternalPackage {
    /// Unique ID of the package
    pub id: String,
    /// Package name
    pub name: String,
    /// List of external functions included in the package
    pub functions: Vec<CorePluginExternalFunction>,
    /// Package JavaScript code to be executed
    pub package_js: String,
    /// The plugin is external (always true for this type)
    pub external: bool,
}

impl CorePluginExternalPackage {
    /// Creates a new `CorePluginExternalPackage`.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier for the package.
    /// * `name` - The name of the package.
    /// * `functions` - A vector of `CorePluginExternalFunction` instances included in this package.
    /// * `package_js` - The JavaScript code for the package.
    pub fn new(
        id: String,
        name: String,
        functions: Vec<CorePluginExternalFunction>,
        package_js: String,
    ) -> Self {
        Self {
            id,
            name,
            functions,
            package_js,
            external: true,
        }
    }
}

impl PluginPackageTrait for CorePluginExternalPackage {
    type Function = CorePluginExternalFunction;

    fn is_external(&self) -> bool {
        self.external
    }

    fn get_package_id(&self) -> String {
        self.id.clone()
    }

    fn get_package_name(&self) -> String {
        self.name.clone()
    }

    fn get_functions(&self) -> Vec<Self::Function> {
        self.functions.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use deno_core::op2;

    fn dummy_plugin_function() -> crate::proto::sapphillon::v1::PluginFunction {
        crate::proto::sapphillon::v1::PluginFunction {
            function_id: "fid".to_string(),
            function_name: "fname".to_string(),
            description: "desc".to_string(),
            permissions: vec![],
            function_define: None,
        }
    }

    fn dummy_plugin_package() -> crate::proto::sapphillon::v1::PluginPackage {
        crate::proto::sapphillon::v1::PluginPackage {
            package_id: "pid".to_string(),
            package_name: "pname".to_string(),
            package_version: "1.0.0".to_string(),
            description: "desc".to_string(),
            functions: vec![dummy_plugin_function()],
            plugin_store_url: "".to_string(),
            internal_plugin: None,
            verified: None,
            deprecated: None,
            installed_at: None,
            updated_at: None,
        }
    }

    #[op2(fast)]
    fn dummy_op() -> u32 {
        42
    }

    #[test]
    fn test_core_plugin_function_new() {
        let func = CorePluginFunction::new(
            "id".to_string(),
            "name".to_string(),
            "description".to_string(),
            dummy_op(),
            None,
        );
        assert_eq!(func.id, "id");
        assert_eq!(func.name, "name");
        assert_eq!(func.description, "description");
    }

    #[test]
    fn test_core_plugin_function_new_from_plugin_function() {
        let pf = dummy_plugin_function();
        let func = CorePluginFunction::new_from_plugin_function(&pf, dummy_op());
        assert_eq!(func.id, pf.function_id);
        assert_eq!(func.name, pf.function_name);
    }

    #[test]
    fn test_core_plugin_package_new() {
        let f = CorePluginFunction::new(
            "id".to_string(),
            "name".to_string(),
            "desc".to_string(),
            dummy_op(),
            Some("pre_run_js".to_string()),
        );
        let pkg = CorePluginPackage::new("pid".to_string(), "pname".to_string(), vec![f]);
        assert_eq!(pkg.id, "pid");
        assert_eq!(pkg.name, "pname");
        assert_eq!(pkg.functions.len(), 1);
    }

    #[test]
    fn test_core_plugin_package_new_from_plugin_package() {
        let pf = dummy_plugin_function();
        let f = CorePluginFunction::new_from_plugin_function(&pf, dummy_op());
        let pp = dummy_plugin_package();
        let pkg = CorePluginPackage::new_from_plugin_package(&pp, vec![f]);
        assert_eq!(pkg.id, pp.package_id);
        assert_eq!(pkg.name, pp.package_name);
        assert_eq!(pkg.functions.len(), 1);
    }

    #[test]
    fn test_core_plugin_external_function_new() {
        let func = CorePluginExternalFunction::new(
            "ext_id".to_string(),
            "ext_name".to_string(),
            "ext_description".to_string(),
            "console.log('external');".to_string(),
        );
        assert_eq!(func.id, "ext_id");
        assert_eq!(func.name, "ext_name");
        assert_eq!(func.description, "ext_description");
        assert_eq!(func.package_js, "console.log('external');");
        assert!(func.external);
    }

    #[test]
    fn test_core_plugin_external_package_new() {
        let f1 = CorePluginExternalFunction::new(
            "ext_id1".to_string(),
            "ext_name1".to_string(),
            "ext_desc1".to_string(),
            "const a = 1;".to_string(),
        );
        let f2 = CorePluginExternalFunction::new(
            "ext_id2".to_string(),
            "ext_name2".to_string(),
            "ext_desc2".to_string(),
            "const b = 2;".to_string(),
        );
        let pkg = CorePluginExternalPackage::new(
            "ext_pkg_id".to_string(),
            "ext_pkg_name".to_string(),
            vec![f1, f2],
            "export default {};".to_string(),
        );
        assert_eq!(pkg.id, "ext_pkg_id");
        assert_eq!(pkg.name, "ext_pkg_name");
        assert_eq!(pkg.functions.len(), 2);
        assert_eq!(pkg.package_js, "export default {};");
        assert!(pkg.external);
    }

    #[test]
    fn test_plugin_function_trait_is_external() {
        // Test internal plugin function
        let internal_func = CorePluginFunction::new(
            "internal_id".to_string(),
            "internal_name".to_string(),
            "internal_description".to_string(),
            dummy_op(),
            None,
        );
        assert!(!internal_func.is_external());
    }

    #[test]
    fn test_plugin_function_trait_get_function_id() {
        let func = CorePluginFunction::new(
            "test_id".to_string(),
            "test_name".to_string(),
            "test_description".to_string(),
            dummy_op(),
            None,
        );
        assert_eq!(func.get_function_id(), "test_id");
    }

    #[test]
    fn test_plugin_function_trait_get_function_name() {
        let func = CorePluginFunction::new(
            "test_id".to_string(),
            "test_name".to_string(),
            "test_description".to_string(),
            dummy_op(),
            None,
        );
        assert_eq!(func.get_function_name(), "test_name");
    }

    #[test]
    fn test_plugin_function_trait_get_opdecl() {
        let func = CorePluginFunction::new(
            "test_id".to_string(),
            "test_name".to_string(),
            "test_description".to_string(),
            dummy_op(),
            None,
        );
        let opdecl = func.get_opdecl();
        // OpDecl should be cloned successfully
        assert_eq!(opdecl.name, "dummy_op");
    }

    #[test]
    fn test_plugin_function_trait_get_pre_run_js() {
        // Test with Some pre_run_js
        let func_with_js = CorePluginFunction::new(
            "test_id".to_string(),
            "test_name".to_string(),
            "test_description".to_string(),
            dummy_op(),
            Some("console.log('pre-run');".to_string()),
        );
        assert_eq!(
            func_with_js.get_pre_run_js(),
            Some("console.log('pre-run');".to_string())
        );

        // Test with None pre_run_js
        let func_without_js = CorePluginFunction::new(
            "test_id".to_string(),
            "test_name".to_string(),
            "test_description".to_string(),
            dummy_op(),
            None,
        );
        assert_eq!(func_without_js.get_pre_run_js(), None);
    }

    #[test]
    fn test_plugin_function_trait_all_methods() {
        // Comprehensive test of all trait methods together
        let func = CorePluginFunction::new(
            "comprehensive_id".to_string(),
            "comprehensive_name".to_string(),
            "comprehensive_description".to_string(),
            dummy_op(),
            Some("const x = 42;".to_string()),
        );

        assert!(!func.is_external());
        assert_eq!(func.get_function_id(), "comprehensive_id");
        assert_eq!(func.get_function_name(), "comprehensive_name");
        assert_eq!(func.get_pre_run_js(), Some("const x = 42;".to_string()));
        
        let opdecl = func.get_opdecl();
        assert_eq!(opdecl.name, "dummy_op");
    }

    #[test]
    fn test_plugin_package_trait_is_external_with_internal_functions() {
        let func = CorePluginFunction::new(
            "internal_id".to_string(),
            "internal_name".to_string(),
            "internal_description".to_string(),
            dummy_op(),
            None,
        );
        let pkg = CorePluginPackage::new(
            "pkg_id".to_string(),
            "pkg_name".to_string(),
            vec![func],
        );
        // Package with internal functions should not be external
        assert!(!pkg.is_external());
    }

    #[test]
    fn test_plugin_package_trait_is_external_with_empty_functions() {
        let pkg = CorePluginPackage::new(
            "empty_pkg_id".to_string(),
            "empty_pkg_name".to_string(),
            vec![],
        );
        // Package with no functions should not be external
        assert!(!pkg.is_external());
    }

    #[test]
    fn test_plugin_package_trait_get_package_id() {
        let func = CorePluginFunction::new(
            "func_id".to_string(),
            "func_name".to_string(),
            "func_description".to_string(),
            dummy_op(),
            None,
        );
        let pkg = CorePluginPackage::new(
            "test_package_id".to_string(),
            "test_package_name".to_string(),
            vec![func],
        );
        assert_eq!(pkg.get_package_id(), "test_package_id");
    }

    #[test]
    fn test_plugin_package_trait_get_package_name() {
        let func = CorePluginFunction::new(
            "func_id".to_string(),
            "func_name".to_string(),
            "func_description".to_string(),
            dummy_op(),
            None,
        );
        let pkg = CorePluginPackage::new(
            "test_package_id".to_string(),
            "test_package_name".to_string(),
            vec![func],
        );
        assert_eq!(pkg.get_package_name(), "test_package_name");
    }

    #[test]
    fn test_plugin_package_trait_get_functions() {
        let func1 = CorePluginFunction::new(
            "func1_id".to_string(),
            "func1_name".to_string(),
            "func1_description".to_string(),
            dummy_op(),
            None,
        );
        let func2 = CorePluginFunction::new(
            "func2_id".to_string(),
            "func2_name".to_string(),
            "func2_description".to_string(),
            dummy_op(),
            Some("pre_run".to_string()),
        );
        let pkg = CorePluginPackage::new(
            "pkg_id".to_string(),
            "pkg_name".to_string(),
            vec![func1.clone(), func2.clone()],
        );
        
        let functions = pkg.get_functions();
        assert_eq!(functions.len(), 2);
        assert_eq!(functions[0].id, "func1_id");
        assert_eq!(functions[1].id, "func2_id");
    }

    #[test]
    fn test_plugin_package_trait_all_methods() {
        // Comprehensive test of all trait methods together
        let func1 = CorePluginFunction::new(
            "comprehensive_func1_id".to_string(),
            "comprehensive_func1_name".to_string(),
            "comprehensive_func1_description".to_string(),
            dummy_op(),
            None,
        );
        let func2 = CorePluginFunction::new(
            "comprehensive_func2_id".to_string(),
            "comprehensive_func2_name".to_string(),
            "comprehensive_func2_description".to_string(),
            dummy_op(),
            Some("const y = 100;".to_string()),
        );
        let pkg = CorePluginPackage::new(
            "comprehensive_pkg_id".to_string(),
            "comprehensive_pkg_name".to_string(),
            vec![func1, func2],
        );

        assert!(!pkg.is_external());
        assert_eq!(pkg.get_package_id(), "comprehensive_pkg_id");
        assert_eq!(pkg.get_package_name(), "comprehensive_pkg_name");
        
        let functions = pkg.get_functions();
        assert_eq!(functions.len(), 2);
        assert_eq!(functions[0].get_function_id(), "comprehensive_func1_id");
        assert_eq!(functions[1].get_function_id(), "comprehensive_func2_id");
    }

    #[test]
    fn test_external_plugin_function_trait_is_external() {
        let ext_func = CorePluginExternalFunction::new(
            "ext_id".to_string(),
            "ext_name".to_string(),
            "ext_description".to_string(),
            "console.log('external');".to_string(),
        );
        // External function should always return true
        assert!(ext_func.is_external());
    }

    #[test]
    fn test_external_plugin_function_trait_get_function_id() {
        let ext_func = CorePluginExternalFunction::new(
            "external_test_id".to_string(),
            "external_test_name".to_string(),
            "external_test_description".to_string(),
            "const x = 1;".to_string(),
        );
        assert_eq!(ext_func.get_function_id(), "external_test_id");
    }

    #[test]
    fn test_external_plugin_function_trait_get_function_name() {
        let ext_func = CorePluginExternalFunction::new(
            "external_test_id".to_string(),
            "external_test_name".to_string(),
            "external_test_description".to_string(),
            "const x = 1;".to_string(),
        );
        assert_eq!(ext_func.get_function_name(), "external_test_name");
    }

    #[test]
    fn test_external_plugin_function_trait_get_opdecl() {
        let ext_func = CorePluginExternalFunction::new(
            "external_test_id".to_string(),
            "external_test_name".to_string(),
            "external_test_description".to_string(),
            "const x = 1;".to_string(),
        );
        let opdecl = ext_func.get_opdecl();
        // OpDecl should be rsjs_bridge_opdecl
        assert_eq!(opdecl.name, "rsjs_bridge_opdecl");
    }

    #[test]
    fn test_external_plugin_function_trait_get_pre_run_js() {
        let ext_func = CorePluginExternalFunction::new(
            "external_test_id".to_string(),
            "external_test_name".to_string(),
            "external_test_description".to_string(),
            "const x = 1;".to_string(),
        );
        // External functions should always return None for pre_run_js
        assert_eq!(ext_func.get_pre_run_js(), None);
    }

    #[test]
    fn test_external_plugin_function_trait_all_methods() {
        // Comprehensive test of all trait methods for external function
        let ext_func = CorePluginExternalFunction::new(
            "comprehensive_ext_id".to_string(),
            "comprehensive_ext_name".to_string(),
            "comprehensive_ext_description".to_string(),
            "export default { test: () => 'hello' };".to_string(),
        );

        assert!(ext_func.is_external());
        assert_eq!(ext_func.get_function_id(), "comprehensive_ext_id");
        assert_eq!(ext_func.get_function_name(), "comprehensive_ext_name");
        assert_eq!(ext_func.get_pre_run_js(), None);

        let opdecl = ext_func.get_opdecl();
        assert_eq!(opdecl.name, "rsjs_bridge_opdecl");
    }

    #[test]
    fn test_external_plugin_package_trait_is_external() {
        let func = CorePluginExternalFunction::new(
            "ext_func_id".to_string(),
            "ext_func_name".to_string(),
            "ext_func_description".to_string(),
            "const a = 1;".to_string(),
        );
        let pkg = CorePluginExternalPackage::new(
            "ext_pkg_id".to_string(),
            "ext_pkg_name".to_string(),
            vec![func],
            "export default {};".to_string(),
        );
        // External package should always return true
        assert!(pkg.is_external());
    }

    #[test]
    fn test_external_plugin_package_trait_is_external_with_empty_functions() {
        let pkg = CorePluginExternalPackage::new(
            "empty_ext_pkg_id".to_string(),
            "empty_ext_pkg_name".to_string(),
            vec![],
            "export default {};".to_string(),
        );
        // External package should always return true even if empty
        assert!(pkg.is_external());
    }

    #[test]
    fn test_external_plugin_package_trait_get_package_id() {
        let func = CorePluginExternalFunction::new(
            "ext_func_id".to_string(),
            "ext_func_name".to_string(),
            "ext_func_description".to_string(),
            "const a = 1;".to_string(),
        );
        let pkg = CorePluginExternalPackage::new(
            "test_ext_package_id".to_string(),
            "test_ext_package_name".to_string(),
            vec![func],
            "export default {};".to_string(),
        );
        assert_eq!(pkg.get_package_id(), "test_ext_package_id");
    }

    #[test]
    fn test_external_plugin_package_trait_get_package_name() {
        let func = CorePluginExternalFunction::new(
            "ext_func_id".to_string(),
            "ext_func_name".to_string(),
            "ext_func_description".to_string(),
            "const a = 1;".to_string(),
        );
        let pkg = CorePluginExternalPackage::new(
            "test_ext_package_id".to_string(),
            "test_ext_package_name".to_string(),
            vec![func],
            "export default {};".to_string(),
        );
        assert_eq!(pkg.get_package_name(), "test_ext_package_name");
    }

    #[test]
    fn test_external_plugin_package_trait_get_functions() {
        let func1 = CorePluginExternalFunction::new(
            "ext_func1_id".to_string(),
            "ext_func1_name".to_string(),
            "ext_func1_description".to_string(),
            "const a = 1;".to_string(),
        );
        let func2 = CorePluginExternalFunction::new(
            "ext_func2_id".to_string(),
            "ext_func2_name".to_string(),
            "ext_func2_description".to_string(),
            "const b = 2;".to_string(),
        );
        let pkg = CorePluginExternalPackage::new(
            "ext_pkg_id".to_string(),
            "ext_pkg_name".to_string(),
            vec![func1.clone(), func2.clone()],
            "export default {};".to_string(),
        );

        let functions = pkg.get_functions();
        assert_eq!(functions.len(), 2);
        assert_eq!(functions[0].id, "ext_func1_id");
        assert_eq!(functions[1].id, "ext_func2_id");
    }

    #[test]
    fn test_external_plugin_package_trait_all_methods() {
        // Comprehensive test of all trait methods for external package
        let func1 = CorePluginExternalFunction::new(
            "comprehensive_ext_func1_id".to_string(),
            "comprehensive_ext_func1_name".to_string(),
            "comprehensive_ext_func1_description".to_string(),
            "const x = 100;".to_string(),
        );
        let func2 = CorePluginExternalFunction::new(
            "comprehensive_ext_func2_id".to_string(),
            "comprehensive_ext_func2_name".to_string(),
            "comprehensive_ext_func2_description".to_string(),
            "const y = 200;".to_string(),
        );
        let pkg = CorePluginExternalPackage::new(
            "comprehensive_ext_pkg_id".to_string(),
            "comprehensive_ext_pkg_name".to_string(),
            vec![func1, func2],
            "export default { init: () => console.log('initialized') };".to_string(),
        );

        assert!(pkg.is_external());
        assert_eq!(pkg.get_package_id(), "comprehensive_ext_pkg_id");
        assert_eq!(pkg.get_package_name(), "comprehensive_ext_pkg_name");

        let functions = pkg.get_functions();
        assert_eq!(functions.len(), 2);
        assert_eq!(functions[0].get_function_id(), "comprehensive_ext_func1_id");
        assert_eq!(functions[1].get_function_id(), "comprehensive_ext_func2_id");
        
        // Verify all functions are external
        assert!(functions[0].is_external());
        assert!(functions[1].is_external());
    }
}
