// TODO: maybe write a macro to do the process spawning shenanigans?

use std::cell::Cell;

use mark_flaky_tests::flaky;

#[macro_use]
mod common;

#[flaky]
#[test]
#[ignore]
#[should_panic(expected = "not yet...")]
fn passes_after_too_many_tries() {
    thread_local! {
        static TRIES: Cell<usize> = Cell::new(0);
    }

    TRIES.with(|tries| {
        if tries.get() < 2 {
            tries.set(tries.get() + 1);
            panic!("not yet...");
        }
    });
}
gen_test_runner!(passes_after_too_many_tries, MARK_FLAKY_TESTS_RETRIES => "2");

// Run this test with `MARK_FLAKY_TESTS_STRICT=true`
#[flaky]
#[test]
#[ignore]
#[should_panic(expected = "too late!")]
fn fails_on_the_second_try() {
    thread_local! {
        static TRIES: Cell<usize> = Cell::new(0);
    }

    TRIES.with(|tries| {
        if tries.get() != 0 {
            panic!("too late!");
        }
        tries.set(tries.get() + 1);
    });
}
gen_test_runner!(fails_on_the_second_try, MARK_FLAKY_TESTS_STRICT => "true");
