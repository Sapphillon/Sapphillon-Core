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

//! Dummy implementations for npm-related traits (not used in this environment)

use node_resolver::errors::{PackageFolderResolveError, PackageNotFoundError};
use node_resolver::{InNpmPackageChecker, NpmPackageFolderResolver, UrlOrPathRef};
use std::path::PathBuf;
use url::Url;

/// A dummy InNpmPackageChecker that always returns false
/// (indicating no specifier is inside an npm package).
#[derive(Debug, Clone, Copy)]
pub struct NoopInNpmPackageChecker;

impl InNpmPackageChecker for NoopInNpmPackageChecker {
    fn in_npm_package(&self, _specifier: &Url) -> bool {
        false
    }
}

/// A dummy NpmPackageFolderResolver that always returns an error
/// (npm packages are not supported in this dummy implementation).
#[derive(Debug, Clone, Copy)]
pub struct NoopNpmPackageFolderResolver;

impl NpmPackageFolderResolver for NoopNpmPackageFolderResolver {
    fn resolve_package_folder_from_package(
        &self,
        specifier: &str,
        referrer: &UrlOrPathRef,
    ) -> std::result::Result<PathBuf, PackageFolderResolveError> {
        Err(PackageNotFoundError {
            package_name: specifier.to_string(),
            referrer: referrer.display(),
            referrer_extra: None,
        }
        .into())
    }
}

/// A dummy ExtNodeSys implementation using the real filesystem.
/// This is needed for MainWorker but we don't support Node.js features.
pub type NoopExtNodeSys = sys_traits::impls::RealSys;
