use mark_flaky_tests::flaky;
use std::cell::Cell;

#[flaky]
#[test]
fn just_passes() {}

#[flaky]
#[test]
fn passes_after_a_few_tries() {
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

#[flaky]
#[test]
#[should_panic]
fn passes_after_too_many_tries() {
    thread_local! {
        static TRIES: Cell<usize> = Cell::new(0);
    }

    TRIES.with(|tries| {
        if tries.get() < 3 {
            tries.set(tries.get() + 1);
            panic!("not yet...");
        }
    });
}
