use crate::Assert;
use std::fmt::Debug;

/// DSL for [Vec].
impl<T> Assert<Vec<T>>
where
    T: PartialEq + Debug,
{
    /// Assert that the actual vector contains a specific `expected`
    /// value.
    ///
    /// ```
    /// # use assert4rs::{Assert};
    /// Assert::that(vec![1, 2, 3]).contains(&2);
    /// ```
    pub fn contains(self, expected: &T) -> Self {
        assert!(
            self.actual.contains(expected),
            "Assertion failed: `(actual.contains(expected))`
  Actual:   `{:?}`
  Expected: `{:?}`",
            self.actual,
            expected
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
    pub fn is_empty(self) -> Self {
        assert!(
            self.actual.is_empty(),
            "Assertion failed: `(actual.is_empty())`
  Actual: `{:?}`",
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
    pub fn has_length(self, expected: usize) -> Self {
        assert!(
            self.actual.len() == expected,
            "Assertion failed: `(actual.len() == expected)`
  Actual:   `{}`
  Expected: `{}`",
            self.actual.len(),
            expected,
        );
        self
    }
}
