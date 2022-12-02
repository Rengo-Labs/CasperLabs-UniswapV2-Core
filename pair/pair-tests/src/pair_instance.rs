use casper_types::{
    account::AccountHash, bytesrepr::ToBytes, runtime_args, Key, RuntimeArgs, U256,
};
use casperlabs_erc20::Address;
use casperlabs_test_env::{TestContract, TestEnv};

pub fn address_to_str(owner: &Address) -> String {
    let preimage = owner.to_bytes().unwrap();
    base64::encode(&preimage)
}

pub fn deploy_token0(
    env: &TestEnv,
    owner: AccountHash,
    name: String,
    symbol: String,
    decimals: u8,
    initial_supply: U256,
    time: u64,
) -> TestContract {
    TestContract::new(
        env,
        "erc20-token.wasm",
        "token0_contract",
        owner,
        runtime_args! {
            "name" => name,
            "symbol" => symbol,
            "decimals" => decimals,
            "initial_supply" => initial_supply
        },
        time,
    )
}

pub fn deploy_token1(
    env: &TestEnv,
    owner: AccountHash,
    name: String,
    symbol: String,
    decimals: u8,
    initial_supply: U256,
    time: u64,
) -> TestContract {
    TestContract::new(
        env,
        "erc20-token.wasm",
        "token1_contract",
        owner,
        runtime_args! {
            "name" => name,
            "symbol" => symbol,
            "decimals" => decimals,
            "initial_supply" => initial_supply
        },
        time,
    )
}

pub fn deploy_factory(
    env: &TestEnv,
    owner: AccountHash,
    fee_to_setter: Key,
    time: u64,
) -> TestContract {
    TestContract::new(
        env,
        "factory.wasm",
        "factory",
        owner,
        runtime_args! {
            "fee_to_setter" => fee_to_setter
        },
        time,
    )
}

pub fn deploy_wcspr(
    env: &TestEnv,
    owner: AccountHash,
    name: String,
    symbol: String,
    decimals: u8,
    initial_supply: U256,
    time: u64,
) -> TestContract {
    TestContract::new(
        env,
        "wcspr-token.wasm",
        "wcspr",
        owner,
        runtime_args! {
            "name" => name,
            "symbol" => symbol,
            "decimals" => decimals,
            "initial_supply" => initial_supply
        },
        time,
    )
}

pub fn deploy_flashswapper(
    env: &TestEnv,
    owner: AccountHash,
    wcspr: Key,
    dai: Key,
    factory: Key,
    time: u64,
) -> TestContract {
    TestContract::new(
        env,
        "flashswapper-token.wasm",
        "flash_swapper",
        owner,
        runtime_args! {
            "wcspr" => wcspr,
            "dai" => dai,
            "uniswap_v2_factory" => factory
        },
        time,
    )
}

#[allow(clippy::too_many_arguments)]
pub fn deploy_pair(
    env: &TestEnv,
    owner: AccountHash,
    name: &str,
    symbol: &str,
    decimals: u8,
    supply: U256,
    callee_package_hash: Key,
    factory_hash: Key,
    time: u64,
) -> TestContract {
    TestContract::new(
        env,
        "pair-token.wasm",
        "pair",
        owner,
        runtime_args! {
            "name" => name,
            "symbol" => symbol,
            "decimals" => decimals,
            "initial_supply" => supply,
            "callee_package_hash" => callee_package_hash,
            "factory_hash" => factory_hash
        },
        time,
    )
}
