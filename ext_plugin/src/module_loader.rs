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

//! Dummy ModuleLoader that doesn't support ES module imports

use deno_error::JsErrorBox;
use deno_runtime::deno_core::{
    ModuleLoadOptions, ModuleLoadResponse, ModuleLoader, ModuleSpecifier, ResolutionKind,
};

/// A module loader that doesn't support loading any modules.
/// Only inline script execution via `execute_script` is supported.
pub struct NoopModuleLoader;

impl ModuleLoader for NoopModuleLoader {
    fn resolve(
        &self,
        specifier: &str,
        _referrer: &str,
        _kind: ResolutionKind,
    ) -> std::result::Result<ModuleSpecifier, JsErrorBox> {
        // Return the specifier as-is if it's a valid URL
        ModuleSpecifier::parse(specifier)
            .map_err(|e| JsErrorBox::generic(format!("Module resolution not supported: {e}")))
    }

    fn load(
        &self,
        module_specifier: &ModuleSpecifier,
        _maybe_referrer: Option<&deno_runtime::deno_core::ModuleLoadReferrer>,
        _options: ModuleLoadOptions,
    ) -> ModuleLoadResponse {
        let specifier = module_specifier.clone();
        ModuleLoadResponse::Sync(Err(JsErrorBox::generic(format!(
            "Module loading not supported: {specifier}"
        ))))
    }
}
