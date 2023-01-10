use tests_common::{account::AccountHash, runtime_args, *};

pub fn call(
    env: &TestEnv,
    sender: AccountHash,
    entrypoint: &str,
    package_hash: Key,
    amount: U512,
    time: u64,
) -> TestContract {
    TestContract::new(
        env,
        "session-code-wcspr.wasm",
        "session-code-wcspr",
        sender,
        runtime_args! {
            "entrypoint" => entrypoint,
            "package_hash" => package_hash,
            "amount" => amount,
        },
        time,
    )
}
