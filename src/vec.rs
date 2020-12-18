use crate::Assert;
use std::fmt::Debug;

/// DSL for [Vec].
pub trait AssertVec<T> {
    /// Assert that the actual vector contains a specific `expected`
    /// value.
    ///
    /// ```
    /// # use assert4rs::{Assert, AssertVec};
    /// Assert::that(vec![1, 2, 3]).contains(2);
    /// ```
    fn contains(self, expected: T) -> Self;
    /// Returns an [Assert] for a value from the [Vec].
    ///
    /// ```
    /// # use assert4rs::{Assert, AssertOption, AssertVec};
    /// Assert::that(vec!['a', 'b', 'c']).get(1).is_some('b');
    /// Assert::that(vec!['a', 'b', 'c']).get(5).is_none();
    /// ```
    fn get(&mut self, index: usize) -> Assert<Option<T>>;
}

impl<T> AssertVec<T> for Assert<Vec<T>>
where
    T: PartialEq + Debug,
{
    fn contains(self, expected: T) -> Self {
        assert!(
            self.actual.contains(&expected),
            "Assertion failed: `(actual.contains(expected))`
  Actual:   `{:?}`
  Expected: `{:?}`",
            self.actual,
            expected
        );
        self
    }

    fn get(&mut self, index: usize) -> Assert<Option<T>> {
        match self.actual.get(index) {
            None => Assert::that(None),
            Some(_) => Assert::that(Some(self.actual.swap_remove(index))),
        }
    }
}
