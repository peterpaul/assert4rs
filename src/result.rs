use std::fmt::Debug;

use crate::Assert;

impl<T, E> Assert<Result<T, E>>
where
    T: Debug,
    E: Debug,
{
    /// Assert that `actual` is [Ok].
    ///
    /// ```
    /// # use assert4rs::Assert;
    /// let result: Result<i32, i32> = Ok(2);
    /// Assert::that(result).is_ok();
    /// ```
    ///
    /// ```should_panic
    /// # use assert4rs::Assert;
    /// let result: Result<i32, i32> = Err(2);
    /// Assert::that(result).is_ok();
    /// ```
    pub fn is_ok(self) -> Self {
        match &self.actual {
            Ok(_) => self,
            Err(error) => panic!(
                "Assertion failed: `(actual.is_ok())`
  Actual:   `Err({:?})`",
                error
            ),
        }
    }

    /// Assert that `actual` is [Err].
    ///
    /// ```
    /// # use assert4rs::Assert;
    /// let result: Result<i32, i32> = Err(2);
    /// Assert::that(result).is_err();
    /// ```
    ///
    /// ```should_panic
    /// # use assert4rs::Assert;
    /// let result: Result<i32, i32> = Ok(2);
    /// Assert::that(result).is_err();
    /// ```
    pub fn is_err(self) -> Self {
        match &self.actual {
            Ok(value) => panic!(
                "Assertion failed: `(actual.is_err())`
  Actual:   `Ok({:?})`",
                value
            ),
            Err(_) => self,
        }
    }

    /// Unwrap the [Ok] value, panic for [Err].
    ///
    /// ```
    /// # use assert4rs::Assert;
    /// let result: Result<i32, i32> = Ok(2);
    /// Assert::that(result).unwrap().is(2);
    /// ```
    ///
    /// ```should_panic
    /// # use assert4rs::Assert;
    /// let result: Result<i32, i32> = Err(2);
    /// Assert::that(result).unwrap();
    /// ```
    pub fn unwrap(self) -> Assert<T> {
        match self.actual {
            Ok(value) => Assert::that(value),
            Err(error) => panic!(
                "Assertion failed: `(actual.is_ok())`
  Actual:   `Err({:?})`",
                error
            ),
        }
    }

    /// Unwrap the [Err] value, panic for [Ok].
    ///
    /// ```
    /// # use assert4rs::Assert;
    /// let result: Result<i32, i32> = Err(2);
    /// Assert::that(result).unwrap_err().is(2);
    /// ```
    ///
    /// ```should_panic
    /// # use assert4rs::Assert;
    /// let result: Result<i32, i32> = Ok(2);
    /// Assert::that(result).unwrap_err();
    /// ```
    pub fn unwrap_err(self) -> Assert<E> {
        match self.actual {
            Ok(value) => panic!(
                "Assertion failed: `(actual.is_err())`
  Actual:   `Ok({:?})`",
                value
            ),
            Err(error) => Assert::that(error),
        }
    }
}
