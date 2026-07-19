//! Structural (element/entry-aware) diff helpers for collection equality
//! checks. Unlike `diff.rs` (which diffs `{:?}`-rendered text), these
//! functions operate directly on typed collections, so they can name
//! actual missing/extra/changed elements instead of a byte offset.

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// Returns `(extra, missing)`: elements in `actual` but not `expected`,
/// and elements in `expected` but not `actual`, matched by value
/// (ignoring position). Deterministic without any bound beyond
/// `PartialEq` — walks `actual`/`expected` in their own existing order.
#[allow(dead_code)]
pub(crate) fn sequence_diff<'a, U: PartialEq>(
    actual: &'a [U],
    expected: &'a [U],
) -> (Vec<&'a U>, Vec<&'a U>) {
    let mut expected_remaining: Vec<&U> = expected.iter().collect();
    let mut extra = Vec::new();
    for a in actual {
        if let Some(pos) = expected_remaining.iter().position(|e| **e == *a) {
            expected_remaining.remove(pos);
        } else {
            extra.push(a);
        }
    }
    (extra, expected_remaining)
}

/// Returns `(extra, missing)`: the symmetric difference between `actual`
/// and `expected`, each side sorted for deterministic output (`HashSet`
/// iteration order is not itself deterministic).
#[allow(dead_code)]
pub(crate) fn set_diff<'a, T: Eq + Hash + Ord>(
    actual: &'a HashSet<T>,
    expected: &'a HashSet<T>,
) -> (Vec<&'a T>, Vec<&'a T>) {
    let mut extra: Vec<&T> = actual.difference(expected).collect();
    let mut missing: Vec<&T> = expected.difference(actual).collect();
    extra.sort();
    missing.sort();
    (extra, missing)
}

/// Returns `(missing_keys, extra_keys, changed)`: keys `expected` has
/// that `actual` doesn't, keys `actual` has that `expected` doesn't, and
/// keys present in both with different values (as `(key, actual_value,
/// expected_value)`). All three sorted by key for deterministic output.
#[allow(dead_code, clippy::type_complexity)]
pub(crate) fn map_diff<'a, K: Eq + Hash + Ord, V: PartialEq>(
    actual: &'a HashMap<K, V>,
    expected: &'a HashMap<K, V>,
) -> (Vec<&'a K>, Vec<&'a K>, Vec<(&'a K, &'a V, &'a V)>) {
    let mut missing_keys = Vec::new();
    let mut changed = Vec::new();
    for (k, ev) in expected {
        match actual.get(k) {
            None => missing_keys.push(k),
            Some(av) if av != ev => changed.push((k, av, ev)),
            _ => {}
        }
    }
    let mut extra_keys: Vec<&K> = actual
        .keys()
        .filter(|k| !expected.contains_key(k))
        .collect();
    missing_keys.sort();
    extra_keys.sort();
    changed.sort_by_key(|(k, _, _)| *k);
    (missing_keys, extra_keys, changed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn sequence_diff_identical_has_no_difference() {
        let (extra, missing) = sequence_diff(&[1, 2, 3], &[1, 2, 3]);
        assert!(extra.is_empty());
        assert!(missing.is_empty());
    }

    #[test]
    fn sequence_diff_finds_inserted_element() {
        let (extra, missing) = sequence_diff(&[1, 9, 2, 3], &[1, 2, 3]);
        assert_eq!(extra, vec![&9]);
        assert!(missing.is_empty());
    }

    #[test]
    fn sequence_diff_finds_removed_element() {
        let (extra, missing) = sequence_diff(&[1, 2], &[1, 2, 3]);
        assert!(extra.is_empty());
        assert_eq!(missing, vec![&3]);
    }

    #[test]
    fn sequence_diff_finds_both_extra_and_missing() {
        let (extra, missing) = sequence_diff(&[1, 9], &[1, 2]);
        assert_eq!(extra, vec![&9]);
        assert_eq!(missing, vec![&2]);
    }

    #[test]
    fn set_diff_identical_has_no_difference() {
        let a = HashSet::from([1, 2, 3]);
        let b = HashSet::from([1, 2, 3]);
        let (extra, missing) = set_diff(&a, &b);
        assert!(extra.is_empty());
        assert!(missing.is_empty());
    }

    #[test]
    fn set_diff_is_sorted() {
        let actual = HashSet::from([1, 2, 3]);
        let expected = HashSet::from([1, 2, 4]);
        let (extra, missing) = set_diff(&actual, &expected);
        assert_eq!(extra, vec![&3]);
        assert_eq!(missing, vec![&4]);
    }

    #[test]
    fn map_diff_finds_missing_extra_and_changed() {
        let actual = HashMap::from([("a", 1), ("b", 2), ("d", 4)]);
        let expected = HashMap::from([("a", 1), ("b", 99), ("c", 3)]);
        let (missing_keys, extra_keys, changed) = map_diff(&actual, &expected);
        assert_eq!(missing_keys, vec![&"c"]);
        assert_eq!(extra_keys, vec![&"d"]);
        assert_eq!(changed, vec![(&"b", &2, &99)]);
    }

    #[test]
    fn map_diff_identical_has_no_difference() {
        let a = HashMap::from([("a", 1)]);
        let b = HashMap::from([("a", 1)]);
        let (missing_keys, extra_keys, changed) = map_diff(&a, &b);
        assert!(missing_keys.is_empty());
        assert!(extra_keys.is_empty());
        assert!(changed.is_empty());
    }
}
