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
    // Prefer real canonicalization when possible (resolves symlinks).
    if let Ok(abs) = std::fs::canonicalize(p) {
        return abs;
    }

    // Fallback: do a purely lexical normalization that collapses '.' and '..'
    // but never pops past a platform-specific anchor (prefix/root).
    // Accept either forward-slash or backslash as separator: normalize backslashes
    // to forward slashes for lexical processing so Windows-style inputs like
    // "C:\\path\\to\\file" are handled correctly on Unix hosts as well.
    // We perform this normalization on a UTF-8 lossy string representation of
    // the path (safe for non-UTF8 inputs; lossy conversion is acceptable for
    // the purpose of splitting separators).
    let input_path_for_lex: PathBuf = {
        // Convert to string (lossy), replace backslashes with forward slashes.
        let s = p.as_os_str().to_string_lossy().to_string();
        if s.contains('\\') {
            PathBuf::from(s.replace('\\', "/"))
        } else {
            p.to_path_buf()
        }
    };
    #[derive(Debug)]
    enum Seg {
        Prefix(std::ffi::OsString),
        Root,
        Normal(std::ffi::OsString),
    }

    let mut stack: Vec<Seg> = Vec::new();

    for comp in input_path_for_lex.components() {
        match comp {
            Component::Prefix(pre) => stack.push(Seg::Prefix(pre.as_os_str().to_os_string())),
            Component::RootDir => stack.push(Seg::Root),
            Component::CurDir => { /* skip */ }
            Component::ParentDir => {
                // Pop the last normal segment if present. Do not pop past Root or Prefix.
                if let Some(pos) = stack.iter().rposition(|s| matches!(s, Seg::Normal(_))) {
                    stack.remove(pos);
                } else {
                    // Nothing to pop (we're at the anchor or empty) -> ignore the ParentDir
                }
            }
            Component::Normal(os) => stack.push(Seg::Normal(os.to_os_string())),
        }
    }

    // Reconstruct PathBuf from the stack.
    let mut out = PathBuf::new();
    for seg in stack {
        match seg {
            Seg::Prefix(s) => out.push(s),
            Seg::Root => {
                // Push a platform-appropriate root token. On unix this will become '/'.
                out.push(std::path::MAIN_SEPARATOR.to_string());
            }
            Seg::Normal(s) => out.push(s),
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
    if a.len() == 1 && a[0].as_ref() == Path::new("*") {
        return true;
    }
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
    if a.len() == 1 && a[0].as_ref() == Path::new("*") {
        return true;
    }
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

    #[test]
    fn test_paths_cover_by_ancestor_wildcard() {
        let a = pvec(&["*"]);
        let b = pvec(&["a/file.txt", "a/b/c", "x/y/z"]);
        assert!(paths_cover_by_ancestor(&a, &b));
    }

    #[test]
    fn test_paths_cover_as_set_wildcard() {
        let a = pvec(&["*"]);
        let b = pvec(&["a/b", "c/e"]);
        assert!(paths_cover_as_set(&a, &b));
    }

    #[test]
    fn test_normalize_forgiving_windows_style_prefix() {
        // On non-windows hosts we can still construct a Path that contains a "C:\"-style
        // prefix. The lexical normalizer should preserve the prefix and not allow '..' to
        // pop past it.
        let input = Path::new("C:/dir/sub/../file.txt");
        let out = normalize_forgiving(input);
        // Expect the path to become C:/dir/file.txt (prefix preserved, .. collapsed)
        // Note: representation may vary between platforms. On Windows components will
        // include a Prefix; on Unix the drive letter may appear as a Normal component
        // like "C:". We'll accept either form.
        let comps: Vec<String> = out.components().map(|c| format!("{c:?}")).collect();
        // Accept either a Prefix component (Windows) or a Normal component that
        // contains "C:" (common representation on non-Windows hosts for drive-like inputs).
        let has_prefix = comps.iter().any(|s| s.contains("Prefix"));
        let has_drive_normal = comps.iter().any(|s| s.contains("C:"));
        assert!(
            has_prefix || has_drive_normal,
            "expected prefix or C: drive component, got: {comps:?}"
        );
        // Ensure dir and file components survived normalization
        assert!(
            comps.iter().any(|s| s.contains("dir")),
            "expected component dir in {comps:?}"
        );
        assert!(
            comps.iter().any(|s| s.contains("file.txt")),
            "expected component file.txt in {comps:?}"
        );
    }

    #[test]
    fn test_normalize_forgiving_backslashes() {
        // Ensure that backslash separators are treated like forward slashes
        // and that '..' collapses correctly.
        let input = Path::new("C:\\dir\\sub\\..\\file.txt");
        let out = normalize_forgiving(input);
        let comps: Vec<String> = out.components().map(|c| format!("{c:?}")).collect();
        // Should preserve drive/prefix or at least the 'C:' component and include dir and file
        assert!(
            comps
                .iter()
                .any(|s| s.contains("C:") || s.contains("Prefix"))
        );
        assert!(comps.iter().any(|s| s.contains("dir")));
        assert!(comps.iter().any(|s| s.contains("file.txt")));
    }

    #[test]
    fn test_paths_cover_by_ancestor_backslashes() {
        // Bases and targets supplied with backslashes should be normalized and matched.
        let bases = pvec(&["C:\\project\\src", "C:\\other"]);
        let targets = pvec(&["C:\\project\\src\\main.rs", "C:/project/src/lib.rs"]);
        assert!(paths_cover_by_ancestor(&bases, &targets));
    }

    #[test]
    fn test_absolute_parent_on_absolute_path() {
        // An absolute path that starts with '..' should not pop past root.
        let input = Path::new("/../etc/passwd");
        let out = normalize_forgiving(input);
        // Should normalize to '/etc/passwd' on unix-like systems
        assert!(out.ends_with("etc/passwd") || out.as_path() == Path::new("etc/passwd"));
    }

    #[test]
    fn test_redundant_separators_collapsed() {
        let input = Path::new("a//b///c");
        let out = normalize_forgiving(input);
        assert_eq!(out, PathBuf::from("a/b/c"));
    }

    #[test]
    fn test_paths_cover_as_set_backslashes() {
        let a = pvec(&["C:\\a\\b", "D:/x/y"]);
        let b = pvec(&["C:/a/b", "D:\\x\\y"]);
        assert!(paths_cover_as_set(&a, &b));
    }

    #[test]
    fn test_unc_like_input() {
        // UNC-like inputs (\\server\\share\path) should be treated sensibly;
        // normalization will convert backslashes and keep the prefix-like segments.
        let base = Path::new("\\\\server\\share\\dir");
        let target = Path::new("\\\\server\\share\\dir\\file.txt");
        assert!(paths_cover_by_ancestor(&[base], &[target]));
    }

    #[cfg(unix)]
    #[test]
    fn test_canonicalize_resolved_when_exists() {
        // Create a tiny temporary dir and a symlink to test canonicalize path takeover.
        // Avoid pulling in the `tempfile` crate by using a timestamped directory under
        // the platform temp dir.
        let td_base = std::env::temp_dir();
        let uniq = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let td = td_base.join(format!("sapphillon_test_{uniq}"));
        let _ = std::fs::create_dir_all(&td);
        let dir = td.join("sub");
        std::fs::create_dir(&dir).unwrap();
        let target = dir.join("file.txt");
        std::fs::write(&target, "hi").unwrap();
        let link = td.join("link_to_sub");
        #[cfg(unix)]
        std::os::unix::fs::symlink(&dir, &link).unwrap();

        // canonicalize should resolve the symlink; normalize_forgiving will prefer canonicalize
        let resolved = normalize_forgiving(&link);
        assert!(resolved.exists());

        // Try to clean up; ignore errors if cleanup fails.
        let _ = std::fs::remove_dir_all(&td);
    }
}
