use casper_engine_test_support::AccountHash;
use casper_types::{Key, U256,runtime_args, RuntimeArgs,U128};
use test_env::{Sender, TestEnv, TestContract};

use crate::pair_instance::PAIRInstance;

const NAME: &str = "ERC20";
const SYMBOL: &str = "ERC";
const DECIMALS: u8 = 8;
const INIT_TOTAL_SUPPLY: u64 = 1000;

fn deploy() -> (TestEnv, PAIRInstance, AccountHash, TestContract) {

    let env = TestEnv::new();
    let owner = env.next_user();

    // deploy factory contract
    let _env_factory = TestEnv::new();
    // let owner_factory = env.next_user();
    let factory_contract = TestContract::new(
        //&env_factory,
        &env,
        "factory.wasm",
        "factory",
        Sender(owner),
        runtime_args! {
            "fee_to_setter" => Key::from(owner)
            // "contract_name" 
            // contract_name is passed seperately, so we don't need to pass it here.
        }
    );
    let token = PAIRInstance::new(
        &env,
        NAME,
        Sender(owner),
        NAME,
        SYMBOL,
        DECIMALS,
        INIT_TOTAL_SUPPLY.into(),
        Key::from_formatted_str("hash-0000000000000000000000000000000000000000000000000000000000000000").unwrap(),
        Key::Hash(factory_contract.contract_hash())
    );
    (env, token, owner, factory_contract)
}
fn deploy_token0(env: &TestEnv) -> TestContract {
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
fn test_pair_deploy() {
    let (env, token, owner,_factory_hash) = deploy();
    let user = env.next_user();
    assert_eq!(token.name(), NAME);
    assert_eq!(token.symbol(), SYMBOL);
    assert_eq!(token.decimals(), DECIMALS);
    assert_eq!(token.total_supply(), INIT_TOTAL_SUPPLY.into());
    assert_eq!(token.balance_of(owner), INIT_TOTAL_SUPPLY.into());
    assert_eq!(token.balance_of(user), 0.into());
    assert_eq!(token.allowance(owner, user), 0.into());
    assert_eq!(token.allowance(user, owner), 0.into());
}

#[test]
fn test_pair_transfer() {
    let (env, token, owner,_factory_hash) = deploy();
    let user = env.next_user();
    let amount = 10.into();
    token.transfer(Sender(owner), user, amount);
    assert_eq!(
        token.balance_of(owner),U256::from(INIT_TOTAL_SUPPLY) - amount
    );
    assert_eq!(token.balance_of(user), amount);
}

#[test]
#[should_panic]
fn test_pair_transfer_too_much() {
    let (env, token, owner,_factory_hash) = deploy();
    let user = env.next_user();
    let amount = U256::from(INIT_TOTAL_SUPPLY) + U256::one();
    token.transfer(Sender(owner), user, amount);
}

#[test]
fn test_pair_approve() {
    let (env, token, owner,_factory_hash) = deploy();
    let user = env.next_user();
    let amount = 10.into();
    token.approve(Sender(owner), user, amount);
    assert_eq!(token.balance_of(owner), INIT_TOTAL_SUPPLY.into());
    assert_eq!(token.balance_of(user), 0.into());
    assert_eq!(token.allowance(owner, user), amount);
    assert_eq!(token.allowance(user, owner), 0.into());
}
#[test]
fn test_pair_initialize() {
    let (env, token, owner, factory_hash) = deploy();
    let user = env.next_user();
    let token0 = deploy_token0(&env);
    let token1 = deploy_token1(&env);
    let token0 = Key::Hash(token0.contract_hash());
    let token1 = Key::Hash(token1.contract_hash());
    let factory_hash = Key::Hash(factory_hash.contract_hash());
    token.initialize(Sender(owner), token0, token1, factory_hash);
    println!("Key::from(A): {}", owner);
    println!("Key::from(B): {}", Key::from(owner).to_formatted_string());
    token.set_fee_to(Sender(owner), user);
    assert_eq!(token.factory_hash(), factory_hash);
    assert_eq!(token.token0(), token0);
    assert_eq!(token.token1(), token1);
}

#[test]
fn test_pair_set_treasury_fee_percent() {
    let (_env, token, owner,_factory_hash) = deploy();
    let treasury_fee: U256 = 10.into();
    token.set_treasury_fee_percent(Sender(owner), treasury_fee);
    assert_eq!(token.treasury_fee(), treasury_fee);
    // treasuary fee cannot be more than 30
    let treasury_fee: U256 = 31.into();
    token.set_treasury_fee_percent(Sender(owner), treasury_fee);
    assert_eq!(token.treasury_fee(), 30.into());
}

#[test]
fn test_pair_skim() {
    let (env, token, owner, factory_hash) = deploy();
    let user = env.next_user();
    let token0 = deploy_token0(&env);
    let token1 = deploy_token1(&env);
    let token0 = Key::Hash(token0.contract_hash());
    let token1 = Key::Hash(token1.contract_hash());
    let factory_hash = Key::Hash(factory_hash.contract_hash());
    let reserve0: U128 = 20.into();
    let reserve1: U128 = 10.into();
    let amount: U256 = 50.into();
    
    token.initialize(Sender(owner), token0, token1, factory_hash);
    assert_eq!(token.token0(), token0);
    assert_eq!(token.token1(), token1);
    assert_eq!(token.factory_hash(), factory_hash);
    token.set_reserve0(Sender(owner), reserve0);
    token.set_reserve1(Sender(owner), reserve1);
    token.mint_with_caller(Sender(owner), token0,token.self_contract_hash(), amount);
    token.mint_with_caller(Sender(owner), token1,token.self_contract_hash(), amount);
    token.simple_mint(Sender(owner),token0, amount);
    token.simple_mint(Sender(owner),token1, amount);
    token.skim(Sender(owner), user);
    let amount:U256=U256::from(INIT_TOTAL_SUPPLY) + amount + amount;
    assert_eq!(token.total_supply(), amount);
    assert_eq!(token.balance_of(owner), INIT_TOTAL_SUPPLY.into());
    assert_eq!(token.balance_of(user), 70.into());
    assert_eq!(token.balance_of(token0),20.into());
    assert_eq!(token.balance_of(token1),10.into());
    assert_eq!(token.reserve0(), reserve0);
    assert_eq!(token.reserve1(), reserve1);
}
#[test]
fn test_pair_sync() {
    let (env, token, owner, factory_hash) = deploy();
    let user = env.next_user();
    let token0 = deploy_token0(&env);
    let token1 = deploy_token1(&env);
    let token0 = Key::Hash(token0.contract_hash());
    let token1 = Key::Hash(token1.contract_hash());
    let factory_hash = Key::Hash(factory_hash.contract_hash());
    let reserve0: U128 = 20.into();
    let reserve1: U128 = 10.into();
    let amount: U256 = 50.into();
    token.initialize(Sender(owner), token0, token1, factory_hash);
    assert_eq!(token.factory_hash(), factory_hash);
    assert_eq!(token.token0(), token0);
    assert_eq!(token.token1(), token1);
    token.set_reserve0(Sender(owner), reserve0);
    token.set_reserve1(Sender(owner), reserve1);
    token.mint_with_caller(Sender(owner), token0,token.self_contract_hash(), amount);
    token.mint_with_caller(Sender(owner), token1,token.self_contract_hash(), amount);
    token.sync(Sender(owner));
    assert_eq!(token.total_supply(), INIT_TOTAL_SUPPLY.into());
    assert_eq!(token.balance_of(owner), INIT_TOTAL_SUPPLY.into());
    assert_eq!(token.balance_of(user), 0.into());
    assert_eq!(token.reserve0(), 50.into());
    assert_eq!(token.reserve1(), 50.into());

}

#[test]
fn test_pair_transfer_from() {
    let (env, token, owner,_factory_hash) = deploy();
    let spender = env.next_user();
    let recipient = env.next_user();
    let allowance = 10.into();
    let amount = 3.into();
    token.approve(Sender(owner), spender, allowance);
    token.transfer_from(Sender(spender), owner, recipient, amount);
    assert_eq!(
        token.balance_of(owner),
        U256::from(INIT_TOTAL_SUPPLY) - amount
    );
    assert_eq!(token.balance_of(spender), 0.into());
    assert_eq!(token.balance_of(recipient), amount);
    assert_eq!(token.allowance(owner, spender), allowance - amount);
}

#[test]
#[should_panic]
fn test_pair_transfer_from_too_much() {
    let (env, token, owner, _factory_hash) = deploy();
    let spender = env.next_user();
    let recipient = env.next_user();
    let allowance = 10.into();
    let amount = 12.into();
    token.approve(Sender(owner), spender, allowance);
    token.transfer_from(Sender(spender), owner, recipient, amount);
}

#[test]
#[should_panic]
fn test_calling_construction() {
    let (_, token, owner, factory_hash) = deploy();
    token.constructor(
        Sender(owner),
        NAME,
        SYMBOL,
        DECIMALS,
        INIT_TOTAL_SUPPLY.into(),
        Key::from_formatted_str("hash-0000000000000000000000000000000000000000000000000000000000000000").unwrap(),
        Key::Hash(factory_hash.contract_hash())
        
    );
}
