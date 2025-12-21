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

//! Generate EntryPoint Function to run External Plugins

use serde::{Serialize, Deserialize};
use serde_json::{Value, to_string, from_str};
use std::collections::HashMap;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct RsJsBridgeArgs {
    // Function Name
    pub func_name: String,
    // Function Args and data
    pub args: HashMap<String, Value>,
}

impl RsJsBridgeArgs {
    #[allow(dead_code)]
    pub fn new_from_str(s: &str) -> Result<Self> {
        Ok(from_str(s)?)
    }
    
    #[allow(dead_code)]
    pub fn to_string(&self) -> Result<String> {
        Ok(to_string(self)?)
    }
}


#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct RsJsBridgeReturns {
    // Return data
    pub args: HashMap<String, Value>,
}

impl RsJsBridgeReturns {
    #[allow(dead_code)]
    pub fn new_from_str(s: &str) -> Result<Self> {
        Ok(from_str(s)?)
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> Result<String> {
        Ok(to_string(self)?)
    }
}