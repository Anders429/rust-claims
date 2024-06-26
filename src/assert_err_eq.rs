/// Asserts that expression returns [`Err(E)`] variant
/// and its value of `E` type equals to the right expression.
///
/// ## Uses
///
/// Assertions are always checked in both debug and release builds, and cannot be disabled.
/// See [`debug_assert_err_eq!`] for assertions that are not enabled in release builds by default.
///
/// ## Custom messages
///
/// This macro has a second form, where a custom panic message can be provided
/// with or without arguments for formatting. See [`std::fmt`] for syntax for this form.
///
/// ## Examples
///
/// ```rust
/// # #[macro_use] extern crate claims;
/// # fn main() {
/// let res: Result<(), i32> = Err(1);
///
/// assert_err_eq!(res, 1);
///
/// // With custom messages
/// assert_err_eq!(res, 1, "Everything is good with {:?}", res);
/// # }
/// ```
///
/// Value of `E` type from `Err(E)` will be returned from the macro call:
///
/// ```rust
/// # #[macro_use] extern crate claims;
/// # fn main() {
/// let res: Result<(), i32> = Err(1);
///
/// let value = assert_err_eq!(res, 1);
/// assert_eq!(value, 1);
/// # }
/// ```
///
/// `Ok(..)` variant will cause panic:
///
/// ```rust,should_panic
/// # #[macro_use] extern crate claims;
/// # fn main() {
/// let res: Result<(), i32> = Ok(());
///
/// assert_err_eq!(res, 1);  // Will panic
/// # }
/// ```
///
/// [`Err(E)`]: https://doc.rust-lang.org/core/result/enum.Result.html#variant.Err
/// [`std::fmt`]: https://doc.rust-lang.org/std/fmt/index.html
/// [`debug_assert_err_eq!`]: ./macro.debug_assert_err_eq.html
#[cfg(rustc_1_11)]
#[macro_export]
macro_rules! assert_err_eq {
    ($cond:expr, $expected:expr,) => {
        $crate::assert_err_eq!($cond, $expected);
    };
    ($cond:expr, $expected:expr) => {
        match $cond {
            Err(t) => {
                assert_eq!(t, $expected);
                t
            },
            ok @ Ok(..) => {
                panic!("assertion failed, expected Err(..), got {:?}", ok);
            }
        }
    };
    ($cond:expr, $expected:expr, $($arg:tt)+) => {
        match $cond {
            Err(t) => {
                assert_eq!(t, $expected, $($arg)+);
                t
            },
            ok @ Ok(..) => {
                panic!("assertion failed, expected Err(..), got {:?}: {}", ok, format_args!($($arg)+));
            }
        }
    };
}

#[cfg(not(rustc_1_11))]
#[macro_export]
macro_rules! assert_err_eq {
    ($cond:expr, $expected:expr,) => {
        $crate::assert_err_eq!($cond, $expected);
    };
    ($cond:expr, $expected:expr) => {
        match $cond {
            Err(t) => {
                assert_eq!(t, $expected);
                t
            },
            ok @ Ok(..) => {
                panic!("assertion failed, expected Err(..), got {:?}", ok);
            }
        }
    };
    ($cond:expr, $expected:expr, $($arg:tt)+) => {
        match $cond {
            Err(t) => {
                assert_eq!(t, $expected);
                t
            },
            ok @ Ok(..) => {
                panic!("assertion failed, expected Err(..), got {:?}: {}", ok, format_args!($($arg)+));
            }
        }
    };
}

/// Asserts that expression returns [`Err(E)`] variant in runtime.
///
/// Like [`assert_err_eq!`], this macro also has a second version,
/// where a custom panic message can be provided.
///
/// ## Uses
///
/// See [`debug_assert!`] documentation for possible use cases.
/// The same applies to this macro.
///
/// [`Err(E)`]: https://doc.rust-lang.org/core/result/enum.Result.html#variant.Err
/// [`debug_assert!`]: https://doc.rust-lang.org/std/macro.debug_assert.html
/// [`assert_err_eq!`]: ./macro.assert_err_eq.html
#[macro_export]
macro_rules! debug_assert_err_eq {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::assert_err_eq!($($arg)*); })
}

#[cfg(test)]
#[cfg(not(has_private_in_public_issue))]
mod tests {
    #[test]
    #[cfg_attr(
        not(rustc_1_11),
        ignore = "custom message propagation is only available in rustc 1.11.0 or later"
    )]
    #[should_panic(expected = "foo")]
    fn custom_message_propagation() {
        let _ = assert_err_eq!(Err::<(), _>(1), 2, "foo");
    }
}
