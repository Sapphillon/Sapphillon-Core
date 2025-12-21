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

//! Bridge types used to marshal arguments and return values for external plugins.

use serde::{Serialize, Deserialize};
use serde_json::{Value, to_string, from_str};
use std::collections::HashMap;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct RsJsBridgeArgs {
    /// Name of the JavaScript entry point function to invoke.
    pub func_name: String,
    /// Named arguments serialized as JSON values.
    pub args: HashMap<String, Value>,
}

impl RsJsBridgeArgs {
    /// Deserialize the payload from JSON.
    #[allow(dead_code)]
    pub fn new_from_str(s: &str) -> Result<Self> {
        Ok(from_str(s)?)
    }
    /// Serialize the payload to JSON.
    #[allow(dead_code)]
    pub fn to_string(&self) -> Result<String> {
        Ok(to_string(self)?)
    }
}


#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct RsJsBridgeReturns {
    /// Named return values produced by the plugin.
    pub args: HashMap<String, Value>,
}

impl RsJsBridgeReturns {
    /// Deserialize return values from JSON output produced by JavaScript.
    #[allow(dead_code)]
    pub fn new_from_str(s: &str) -> Result<Self> {
        Ok(from_str(s)?)
    }

    /// Serialize return values to JSON for downstream consumers.
    #[allow(dead_code)]
    pub fn to_string(&self) -> Result<String> {
        Ok(to_string(self)?)
    }
}

#[cfg(test)]
mod tests {
    use super::{RsJsBridgeArgs, RsJsBridgeReturns};
    use serde_json::json;

    #[test]
    fn args_serialize_round_trip() -> Result<(), anyhow::Error> {
        let args = RsJsBridgeArgs {
            func_name: "handleEvent".to_string(),
            args: vec![
                ("id".to_string(), json!(42)),
                ("payload".to_string(), json!({"flag": true})),
            ]
            .into_iter()
            .collect(),
        };

        let serialized = args.to_string()?;
        let parsed = RsJsBridgeArgs::new_from_str(&serialized)?;
        assert_eq!(args, parsed);
        Ok(())
    }

    #[test]
    fn returns_serialize_round_trip() -> Result<(), anyhow::Error> {
        let returns = RsJsBridgeReturns {
            args: vec![
                ("status".to_string(), json!("ok")),
                ("data".to_string(), json!([1, 2, 3])),
            ]
            .into_iter()
            .collect(),
        };

        let serialized = returns.to_string()?;
        let parsed = RsJsBridgeReturns::new_from_str(&serialized)?;
        assert_eq!(returns, parsed);
        Ok(())
    }
}