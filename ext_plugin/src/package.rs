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
use deno_core::serde_v8;

#[derive(Debug, Deserialize)]
pub struct Package {
    meta: Meta,
    functions: HashMap<String, FunctionSchema>,
}

#[derive(Debug, Deserialize)]
pub struct Meta {
    name: String,
    version: String,
    description: String,
    package_id: String,
}

#[derive(Debug, Deserialize)]
pub struct FunctionSchema {
    permissions: Vec<Permission>,
    description: String,
    parameters: Vec<Parameter>,
    returns: Vec<ReturnInfo>,
}

#[derive(Debug, Deserialize)]
pub struct Permission {
    #[serde(rename = "type")]
    perm_type: String,
    resource: String,
}

#[derive(Debug, Deserialize)]
pub struct Parameter {
    name: String,
    #[serde(rename = "type")]
    param_type: String,
    description: String,
}

#[derive(Debug, Deserialize)]
pub struct ReturnInfo {
    #[serde(rename = "type")]
    return_type: String,
    description: String,
}