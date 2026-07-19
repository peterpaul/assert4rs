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
    #[track_caller]
    pub fn is_ok(self) -> Self {
        match &self.actual {
            Ok(_) => self,
            Err(error) => panic!(
                "{}\n  Actual:   `Err({:?})`",
                self.header("actual.is_ok()"),
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
    #[track_caller]
    pub fn is_err(self) -> Self {
        match &self.actual {
            Ok(value) => panic!(
                "{}\n  Actual:   `Ok({:?})`",
                self.header("actual.is_err()"),
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
    #[track_caller]
    pub fn unwrap(self) -> Assert<T> {
        let header = self.header("actual.is_ok()");
        match self.actual {
            Ok(value) => Assert::that(value),
            Err(error) => panic!("{header}\n  Actual:   `Err({error:?})`"),
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
    #[track_caller]
    pub fn unwrap_err(self) -> Assert<E> {
        let header = self.header("actual.is_err()");
        match self.actual {
            Ok(value) => panic!("{header}\n  Actual:   `Ok({value:?})`"),
            Err(error) => Assert::that(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Assert;

    #[test]
    #[should_panic(expected = "Assertion failed for `x`: `(actual.is_ok())`")]
    fn is_ok_reports_label_when_named() {
        let result: Result<i32, i32> = Err(2);
        Assert::that(result).named("x").is_ok();
    }

    #[test]
    #[should_panic(expected = "Assertion failed for `x`: `(actual.is_err())`")]
    fn unwrap_err_reports_label_when_named() {
        let result: Result<i32, i32> = Ok(2);
        Assert::that(result).named("x").unwrap_err();
    }
}
