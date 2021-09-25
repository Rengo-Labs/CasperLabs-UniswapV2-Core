use casper_engine_test_support::AccountHash;
use casper_types::{ runtime_args, Key, RuntimeArgs,U256};
use test_env::{Sender, TestEnv, TestContract};

use crate::factory_instance::FACTORYInstance;

const NAME_FACTORY: &str = "Factory";

fn deploy() -> (TestEnv, FACTORYInstance, AccountHash, TestContract) {
    let env = TestEnv::new();
    let owner = env.next_user();

    let _env_pair = TestEnv::new();
    // let owner_factory = env.next_user();
    

    let token = FACTORYInstance::new(
        &env,
        NAME_FACTORY,
        Sender(owner),
        owner,
    );
    let name: &str = "ERC20";
    let symbol: &str = "ERC";
    let decimals: u8 = 8;
    let init_total_supply: U256 = 1000.into();
        
    let pair_contract = TestContract::new(
        //&env_factory,
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
        }
    );
    // println!("PAIR CONTRACT {:?}",pair_contract);
    (env, token, owner,pair_contract)
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
        }
    );
    println!("token0_contract: {}", Key::Hash(token0_contract.contract_hash()).to_formatted_string());
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
        }
    );
    println!("token1_contract: {}", Key::Hash(token1_contract.contract_hash()).to_formatted_string());
    token1_contract
}


#[test]
fn test_factory_deploy() {
    let (_env, token, owner,_pair_hash) = deploy();
    assert_eq!(token.fee_to_setter(), Key::Account(owner));

}
#[test]
fn test_factory_set_fee_to_setter() {
    let (env, token, owner,_pair_hash) = deploy();
    let user = env.next_user();
    assert_eq!(token.fee_to_setter(), Key::Account(owner));
    token.set_fee_to_setter(Sender(owner), user,);
    assert_eq!(token.fee_to_setter(), Key::Account(user));
}
#[test]
fn test_factory_set_fee_to() {
    let (env, token, owner,_pair_hash) = deploy();
    let user = env.next_user();
    assert_eq!(token.fee_to_setter(), Key::Account(owner));
    token.set_fee_to(Sender(owner), user,);
    assert_eq!(token.fee_to_setter(), Key::Account(owner));
    assert_eq!(token.fee_to(), Key::Account(user));
}
#[test]
fn test_factory_create_pair() {
    let (env, token, owner,pair_hash) = deploy();
    assert_eq!(token.fee_to_setter(), Key::Account(owner));
    let token0 = deploy_token0(&env);
    let token1 = deploy_token1(&env);
    let token0 = Key::Hash(token0.contract_hash());
    let token1 = Key::Hash(token1.contract_hash());
    let pair_hash = Key::Hash(pair_hash.contract_hash());
    token.create_pair(Sender(owner), token0, token1, pair_hash);
    let pair_0_1: Key=  token.get_pair(token0, token1);
    let pair_1_0: Key=  token.get_pair(token1, token0);
    let all_pairs: Vec<Key>=token.all_pairs();
    assert_eq!(pair_0_1, pair_1_0);
    assert_eq!(pair_0_1, pair_hash);
    assert_eq!(pair_1_0, pair_hash);
    assert_eq!(all_pairs.len(), 1);
}

#[test]
#[should_panic]
fn test_calling_construction() {
    let (env, token, owner,_pair_hash) = deploy();
    let user = env.next_user();
    token.constructor(
        Sender(owner),
        user,
    );
}
