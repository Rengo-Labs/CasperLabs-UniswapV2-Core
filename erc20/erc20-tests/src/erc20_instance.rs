use casper_types::{account::AccountHash, bytesrepr::ToBytes, runtime_args, RuntimeArgs, U256};
use casperlabs_erc20::Address;
use casperlabs_test_env::{TestContract, TestEnv};

pub fn address_to_str(owner: &Address) -> String {
    let preimage = owner.to_bytes().unwrap();
    base64::encode(&preimage)
}

pub fn deploy_erc20(
    env: &TestEnv,
    contract_name: &str,
    sender: AccountHash,
    name: &str,
    symbol: &str,
    decimals: u8,
    supply: U256,
    time: u64,
) -> TestContract {
    TestContract::new(
        env,
        "erc20-token.wasm",
        contract_name,
        sender,
        runtime_args! {
            "initial_supply" => supply,
            "name" => name,
            "symbol" => symbol,
            "decimals" => decimals
        },
        time,
    )
}
