use casper_engine_test_support::AccountHash;
use casper_types::{runtime_args, Key, RuntimeArgs, U256};
use test_env::{Sender, TestContract, TestEnv};

use crate::factory_instance::FACTORYInstance;

const NAME_FACTORY: &str = "Factory";

fn deploy() -> (TestEnv, FACTORYInstance, AccountHash, TestContract) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let _env_pair = TestEnv::new();
    let token = FACTORYInstance::new(&env, NAME_FACTORY, Sender(owner), owner);
    let wcspr = deploy_wcspr(&env);
    let dai = deploy_dai(&env);
    let name: &str = "ERC20";
    let symbol: &str = "ERC";
    let decimals: u8 = 8;
    let init_total_supply: U256 = 1000.into();
    let callee_contract = TestContract::new(
        //&env_factory,
        &env,
        "flash-swapper.wasm",
        "flash_swapper",
        Sender(owner),
        runtime_args! {
            "wcspr" => Key::Hash(wcspr.contract_hash()),
            "dai" => Key::Hash(dai.contract_hash()),
            "uniswap_v2_factory" => token.self_contract_hash()
        },
    );
    let pair_contract = TestContract::new(
        &env,
        "pair-token.wasm",
        "Pair",
        Sender(owner),
        runtime_args! {
        "name" => name,
        "symbol" => symbol,
        "decimals" => decimals,
        "initial_supply" => init_total_supply,
        "callee_contract_hash" => Key::Hash(callee_contract.contract_hash()),
        "factory_hash" => token.self_contract_hash(),
            // contract_name is passed seperately, so we don't need to pass it here.
        },
    );
    (env, token, owner, pair_contract)
}

fn deploy_token0(env: &TestEnv) -> TestContract {
    let _owner = env.next_user();
    let decimals: u8 = 18;
    let init_total_supply: U256 = 1000.into();
    let token0_env = TestEnv::new();
    let token0_owner = token0_env.next_user();
    let token0_contract = TestContract::new(
        &env,
        "erc20-token.wasm",
        "token0_contract",
        Sender(token0_owner),
        runtime_args! {
        "initial_supply" => init_total_supply,
        "name" => "token0",
        "symbol" => "tk0",
        "decimals" => decimals
        },
    );
    token0_contract
}

fn deploy_token1(env: &TestEnv) -> TestContract {
    let decimals: u8 = 18;
    let init_total_supply: U256 = 1000.into();
    let token1_env = TestEnv::new();
    let token1_owner = token1_env.next_user();
    let token1_contract = TestContract::new(
        &env,
        "erc20-token.wasm",
        "token1_contract",
        Sender(token1_owner),
        runtime_args! {
        "initial_supply" => init_total_supply,
        "name" => "token1",
        "symbol" => "tk1",
        "decimals" => decimals
        },
    );
    token1_contract
}

fn deploy_wcspr(env: &TestEnv) -> TestContract {
    let decimals: u8 = 18;
    let init_total_supply: U256 = 1000.into();
    let wcspr_env = TestEnv::new();
    let wcspr_owner = wcspr_env.next_user();
    let wcspr_contract = TestContract::new(
        &env,
        "wcspr-token.wasm",
        "wcspr_contract",
        Sender(wcspr_owner),
        runtime_args! {
        "initial_supply" => init_total_supply,
        "name" => "wcspr",
        "symbol" => "wcspr",
        "decimals" => decimals
        },
    );
    wcspr_contract
}
fn deploy_dai(env: &TestEnv) -> TestContract {
    let decimals: u8 = 18;
    let init_total_supply: U256 = 1000.into();
    let dai_env = TestEnv::new();
    let dai_owner = dai_env.next_user();
    let dai_contract = TestContract::new(
        &env,
        "wcspr-token.wasm",
        "dai_contract",
        Sender(dai_owner),
        runtime_args! {
        "initial_supply" => init_total_supply,
        "name" => "dai",
        "symbol" => "dai",
        "decimals" => decimals
        },
    );
    dai_contract
}

#[test]
fn test_factory_deploy() {
    let (_env, token, owner, _pair_hash) = deploy();
    assert_eq!(token.fee_to_setter(), Key::Account(owner));
}

#[test]
fn test_factory_set_fee_to_setter() {
    let (env, token, owner, _pair_hash) = deploy();
    let user = env.next_user();
    assert_eq!(token.fee_to_setter(), Key::Account(owner));
    token.set_fee_to_setter(Sender(owner), user);
    assert_eq!(token.fee_to_setter(), Key::Account(user));
}

#[test]
fn test_factory_set_fee_to() {
    let (env, token, owner, _pair_hash) = deploy();
    let user = env.next_user();
    assert_eq!(token.fee_to_setter(), Key::Account(owner));
    token.set_fee_to(Sender(owner), user);
    assert_eq!(token.fee_to_setter(), Key::Account(owner));
    assert_eq!(token.fee_to(), Key::Account(user));
}

#[test]
fn test_factory_create_pair() {
    let (env, token, owner, pair_hash) = deploy();
    assert_eq!(token.fee_to_setter(), Key::Account(owner));
    let token0 = deploy_token0(&env);
    let token1 = deploy_token1(&env);
    let token0 = Key::Hash(token0.contract_hash());
    let token1 = Key::Hash(token1.contract_hash());
    let pair_hash = Key::Hash(pair_hash.contract_hash());
    let user = env.next_user();
    token.set_white_list(Sender(owner), Key::Account(user));
    assert_eq!(
        token.get_white_lists(Key::Account(user)),
        Key::Account(user)
    );
    token.create_pair(Sender(user), token0, token1, pair_hash);
    let pair_0_1: Key = token.get_pair(token0, token1);
    let pair_1_0: Key = token.get_pair(token1, token0);
    let all_pairs: Vec<Key> = token.all_pairs();
    assert_eq!(pair_0_1, pair_1_0);
    assert_eq!(pair_0_1, pair_hash);
    assert_eq!(pair_1_0, pair_hash);
    assert_eq!(all_pairs.len(), 1);
}

#[test]
fn test_factory_set_white_list() {
    let (env, token, owner, _pair_hash) = deploy();
    assert_eq!(token.fee_to_setter(), Key::Account(owner));
    let user = env.next_user();
    token.set_white_list(Sender(owner), Key::Account(user));

    assert_eq!(
        token.get_white_lists(Key::Account(user)),
        Key::Account(user)
    );
    token.set_white_list(Sender(owner), Key::Account(owner));
    assert_eq!(
        token.get_white_lists(Key::Account(owner)),
        Key::Account(owner)
    );
}

#[test]
#[should_panic]
fn test_factory_set_white_list_with_non_owner() {
    let (env, token, owner, _pair_hash) = deploy();
    assert_eq!(token.fee_to_setter(), Key::Account(owner));
    let user = env.next_user();
    token.set_white_list(Sender(user), Key::Account(user));
}

#[test]
#[should_panic]
fn test_calling_construction() {
    let (env, token, owner, _pair_hash) = deploy();
    let user = env.next_user();
    token.constructor(Sender(owner), user);
}
