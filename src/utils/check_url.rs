use std::collections::{HashMap, HashSet};
use url::Url;

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
fn canonical_port(p: u16) -> u16 {
    // Treat default web ports (80, 443) as equivalent by mapping them to a
    // canonical sentinel (0). Non-default ports are preserved.
    if p == 80 || p == 443 { 0 } else { p }
}

/// Parse a possibly scheme-less URL string. If the input contains "://"
/// we parse it as-is; otherwise we prepend "https://" (scheme-less is treated
/// as HTTPS by default per specification).
fn parse_with_default_scheme(s: &str) -> Result<Url, url::ParseError> {
    if s.contains("://") {
        Url::parse(s)
    } else {
        Url::parse(&format!("https://{s}"))
    }
}

/// Construct a comparison origin key from a Url consisting of:
/// (lowercased host, canonicalized port).
///
/// We intentionally IGNORE scheme so that http/https are considered equivalent
/// for matching. Ports 80 and 443 are treated as equivalent by mapping them to
/// the sentinel port 0; other explicit ports remain distinct. Returns None if
/// host or default port is unavailable (opaque URLs).
fn origin_key(u: &Url) -> Option<(String, u16)> {
    let host = u.host_str()?.to_ascii_lowercase();
    let port = u.port_or_known_default()?;
    Some((host, canonical_port(port)))
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
    if a.len() == 1 && a[0].as_ref() == "*" {
        return true;
    }
    // オリジンごとに最小集合のベースセグメントを持つ
    let mut bases: HashMap<(String, u16), Vec<Vec<String>>> = HashMap::new();

    for s in a {
        let Ok(url) = parse_with_default_scheme(s.as_ref()) else {
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
        let Ok(url) = parse_with_default_scheme(s.as_ref()) else {
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
    if a.len() == 1 && a[0].as_ref() == "*" {
        return true;
    }
    let set_a: HashSet<String> = a
        .iter()
        .filter_map(|s| parse_with_default_scheme(s.as_ref()).ok())
        .map(|u| u.to_string()) // 既定ポートはシリアライズに出ない
        .collect();
    b.iter()
        .filter_map(|s| parse_with_default_scheme(s.as_ref()).ok())
        .all(|u| set_a.contains(&u.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(k1, ("example.com".into(), 0));
        assert_eq!(k1, k2);
    }

    #[test]
    fn test_origin_key_non_default_and_https() {
        let u1 = Url::parse("http://example.com:8080/").unwrap();
        let u2 = Url::parse("https://example.com/").unwrap();
        assert_eq!(origin_key(&u1).unwrap(), ("example.com".into(), 8080));
        assert_eq!(origin_key(&u2).unwrap(), ("example.com".into(), 0));
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
        assert_eq!(origin_key(&u).unwrap(), ("[2001:db8::1]".into(), 0));
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
        assert!(urls_cover_by_ancestor(&a, &b));
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
    // Additional tests for scheme-less inputs and port handling
    #[test]
    fn test_urls_cover_by_ancestor_scheme_less_input() {
        // scheme-less target should be treated as https and match https base
        let a = vec!["https://example.com/base"];
        let b = vec!["example.com/base/x"];
        assert!(urls_cover_by_ancestor(&a, &b));

        // scheme-less base is parsed as https; http target should still match because scheme is ignored
        let a2 = vec!["example.com/base"];
        let b2 = vec!["http://example.com/base/x"];
        assert!(urls_cover_by_ancestor(&a2, &b2));
    }

    #[test]
    fn test_urls_cover_by_ancestor_port_80_443_equivalence() {
        // explicit 80 and 443 should be treated equivalent
        let a = vec!["http://example.com:80/base"];
        let b = vec!["https://example.com:443/base/x"];
        assert!(urls_cover_by_ancestor(&a, &b));
    }

    #[test]
    fn test_urls_cover_by_ancestor_same_nondefault_port_across_scheme() {
        // same non-default port (8080) should match across schemes
        let a = vec!["http://example.com:8080/base"];
        let b = vec!["https://example.com:8080/base/x"];
        assert!(urls_cover_by_ancestor(&a, &b));
    }

    #[test]
    fn test_urls_cover_by_ancestor_different_nondefault_ports_not_equal() {
        // different non-default ports must not match
        let a = vec!["http://example.com:8080/base"];
        let b = vec!["https://example.com:9090/base/x"];
        assert!(!urls_cover_by_ancestor(&a, &b));
    }

    #[test]
    fn test_urls_cover_by_ancestor_wildcard() {
        let a = vec!["*"];
        let b = vec![
            "https://example.com/project/src/lib/mod.rs",
            "https://data.example.com/data/input/file.csv?part=1",
            "http://example.com/base/child",
        ];
        assert!(urls_cover_by_ancestor(&a, &b));
    }

    #[test]
    fn test_urls_cover_as_set_wildcard() {
        let a = vec!["*"];
        let b = vec!["https://example.com/api", "https://example.com/"];
        assert!(urls_cover_as_set(&a, &b));
    }
}
