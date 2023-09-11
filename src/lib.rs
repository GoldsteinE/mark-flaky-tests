//! There're some tests that sometimes pass and sometimes fail. We call them "flaky".
//!
//! This crate provides a macro attribute `#[flaky]` that allows you to mark all the flaky tests
//! in your codebase. You then have two options:
//!
//! 1. In default mode, `#[flaky]` will retry a test for a few times and pass it if at least one
//!    run has passed.
//! 2. In strict mode, `#[flaky]` will still run test for a few times, but will only pass it
//!    if every run has passed.
//!
//! To enable strict mode, set the environment variable `MARK_FLAKY_TESTS_STRICT=true`.
//!
//! To adjust the amount of times a test is retried, set the environment variable
//! `MARK_FLAKY_TESTS_RETRIES` to the desired amount. Default is 3.
//!
//! To use `#[flaky]` with `#[tokio::test]`, enable the `tokio` feature.
//!
//! Tests that return [`ExitCode`] are currently not supported due to std API limitations.
//!
//! [`ExitCode`]: ::std::process::ExitCode

// lint me harder
#![forbid(unsafe_code, non_ascii_idents)]
#![deny(
    future_incompatible,
    keyword_idents,
    noop_method_call,
    unused_qualifications,
    clippy::wildcard_dependencies,
    clippy::empty_line_after_outer_attr
)]
#![warn(clippy::pedantic, missing_docs)]

/// Mark test as flaky.
///
/// See [crate docs][crate] for details.
pub use mark_flaky_tests_macro::flaky;

#[doc(hidden)]
pub mod _priv {
    #[cfg(feature = "tokio")]
    pub use futures;

    /// Defines whether a result is considered a test failure.
    pub trait IsFailure {
        fn is_failure(&self) -> bool;
    }

    // Tests returning unit types succeed unless panic.
    impl IsFailure for () {
        fn is_failure(&self) -> bool {
            false
        }
    }

    // Tests returning `Result<T, E>` succeed if result is ok and the inner value succeeds.
    impl<T: IsFailure, E> IsFailure for Result<T, E> {
        fn is_failure(&self) -> bool {
            self.as_ref().map(T::is_failure).unwrap_or(true)
        }
    }

    // Not sure why would you want to make tests that never return, but whatever floats your boat.
    impl IsFailure for std::convert::Infallible {
        fn is_failure(&self) -> bool {
            match *self {}
        }
    }

    // We even can implement this for the never type.
    impl IsFailure for Never {
        fn is_failure(&self) -> bool {
            *self
        }
    }

    // Thanks to the `never-say-never` crate for publishing this trick!
    pub trait GetNever {
        type Never;
    }

    impl<R, F: FnOnce() -> R> GetNever for F {
        type Never = R;
    }

    type Never = <fn() -> ! as GetNever>::Never;
}
