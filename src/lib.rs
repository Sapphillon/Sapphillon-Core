// Sapphillon-Core
// SPDX-FileCopyrightText: 2025 Yuta Takahashi
// SPDX-License-Identifier: MPL-2.0 OR GPL-3.0-or-later

#![cfg(not(doctest))]

pub use crate::error::Error;
pub use deno_core;
pub use deno_error;
pub use deno_runtime;
pub use deno_semver;
pub use node_resolver;
pub use proto::pbjson;
pub use proto::prost;
pub use proto::prost_types;
pub use proto::tonic;
pub use proto::tonic_prost;
pub use sys_traits;

pub mod core;
pub mod error;
pub mod permission;
pub mod plugin;
pub use ext_plugin;
pub use proto;
pub mod extplugin_rsjs_bridge;
pub mod runtime;
pub mod utils;
pub mod workflow;

/// Initialize tracing subscriber for tests.
/// Uses `Once` to ensure single initialization across all tests.
/// Respects `RUST_LOG` environment variable, defaults to `warn` level.
#[cfg(test)]
fn init_test_logging() {
    use std::sync::Once;
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        let filter = tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("trace"));
        tracing_subscriber::fmt()
            .with_env_filter(filter)
            .with_test_writer()
            .init();
    });
}
