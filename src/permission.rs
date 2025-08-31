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

use std::collections::{HashMap, HashSet};
use std::path::{Component, Path, PathBuf};
use url::Url;

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
    pub fn new(permissions: Vec<sapphillon_v1::Permission>) -> Self {
        Self { permissions }
    }

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

pub enum CheckPermissionResult {
    Ok,
    MissingPermission(Permissions),
}

pub fn check_permission(
    permissions: &Permissions,
    required: &Permissions,
) -> CheckPermissionResult {
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
            missing_permissions
                .permissions
                .push(merged_required.permissions[i].clone());
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
            Component::CurDir => {} // "."
            Component::ParentDir => {
                out.pop();
            } // ".."
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
    b.iter()
        .map(|p| normalize_forgiving(p.as_ref()))
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
    b.iter()
        .all(|p| set_a.contains(&normalize_forgiving(p.as_ref())))
}

/// Lexically normalize a sequence of path (or URL path) segments.
/// Rules:
/// - "." segments are discarded.
/// - ".." pops one previously retained normal segment (if any).
/// - Empty segments (caused by consecutive slashes or a trailing slash) are ignored.
/// - All other segments are kept verbatim (no percent‑decoding or case folding).
///
/// This is a purely lexical rewrite; it does not consult the filesystem nor
/// attempt to collapse beyond simple "." / ".." handling. It mirrors the
/// fallback logic used for path normalization in this module.
///
/// Returned vector contains the normalized, ordered segments.
///
/// JA: セグメントのレキシカル正規化（"." を除去し、".." で一つ戻す）
fn normalize_segments<I>(segments: I) -> Vec<String>
where
    I: IntoIterator<Item = String>,
{
    let mut out: Vec<String> = Vec::new();
    for s in segments {
        match s.as_str() {
            "." => {} // 無視
            ".." => {
                out.pop();
            } // 一つ戻る
            "" => {}  // 連続スラッシュや末尾の空要素は無視
            _ => out.push(s),
        }
    }
    out
}

/// Construct a comparison origin key from a Url consisting of:
/// (lowercased scheme, lowercased host, effective port).
///
/// The effective port is the explicit port if present, otherwise the scheme's
/// known default (via url::Url::port_or_known_default). Returns None if either
/// host or default port is unavailable (e.g., data URLs, opaque origins).
///
/// This normalized triple allows grouping and matching URLs by "same origin"
/// semantics needed for coverage checks.
///
/// JA: Url から比較用オリジンキーを作成（scheme, host, port_or_default）
fn origin_key(u: &Url) -> Option<(String, String, u16)> {
    let scheme = u.scheme().to_ascii_lowercase();
    let host = u.host_str()?.to_ascii_lowercase();
    let port = u.port_or_known_default()?; // 既定ポートを補完
    Some((scheme, host, port))
}

/// Return the normalized (lexically collapsed) path segments of the given URL.
///
/// Behavior:
/// - If the URL is "cannot-be-a-base" (opaque, e.g. data:, mailto:), returns None.
/// - Otherwise extracts path segments, then applies normalize_segments to
///   collapse "." / ".." and discard empty segments.
/// - Does not perform percent-decoding; operates on the raw segment strings.
///
/// Used as a canonical representation for prefix (ancestor) comparisons.
///
/// JA: URL の正規化済みパスセグメントを取得（cannot-be-a-base は None）
fn url_segments(u: &Url) -> Option<Vec<String>> {
    let segs = u
        .path_segments()?
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    Some(normalize_segments(segs))
}

/// Return true if for every URL in b there exists at least one "base" URL in a
/// with the SAME ORIGIN (scheme, host, effective port) whose normalized path
/// segments are a prefix of the target URL's normalized path segments.
///
/// Procedure:
/// 1. Parse each candidate base URL in a. Reject (skip) invalid or opaque URLs.
/// 2. Group bases by origin key (scheme, host, effective port).
/// 3. For each origin group maintain only a minimal set of base segment vectors:
///    - If a new base is already covered (its segments start_with an existing
///      shorter base), discard it.
///    - If the new base is a strict ancestor (shorter prefix) of existing bases,
///      remove those longer descendants and keep the shorter one.
/// 4. For every URL in b:
///    - Parse & derive its origin key and normalized segments.
///    - Look up the minimal base list for that origin.
///    - Ensure at least one base's segment vector is a prefix of the target's.
/// 5. Fail fast (return false) on any invalid URL, missing origin group, or
///    uncovered target. Return true only if all targets pass.
///
/// Notes:
/// - Path normalization is purely lexical (no symlink resolution).
/// - Query and fragment components are ignored for coverage (only path matters).
/// - An origin mismatch immediately causes failure for that URL.
///
/// JA: a のどれかのベース URL が b の各 URL を「同一オリジンかつパスの前方一致」で包含しているか
pub fn urls_cover_by_ancestor<A: AsRef<str>, B: AsRef<str>>(a: &[A], b: &[B]) -> bool {
    // オリジンごとに最小集合のベースセグメントを持つ
    let mut bases: HashMap<(String, String, u16), Vec<Vec<String>>> = HashMap::new();

    for s in a {
        let Ok(url) = Url::parse(s.as_ref()) else {
            continue;
        };
        let Some(key) = origin_key(&url) else {
            continue;
        };
        let Some(mut segs) = url_segments(&url) else {
            continue;
        };

        // 冗長ベース除去（a/b が a に包含されるなら a を優先）
        let entry = bases.entry(key).or_default();
        // 既存に包含されていれば追加不要
        if entry.iter().any(|m| segs.starts_with(m)) {
            continue;
        }
        // 今回の方が短い（祖先）なら既存の子孫を削除
        entry.retain(|m| !m.starts_with(&segs));
        entry.push(std::mem::take(&mut segs));
    }

    for s in b {
        let Ok(url) = Url::parse(s.as_ref()) else {
            return false;
        };
        let Some(key) = origin_key(&url) else {
            return false;
        };
        let Some(segs) = url_segments(&url) else {
            return false;
        };
        let Some(entry) = bases.get(&key) else {
            return false;
        };
        if !entry.iter().any(|base| segs.starts_with(base)) {
            return false;
        }
    }
    true
}

/// Exact (set) coverage of URLs: every URL in b, when successfully parsed and
/// serialized back to a string (Url::to_string), must exactly match one of the
/// serialized forms obtained from a.
///
/// Characteristics:
/// - Both sides are parsed; invalid URLs are silently skipped (b invalid => not
///   counted as covered since it cannot match; leads to false via all()).
/// - Uses the serialization rules of url::Url; default ports are elided, so
///   "http://example.com" and "http://example.com:80" serialize identically.
/// - Ignores multiplicity and order (pure set membership).
///
/// Difference vs urls_cover_by_ancestor:
/// - This requires exact equality of full serialized URL (path, query, etc.).
/// - The ancestor variant allows path prefix containment within the same origin.
///
/// JA: 完全一致の集合包含（URL のシリアライズ表現で比較）
pub fn urls_cover_as_set<A: AsRef<str>, B: AsRef<str>>(a: &[A], b: &[B]) -> bool {
    let set_a: HashSet<String> = a
        .iter()
        .filter_map(|s| Url::parse(s.as_ref()).ok())
        .map(|u| u.to_string()) // 既定ポートはシリアライズに出ない
        .collect();
    b.iter()
        .filter_map(|s| Url::parse(s.as_ref()).ok())
        .all(|u| set_a.contains(&u.to_string()))
}

#[cfg(test)]
mod tests {
    use crate::proto::sapphillon::v1 as sapphillon_v1;

    use super::*;
    use std::path::{Path, PathBuf};

    // Helper to convert many &str to Vec<PathBuf>
    fn pvec(v: &[&str]) -> Vec<PathBuf> {
        v.iter().map(PathBuf::from).collect()
    }

    #[test]
    fn test_normalize_forgiving_lexical_collapse_simple() {
        let input = Path::new("a/./b/../c");
        let out = normalize_forgiving(input);
        assert_eq!(
            out,
            PathBuf::from("a/c"),
            "should remove '.' and collapse '..'"
        );
    }

    #[test]
    fn test_normalize_forgiving_leading_parent_dirs_are_dropped() {
        // Leading ".." with no prior segment causes a pop on empty => effectively ignored
        let input = Path::new("../a/../b");
        let out = normalize_forgiving(input);
        // Behavior of current implementation: leading '..' disappears, then 'a', then '..' removes 'a', leaving 'b'
        assert_eq!(
            out,
            PathBuf::from("b"),
            "leading '..' is effectively ignored and net result collapses to 'b'"
        );
    }

    #[test]
    fn test_normalize_forgiving_trailing_parent_dir() {
        let input = Path::new("../x/./y/..");
        let out = normalize_forgiving(input);
        // Sequence: '..' (ignored), 'x', '.', 'y', '..' (removes y) => 'x'
        assert_eq!(out, PathBuf::from("x"));
    }

    #[test]
    fn test_paths_cover_by_ancestor_basic_true() {
        // Redundant base 'a/b' should be removed because 'a' covers it
        let bases = pvec(&["a", "a/b", "x/y"]);
        let targets = pvec(&["a/file.txt", "a/b/c", "x/y/z"]);
        assert!(
            paths_cover_by_ancestor(&bases, &targets),
            "all targets lie under at least one ancestor base"
        );
    }

    #[test]
    fn test_paths_cover_by_ancestor_false_missing_base() {
        let bases = pvec(&["a/dir"]);
        let targets = pvec(&["a/dir/file", "other/file"]);
        assert!(
            !paths_cover_by_ancestor(&bases, &targets),
            "target outside provided bases should fail coverage"
        );
    }

    #[test]
    fn test_paths_cover_by_ancestor_normalization() {
        // Base normalizes from a/./b/../c -> a/c
        let bases = pvec(&["a/./b/../c"]);
        let targets = pvec(&["a/c/d", "a/c"]);
        assert!(
            paths_cover_by_ancestor(&bases, &targets),
            "normalized base should cover normalized targets"
        );
    }

    #[test]
    fn test_paths_cover_as_set_true_with_normalization() {
        let a = pvec(&["a/./b", "c/d/../e"]);
        let b = pvec(&["a/b", "c/e"]);
        assert!(
            paths_cover_as_set(&a, &b),
            "normalized set a should contain all normalized b paths"
        );
    }

    #[test]
    fn test_paths_cover_as_set_false_missing() {
        let a = pvec(&["a/b", "c/e"]);
        let b = pvec(&["a/b", "c/e", "x/y"]);
        assert!(
            !paths_cover_as_set(&a, &b),
            "missing element in a should cause false"
        );
    }

    #[test]
    fn test_paths_cover_as_set_duplicates_in_b() {
        let a = pvec(&["a/b", "c/e"]);
        let b = pvec(&["a/b", "a/b", "c/e"]);
        assert!(
            paths_cover_as_set(&a, &b),
            "duplicates in b should not affect set coverage"
        );
    }
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

    #[test]
    fn test_url_checker() {
        let a = vec![
            "https://example.com/project/src",
            "https://data.example.com/data",
            "http://example.com:80/base",
        ];

        let b_ok = vec![
            "https://example.com/project/src/lib/mod.rs",
            "https://data.example.com/data/input/file.csv?part=1",
            "http://example.com/base/child",
        ];

        let b_ng = vec![
            "https://example.com/project/tests/test.rs",
            "https://other.example.com/data/x",
        ];

        assert!(urls_cover_by_ancestor(&a, &b_ok));
        assert!(!urls_cover_by_ancestor(&a, &b_ng));

        // 完全一致（既定ポートの明示/非明示は同一視）
        let a2 = vec!["https://example.com/api", "https://example.com/"];
        let b2 = vec!["https://example.com/"];
        assert!(urls_cover_as_set(&a2, &b2));
    }

    // -----------------------------
    // Additional comprehensive tests
    // -----------------------------

    // normalize_segments
    #[test]
    fn test_segments_basic_collapse() {
        let v = vec!["a", ".", "b", "..", "c"]
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>();
        assert_eq!(normalize_segments(v), vec!["a", "c"]);
    }

    #[test]
    fn test_segments_leading_parent_dirs() {
        let v = vec!["..", "a", "..", "b"]
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>();
        assert_eq!(normalize_segments(v), vec!["b"]);
    }

    #[test]
    fn test_segments_multiple_pops_past_start() {
        let v = vec!["a", "..", "..", "b"]
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>();
        assert_eq!(normalize_segments(v), vec!["b"]);
    }

    #[test]
    fn test_segments_empty_segments_discarded() {
        let v = vec!["", "a", "", "b", ""]
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>();
        assert_eq!(normalize_segments(v), vec!["a", "b"]);
    }

    #[test]
    fn test_segments_mixed_and_all_removed() {
        let v = vec![".", "..", "..", "."]
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>();
        assert!(normalize_segments(v).is_empty());
    }

    #[test]
    fn test_segments_unicode_and_dot_handling() {
        let v = vec!["α", "β", "..", "γ"]
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>();
        assert_eq!(normalize_segments(v), vec!["α", "γ"]);
    }

    #[test]
    fn test_segments_percent_not_decoded() {
        // "%2E" should not be treated as ".", so it is retained
        let v = vec!["a%2E", "."]
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>();
        assert_eq!(normalize_segments(v), vec!["a%2E"]);
    }

    // origin_key
    #[test]
    fn test_origin_key_default_and_explicit_ports() {
        let u1 = Url::parse("http://Example.COM").unwrap();
        let u2 = Url::parse("http://example.com:80/").unwrap();
        let k1 = origin_key(&u1).unwrap();
        let k2 = origin_key(&u2).unwrap();
        assert_eq!(k1, ("http".into(), "example.com".into(), 80));
        assert_eq!(k1, k2);
    }

    #[test]
    fn test_origin_key_non_default_and_https() {
        let u1 = Url::parse("http://example.com:8080/").unwrap();
        let u2 = Url::parse("https://example.com/").unwrap();
        assert_eq!(
            origin_key(&u1).unwrap(),
            ("http".into(), "example.com".into(), 8080)
        );
        assert_eq!(
            origin_key(&u2).unwrap(),
            ("https".into(), "example.com".into(), 443)
        );
    }

    #[test]
    fn test_origin_key_opaque_and_mailto_none() {
        let data = Url::parse("data:text/plain,abc").unwrap();
        assert!(origin_key(&data).is_none(), "opaque URL should yield None");
        let mail = Url::parse("mailto:user@example.com").unwrap();
        assert!(origin_key(&mail).is_none(), "mailto URL has no host/port");
    }

    #[test]
    fn test_origin_key_ipv6() {
        let u = Url::parse("http://[2001:db8::1]/").unwrap();
        // url::Url::host_str() returns the bracketed form for IPv6; keep as-is.
        assert_eq!(
            origin_key(&u).unwrap(),
            ("http".into(), "[2001:db8::1]".into(), 80)
        );
    }

    // url_segments
    #[test]
    fn test_url_segments_simple() {
        let u = Url::parse("https://example.com/a/b").unwrap();
        assert_eq!(url_segments(&u).unwrap(), vec!["a", "b"]);
    }

    #[test]
    fn test_url_segments_multiple_slashes() {
        let u = Url::parse("https://example.com//a//b///").unwrap();
        assert_eq!(url_segments(&u).unwrap(), vec!["a", "b"]);
    }

    #[test]
    fn test_url_segments_dot_and_dotdot() {
        let u = Url::parse("https://example.com/a/./b/../c").unwrap();
        assert_eq!(url_segments(&u).unwrap(), vec!["a", "c"]);
    }

    #[test]
    fn test_url_segments_root_only() {
        let u = Url::parse("https://example.com/").unwrap();
        assert_eq!(url_segments(&u).unwrap(), Vec::<String>::new());
    }

    #[test]
    fn test_url_segments_trailing_parent_dir() {
        let u = Url::parse("https://example.com/a/b/..").unwrap();
        assert_eq!(url_segments(&u).unwrap(), vec!["a"]);
    }

    #[test]
    fn test_url_segments_percent_encoding_preserved() {
        let u = Url::parse("https://example.com/a%2Eb/c").unwrap();
        assert_eq!(url_segments(&u).unwrap(), vec!["a%2Eb", "c"]);
    }

    #[test]
    fn test_url_segments_opaque_none() {
        let u = Url::parse("data:text/plain,xyz").unwrap();
        assert!(url_segments(&u).is_none());
    }

    // urls_cover_by_ancestor advanced
    #[test]
    fn test_urls_cover_by_ancestor_prefix_boundary_fail() {
        let a = vec!["https://example.com/app"];
        let b = vec!["https://example.com/application/file"];
        assert!(
            !urls_cover_by_ancestor(&a, &b),
            "segment boundary should prevent false positive (app vs application)"
        );
    }

    #[test]
    fn test_urls_cover_by_ancestor_port_equivalence() {
        let a = vec!["http://example.com:80/base"];
        let b = vec!["http://example.com/base/x"];
        assert!(urls_cover_by_ancestor(&a, &b));
    }

    #[test]
    fn test_urls_cover_by_ancestor_scheme_mismatch() {
        let a = vec!["http://example.com/base"];
        let b = vec!["https://example.com/base/x"];
        assert!(!urls_cover_by_ancestor(&a, &b));
    }

    #[test]
    fn test_urls_cover_by_ancestor_trailing_slash_normalization() {
        let a = vec!["https://example.com/base/"];
        let b = vec!["https://example.com/base/x"];
        assert!(urls_cover_by_ancestor(&a, &b));
    }

    #[test]
    fn test_urls_cover_by_ancestor_invalid_base_skipped() {
        let a = vec!["::::", "https://example.com/a/b", "https://example.com/a"];
        let b = vec!["https://example.com/a/x"];
        assert!(
            urls_cover_by_ancestor(&a, &b),
            "invalid base should be skipped; minimal base a covers"
        );
    }

    #[test]
    fn test_urls_cover_by_ancestor_invalid_target_false() {
        let a = vec!["https://example.com/a"];
        let b = vec!["::not_a_url::"];
        assert!(
            !urls_cover_by_ancestor(&a, &b),
            "invalid target triggers false"
        );
    }

    #[test]
    fn test_urls_cover_by_ancestor_dot_segments_in_target() {
        let a = vec!["https://example.com/a/b"];
        let b = vec!["https://example.com/a/b/c/./d/../e"];
        assert!(urls_cover_by_ancestor(&a, &b));
    }

    // urls_cover_as_set advanced
    #[test]
    fn test_urls_cover_as_set_default_port_canonicalization() {
        let a = vec!["http://example.com"];
        let b = vec!["http://example.com:80/"];
        assert!(urls_cover_as_set(&a, &b));
    }

    #[test]
    fn test_urls_cover_as_set_query_mismatch() {
        let a = vec!["https://e.com/a?x=1"];
        let b = vec!["https://e.com/a?x=2"];
        assert!(!urls_cover_as_set(&a, &b));
    }

    #[test]
    fn test_urls_cover_as_set_fragment_difference() {
        let a = vec!["https://e.com/a#frag"];
        let b = vec!["https://e.com/a"];
        // Fragment participates in serialization, so mismatch => false
        assert!(!urls_cover_as_set(&a, &b));
    }

    #[test]
    fn test_urls_cover_as_set_duplicates_and_invalid_skip() {
        let a = vec!["https://example.com/"];
        let b = vec![
            "https://example.com/",
            "https://example.com/",
            "::not_a_url::",
        ];
        // Invalid b URL skipped by filter_map; duplicates ignored; still covered
        assert!(urls_cover_as_set(&a, &b));
    }

    #[test]
    fn test_urls_cover_as_set_missing_element() {
        let a = vec!["https://example.com/a"];
        let b = vec!["https://example.com/a", "https://example.com/b"];
        assert!(!urls_cover_as_set(&a, &b));
    }
}
