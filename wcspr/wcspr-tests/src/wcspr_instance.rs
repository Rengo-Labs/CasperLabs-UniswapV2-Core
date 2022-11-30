use casper_types::{
    account::AccountHash, bytesrepr::ToBytes, runtime_args, Key, RuntimeArgs, U512,
};
use casperlabs_erc20::Address;
use casperlabs_test_env::{TestContract, TestEnv};

pub fn deploy_wcspr(
    env: &TestEnv,
    contract_name: &str,
    sender: AccountHash,
    name: &str,
    symbol: &str,
    decimals: u8,
    time: u64,
) -> TestContract {
    TestContract::new(
        env,
        "wcspr-token.wasm",
        contract_name,
        sender,
        runtime_args! {
            "name" => name,
            "symbol" => symbol,
            "decimals"=>decimals
        },
        time,
    )
}

pub fn address_to_str(owner: &Address) -> String {
    let preimage = owner.to_bytes().unwrap();
    base64::encode(&preimage)
}

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
