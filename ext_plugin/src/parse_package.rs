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

//! Parse Package Infomation

use anyhow::Result;
use crate::package::SapphillonPackage;
use deno_core::{
    v8, JsRuntime, RuntimeOptions, serde_v8
};
use deno_core::scope;


pub async fn parse_package_info(package_script: &str) -> Result<SapphillonPackage> {
    let package_script = format!("{package_script}\nSapphillon.Package;");

    let mut runtime = JsRuntime::new(RuntimeOptions::default());
    let output = runtime.execute_script("<init>", package_script)?;

    // Use the runtime's handle scope (returns a pinned HandleScope reference)
    scope!(scope, &mut runtime);
    let local = v8::Local::new(scope, output);
    let package: SapphillonPackage = serde_v8::from_v8(scope, local)?;
    Ok(package)
}

#[cfg(test)]
mod tests {
    use super::parse_package_info;
    use crate::package::{
        FunctionSchema, Meta, Parameter, Permission, ReturnInfo, SapphillonPackage,
    };
    use std::collections::HashMap;

    fn expected_test_package() -> SapphillonPackage {
        let meta = Meta {
            name: "math-plugin".to_string(),
            version: "1.0.0".to_string(),
            description: "".to_string(),
            package_id: "com.example".to_string(),
        };

        let add = FunctionSchema {
            permissions: vec![Permission {
                perm_type: "FileSystemRead".to_string(),
                resource: "/etc".to_string(),
            }],
            description: "Adds two numbers.".to_string(),
            parameters: vec![
                Parameter {
                    name: "a".to_string(),
                    param_type: "number".to_string(),
                    description: "The number to be added to".to_string(),
                },
                Parameter {
                    name: "b".to_string(),
                    param_type: "number".to_string(),
                    description: "The number to add".to_string(),
                },
            ],
            returns: vec![ReturnInfo {
                return_type: "number".to_string(),
                description: "The sum".to_string(),
            }],
        };

        let mul = FunctionSchema {
            permissions: vec![Permission {
                perm_type: "FileSystemRead".to_string(),
                resource: "/etc".to_string(),
            }],
            description: "Multiplies two numbers.".to_string(),
            parameters: vec![
                Parameter {
                    name: "a".to_string(),
                    param_type: "number".to_string(),
                    description: "The first factor".to_string(),
                },
                Parameter {
                    name: "b".to_string(),
                    param_type: "number".to_string(),
                    description: "The second factor".to_string(),
                },
            ],
            returns: vec![ReturnInfo {
                return_type: "number".to_string(),
                description: "The product".to_string(),
            }],
        };

        let mut functions = HashMap::new();
        functions.insert("add".to_string(), add);
        functions.insert("mul".to_string(), mul);

        SapphillonPackage { meta, functions }
    }

    #[tokio::test]
    async fn parse_package_info_parses_test_package_js() {
        let fixture = include_str!("test_package.js");
        let package_script = format!(
            "globalThis.Sapphillon = globalThis.Sapphillon || {{}};\n{fixture}",
        );
        let actual = parse_package_info(&package_script)
            .await
            .expect("parse_package_info should succeed");
        let expected = expected_test_package();
        assert_eq!(actual, expected);
    }
}
