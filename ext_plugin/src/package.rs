// Sapphillon-Core
// Copyright 2025 Yuta Takahashi
//
// This file is part of Sapphillon-Core
//
// Sapphillon-Core is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! Define Package info

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::to_string as to_json_string;
use std::collections::HashMap;
use std::fmt;

/// Parsed plugin package schema.
///
/// This is the Rust representation of `Sapphillon.Package` exported from the
/// package script (JavaScript) and deserialized via `serde_v8`.
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct SapphillonPackage {
    /// Package metadata.
    pub meta: Meta,
    /// Function schemas keyed by function name.
    pub functions: HashMap<String, FunctionSchema>,
    /// The original JavaScript package script.
    #[serde(skip, default)]
    pub package_script: String,
}

/// Package metadata (typically derived from `package.toml`).
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Meta {
    /// Human-readable package name.
    pub name: String,
    /// Semantic version string.
    pub version: String,
    /// Package description.
    pub description: String,
    /// Unique package identifier (e.g. reverse domain notation).
    pub package_id: String,
}

/// Function schema (typically derived from JSDoc).
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct FunctionSchema {
    /// Permission requirements to execute the function.
    pub permissions: Vec<Permission>,
    /// Function description.
    pub description: String,
    /// Parameter list.
    pub parameters: Vec<Parameter>,
    /// Return value(s) information.
    pub returns: Vec<ReturnInfo>,
}

/// Permission requirement entry.
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Permission {
    #[serde(rename = "type")]
    /// Permission type string.
    pub perm_type: String,
    /// Resource scope for the permission.
    pub resource: String,
}

/// Function parameter entry.
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Parameter {
    /// Parameter index (order).
    ///
    /// This is useful when the source schema is represented as an object/map
    /// and ordering information must be preserved.
    #[serde(default)]
    pub idx: usize,
    /// Parameter name.
    pub name: String,
    #[serde(rename = "type")]
    /// Parameter type string.
    pub param_type: String,
    /// Parameter description.
    pub description: String,
}

/// Function return value entry.
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct ReturnInfo {
    /// Return value index (order).
    #[serde(default)]
    pub idx: usize,
    #[serde(rename = "type")]
    /// Return type string.
    pub return_type: String,
    /// Return description.
    pub description: String,
}

impl fmt::Display for SapphillonPackage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.meta)?;
        if self.functions.is_empty() {
            return write!(f, "functions: (none)");
        }

        writeln!(f, "functions:")?;
        let mut keys: Vec<&String> = self.functions.keys().collect();
        keys.sort();
        for name in keys {
            if let Some(schema) = self.functions.get(name) {
                writeln!(f, "- {name}: {schema}")?;
            }
        }
        Ok(())
    }
}

impl SapphillonPackage {
    pub async fn new_async(package_script: &str) -> Result<SapphillonPackage> {
        let mut package = crate::parse_package::parse_package_info(package_script).await?;
        package.package_script = package_script.to_string();
        Ok(package)
    }

    pub fn new(package_script: &str) -> Result<SapphillonPackage> {
        let rt = tokio::runtime::Runtime::new()?;
        let mut package = rt.block_on(crate::parse_package::parse_package_info(package_script))?;
        package.package_script = package_script.to_string();
        Ok(package)
    }

    /// Generate JavaScript code that installs `globalThis.entrypoint`.
    ///
    /// The generated entrypoint accepts a JSON string of `RsJsBridgeArgs`,
    /// routes the call to the corresponding `Sapphillon.Package` handler,
    /// and returns a JSON string compatible with `RsJsBridgeReturns`.
    fn entrypoint_script(&self) -> serde_json::Result<String> {
        let schema_json = to_json_string(self)?;
        // Keep the JS small and dependency-free; only rely on the already loaded package script.
        const TEMPLATE: &str = r#"
(() => {
    const __schema = __SCHEMA_JSON__;

    const __resolveHandler = (funcName) => {
        const fnEntry = globalThis.Sapphillon?.Package?.functions?.[funcName];
        if (!fnEntry) throw new Error(`Unknown function: ${funcName}`);
        const handler = typeof fnEntry === "function" ? fnEntry : fnEntry.handler;
        if (typeof handler !== "function") throw new Error(`Handler for ${funcName} is not a function`);
        return handler;
    };

    const __orderParams = (schema, rawArgs) => {
        const params = schema?.parameters ?? [];
        return params
            .slice()
            .sort((a, b) => (a?.idx ?? 0) - (b?.idx ?? 0))
            .map((p) => rawArgs?.[p.name]);
    };

    const __buildReturns = (schema, value) => {
        const declared = schema?.returns ?? [];
        const out = {};
        if (declared.length > 1 && Array.isArray(value)) {
            for (const info of declared) {
                const idx = Number.isInteger(info?.idx) ? info.idx : 0;
                out[`ret${idx}`] = value[idx];
            }
            return out;
        }
        out.result = value;
        return out;
    };

    globalThis.entrypoint = async function(entryArg) {
        let payload;
        try {
            payload = JSON.parse(entryArg);
        } catch (err) {
            throw new Error(`Invalid RsJsBridgeArgs JSON: ${err?.message ?? err}`);
        }

        if (!payload || typeof payload.func_name !== "string") {
            throw new Error("RsJsBridgeArgs.func_name must be a string");
        }

        const funcName = payload.func_name;
        const schema = __schema.functions?.[funcName];
        if (!schema) throw new Error(`Function schema not found: ${funcName}`);

        const handler = __resolveHandler(funcName);
        const orderedArgs = __orderParams(schema, payload.args ?? {});

        let result = handler(...orderedArgs);
        if (result && typeof result.then === "function") {
            result = await result;
        }

        const returns = __buildReturns(schema, result);
        return JSON.stringify({ args: returns });
    };
})();
"#;


        Ok(TEMPLATE.replace("__SCHEMA_JSON__", &schema_json))
    }

    /// Execute a package function with the given arguments.
    ///
    /// This method combines the package script with the entrypoint script,
    /// invokes the specified function with the provided arguments, and returns
    /// the result.
    ///
    /// # Arguments
    /// * `args` - The bridge arguments containing the function name and parameters
    /// * `permissions_options` - Optional Deno permissions for the execution
    ///
    /// # Returns
    /// * `Ok(RsJsBridgeReturns)` with the function's return values
    /// * `Err(...)` if execution fails
    ///
    /// # Example
    /// ```rust,ignore
    /// use ext_plugin::{SapphillonPackage, RsJsBridgeArgs};
    /// use serde_json::json;
    ///
    /// let package = SapphillonPackage::new(package_script)?;
    /// let args = RsJsBridgeArgs {
    ///     func_name: "add".to_string(),
    ///     args: vec![
    ///         ("a".to_string(), json!(2)),
    ///         ("b".to_string(), json!(3)),
    ///     ].into_iter().collect(),
    /// };
    /// let result = package.execute(args, &None).await?;
    /// ```
    #[allow(dead_code)]
    pub async fn execute(
        &self,
        args: crate::rust_js_bridge::RsJsBridgeArgs,
        permissions_options: &Option<deno_permissions::PermissionsOptions>,
    ) -> Result<crate::rust_js_bridge::RsJsBridgeReturns> {
        // Combine package script with entrypoint
        let entry_script = self.entrypoint_script()?;
        let script = format!("{}\n{}", self.package_script, entry_script);

        // Serialize arguments
        let input = args.to_string()?;

        // Execute and get output
        let output = crate::runner::run_js_with_string_arg(&script, &input, permissions_options)
            .await
            .map_err(|e| anyhow::anyhow!("JavaScript execution failed: {}", e))?;

        // Deserialize return values
        let returns = crate::rust_js_bridge::RsJsBridgeReturns::new_from_str(&output)?;
        Ok(returns)
    }
}

impl fmt::Display for Meta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "meta: name={} version={} package_id={} description={}",
            self.name, self.version, self.package_id, self.description
        )
    }
}

impl fmt::Display for FunctionSchema {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}; ", self.description)?;
        if self.permissions.is_empty() {
            write!(f, "permissions=(none); ")?;
        } else {
            write!(f, "permissions=[")?;
            for (i, p) in self.permissions.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{p}")?;
            }
            write!(f, "]; ")?;
        }

        write!(f, "params=[")?;
        for (i, p) in self.parameters.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{p}")?;
        }
        write!(f, "]; returns=[")?;
        for (i, r) in self.returns.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{r}")?;
        }
        write!(f, "]")
    }
}

impl fmt::Display for Permission {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.perm_type, self.resource)
    }
}

impl fmt::Display for Parameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{} ({})",
            self.name, self.param_type, self.description
        )
    }
}

impl fmt::Display for ReturnInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.return_type, self.description)
    }
}

#[cfg(test)]
mod tests {
    use super::SapphillonPackage;
    use crate::rust_js_bridge::RsJsBridgeArgs;
    use serde_json::json;

    fn test_package_script() -> String {
        // Ensure the global namespace exists before loading the fixture.
        let fixture = include_str!("test_package.js");
        format!("globalThis.Sapphillon = globalThis.Sapphillon || {{}};\n{fixture}")
    }

    #[tokio::test]
    async fn execute_invokes_handler_and_round_trips_json() {
        let package_script = test_package_script();
        let package = SapphillonPackage::new_async(&package_script)
            .await
            .expect("package creation succeeds");

        let args = RsJsBridgeArgs {
            func_name: "add".to_string(),
            args: vec![("a".to_string(), json!(2)), ("b".to_string(), json!(3))]
                .into_iter()
                .collect(),
        };

        let returns = package
            .execute(args, &None)
            .await
            .expect("execution succeeds");

        assert_eq!(returns.args.get("result"), Some(&json!(5)));
    }

    #[tokio::test]
    async fn package_script_is_stored() {
        let package_script = test_package_script();
        let package = SapphillonPackage::new_async(&package_script)
            .await
            .expect("package creation succeeds");

        assert!(!package.package_script.is_empty());
        assert_eq!(package.package_script, package_script);
    }
}
