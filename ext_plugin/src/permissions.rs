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

//! Permission Related Opration

use deno_runtime::permissions::RuntimePermissionDescriptorParser;
use std::sync::Arc;
use deno_permissions::{Permissions, PermissionsOptions};
use anyhow::Result;

pub(crate) fn create_descriptor_parser()
-> Arc<RuntimePermissionDescriptorParser<sys_traits::impls::RealSys>> {
    Arc::new(RuntimePermissionDescriptorParser::<
        sys_traits::impls::RealSys,
    >::new(sys_traits::impls::RealSys))
}

#[allow(dead_code)]
pub(crate) fn create_permissions(permissions_options: &Option<PermissionsOptions>) -> Result<Permissions> {
    let parser = create_descriptor_parser();
    Ok(Permissions::from_options(&*parser, &permissions_options.clone().unwrap_or_default())?)
}
