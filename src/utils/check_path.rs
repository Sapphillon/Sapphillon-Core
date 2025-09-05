use std::collections::HashSet;
use std::path::{Component, Path, PathBuf};

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

#[cfg(test)]
mod tests {
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
}
