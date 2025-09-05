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

use crate::proto::sapphillon::v1 as sapphillon_v1;

use crate::utils::{check_path::paths_cover_by_ancestor, check_url::urls_cover_by_ancestor};
use std::collections::HashMap;
use std::path::PathBuf;

impl std::fmt::Display for sapphillon_v1::Permission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let permission_type = self.permission_type;
        let perm = sapphillon_v1::PermissionType::try_from(permission_type).unwrap();
        let resources = self.resource.join(", ");
        write!(
            f,
            "Permission {{{{ type: {}, resources: [{}] }}}}",
            perm.as_str_name(),
            resources
        )
    }
}

/// Associates a plugin function identifier with a set of permissions.
///
/// Typically used to describe the permissions that a plugin function requires
/// (or is granted) so the runtime can check access before invocation.
///
/// Example:
/// ```rust
/// # use crate::permission::Permissions;
/// let pfp = PluginFunctionPermissions {
///     plugin_function_id: "com.example.plugin.do_work".to_string(),
///     permissions: Permissions::new(vec![]),
/// };
/// ```
#[derive(Debug, Clone)]
pub struct PluginFunctionPermissions {
    pub plugin_function_id: String,
    pub permissions: Permissions,
}

/// A collection wrapper around protobuf permissions.
///
/// `Permissions` holds a vector of [`sapphillon_v1::Permission`] messages and
/// provides utility operations for common tasks such as merging entries that
/// refer to the same logical permission type.
///
/// Invariants and behavior:
/// - Before `merge()`, the vector may contain multiple entries with the same
///   `permission_type`. After `merge()`, there will be at most one entry per
///   distinct `permission_type`.
/// - The underlying permissions are the canonical protobuf message type used
///   across the codebase; this wrapper focuses on convenience helpers.
///
/// Merge semantics:
/// - `display_name` and `description` from merged entries are concatenated with
///   ", " (preserving both inputs).
/// - `resource` vectors are concatenated, preserving all entries from inputs.
/// - `permission_level` becomes the maximum of the merged entries.
///
/// Example:
/// ```rust
/// # use crate::proto::sapphillon::v1 as sapphillon_v1;
/// # use crate::permission::Permissions;
/// let p1 = sapphillon_v1::Permission {
///     permission_type: sapphillon_v1::PermissionType::FilesystemRead as i32,
///     resource: vec!["/tmp/a".to_string()],
///     ..Default::default()
/// };
/// let p2 = sapphillon_v1::Permission {
///     permission_type: sapphillon_v1::PermissionType::FilesystemRead as i32,
///     resource: vec!["/tmp/b".to_string()],
///     ..Default::default()
/// };
/// let merged = Permissions::new(vec![p1, p2]).merge();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Permissions {
    pub permissions: Vec<sapphillon_v1::Permission>,
}

impl std::fmt::Display for Permissions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = self
            .permissions
            .iter()
            .map(|p| format!("{p}"))
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "Permissions: [{msg}]")
    }
}

impl Permissions {
    /// Construct a `Permissions` wrapper from a vector of protobuf messages.
    ///
    /// This consumes the provided vector and returns a `Permissions` that owns
    /// the inner data. This function performs no normalization or merging; use
    /// [`merge`] when you want to coalesce entries by permission type.
    ///
    /// Complexity: O(1) (ownership move of the vector).
    pub fn new(permissions: Vec<sapphillon_v1::Permission>) -> Self {
        Self { permissions }
    }

    /// Merge permissions that share the same `permission_type`.
    ///
    /// Merge strategy:
    /// - `display_name` and `description` are concatenated with ", ".
    /// - `resource` vectors are concatenated, preserving all entries from inputs.
    /// - `permission_level` becomes the maximum of the merged entries.
    ///
    /// The returned `Permissions` contains at most one `Permission` value per
    /// distinct `permission_type`. Order of resulting entries is unspecified.
    ///
    /// Complexity: O(n) time and O(n) additional memory in the general case.
    pub fn merge(self) -> Self {
        let mut perm_map: HashMap<i32, sapphillon_v1::Permission> = HashMap::new();

        self.permissions
            .iter()
            .for_each(|p| match perm_map.get(&p.permission_type) {
                Some(perm) => {
                    let new_permission = sapphillon_v1::Permission {
                        display_name: p.display_name.clone() + ", " + &perm.display_name.clone(),
                        description: p.description.clone() + ", " + &perm.description.clone(),
                        permission_type: p.permission_type,
                        resource: [p.resource.clone(), perm.resource.clone()].concat(),
                        permission_level: std::cmp::max(perm.permission_level, p.permission_level),
                    };
                    perm_map.insert(p.permission_type, new_permission);
                }
                None => {
                    perm_map.insert(p.permission_type, p.clone());
                }
            });
        Permissions::new(perm_map.into_values().collect())
    }
}

/// Result of a permission check between granted and required permissions.
///
/// - `Ok` indicates all required permissions are satisfied by the granted set.
/// - `MissingPermission` contains a `Permissions` value listing the required
///   permissions that were not covered by the granted permissions.
pub enum CheckPermissionResult {
    Ok,
    MissingPermission(Permissions),
}

/// Check whether a set of granted `permissions` covers all `required`
/// permissions and return a `CheckPermissionResult`.
///
/// Matching rules by permission type:
/// - Filesystem read/write (permission types 4 and 5): the granted resource
///   paths are treated as base directories; a required path is satisfied if
///   it is lexically under (has an ancestor) at least one granted base. The
///   check uses `paths_cover_by_ancestor`.
/// - Network/URL-based access (permission type 6): a required URL is satisfied
///   if there exists a granted base URL with the same origin (scheme, host,
///   effective port) whose normalized path segments are a prefix of the
///   required URL's segments. The check uses `urls_cover_by_ancestor`.
/// - Other permission types: presence of any granted permission with the same
///   `permission_type` is treated as sufficient coverage.
///
/// Implementation notes:
/// - Both `permissions` and `required` are merged first (see `Permissions::merge`)
///   so that multiple entries for the same `permission_type` are coalesced.
/// - Returned `MissingPermission` contains the subset of required permissions
///   that could not be satisfied by any granted permission according to the
///   rules above.
///
/// Example:
/// let granted = Permissions::new(vec![ /* ... */ ]);
/// let required = Permissions::new(vec![ /* ... */ ]);
/// match check_permission(&granted, &required) {
///     CheckPermissionResult::Ok => { /* allowed */ }
///     CheckPermissionResult::MissingPermission(m) => { /* handle missing */ }
/// }
pub fn check_permission(
    permissions: &Permissions,
    required: &Permissions,
) -> CheckPermissionResult {
    let merged_permissions = permissions.clone().merge();
    let merged_required = required.clone().merge();
    let mut missing_permissions = Permissions::new(vec![]);

    // For each required permission, ensure at least one granted permission covers it.
    'req_loop: for req in &merged_required.permissions {
        for perm in &merged_permissions.permissions {
            if perm.permission_type != req.permission_type {
                continue;
            }

            match perm.permission_type {
                // Filesystem read/write: check path ancestor coverage
                4 | 5 => {
                    let perm_paths: Vec<PathBuf> =
                        perm.resource.iter().map(PathBuf::from).collect();
                    let req_paths: Vec<PathBuf> = req.resource.iter().map(PathBuf::from).collect();

                    if paths_cover_by_ancestor(&perm_paths, &req_paths) {
                        continue 'req_loop;
                    }
                }

                // Network/URL-based permissions: use URL ancestor coverage
                6 => {
                    let perm_urls: Vec<&str> = perm.resource.iter().map(|s| s.as_str()).collect();
                    let req_urls: Vec<&str> = req.resource.iter().map(|s| s.as_str()).collect();

                    if urls_cover_by_ancestor(&perm_urls, &req_urls) {
                        continue 'req_loop;
                    }
                }

                // Other permission types: presence of the same type is sufficient
                _ => {
                    continue 'req_loop;
                }
            }
        }

        // No granting permission covered this required permission
        missing_permissions.permissions.push(req.clone());
    }

    if missing_permissions.permissions.is_empty() {
        CheckPermissionResult::Ok
    } else {
        CheckPermissionResult::MissingPermission(missing_permissions)
    }
}

#[cfg(test)]
mod tests {
    use crate::proto::sapphillon::v1 as sapphillon_v1;

    use super::*;

    #[test]
    fn test_display_for_permission() {
        let p1 = sapphillon_v1::Permission {
            permission_type: sapphillon_v1::PermissionType::FilesystemRead as i32,
            resource: vec!["/tmp/test.txt".to_string()],
            ..Default::default()
        };
        assert_eq!(
            p1.to_string(),
            "Permission {{ type: PERMISSION_TYPE_FILESYSTEM_READ, resources: [/tmp/test.txt] }}"
        );

        let p2 = sapphillon_v1::Permission {
            permission_type: sapphillon_v1::PermissionType::NetAccess as i32,
            resource: vec!["google.com".to_string(), "example.com".to_string()],
            ..Default::default()
        };
        assert_eq!(
            p2.to_string(),
            "Permission {{ type: PERMISSION_TYPE_NET_ACCESS, resources: [google.com, example.com] }}"
        );

        let p3 = sapphillon_v1::Permission {
            permission_type: sapphillon_v1::PermissionType::Execute as i32,
            resource: vec![],
            ..Default::default()
        };
        assert_eq!(
            p3.to_string(),
            "Permission {{ type: PERMISSION_TYPE_EXECUTE, resources: [] }}"
        );
    }
    #[test]
    fn test_display_for_permissions() {
        let p1 = sapphillon_v1::Permission {
            permission_type: sapphillon_v1::PermissionType::FilesystemRead as i32,
            resource: vec!["/tmp/a".to_string()],
            ..Default::default()
        };
        let p2 = sapphillon_v1::Permission {
            permission_type: sapphillon_v1::PermissionType::FilesystemWrite as i32,
            resource: vec!["/tmp/b".to_string()],
            ..Default::default()
        };

        let perms1 = Permissions::new(vec![p1.clone()]);
        assert_eq!(
            perms1.to_string(),
            "Permissions: [Permission {{ type: PERMISSION_TYPE_FILESYSTEM_READ, resources: [/tmp/a] }}]"
        );

        let perms2 = Permissions::new(vec![p1, p2]);
        // The order is not guaranteed because of the underlying HashMap in merge(), so we check for both possibilities.
        let expected1 = "Permissions: [Permission {{ type: PERMISSION_TYPE_FILESYSTEM_READ, resources: [/tmp/a] }}, Permission {{ type: PERMISSION_TYPE_FILESYSTEM_WRITE, resources: [/tmp/b] }}]";
        let expected2 = "Permissions: [Permission {{ type: PERMISSION_TYPE_FILESYSTEM_WRITE, resources: [/tmp/b] }}, Permission {{ type: PERMISSION_TYPE_FILESYSTEM_READ, resources: [/tmp/a] }}]";
        let actual = perms2.to_string();
        assert!(actual == expected1 || actual == expected2);

        let perms3 = Permissions::new(vec![]);
        assert_eq!(perms3.to_string(), "Permissions: []");
    }

    // -----------------------------
    // Tests for Permissions::new and merge behaviour
    // -----------------------------
    #[test]
    fn test_permissions_new_empty_and_single() {
        // empty
        let empty = Permissions::new(vec![]);
        assert!(
            empty.permissions.is_empty(),
            "empty permissions should be empty"
        );

        // single element preserved
        let p = sapphillon_v1::Permission {
            permission_type: sapphillon_v1::PermissionType::Execute as i32,
            resource: vec!["/bin".to_string()],
            ..Default::default()
        };
        let perms = Permissions::new(vec![p.clone()]);
        assert_eq!(perms.permissions, vec![p]);
    }

    #[test]
    fn test_permissions_merge_same_type() {
        let p1 = sapphillon_v1::Permission {
            display_name: "A".to_string(),
            description: "d1".to_string(),
            permission_type: sapphillon_v1::PermissionType::FilesystemRead as i32,
            resource: vec!["/tmp/a".to_string()],
            permission_level: 1,
        };
        let p2 = sapphillon_v1::Permission {
            display_name: "B".to_string(),
            description: "d2".to_string(),
            permission_type: sapphillon_v1::PermissionType::FilesystemRead as i32,
            resource: vec!["/tmp/b".to_string()],
            permission_level: 2,
        };

        let merged = Permissions::new(vec![p1.clone(), p2.clone()]).merge();
        // merge groups by permission_type -> only one entry
        assert_eq!(merged.permissions.len(), 1);
        let m = merged.permissions.into_iter().next().unwrap();
        // permission_type preserved
        assert_eq!(m.permission_type, p1.permission_type);
        // both resources present (order not guaranteed)
        assert!(m.resource.contains(&"/tmp/a".to_string()));
        assert!(m.resource.contains(&"/tmp/b".to_string()));
        // permission_level is max
        assert_eq!(m.permission_level, 2);
        // display_name/description should include parts from inputs
        assert!(m.display_name.contains("A") || m.display_name.contains("B"));
        assert!(m.description.contains("d1") || m.description.contains("d2"));
    }

    // -----------------------------
    // Tests for check_permission behaviour
    // -----------------------------
    #[test]
    fn test_check_permission_filesystem_ok() {
        let granted = sapphillon_v1::Permission {
            permission_type: sapphillon_v1::PermissionType::FilesystemRead as i32,
            resource: vec!["/project".to_string()],
            ..Default::default()
        };
        let required = sapphillon_v1::Permission {
            permission_type: sapphillon_v1::PermissionType::FilesystemRead as i32,
            resource: vec!["/project/src/main.rs".to_string()],
            ..Default::default()
        };

        let res = check_permission(
            &Permissions::new(vec![granted]),
            &Permissions::new(vec![required]),
        );
        assert!(matches!(res, CheckPermissionResult::Ok));
    }

    #[test]
    fn test_check_permission_filesystem_missing() {
        let granted = sapphillon_v1::Permission {
            permission_type: sapphillon_v1::PermissionType::FilesystemRead as i32,
            resource: vec!["/other".to_string()],
            ..Default::default()
        };
        let required = sapphillon_v1::Permission {
            permission_type: sapphillon_v1::PermissionType::FilesystemRead as i32,
            resource: vec!["/project/src/main.rs".to_string()],
            ..Default::default()
        };

        let res = check_permission(
            &Permissions::new(vec![granted]),
            &Permissions::new(vec![required]),
        );
        match res {
            CheckPermissionResult::MissingPermission(m) => {
                assert_eq!(m.permissions.len(), 1);
                assert_eq!(
                    m.permissions[0].permission_type,
                    sapphillon_v1::PermissionType::FilesystemRead as i32
                );
            }
            _ => panic!("expected MissingPermission"),
        }
    }

    #[test]
    fn test_check_permission_url_ok() {
        let granted = sapphillon_v1::Permission {
            permission_type: sapphillon_v1::PermissionType::NetAccess as i32,
            resource: vec!["https://example.com/api".to_string()],
            ..Default::default()
        };
        let required = sapphillon_v1::Permission {
            permission_type: sapphillon_v1::PermissionType::NetAccess as i32,
            resource: vec!["https://example.com/api/v1/resource".to_string()],
            ..Default::default()
        };

        let res = check_permission(
            &Permissions::new(vec![granted]),
            &Permissions::new(vec![required]),
        );
        assert!(matches!(res, CheckPermissionResult::Ok));
    }

    #[test]
    fn test_check_permission_url_missing() {
        let granted = sapphillon_v1::Permission {
            permission_type: sapphillon_v1::PermissionType::NetAccess as i32,
            resource: vec!["https://api.example.com/".to_string()],
            ..Default::default()
        };
        let required = sapphillon_v1::Permission {
            permission_type: sapphillon_v1::PermissionType::NetAccess as i32,
            resource: vec!["https://example.com/api/v1/resource".to_string()],
            ..Default::default()
        };

        let res = check_permission(
            &Permissions::new(vec![granted]),
            &Permissions::new(vec![required]),
        );
        assert!(matches!(res, CheckPermissionResult::MissingPermission(_)));
    }

    #[test]
    fn test_check_permission_other_type_presence() {
        let granted = sapphillon_v1::Permission {
            permission_type: sapphillon_v1::PermissionType::Execute as i32,
            resource: vec![],
            ..Default::default()
        };
        let required = sapphillon_v1::Permission {
            permission_type: sapphillon_v1::PermissionType::Execute as i32,
            resource: vec![],
            ..Default::default()
        };

        let res = check_permission(
            &Permissions::new(vec![granted]),
            &Permissions::new(vec![required]),
        );
        assert!(matches!(res, CheckPermissionResult::Ok));
    }

    #[test]
    fn test_check_permission_none() {
        let res = check_permission(&Permissions::new(vec![]), &Permissions::new(vec![]));
        assert!(matches!(res, CheckPermissionResult::Ok));
    }
}
