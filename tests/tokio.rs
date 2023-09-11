#![cfg(feature = "tokio")]

use mark_flaky_tests::flaky;
use std::cell::Cell;

async fn check_that_async_works() {}

#[flaky]
#[tokio::test]
async fn just_passes() {
    check_that_async_works().await;
}

#[flaky]
#[tokio::test]
async fn passes_after_a_few_tries() {
    thread_local! {
        static TRIES: Cell<usize> = Cell::new(0);
    }

    check_that_async_works().await;
    TRIES.with(|tries| {
        if tries.get() < 2 {
            tries.set(tries.get() + 1);
            panic!("not yet...");
        }
    });
}

#[flaky]
#[tokio::test]
#[should_panic(expected = "not yet...")]
async fn passes_after_too_many_tries() {
    thread_local! {
        static TRIES: Cell<usize> = Cell::new(0);
    }

    check_that_async_works().await;
    TRIES.with(|tries| {
        if tries.get() < 4 {
            tries.set(tries.get() + 1);
            panic!("not yet...");
        }
    });
}

#[tokio::test]
#[flaky]
async fn works_with_different_attribute_order() {}

#[tokio::test]
#[flaky]
async fn passes_after_a_few_tries_with_different_attribute_order() {
    thread_local! {
        static TRIES: Cell<usize> = Cell::new(0);
    }

    check_that_async_works().await;
    TRIES.with(|tries| {
        if tries.get() < 2 {
            tries.set(tries.get() + 1);
            panic!("not yet...");
        }
    });
}

#[flaky]
#[tokio::test]
async fn returns_ok_after_a_few_tries() -> Result<(), &'static str> {
    thread_local! {
        static TRIES: Cell<usize> = Cell::new(0);
    }

    check_that_async_works().await;
    TRIES.with(|tries| {
        if tries.get() < 2 {
            tries.set(tries.get() + 1);
            return Err("not yet...");
        }

        Ok(())
    })
}
