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
    let name: &str = "ERC20";
    let symbol: &str = "ERC";
    let decimals: u8 = 8;
    let init_total_supply: U256 = 1000.into();
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
        "callee_contract_hash" => Key::from_formatted_str("hash-0000000000000000000000000000000000000000000000000000000000000000").unwrap(),
        "factory_hash" => token.self_contract_hash(),
            // contract_name is passed seperately, so we don't need to pass it here.
        },
    );
    (env, token, owner, pair_contract)
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
#[should_panic]
fn test_calling_construction() {
    let (env, token, owner, _pair_hash) = deploy();
    let user = env.next_user();
    token.constructor(Sender(owner), user);
}
