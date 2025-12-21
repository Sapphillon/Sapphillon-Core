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

use anyhow::Result;
use deno_permissions::{Permissions, PermissionsOptions};
use deno_runtime::permissions::RuntimePermissionDescriptorParser;
use sapphillon_core::proto::sapphillon::v1::{Permission as SapphillonPermission, PermissionType};
use std::sync::Arc;

pub(crate) fn create_descriptor_parser()
-> Arc<RuntimePermissionDescriptorParser<sys_traits::impls::RealSys>> {
    Arc::new(RuntimePermissionDescriptorParser::<
        sys_traits::impls::RealSys,
    >::new(sys_traits::impls::RealSys))
}

#[allow(dead_code)]
pub(crate) fn create_permissions(
    permissions_options: &Option<PermissionsOptions>,
) -> Result<Permissions> {
    let parser = create_descriptor_parser();
    Ok(Permissions::from_options(
        &*parser,
        &permissions_options.clone().unwrap_or_default(),
    )?)
}

#[allow(dead_code)]
pub(crate) fn permissions_options_from_sapphillon_permissions(
    permissions: &[SapphillonPermission],
) -> PermissionsOptions {
    fn merge_allow_list(target: &mut Option<Vec<String>>, resources: &[String]) {
        if matches!(target, Some(v) if v.is_empty()) {
            // Already "allow all".
            return;
        }

        if resources.is_empty() {
            // Deno semantics: Some(empty) == allow all.
            *target = Some(vec![]);
            return;
        }

        let list = target.get_or_insert_with(Vec::new);
        for r in resources {
            if !list.iter().any(|existing| existing == r) {
                list.push(r.clone());
            }
        }
    }

    let mut options = PermissionsOptions::default();

    let mut allow_read: Option<Vec<String>> = None;
    let mut allow_write: Option<Vec<String>> = None;
    let mut allow_net: Option<Vec<String>> = None;
    let mut allow_run: Option<Vec<String>> = None;

    for permission in permissions {
        let Ok(permission_type) = PermissionType::try_from(permission.permission_type) else {
            continue;
        };

        match permission_type {
            PermissionType::FilesystemRead => {
                merge_allow_list(&mut allow_read, &permission.resource);
            }
            PermissionType::FilesystemWrite => {
                merge_allow_list(&mut allow_write, &permission.resource);
            }
            PermissionType::NetAccess => {
                merge_allow_list(&mut allow_net, &permission.resource);
            }
            PermissionType::Execute => {
                merge_allow_list(&mut allow_run, &permission.resource);
            }
            PermissionType::AllowAll => {
                allow_read = Some(vec![]);
                allow_write = Some(vec![]);
                allow_net = Some(vec![]);
                allow_run = Some(vec![]);
            }
            PermissionType::AllowMcp | PermissionType::Unspecified => {
                // Not a Deno runtime permission; ignore here.
            }
        }
    }

    options.allow_read = allow_read;
    options.allow_write = allow_write;
    options.allow_net = allow_net;
    options.allow_run = allow_run;
    options
}

#[cfg(test)]
mod tests {
    use super::permissions_options_from_sapphillon_permissions;
    use deno_permissions::PermissionsOptions;
    use sapphillon_core::proto::sapphillon::v1::{Permission, PermissionType};

    fn perm(permission_type: PermissionType, resource: Vec<&str>) -> Permission {
        Permission {
            display_name: "".to_string(),
            description: "".to_string(),
            permission_type: permission_type as i32,
            resource: resource.into_iter().map(|s| s.to_string()).collect(),
            permission_level: 0,
        }
    }

    #[test]
    fn empty_list_is_default() {
        let options = permissions_options_from_sapphillon_permissions(&[]);
        assert!(options.allow_read.is_none());
        assert!(options.allow_write.is_none());
        assert!(options.allow_net.is_none());
        assert!(options.allow_run.is_none());
    }

    #[test]
    fn filesystem_read_maps_to_allow_read_and_dedupes() {
        let options = permissions_options_from_sapphillon_permissions(&[perm(
            PermissionType::FilesystemRead,
            vec!["/a", "/b", "/a"],
        )]);

        assert_eq!(
            options.allow_read,
            Some(vec!["/a".to_string(), "/b".to_string()])
        );
        assert!(options.allow_write.is_none());
        assert!(options.allow_net.is_none());
        assert!(options.allow_run.is_none());
    }

    #[test]
    fn multiple_permission_types_merge() {
        let options = permissions_options_from_sapphillon_permissions(&[
            perm(PermissionType::FilesystemRead, vec!["/read"]),
            perm(PermissionType::FilesystemWrite, vec!["/write"]),
            perm(PermissionType::NetAccess, vec!["127.0.0.1:8080"]),
            perm(PermissionType::Execute, vec!["/bin/sh"]),
        ]);

        assert_eq!(options.allow_read, Some(vec!["/read".to_string()]));
        assert_eq!(options.allow_write, Some(vec!["/write".to_string()]));
        assert_eq!(options.allow_net, Some(vec!["127.0.0.1:8080".to_string()]));
        assert_eq!(options.allow_run, Some(vec!["/bin/sh".to_string()]));
    }

    #[test]
    fn empty_resource_means_allow_all_for_that_category() {
        let options = permissions_options_from_sapphillon_permissions(&[
            perm(PermissionType::NetAccess, vec![]),
            perm(PermissionType::NetAccess, vec!["example.com:443"]),
        ]);

        assert_eq!(options.allow_net, Some(vec![]));

        // Ensure it still otherwise behaves like default.
        let default = PermissionsOptions::default();
        assert_eq!(options.allow_read, default.allow_read);
        assert_eq!(options.allow_write, default.allow_write);
        assert_eq!(options.allow_run, default.allow_run);
    }
}
