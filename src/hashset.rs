use crate::Assert;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

/// DSL for [HashSet].
impl<T> Assert<HashSet<T>>
where
    T: Eq + Hash + Debug,
{
    /// Assert that the actual set contains a specific `expected` value.
    ///
    /// ```
    /// # use assert4rs::Assert;
    /// # use std::collections::HashSet;
    /// Assert::that(HashSet::from([1, 2, 3])).contains(&2);
    /// ```
    ///
    /// ```should_panic
    /// # use assert4rs::Assert;
    /// # use std::collections::HashSet;
    /// Assert::that(HashSet::from([1, 2, 3])).contains(&9);
    /// ```
    #[track_caller]
    pub fn contains(self, expected: &T) -> Self {
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

    /// Assert that the actual set is empty.
    ///
    /// ```
    /// # use assert4rs::Assert;
    /// # use std::collections::HashSet;
    /// Assert::that(HashSet::<i32>::new()).is_empty();
    /// ```
    ///
    /// ```should_panic
    /// # use assert4rs::Assert;
    /// # use std::collections::HashSet;
    /// Assert::that(HashSet::from([1])).is_empty();
    /// ```
    #[track_caller]
    pub fn is_empty(self) -> Self {
        assert!(
            self.actual.is_empty(),
            "{}\n  Actual: `{:?}`",
            self.header("actual.is_empty()"),
            self.actual,
        );
        self
    }

    /// Assert that the actual set has the given length.
    ///
    /// ```
    /// # use assert4rs::Assert;
    /// # use std::collections::HashSet;
    /// Assert::that(HashSet::from([1, 2, 3])).has_length(3);
    /// ```
    ///
    /// ```should_panic
    /// # use assert4rs::Assert;
    /// # use std::collections::HashSet;
    /// Assert::that(HashSet::from([1, 2, 3])).has_length(2);
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
    use std::collections::HashSet;

    #[test]
    #[should_panic(
        expected = "Assertion failed: `(actual.contains(expected))`\n  Actual:   `{1}`\n  Expected to contain: `9`\n  Missing: `9`"
    )]
    fn contains_reports_full_message() {
        Assert::that(HashSet::from([1])).contains(&9);
    }

    #[test]
    #[should_panic(expected = "Assertion failed for `x`: `(actual.contains(expected))`")]
    fn contains_reports_label_when_named() {
        Assert::that(HashSet::from([1])).named("x").contains(&9);
    }

    #[test]
    #[should_panic(expected = "Assertion failed: `(actual.is_empty())`\n  Actual: `{1}`")]
    fn is_empty_reports_full_message() {
        Assert::that(HashSet::from([1])).is_empty();
    }

    #[test]
    #[should_panic(expected = "Assertion failed for `x`: `(actual.is_empty())`")]
    fn is_empty_reports_label_when_named() {
        Assert::that(HashSet::from([1])).named("x").is_empty();
    }

    #[test]
    #[should_panic(
        expected = "Assertion failed: `(actual.len() == expected)`\n  Actual:   `1`\n  Expected: `5`"
    )]
    fn has_length_reports_full_message() {
        Assert::that(HashSet::from([1])).has_length(5);
    }

    #[test]
    #[should_panic(expected = "Assertion failed for `x`: `(actual.len() == expected)`")]
    fn has_length_reports_label_when_named() {
        Assert::that(HashSet::from([1])).named("x").has_length(5);
    }
}
