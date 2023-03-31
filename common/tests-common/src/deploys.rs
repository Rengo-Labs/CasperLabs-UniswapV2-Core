use casperlabs_test_env::{TestContract, TestEnv};
use common::{account::AccountHash, *};

#[allow(clippy::too_many_arguments)]
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

#[allow(clippy::too_many_arguments)]
pub fn deploy_wcspr(
    env: &TestEnv,
    contract_name: &str,
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
        contract_name,
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
    contract_name: &str,
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
        contract_name,
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
#[allow(clippy::too_many_arguments)]
pub fn deploy_erc20_secure(
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
        "erc20-secure.wasm",
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
