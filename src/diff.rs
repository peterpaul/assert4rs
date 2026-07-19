//! Text-based diff helpers used to render assertion failure messages.
//!
//! These functions operate on already-formatted text (typically the
//! output of `{:?}`) and know nothing about `Assert` or any assertion
//! semantics, so they can be tested directly with plain strings.

/// The byte index and the two characters at that index where `a` and
/// `b` first diverge. `None` on either side means that side's string
/// ended first (a strict-prefix case).
#[allow(dead_code)]
pub(crate) struct Difference {
    pub(crate) index: usize,
    pub(crate) actual: Option<char>,
    pub(crate) expected: Option<char>,
}

/// Finds the first point where `a` and `b` differ. Returns `None` if
/// `a` and `b` are identical.
#[allow(dead_code)]
pub(crate) fn first_difference(a: &str, b: &str) -> Option<Difference> {
    let mut a_iter = a.char_indices();
    let mut b_iter = b.chars();

    loop {
        match (a_iter.next(), b_iter.next()) {
            (Some((index, ca)), Some(cb)) => {
                if ca != cb {
                    return Some(Difference {
                        index,
                        actual: Some(ca),
                        expected: Some(cb),
                    });
                }
            }
            (Some((index, ca)), None) => {
                return Some(Difference {
                    index,
                    actual: Some(ca),
                    expected: None,
                });
            }
            (None, Some(cb)) => {
                return Some(Difference {
                    index: a.len(),
                    actual: None,
                    expected: Some(cb),
                });
            }
            (None, None) => return None,
        }
    }
}

/// Renders a human-readable description of a [`Difference`], e.g.
/// `"differs at byte 8 ('p' vs 'o')"` or, for a length mismatch,
/// `"actual has extra content at byte 12, starting with 'x'"`.
#[allow(dead_code)]
pub(crate) fn describe(diff: &Difference) -> String {
    match (diff.actual, diff.expected) {
        (Some(a), Some(b)) => {
            format!("differs at byte {} ({a:?} vs {b:?})", diff.index)
        }
        (Some(a), None) => {
            format!(
                "actual has extra content at byte {}, starting with {a:?}",
                diff.index
            )
        }
        (None, Some(b)) => {
            format!(
                "expected has extra content at byte {}, starting with {b:?}",
                diff.index
            )
        }
        (None, None) => unreachable!("Difference always has at least one side set"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_strings_have_no_difference() {
        assert!(first_difference("abc", "abc").is_none());
    }

    #[test]
    fn single_char_difference_is_located() {
        let diff = first_difference("abc", "abx").unwrap();
        assert_eq!(diff.index, 2);
        assert_eq!(diff.actual, Some('c'));
        assert_eq!(diff.expected, Some('x'));
    }

    #[test]
    fn actual_longer_than_expected() {
        let diff = first_difference("abcd", "abc").unwrap();
        assert_eq!(diff.index, 3);
        assert_eq!(diff.actual, Some('d'));
        assert_eq!(diff.expected, None);
    }

    #[test]
    fn expected_longer_than_actual() {
        let diff = first_difference("abc", "abcd").unwrap();
        assert_eq!(diff.index, 3);
        assert_eq!(diff.actual, None);
        assert_eq!(diff.expected, Some('d'));
    }

    #[test]
    fn empty_strings_have_no_difference() {
        assert!(first_difference("", "").is_none());
    }

    #[test]
    fn multi_byte_utf8_boundary() {
        // 'é' is 2 bytes in UTF-8; the difference is at char index 1,
        // byte index 1 (right after 'a').
        let diff = first_difference("aée", "aoe").unwrap();
        assert_eq!(diff.index, 1);
        assert_eq!(diff.actual, Some('é'));
        assert_eq!(diff.expected, Some('o'));
    }

    #[test]
    fn describe_formats_char_difference() {
        let diff = first_difference("abc", "abx").unwrap();
        assert_eq!(describe(&diff), "differs at byte 2 ('c' vs 'x')");
    }

    #[test]
    fn describe_formats_actual_longer() {
        let diff = first_difference("abcd", "abc").unwrap();
        assert_eq!(
            describe(&diff),
            "actual has extra content at byte 3, starting with 'd'"
        );
    }

    #[test]
    fn describe_formats_expected_longer() {
        let diff = first_difference("abc", "abcd").unwrap();
        assert_eq!(
            describe(&diff),
            "expected has extra content at byte 3, starting with 'd'"
        );
    }
}
