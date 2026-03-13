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
//

//! Root certificate store provider for TLS/HTTPS support

use anyhow::Result;
use deno_error::JsErrorBox;
use deno_lib::args::get_root_cert_store;
use deno_runtime::deno_tls::RootCertStoreProvider;
use deno_runtime::deno_tls::rustls::RootCertStore;
use once_cell::sync::OnceCell;

/// Root certificate store provider for Sapphillon.
/// Lazily initializes the root certificate store on first access.
pub struct SapphillonRootCertStoreProvider {
    cell: OnceCell<RootCertStore>,
}

impl SapphillonRootCertStoreProvider {
    pub fn new() -> Self {
        Self {
            cell: Default::default(),
        }
    }
}

impl Default for SapphillonRootCertStoreProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl RootCertStoreProvider for SapphillonRootCertStoreProvider {
    fn get_or_try_init(&self) -> Result<&RootCertStore, JsErrorBox> {
        self.cell
            .get_or_try_init(|| get_root_cert_store(None, None, None))
            .map_err(|e| JsErrorBox::generic(e.to_string()))
    }
}
