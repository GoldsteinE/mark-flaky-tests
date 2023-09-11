use mark_flaky_tests::flaky;
use std::cell::Cell;

#[macro_use]
mod common;

#[flaky]
#[test]
fn returns_ok_after_a_few_tries() -> Result<(), &'static str> {
    thread_local! {
        static TRIES: Cell<usize> = Cell::new(0);
    }

    TRIES.with(|tries| {
        if tries.get() < 2 {
            tries.set(tries.get() + 1);
            return Err("not yet...");
        }

        Ok(())
    })
}

#[flaky]
#[test]
#[ignore]
fn returns_ok_after_too_many_tries() -> Result<(), &'static str> {
    thread_local! {
        static TRIES: Cell<usize> = Cell::new(0);
    }

    TRIES.with(|tries| {
        if tries.get() < 3 {
            tries.set(tries.get() + 1);
            return Err("not yet...");
        }

        Ok(())
    })
}
gen_test_runner!(
    #[should_fail]
    returns_ok_after_too_many_tries
);

#[flaky]
#[test]
#[ignore]
fn returns_ok_err() -> Result<Result<(), &'static str>, &'static str> {
    Ok(Err("nope"))
}
gen_test_runner!(
    #[should_fail]
    returns_ok_err
);

#[flaky]
#[test]
fn returns_ok_ok_after_a_few_tries() -> Result<Result<(), &'static str>, &'static str> {
    thread_local! {
        static TRIES: Cell<usize> = Cell::new(0);
    }

    TRIES.with(|tries| {
        if tries.get() < 2 {
            tries.set(tries.get() + 1);
            return Ok(Err("not yet..."));
        }

        Ok(Ok(()))
    })
}
