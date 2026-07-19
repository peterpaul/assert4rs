use crate::Assert;
use std::fmt::Debug;

/// Builds the `is_eq_to` panic message for a sequence-like pair, or
/// `None` if they're equal. Shared by the `Vec`/array/slice impls below
/// so the message-building logic isn't duplicated three times.
fn build_is_eq_to_message<U: PartialEq + Debug>(
    header: String,
    actual: &[U],
    expected: &[U],
) -> Option<String> {
    if actual == expected {
        return None;
    }
    let (extra, missing) = crate::structural_diff::sequence_diff(actual, expected);
    let mut message = format!("{header}\n  Actual:   `{actual:?}`\n  Expected: `{expected:?}`");
    if !extra.is_empty() {
        message.push_str(&format!("\n  Extra:    `{extra:?}`"));
    }
    if !missing.is_empty() {
        message.push_str(&format!("\n  Missing:  `{missing:?}`"));
    }
    Some(message)
}

/// DSL for [Vec].
impl<U: PartialEq + Debug> Assert<Vec<U>> {
    /// Assert that `self` equals `expected`, reporting a structural
    /// (element-aware) diff on failure instead of a text diff.
    ///
    /// ```
    /// # use assert4rs::Assert;
    /// Assert::that(vec![1, 2, 3]).is_eq_to(vec![1, 2, 3]);
    /// ```
    ///
    /// ```should_panic
    /// # use assert4rs::Assert;
    /// Assert::that(vec![1, 9, 2, 3]).is_eq_to(vec![1, 2, 3]);
    /// ```
    #[track_caller]
    pub fn is_eq_to(self, expected: Vec<U>) -> Self {
        if let Some(message) = build_is_eq_to_message(
            self.header("actual.is_eq_to(expected)"),
            self.actual.as_slice(),
            expected.as_slice(),
        ) {
            panic!("{message}");
        }
        self
    }
}

/// DSL for arrays.
impl<U: PartialEq + Debug, const N: usize> Assert<[U; N]> {
    /// Assert that `self` equals `expected`, reporting a structural
    /// (element-aware) diff on failure instead of a text diff.
    ///
    /// ```
    /// # use assert4rs::Assert;
    /// Assert::that([1, 2, 3]).is_eq_to([1, 2, 3]);
    /// ```
    ///
    /// ```should_panic
    /// # use assert4rs::Assert;
    /// Assert::that([1, 9, 2, 3]).is_eq_to([1, 2, 3, 3]);
    /// ```
    #[track_caller]
    pub fn is_eq_to(self, expected: [U; N]) -> Self {
        if let Some(message) = build_is_eq_to_message(
            self.header("actual.is_eq_to(expected)"),
            self.actual.as_slice(),
            expected.as_slice(),
        ) {
            panic!("{message}");
        }
        self
    }
}

/// DSL for slices.
impl<'a, U: PartialEq + Debug> Assert<&'a [U]> {
    /// Assert that `self` equals `expected`, reporting a structural
    /// (element-aware) diff on failure instead of a text diff.
    ///
    /// ```
    /// # use assert4rs::Assert;
    /// Assert::that(&[1, 2, 3][..]).is_eq_to(&[1, 2, 3][..]);
    /// ```
    ///
    /// ```should_panic
    /// # use assert4rs::Assert;
    /// Assert::that(&[1, 9, 2, 3][..]).is_eq_to(&[1, 2, 3][..]);
    /// ```
    #[track_caller]
    pub fn is_eq_to(self, expected: &'a [U]) -> Self {
        if let Some(message) = build_is_eq_to_message(
            self.header("actual.is_eq_to(expected)"),
            self.actual,
            expected,
        ) {
            panic!("{message}");
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::Assert;

    #[test]
    #[should_panic(expected = "Assertion failed: `(actual.is_eq_to(expected))`")]
    fn is_eq_to_reports_generic_header_without_label() {
        Assert::that(vec![1, 2, 3]).is_eq_to(vec![1, 2, 4]);
    }

    #[test]
    #[should_panic(expected = "Assertion failed for `x`: `(actual.is_eq_to(expected))`")]
    fn is_eq_to_reports_label_when_named() {
        Assert::that(vec![1, 2, 3])
            .named("x")
            .is_eq_to(vec![1, 2, 4]);
    }

    #[test]
    #[should_panic(expected = "Extra:    `[9]`")]
    fn is_eq_to_reports_extra_for_inserted_element() {
        Assert::that(vec![1, 9, 2, 3]).is_eq_to(vec![1, 2, 3]);
    }

    #[test]
    fn is_eq_to_omits_missing_line_when_nothing_missing() {
        let result = std::panic::catch_unwind(|| {
            Assert::that(vec![1, 9, 2, 3]).is_eq_to(vec![1, 2, 3]);
        });
        let message = result.unwrap_err();
        let message = message.downcast_ref::<String>().unwrap();
        assert!(
            !message.contains("Missing:"),
            "unexpected Missing line in: {message}"
        );
    }

    #[test]
    fn is_eq_to_reports_both_extra_and_missing() {
        let result = std::panic::catch_unwind(|| {
            Assert::that(vec![1, 2, 9]).is_eq_to(vec![1, 2, 3]);
        });
        let message = result.unwrap_err();
        let message = message.downcast_ref::<String>().unwrap();
        assert!(message.contains("Extra:    `[9]`"), "message: {message}");
        assert!(message.contains("Missing:  `[3]`"), "message: {message}");
    }

    #[test]
    #[should_panic(expected = "Missing:  `[1]`")]
    fn is_eq_to_handles_empty_vs_nonempty() {
        Assert::that(Vec::<i32>::new()).is_eq_to(vec![1]);
    }

    #[test]
    fn is_eq_to_works_on_arrays() {
        Assert::that([1, 2, 3]).is_eq_to([1, 2, 3]);
    }

    #[test]
    #[should_panic(expected = "Extra:    `[9]`")]
    fn is_eq_to_works_on_slices() {
        Assert::that(&[1, 9, 2, 3][..]).is_eq_to(&[1, 2, 3][..]);
    }
}
