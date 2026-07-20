use crate::Assert;
use std::fmt::Debug;

/// DSL for [Vec].
impl<T> Assert<Vec<T>> {
    /// Assert that the actual vector contains a specific `expected`
    /// value.
    ///
    /// ```
    /// # use assert4rs::{Assert};
    /// Assert::that(vec![1, 2, 3]).contains(&2);
    /// ```
    #[track_caller]
    pub fn contains(self, expected: &T) -> Self
    where
        T: PartialEq + Debug,
    {
        assert!(
            self.actual.contains(expected),
            "{}\n  Actual:   `{:?}`\n  Expected to contain: `{:?}`\n  Missing: `{:?}`",
            self.header("actual.contains(expected)"),
            self.actual,
            expected,
            expected,
        );
        self
    }

    /// Returns an [Assert] for a value from the [Vec].
    ///
    /// ```
    /// # use assert4rs::{Assert};
    /// Assert::that(vec!['a', 'b', 'c']).get(1).is_some('b');
    /// Assert::that(vec!['a', 'b', 'c']).get(5).is_none();
    /// ```
    pub fn get(mut self, index: usize) -> Assert<Option<T>> {
        if index < self.actual.len() {
            Assert::that(Some(self.actual.swap_remove(index)))
        } else {
            Assert::that(None)
        }
    }

    /// Assert that the actual vector is empty.
    ///
    /// ```
    /// # use assert4rs::Assert;
    /// Assert::that(Vec::<i32>::new()).is_empty();
    /// ```
    ///
    /// ```should_panic
    /// # use assert4rs::Assert;
    /// Assert::that(vec![1, 2, 3]).is_empty();
    /// ```
    #[track_caller]
    pub fn is_empty(self) -> Self
    where
        T: Debug,
    {
        assert!(
            self.actual.is_empty(),
            "{}\n  Actual: `{:?}`",
            self.header("actual.is_empty()"),
            self.actual,
        );
        self
    }

    /// Assert that the actual vector has the given length.
    ///
    /// ```
    /// # use assert4rs::Assert;
    /// Assert::that(vec![1, 2, 3]).has_length(3);
    /// ```
    ///
    /// ```should_panic
    /// # use assert4rs::Assert;
    /// Assert::that(vec![1, 2, 3]).has_length(2);
    /// ```
    #[track_caller]
    pub fn has_length(self, expected: usize) -> Self {
        assert!(
            self.actual.len() == expected,
            "{}\n  Actual:   `{}`\n  Expected: `{}`",
            self.header("actual.len() == expected"),
            self.actual.len(),
            expected,
        );
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::Assert;

    #[test]
    #[should_panic(expected = "Assertion failed for `x`: `(actual.contains(expected))`")]
    fn contains_reports_label_when_named() {
        Assert::that(vec![1, 2, 3]).named("x").contains(&9);
    }

    #[test]
    #[should_panic(
        expected = "Assertion failed: `(actual.contains(expected))`\n  Actual:   `[1, 2, 3]`\n  Expected to contain: `9`\n  Missing: `9`"
    )]
    fn contains_reports_full_message() {
        Assert::that(vec![1, 2, 3]).contains(&9);
    }

    #[test]
    #[should_panic(
        expected = "Assertion failed for `x`: `(actual.is_empty())`\n  Actual: `[1, 2, 3]`"
    )]
    fn is_empty_reports_full_message() {
        Assert::that(vec![1, 2, 3]).named("x").is_empty();
    }

    #[test]
    #[should_panic(
        expected = "Assertion failed for `x`: `(actual.len() == expected)`\n  Actual:   `3`\n  Expected: `5`"
    )]
    fn has_length_reports_full_message() {
        Assert::that(vec![1, 2, 3]).named("x").has_length(5);
    }
}
