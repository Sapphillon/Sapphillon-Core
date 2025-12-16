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
