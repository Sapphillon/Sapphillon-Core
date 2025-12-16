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

use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;

/// Parsed plugin package schema.
///
/// This is the Rust representation of `Sapphillon.Package` exported from the
/// package script (JavaScript) and deserialized via `serde_v8`.
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct SapphillonPackage {
    /// Package metadata.
    pub meta: Meta,
    /// Function schemas keyed by function name.
    pub functions: HashMap<String, FunctionSchema>,
}

/// Package metadata (typically derived from `package.toml`).
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
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
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
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
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct Permission {
    #[serde(rename = "type")]
    /// Permission type string.
    pub perm_type: String,
    /// Resource scope for the permission.
    pub resource: String,
}

/// Function parameter entry.
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct Parameter {
    /// Parameter name.
    pub name: String,
    #[serde(rename = "type")]
    /// Parameter type string.
    pub param_type: String,
    /// Parameter description.
    pub description: String,
}

/// Function return value entry.
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct ReturnInfo {
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
        write!(f, "{}:{} ({})", self.name, self.param_type, self.description)
    }
}

impl fmt::Display for ReturnInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.return_type, self.description)
    }
}