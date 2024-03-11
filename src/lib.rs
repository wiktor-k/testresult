#![doc = include_str!("../README.md")]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]

/// Error with stacktrace
///
/// Any other type of error can be converted to this one but the
/// conversion will always panic.
///
/// This type is useful only in the result of unit tests and cannot be instantiated.
#[derive(Debug)]
#[doc(hidden)]
pub enum TestError {}

impl<T: std::fmt::Display> From<T> for TestError {
    #[track_caller] // Will show the location of the caller in test failure messages
    fn from(error: T) -> Self {
        panic!("error: {} - {}", std::any::type_name::<T>(), error);
    }
}

#[allow(clippy::test_attr_in_doctest)] // the docs illustrate the usage in test functions
/// Unit test result
///
/// This type allows panicking when encountering any type of
/// failure. Thus it allows using `?` operator in unit tests but still
/// get the complete stacktrace and exact place of failure during
/// tests.
///
/// # Examples
///
/// Using [`TestResult`] as a result of the test function:
///
/// ```
/// use testresult::TestResult;
///
/// #[test]
/// fn it_works() -> TestResult {
///     // ...
///     std::fs::File::open("this-file-does-not-exist")?;
///     // ...
///     Ok(())
/// }
/// ```
///
/// As [`TestResult`] is generic one can use it in test helper functions to return
/// objects to test functions.
/// For example [`TestResult`] used in `rstest` fixture returns a [`std::fs::File`] object that
/// can be used by the test:
///
/// ```
/// use rstest::{fixture, rstest};
/// use std::fs::File;
/// use testresult::TestResult;
///
/// #[fixture]
/// fn a_file() -> TestResult<File> {
///     let file = File::open("this-file-does-not-exist")?;
///     // ...
///     Ok(file)
/// }
///
/// #[rstest]
/// fn it_works(file: File) -> TestResult {
///     // ...
///     Ok(())
/// }
/// ```
pub type TestResult<T = ()> = std::result::Result<T, TestError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // ignored test must still compile
              // this checks whether conversion from all errors is accomplished
    fn compilation_works() -> TestResult {
        std::fs::File::open("this-file-does-not-exist")?;
        Ok(())
    }

    // helper function which always fails
    fn test_fn() -> TestResult<std::fs::File> {
        let file = std::fs::File::open("this-file-does-not-exist")?;
        Ok(file)
    }

    #[test]
    fn check_if_panics() -> TestResult {
        let result = std::panic::catch_unwind(|| {
            let _ = test_fn();
        });
        assert!(result.is_err());
        Ok(())
    }
}
