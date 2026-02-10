use std::fmt::Debug;

use crate::Assert;

impl<T, E> Assert<Result<T, E>>
where
    T: Debug,
    E: Debug,
{
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

    pub fn unwrap_ok(self) -> Assert<T> {
        match self.actual {
            Ok(value) => Assert::that(value),
            Err(error) => panic!(
                "Assertion failed: `(actual.is_ok())`
  Actual:   `Err({:?})`",
                error
            ),
        }
    }

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

#[cfg(test)]
mod test {
    use crate::Assert;

    #[test]
    fn result_is_ok_succeeds_for_ok() {
        let result: Result<i32, i32> = Ok(2);
        Assert::that(result).unwrap_ok().is(2);
    }

    #[test]
    #[should_panic(expected = "Assertion failed: `(actual.is_ok())`
  Actual:   `Err(2)`")]
    fn result_is_ok_panics_for_err() {
        let result: Result<i32, i32> = Err(2);
        Assert::that(result).unwrap_ok().is(2);
    }

    #[test]
    fn result_is_err_succeeds_for_err() {
        let result: Result<i32, i32> = Err(2);
        Assert::that(result).unwrap_err().is(2);
    }

    #[test]
    #[should_panic(expected = "Assertion failed: `(actual.is_err())`
  Actual:   `Ok(2)`")]
    fn result_is_err_panics_for_ok() {
        let result: Result<i32, i32> = Ok(2);
        Assert::that(result).unwrap_err().is(2);
    }
}
