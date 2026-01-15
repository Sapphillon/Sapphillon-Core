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

//! # ext_plugin
//!
//! External plugin system for Sapphillon-Core that enables loading and executing
//! JavaScript/TypeScript plugins with fine-grained permission control.
//!
//! ## Overview
//!
//! This crate provides high-level APIs for working with Sapphillon plugins:
//!
//! - [`SapphillonPackage`]: Parse and validate plugin package scripts
//! - Package metadata types: [`Meta`], [`FunctionSchema`], [`Permission`], [`Parameter`], [`ReturnInfo`]
//! - Bridge types for Rust-JS interoperability: [`RsJsBridgeArgs`], [`RsJsBridgeReturns`]
//!
//! ## Examples
//!
//! ### Parsing a Plugin Package
//!
//! ```rust
//! use ext_plugin::SapphillonPackage;
//!
//! # fn main() -> anyhow::Result<()> {
//! // Plugin script that exports a Sapphillon.Package
//! let package_script = r#"
//!     globalThis.Sapphillon = {
//!         Package: {
//!             meta: {
//!                 name: "example-plugin",
//!                 version: "1.0.0",
//!                 description: "An example plugin",
//!                 author_id: "com.example",
//!                 package_id: "com.example.example-plugin"
//!             },
//!             functions: {
//!                 greet: {
//!                     description: "Greets a user",
//!                     permissions: [],
//!                     parameters: [{
//!                         idx: 0,
//!                         name: "username",
//!                         type: "string",
//!                         description: "Name to greet"
//!                     }],
//!                     returns: [{
//!                         idx: 0,
//!                         type: "string",
//!                         description: "Greeting message"
//!                     }],
//!                     handler: (args) => `Hello, ${args.username}!`
//!                 }
//!             }
//!         }
//!     };
//! "#;
//!
//! // Parse the package
//! let package = SapphillonPackage::new(package_script)?;
//!
//! // Access metadata
//! println!("Plugin: {} v{}", package.meta.name, package.meta.version);
//! println!("Functions: {}", package.functions.len());
//!
//! // Inspect function schemas
//! for (func_name, func_schema) in &package.functions {
//!     println!("  - {}: {}", func_name, func_schema.description);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ### Using Bridge Types for Interoperability
//!
//! The new `execute()` method simplifies execution:
//!
//! ```rust
//! use ext_plugin::{SapphillonPackage, RsJsBridgeArgs};
//! use serde_json::json;
//!
//! # fn main() -> anyhow::Result<()> {
//! # let package_script = r#"
//! #     globalThis.Sapphillon = {
//! #         Package: {
//! #             meta: {
//! #                 name: "example-plugin",
//! #                 version: "1.0.0",
//! #                 description: "An example plugin",
//! #                 author_id: "com.example",
//! #                 package_id: "com.example.example-plugin"
//! #             },
//! #             functions: {
//! #                 greet: {
//! #                     description: "Greets a user",
//! #                     permissions: [],
//! #                     parameters: [{
//! #                         idx: 0,
//! #                         name: "username",
//! #                         type: "string",
//! #                         description: "Name to greet"
//! #                     }],
//! #                     returns: [{
//! #                         idx: 0,
//! #                         type: "string",
//! #                         description: "Greeting message"
//! #                     }],
//! #                     handler: (args) => `Hello, ${args.username}!`
//! #                 }
//! #             }
//! #         }
//! #     };
//! # "#;
//! // Parse the package
//! let package = SapphillonPackage::new(package_script)?;
//!
//! // Prepare arguments
//! let args = RsJsBridgeArgs {
//!     func_name: "greet".to_string(),
//!     args: vec![
//!         ("username".to_string(), json!("Alice")),
//!     ].into_iter().collect(),
//! };
//!
//! // Execute the function (async example)
//! # tokio::runtime::Runtime::new()?.block_on(async {
//! let returns = package.execute(args, &None).await?;
//!
//! if let Some(message) = returns.args.get("result") {
//!     println!("Result: {}", message);
//! }
//! # anyhow::Ok(())
//! # })?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Working with Permissions
//!
//! ```rust
//! use ext_plugin::SapphillonPackage;
//!
//! # fn main() -> anyhow::Result<()> {
//! let package_script = r#"
//!     globalThis.Sapphillon = {
//!         Package: {
//!             meta: {
//!                 name: "fs-plugin",
//!                 version: "1.0.0",
//!                 description: "File system operations",
//!                 author_id: "com.example",
//!                 package_id: "com.example.fs-plugin"
//!             },
//!             functions: {
//!                 readConfig: {
//!                     description: "Read configuration file",
//!                     permissions: [{
//!                         type: "fs:read",
//!                         resource: "/etc/config.json"
//!                     }],
//!                     parameters: [],
//!                     returns: []
//!                 }
//!             }
//!         }
//!     };
//! "#;
//!
//! let package = SapphillonPackage::new(package_script)?;
//!
//! // Check required permissions for each function
//! for (func_name, func_schema) in &package.functions {
//!     if !func_schema.permissions.is_empty() {
//!         println!("Function '{}' requires permissions:", func_name);
//!         for perm in &func_schema.permissions {
//!             println!("  - {}: {}", perm.perm_type, perm.resource);
//!         }
//!     }
//! }
//! # Ok(())
//! # }
//! ```

mod cert_store;
mod extplugin_runner_process;
mod module_loader;
mod npm;
mod package;
mod parse_package;
mod permissions;
mod runner;
mod rust_js_bridge;
mod worker;

// Public API: High-level SapphillonPackage types
pub use package::{FunctionSchema, Meta, Parameter, Permission, ReturnInfo, SapphillonPackage};

// Public API: Bridge types for Rust-JS interoperability
pub use rust_js_bridge::{RsJsBridgeArgs, RsJsBridgeReturns};

// Public API: Runner functions for executing JavaScript code
pub use runner::{run_js, run_js_with_string_arg};

pub use extplugin_runner_process::{
    ExternalPluginRunRequest, ExternalPluginRunResponse, IpcPermission, extplugin_client,
    extplugin_server,
};
