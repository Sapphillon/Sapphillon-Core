// Sapphillon-Core
// SPDX-FileCopyrightText: 2025 Yuta Takahashi
// SPDX-License-Identifier: MPL-2.0 OR GPL-3.0-or-later

use deno_core::op2;

#[op2]
#[string]
pub(crate) fn rsjs_bridge_opdecl() -> String {
    "string".to_string()
}
