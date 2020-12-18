use std::fmt::Debug;

pub struct Assert<T> {
    actual: T,
}

/// Entry point for the [Assert] DSL.
///
/// [Assert] provides a fluent API for assertions. An [Assert] holds
/// the `actual` value that can be used in assertion statements.
///
/// ```
/// # use assert4rs::{Assert, AssertEquals};
/// Assert::that(3).is(3);
/// ```
impl<T> Assert<T> {
    /// Create an [Assert] instance for the `actual` value.
    ///
    /// All assertions start with `Assert::that(actual)`.
    pub fn that(actual: T) -> Self {
        Assert { actual }
    }

    /// Maps the `actual` value using lambda `f`.
    ///
    /// This method is intended to map the actual value in order to
    /// apply further assertions.
    ///
    /// ```
    /// # use assert4rs::{Assert, AssertEquals};
    /// Assert::that("3")
    ///     .map(|v| v.parse::<i32>().unwrap())
    ///     .is(3);
    /// ```
    pub fn map<R>(self, f: impl Fn(T) -> R) -> Assert<R> {
        Assert::that(f(self.actual))
    }
}

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

/// DSL for [Option] assertions.
pub trait AssertOption<T> {
    /// Assert that `actual` is equal to [Some] `expected' value.
    fn is_some(self, expected: T) -> Self;
    /// Assert that `actual` is equal to [None].
    fn is_none(self) -> Self;
    /// Unwrap the [Option] value, panic for [None].
    fn unwrap(self) -> Assert<T>;
}

impl<T> AssertOption<T> for Assert<Option<T>>
where
    T: PartialEq + Debug,
{
    fn is_some(self, expected: T) -> Self {
        self.is(Some(expected))
    }

    fn is_none(self) -> Self {
        self.is(None)
    }

    fn unwrap(self) -> Assert<T> {
        match self.actual {
            Some(value) => Assert::that(value),
            None => panic!(
                "Assertion failed: `(actual == Some(expected)`
  Actual:   `None`"
            ),
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_succeeds_for_equal_values() {
        Assert::that(2).is(2);
    }

    #[test]
    fn is_succeeds_for_equal_string_values() {
        Assert::that(String::from("2")).is(String::from("2"));
    }

    #[test]
    fn is_succeeds_for_equal_string_and_string_slice() {
        Assert::that(String::from("2")).is("2");
    }

    #[test]
    #[should_panic(expected = "Assertion failed: `(actual == expected)`
  Actual:   `2`
  Expected: `3`")]
    fn is_panics_for_different_values() {
        Assert::that(2).is(3);
    }

    #[test]
    #[should_panic(expected = "Assertion failed: `(actual != other)`
  Actual:   `2`
  Other:    `2`")]
    fn ne_panics_for_equal_values() {
        Assert::that(2).is_not(2);
    }

    #[test]
    fn ne_succeeds_for_different_values() {
        Assert::that(2).is_not(3);
    }

    #[test]
    fn map_converts_assertion_pass() {
        Assert::that(2).map(|v| v + 2).is(4);
    }

    #[test]
    #[should_panic(expected = "Assertion failed: `(actual == expected)`
  Actual:   `4`
  Expected: `3`")]
    fn map_converts_assertion_fail() {
        Assert::that(2).map(|v| v + 2).is(3);
    }

    #[test]
    fn option_is_some_succeeds_for_equal_values() {
        Assert::that(Some(2)).is_some(2);
    }

    #[test]
    fn option_unwrap_successfully_unwraps_some_value() {
        Assert::that(Some(2)).unwrap().is(2);
    }

    #[test]
    #[should_panic(expected = "Assertion failed: `(actual == Some(expected)`
  Actual:   `None`")]
    fn option_unwrap_panics_for_none() {
        let i: Option<i32> = None;
        Assert::that(i).unwrap();
    }

    #[test]
    #[should_panic(expected = "Assertion failed: `(actual == expected)`
  Actual:   `None`
  Expected: `Some(2)`")]
    fn option_is_some_panics_for_none() {
        Assert::that(None).is_some(2);
    }

    #[test]
    fn option_is_none_succeeds_for_none() {
        let i: Option<i32> = None;
        Assert::that(i).is_none();
    }

    #[test]
    #[should_panic(expected = "Assertion failed: `(actual == expected)`
  Actual:   `Some(2)`
  Expected: `None`")]
    fn option_is_none_panics_for_some() {
        Assert::that(Some(2)).is_none();
    }
}
