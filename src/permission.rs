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

use std::collections::HashMap;

use crate::proto::sapphillon::{self, v1 as sapphillon_v1};

use std::path::{Path, PathBuf, Component};
use std::collections::HashSet;

impl std::fmt::Display for sapphillon_v1::Permission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let permission_type = self.permission_type;
        let perm = sapphillon_v1::PermissionType::try_from(permission_type).unwrap();
        let resources = self.resource.join(", ");
        write!(f, "Permission {{ type: {}, resources: [{}] }}",  perm.as_str_name(), resources)
    }
    
}

#[derive(Debug, Clone, PartialEq)]
pub struct Permissions {
    pub permissions: Vec<sapphillon_v1::Permission>
}

impl std::fmt::Display for Permissions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = self.permissions.iter().map(|p| format!("{p}")).collect::<Vec<_>>().join(", ");
        write!(f, "Permissions: [{msg}]")
    }
    
}


impl Permissions {
    pub fn new(permissions: Vec<sapphillon_v1::Permission>) -> Self {
        Self { permissions }
    }
    
    pub fn merge(self) -> Self {
        let mut perm_map: HashMap<i32, sapphillon_v1::Permission> = HashMap::new();
        
        self.permissions.iter().for_each(
            |p| {
                match perm_map.get(&p.permission_type) {
                    Some(perm) =>  {
                        let new_permission = sapphillon_v1::Permission {
                            display_name: p.display_name.clone() + ", " + &perm.display_name.clone(),
                            description: p.description.clone() + ", " + &perm.description.clone(),
                            permission_type: p.permission_type,
                            resource: [p.resource.clone(), perm.resource.clone()].concat(),
                            permission_level: std::cmp::max(perm.permission_level, p.permission_level)
                        };
                        perm_map.insert(p.permission_type, new_permission);
                    },
                    None => {perm_map.insert(p.permission_type, p.clone());}
                }
            }
        );
        Permissions::new(perm_map.into_values().collect())
    }
    
}

pub enum CheckPermissionResult {
    Ok,
    MissingPermission(Permissions),
}

pub fn check_permission(permissions: &Permissions, required: &Permissions) -> CheckPermissionResult {
    let merged_permissions = permissions.clone().merge();
    let merged_required = required.clone().merge();
    let mut missing_permissions = merged_required.clone();
    
    for i in 0..merged_permissions.permissions.len() {
        let mut found = false;
        for j in 0..merged_permissions.permissions.len() {
            let perm = &merged_permissions.permissions[i];
            let req = &merged_required.permissions[j];
            
            if perm.permission_type == req.permission_type {
                found = true;
            }
        }
        
        if !found {
            missing_permissions.permissions.push(merged_required.permissions[i].clone());
        }
    }

    CheckPermissionResult::Ok
}


/// Normalize the given path in a "forgiving" manner.
///
/// Strategy:
/// 1. Try std::fs::canonicalize to obtain an absolute, fully resolved path
///    (resolves symlinks and removes redundant segments).
/// 2. If canonicalize fails (e.g. non-existent components, permission error),
///    fall back to a purely lexical normalization that:
///      - removes '.' components
///      - processes '..' by popping the previous segment (if any)
///      - keeps all other components (Prefix / RootDir / Normal) verbatim
///
/// The fallback phase does NOT touch the filesystem and does NOT resolve
/// symlinks; it only rewrites the path tokens. A relative input stays relative.
///
/// Returns a PathBuf representing the normalized path.
///
/// Original (JA): 可能なら canonicalize、失敗したらレキシカルに . と .. を畳み込む簡易正規化
fn normalize_forgiving(p: &Path) -> PathBuf {
    if let Ok(abs) = std::fs::canonicalize(p) {
        return abs;
    }
    let mut out = PathBuf::new();
    for comp in p.components() {
        match comp {
            Component::CurDir => {}               // "."
            Component::ParentDir => { out.pop(); } // ".."
            other => out.push(other.as_os_str()), // Prefix/RootDir/Normal unchanged
        }
    }
    out
}

/// Return true if every path in b lies under (has as ancestor / directory prefix)
/// at least one of the paths supplied in a.
///
/// Algorithm:
/// - Normalize all candidate base paths (a) with normalize_forgiving.
/// - Remove redundant bases: if /a is present then /a/b is dropped.
///   Conversely if we encounter a shorter base that contains an existing longer
///   one, we remove the longer one.
/// - Normalize each target path (b) and ensure it starts with at least one
///   minimal base path.
///
/// Notes:
/// - "Ancestor" here means Path::starts_with (directory containment).
/// - Normalization mitigates superficial differences like './', '../' collapses.
/// - Does not guarantee existence on disk.
///
/// Original (JA): a のどれかが b の各要素の祖先（ディレクトリ包含）か
pub fn paths_cover_by_ancestor<A: AsRef<Path>, B: AsRef<Path>>(a: &[A], b: &[B]) -> bool {
    // Normalize base paths
    let mut bases: Vec<PathBuf> = a.iter().map(|p| normalize_forgiving(p.as_ref())).collect();

    // Eliminate redundant bases (e.g., if /a exists then /a/b is unnecessary)
    let mut minimal: Vec<PathBuf> = Vec::new();
    'outer: for base in bases.drain(..) {
        for m in &minimal {
            if base.starts_with(m) {
                continue 'outer;
            }
        }
        // If an existing minimal is contained in the new base, drop the existing one
        minimal.retain(|m| !m.starts_with(&base));
        minimal.push(base);
    }

    // Check every target is covered by at least one minimal base
    b.iter().map(|p| normalize_forgiving(p.as_ref()))
        .all(|t| minimal.iter().any(|base| t.starts_with(base)))
}


/// Return true if every normalized path in b appears exactly (after forgiving
/// normalization) in the set derived from a.
///
/// Semantics:
/// - Both a and b are normalized with normalize_forgiving.
/// - a is collected into a HashSet for O(1) membership checks.
/// - Returns true iff all normalized b paths are members of that set.
/// - Order and multiplicity are ignored (pure coverage as a mathematical set).
///
/// Contrast with paths_cover_by_ancestor:
/// - paths_cover_as_set requires exact equality after normalization.
/// - paths_cover_by_ancestor allows directory prefix containment.
///
/// This is useful when required resources must be explicitly enumerated,
/// not merely contained under a broader directory.
///
/// Original (JA intent): 集合として a が b を（正規化後に）包含するか
pub fn paths_cover_as_set<A: AsRef<Path>, B: AsRef<Path>>(a: &[A], b: &[B]) -> bool {
    let set_a: HashSet<PathBuf> = a.iter().map(|p| normalize_forgiving(p.as_ref())).collect();
    b.iter().all(|p| set_a.contains(&normalize_forgiving(p.as_ref())))
}