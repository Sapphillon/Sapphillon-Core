// Sapphillon-Core
// SPDX-FileCopyrightText: 2025 Yuta Takahashi
// SPDX-License-Identifier: MPL-2.0 OR GPL-3.0-or-later

use crate::proto::sapphillon::v1::{PluginFunction, PluginPackage};
use deno_core::OpDecl;
use std::borrow::Cow;

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
    pub external_plugin: bool
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
            external_plugin: false
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
            external_plugin: false
        }
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
}
