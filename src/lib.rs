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
pub enum ErrorWithStacktrace {}

impl<T: std::fmt::Display> From<T> for ErrorWithStacktrace {
    #[track_caller] // Will show the location of the caller in test failure messages
    fn from(error: T) -> Self {
        panic!("error: {} - {}", std::any::type_name::<T>(), error);
    }
}

/// Unit test result
///
/// This type allows panicking when encountering any type of
/// failure. Thus it allows using `?` operator in unit tests but still
/// get the complete stacktrace and exact place of failure during
/// tests.
///
/// # Examples
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

pub type TestResult = std::result::Result<(), ErrorWithStacktrace>;

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

    #[test]
    fn check_if_panics() -> TestResult {
        let result = std::panic::catch_unwind(|| {
            fn test_fn() -> TestResult {
                std::fs::File::open("this-file-does-not-exist")?;
                Ok(())
            }
            let _ = test_fn();
        });
        assert!(result.is_err());
        Ok(())
    }
}
