// Sapphillon-Core
// Copyright 2025 Yuta Takahashi
//
// This file is part of Sapphillon-Core
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Alternatively, the contents of this file may be used under the terms
// of the GNU General Public License Version 3 or later (the "GPL").

#![cfg(not(doctest))]
pub use crate::error::Error;

pub mod core;
pub mod error;
pub mod permission;
pub mod plugin;
pub mod proto;
pub mod runtime;
pub mod utils;
pub mod workflow;
