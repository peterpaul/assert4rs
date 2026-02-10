//! Fluent assertions for Rust.
//!
//! This crate provides a fluent API for writing assertions in tests.
//! All assertions start with [`Assert::that`] and can be chained:
//!
//! ```
//! use assert4rs::Assert;
//!
//! Assert::that("foo")
//!     .is("foo")
//!     .is_not("bar");
//! ```
//!
//! Type-specific assertions are available for [`Option`], [`Result`],
//! and [`Vec`]:
//!
//! ```
//! use assert4rs::Assert;
//!
//! Assert::that(Some(42)).unwrap().is_gt(0);
//! Assert::that(vec![1, 2, 3]).contains(&2);
//! ```
//!
//! Values can be transformed with [`Assert::map`] to apply further
//! assertions:
//!
//! ```
//! use assert4rs::Assert;
//!
//! Assert::that("3")
//!     .map(|v| v.parse::<i32>().unwrap())
//!     .is(3);
//! ```

pub mod equals;
pub mod option;
pub mod result;
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
    ///
    /// ```should_panic
    /// # use assert4rs::Assert;
    /// Assert::that(2).map(|v| v + 2).is(3);
    /// ```
    pub fn map<R>(self, f: impl FnOnce(T) -> R) -> Assert<R> {
        Assert::that(f(self.actual))
    }
}
