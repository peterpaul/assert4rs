use crate::Assert;

use std::fmt::Debug;

impl<T> Assert<Option<T>>
where
    T: PartialEq + Debug,
{
    /// Assert that `actual` is equal to [Some] `expected' value.
    pub fn is_some(self, expected: T) -> Self {
        self.is(Some(expected))
    }

    /// Assert that `actual` is equal to [None].
    pub fn is_none(self) -> Self {
        self.is(None)
    }

    /// Unwrap the [Option] value, panic for [None].
    pub fn unwrap(self) -> Assert<T> {
        match self.actual {
            Some(value) => Assert::that(value),
            None => panic!(
                "Assertion failed: `(actual.is_some())`
  Actual:   `None`"
            ),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Assert;

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
