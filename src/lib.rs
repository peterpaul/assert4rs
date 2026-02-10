pub mod equals;
pub mod option;
pub mod vec;

/// Entry point for the [Assert] DSL.
///
/// [Assert] provides a fluent API for assertions. An [Assert] holds
/// the `actual` value that can be used in assertion statements.
///
/// ```
/// # use assert4rs::Assert;
/// Assert::that(3).is(3);
/// ```
///
/// It takes ownership of self and returns ownership back so that
/// assertions can be chained in fluent manner.
///
/// Like in the following example:
///
/// ```
/// # use assert4rs::Assert;
/// Assert::that("foo")
///     .is("foo")
///     .is_not("bar");
/// ```
pub struct Assert<T> {
    actual: T,
}

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
    /// # use assert4rs::Assert;
    /// Assert::that("3")
    ///     .map(|v| v.parse::<i32>().unwrap())
    ///     .is(3);
    /// ```
    pub fn map<R>(self, f: impl FnOnce(T) -> R) -> Assert<R> {
        Assert::that(f(self.actual))
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
    #[should_panic(expected = "Assertion failed: `(actual.is_some())`
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
