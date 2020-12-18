use crate::Assert;
use std::fmt::Debug;

/// Used to assert equality or inequality of some value with `self`.
///
/// It takes ownership of self and returns ownership back so that
/// assertions can be chained in fluent manner.
///
/// Like in the following example:
///
/// ```
/// # use assert4rs::{Assert, AssertEquals};
/// Assert::that("foo")
///     .is("foo")
///     .is_not("bar");
/// ```
///
/// When implementing this trait, panic when the assertion fails.
///
/// For example the following code should panic:
///
/// ```should_panic
/// # use assert4rs::{Assert, AssertEquals};
/// Assert::that("foo").is("bar");
/// ```
pub trait AssertEquals<R> {
    /// Assert that `self` is equal to the `expected` value.
    fn is(self, expected: R) -> Self;
    /// Assert that `self` is not equal to the `other` value.
    fn is_not(self, other: R) -> Self;
}

/// Default implementation of [AssertEquals].
impl<T, R> AssertEquals<R> for Assert<T>
where
    T: PartialEq<R> + Debug,
    R: Debug,
{
    fn is(self, expected: R) -> Self {
        assert!(
            self.actual == expected,
            "Assertion failed: `(actual == expected)`
  Actual:   `{:?}`
  Expected: `{:?}`",
            self.actual,
            expected,
        );
        self
    }

    fn is_not(self, other: R) -> Self {
        assert!(
            self.actual != other,
            "Assertion failed: `(actual != other)`
  Actual:   `{:?}`
  Other:    `{:?}`",
            self.actual,
            other,
        );
        self
    }
}
