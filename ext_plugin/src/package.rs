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

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct SapphillonPackage {
    pub meta: Meta,
    pub functions: HashMap<String, FunctionSchema>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Meta {
    pub name: String,
    pub version: String,
    pub description: String,
    pub package_id: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct FunctionSchema {
    pub permissions: Vec<Permission>,
    pub description: String,
    pub parameters: Vec<Parameter>,
    pub returns: Vec<ReturnInfo>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Permission {
    #[serde(rename = "type")]
    pub perm_type: String,
    pub resource: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Parameter {
    pub name: String,
    #[serde(rename = "type")]
    pub param_type: String,
    pub description: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct ReturnInfo {
    #[serde(rename = "type")]
    pub return_type: String,
    pub description: String,
}