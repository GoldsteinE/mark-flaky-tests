pub fn run_test<'a>(
    module: &str,
    test: &str,
    env: impl IntoIterator<Item = (&'a str, &'a str)>,
) -> bool {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned());
    std::process::Command::new(cargo)
        .envs(env)
        .args(["test", "--test", module, "--", "--exact", test, "--ignored"])
        .output()
        .expect("failed to run `cargo test`")
        .status
        .success()
}

#[allow(unused_macros)] // used in other modules
macro_rules! gen_test_runner {
    ($(#[$should_fail:tt])? $name:ident$(, $($key:ident => $val:literal),*$(,)?)?) => {
        paste::paste! {
            #[test]
            fn [< run_ $name >]() {
                let res = crate::common::run_test(
                    module_path!(),
                    stringify!($name),
                    [$($(
                        (stringify!($key), $val),
                    )*)?]
                );
                assert_eq!(res, gen_test_runner!(@ $($should_fail)?));
            }
        }
    };
    (@ should_fail) => { false };
    (@) => { true };
}
