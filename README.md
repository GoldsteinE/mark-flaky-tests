![build status](https://img.shields.io/github/actions/workflow/status/GoldsteinE/mark-flaky-tests/main.yml)

# mark-flaky-tests

<!-- cargo-rdme start -->

There're some tests that sometimes pass and sometimes fail. We call them "flaky".

This crate provides a macro attribute `#[flaky]` that allows you to mark all the flaky tests
in your codebase. You then have two options:

1. In default mode, `#[flaky]` will retry a test for a few times and pass it if at least one
   run has passed.
2. In strict mode, `#[flaky]` will still run test for a few times, but will only pass it
   if every run has passed.

To enable strict mode, set the environment variable `MARK_FLAKY_TESTS_STRICT=true`.

To adjust the amount of times a test is retried, set the environment variable
`MARK_FLAKY_TESTS_RETRIES` to the desired amount. Default is 3.

To use `#[flaky]` with `#[tokio::test]`, enable the `tokio` feature.

Tests that return [`ExitCode`] are currently not supported due to std API limitations.

[`ExitCode`]: https://doc.rust-lang.org/stable/std/process/struct.ExitCode.html

<!-- cargo-rdme end -->
