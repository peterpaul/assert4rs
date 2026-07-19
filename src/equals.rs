use crate::Assert;
use std::fmt::Debug;

/// Length of the `"  Actual:   \`"` / `"  Expected: \`"` prefixes used in
/// the failure message below, so the diff pointer lines up under the
/// value's first character. Both prefixes are the same length by
/// construction (checked by `value_prefixes_are_equal_length` below).
const VALUE_PREFIX_LEN: usize = "  Actual:   `".len();

impl<T> Assert<T>
where
    T: Debug,
{
    /// Assert that `self` is equal to the `expected` value.
    ///
    /// ```
    /// # use assert4rs::Assert;
    /// Assert::that(2).is(2);
    /// Assert::that(String::from("2")).is(String::from("2"));
    /// Assert::that(String::from("2")).is("2");
    /// ```
    ///
    /// ```should_panic
    /// # use assert4rs::Assert;
    /// Assert::that(2).is(3);
    /// ```
    #[track_caller]
    pub fn is<R>(self, expected: R) -> Self
    where
        T: PartialEq<R>,
        R: Debug,
    {
        if self.actual == expected {
            return self;
        }
        let actual_debug = format!("{:?}", self.actual);
        let expected_debug = format!("{:?}", expected);
        let pointer = crate::diff::first_difference(&actual_debug, &expected_debug)
            .map(|d| {
                format!(
                    "\n{}^ {}",
                    // `d.index` is a byte offset into the `{:?}`-formatted string, so
                    // the caret may visually misalign for debug output containing
                    // multi-byte UTF-8 characters before the point of difference.
                    " ".repeat(VALUE_PREFIX_LEN + d.index),
                    crate::diff::describe(&d)
                )
            })
            .unwrap_or_default();
        panic!(
            "{}\n  Actual:   `{actual_debug}`\n  Expected: `{expected_debug}`{pointer}",
            self.header("actual == expected"),
        );
    }

    /// Assert that `self` is not equal to the `other` value.
    ///
    /// ```
    /// # use assert4rs::Assert;
    /// Assert::that(2).is_not(3);
    /// ```
    ///
    /// ```should_panic
    /// # use assert4rs::Assert;
    /// Assert::that(2).is_not(2);
    /// ```
    #[track_caller]
    pub fn is_not<R>(self, other: R) -> Self
    where
        T: PartialEq<R>,
        R: Debug,
    {
        assert!(
            self.actual != other,
            "{}\n  Actual:   `{:?}`\n  Other:    `{:?}`",
            self.header("actual != other"),
            self.actual,
            other,
        );
        self
    }

    /// Assert that `self` is greater than the `other` value.
    ///
    /// ```
    /// # use assert4rs::Assert;
    /// Assert::that(3).is_gt(2);
    /// ```
    ///
    /// ```should_panic
    /// # use assert4rs::Assert;
    /// Assert::that(3).is_gt(3);
    /// ```
    ///
    /// ```should_panic
    /// # use assert4rs::Assert;
    /// Assert::that(3).is_gt(4);
    /// ```
    #[track_caller]
    pub fn is_gt<R>(self, other: R) -> Self
    where
        T: PartialOrd<R>,
        R: Debug,
    {
        assert!(
            self.actual > other,
            "{}\n  Actual:   `{:?}`\n  Other:    `{:?}`",
            self.header("actual > other"),
            self.actual,
            other,
        );
        self
    }

    /// Assert that `self` is greater than or equal to the `other` value.
    ///
    /// ```
    /// # use assert4rs::Assert;
    /// Assert::that(3).is_ge(3);
    /// Assert::that(3).is_ge(2);
    /// ```
    ///
    /// ```should_panic
    /// # use assert4rs::Assert;
    /// Assert::that(3).is_ge(4);
    /// ```
    #[track_caller]
    pub fn is_ge<R>(self, other: R) -> Self
    where
        T: PartialOrd<R>,
        R: Debug,
    {
        assert!(
            self.actual >= other,
            "{}\n  Actual:   `{:?}`\n  Other:    `{:?}`",
            self.header("actual >= other"),
            self.actual,
            other,
        );
        self
    }

    /// Assert that `self` is less than the `other` value.
    ///
    /// ```
    /// # use assert4rs::Assert;
    /// Assert::that(3).is_lt(4);
    /// ```
    ///
    /// ```should_panic
    /// # use assert4rs::Assert;
    /// Assert::that(3).is_lt(2);
    /// ```
    ///
    /// ```should_panic
    /// # use assert4rs::Assert;
    /// Assert::that(3).is_lt(3);
    /// ```
    #[track_caller]
    pub fn is_lt<R>(self, other: R) -> Self
    where
        T: PartialOrd<R>,
        R: Debug,
    {
        assert!(
            self.actual < other,
            "{}\n  Actual:   `{:?}`\n  Other:    `{:?}`",
            self.header("actual < other"),
            self.actual,
            other,
        );
        self
    }

    /// Assert that `self` is less than or equal to the `other` value.
    ///
    /// ```
    /// # use assert4rs::Assert;
    /// Assert::that(3).is_le(3);
    /// Assert::that(3).is_le(4);
    /// ```
    ///
    /// ```should_panic
    /// # use assert4rs::Assert;
    /// Assert::that(3).is_le(2);
    /// ```
    #[track_caller]
    pub fn is_le<R>(self, other: R) -> Self
    where
        T: PartialOrd<R>,
        R: Debug,
    {
        assert!(
            self.actual <= other,
            "{}\n  Actual:   `{:?}`\n  Other:    `{:?}`",
            self.header("actual <= other"),
            self.actual,
            other,
        );
        self
    }

    /// Assert that the actual value satisfies the given predicate.
    ///
    /// ```
    /// # use assert4rs::Assert;
    /// Assert::that(4).satisfies(|v| v % 2 == 0);
    /// ```
    ///
    /// ```should_panic
    /// # use assert4rs::Assert;
    /// Assert::that(3).satisfies(|v| v % 2 == 0);
    /// ```
    #[track_caller]
    pub fn satisfies(self, predicate: impl FnOnce(&T) -> bool) -> Self {
        assert!(
            predicate(&self.actual),
            "{}\n  Actual: `{:?}`",
            self.header("satisfies predicate"),
            self.actual,
        );
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::Assert;

    #[test]
    fn value_prefixes_are_equal_length() {
        assert_eq!("  Actual:   `".len(), "  Expected: `".len());
    }

    #[test]
    #[should_panic(expected = "Assertion failed: `(actual == expected)`")]
    fn is_reports_generic_header_without_label() {
        Assert::that(1).is(2);
    }

    #[test]
    #[should_panic(expected = "Assertion failed for `x`: `(actual == expected)`")]
    fn is_reports_label_when_named() {
        Assert::that(1).named("x").is(2);
    }

    #[test]
    #[should_panic(expected = "differs at byte 0 ('1' vs '2')")]
    fn is_reports_diff_pointer() {
        Assert::that(1).is(2);
    }

    #[test]
    #[should_panic(expected = "differs at byte 8 ('p' vs 'o')")]
    fn is_reports_diff_pointer_for_strings() {
        Assert::that(String::from("hello wprld")).is("hello world");
    }

    #[test]
    #[should_panic(expected = "Assertion failed: `(actual != other)`")]
    fn is_not_has_plain_header() {
        Assert::that(1).is_not(1);
    }

    #[test]
    #[should_panic(
        expected = "Assertion failed: `(actual != other)`\n  Actual:   `1`\n  Other:    `1`"
    )]
    fn is_not_reports_full_message() {
        Assert::that(1).is_not(1);
    }

    #[test]
    fn is_not_has_no_diff_pointer() {
        let result = std::panic::catch_unwind(|| {
            Assert::that(1).is_not(1);
        });
        let message = result.unwrap_err();
        let message = message.downcast_ref::<String>().unwrap();
        assert!(
            !message.contains("differs at byte"),
            "unexpected diff pointer in: {message}"
        );
    }

    #[test]
    #[should_panic(
        expected = "Assertion failed: `(actual > other)`\n  Actual:   `3`\n  Other:    `4`"
    )]
    fn is_gt_reports_full_message() {
        Assert::that(3).is_gt(4);
    }

    #[test]
    #[should_panic(
        expected = "Assertion failed: `(actual >= other)`\n  Actual:   `3`\n  Other:    `4`"
    )]
    fn is_ge_reports_full_message() {
        Assert::that(3).is_ge(4);
    }

    #[test]
    #[should_panic(
        expected = "Assertion failed: `(actual < other)`\n  Actual:   `3`\n  Other:    `2`"
    )]
    fn is_lt_reports_full_message() {
        Assert::that(3).is_lt(2);
    }

    #[test]
    #[should_panic(
        expected = "Assertion failed: `(actual <= other)`\n  Actual:   `3`\n  Other:    `2`"
    )]
    fn is_le_reports_full_message() {
        Assert::that(3).is_le(2);
    }

    #[test]
    #[should_panic(expected = "Assertion failed: `(satisfies predicate)`\n  Actual: `3`")]
    fn satisfies_reports_full_message() {
        Assert::that(3).satisfies(|v| v % 2 == 0);
    }

    #[test]
    fn is_diff_pointer_aligns_under_first_difference() {
        let result = std::panic::catch_unwind(|| {
            Assert::that(1).is(2);
        });
        let message = result.unwrap_err();
        let message = message.downcast_ref::<String>().unwrap();
        let expected_pointer_line = format!("{}^ differs at byte 0 ('1' vs '2')", " ".repeat(13));
        assert!(
            message.contains(&expected_pointer_line),
            "expected pointer line not found in: {message}"
        );
    }

    #[test]
    #[should_panic(expected = "expected has extra content at byte 1, starting with '0'")]
    fn is_reports_diff_for_expected_longer() {
        Assert::that(1).is(10);
    }

    #[test]
    #[should_panic(expected = "actual has extra content at byte 1, starting with '0'")]
    fn is_reports_diff_for_actual_longer() {
        Assert::that(10).is(1);
    }

    #[test]
    #[should_panic(expected = "differs at byte 2 ('é' vs 'o')")]
    fn is_reports_diff_pointer_for_multibyte_strings() {
        Assert::that(String::from("aée")).is("aoe");
    }
}
