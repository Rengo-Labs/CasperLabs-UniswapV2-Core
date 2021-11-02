use casper_engine_test_support::AccountHash;
use casper_types::{runtime_args, Key, RuntimeArgs, U256};
use test_env::{Sender, TestContract, TestEnv};

use crate::pair_instance::PAIRInstance;
use crate::test_instance::TESTInstance;

const NAME: &str = "ERC20";
const SYMBOL: &str = "ERC";
const DECIMALS: u8 = 8;
const INIT_TOTAL_SUPPLY: u64 = 1000;

fn deploy_wcspr(env: &TestEnv) -> TestContract {
    // deploy wcspr contract
    let decimals: u8 = 18;
    let init_total_supply: U256 = 1000.into();

    let owner_wcspr = env.next_user();
    let wcspr = TestContract::new(
        &env,
        "wcspr-token.wasm",
        "wcspr",
        Sender(owner_wcspr),
        runtime_args! {
            "initial_supply" => init_total_supply,
            "name" => "token1",
            "symbol" => "tk1",
            "decimals" => decimals
        },
    );

    wcspr
}

fn deploy_factory(env: &TestEnv, owner: AccountHash) -> TestContract {
    // deploy factory contract
    let owner_factory = env.next_user();
    let factory = TestContract::new(
        &env,
        "factory.wasm",
        "factory",
        Sender(owner_factory),
        runtime_args! {
            "fee_to_setter" => Key::from(owner)
            // contract_name is passed seperately, so we don't need to pass it here.
        },
    );

    factory
}

fn deploy() -> (
    TestEnv,
    PAIRInstance,
    AccountHash,
    TestContract,
    TESTInstance,
) {
    let env = TestEnv::new();
    let owner = env.next_user();

    // deploy factory contract
    let _env_factory = TestEnv::new();
    // let owner_factory = env.next_user();

    let factory_contract = deploy_factory(&env, owner);
    let wcspr = deploy_wcspr(&env);
    let dai = deploy_wcspr(&env);
    let callee_contract = TestContract::new(
        //&env_factory,
        &env,
        "flash-swapper.wasm",
        "flash_swapper",
        Sender(owner),
        runtime_args! {
            "wcspr" => Key::Hash(wcspr.contract_hash()),
            "dai" => Key::Hash(dai.contract_hash()),
            "uniswap_v2_factory" => Key::Hash(factory_contract.contract_hash())
        },
    );
    let token = PAIRInstance::new(
        &env,
        NAME,
        Sender(owner),
        NAME,
        SYMBOL,
        DECIMALS,
        INIT_TOTAL_SUPPLY.into(),
        Key::Hash(callee_contract.contract_hash()),
        Key::Hash(factory_contract.contract_hash()),
    );
    let test = TESTInstance::new(&env, "TEST", Sender(owner), "TEST");
    (env, token, owner, factory_contract, test)
}
fn deploy_token0(env: &TestEnv) -> TestContract {
    let decimals: u8 = 18;
    let init_total_supply: U256 = 0.into();

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
    let init_total_supply: U256 = 0.into();

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

#[test]
fn test_pair_deploy() {
    let (env, token, owner, _factory_hash, _test) = deploy();
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
    let (env, token, owner, _factory_hash, _test) = deploy();
    let user = env.next_user();
    let amount = 10.into();
    token.transfer(Sender(owner), user, amount);
    assert_eq!(
        token.balance_of(owner),
        U256::from(INIT_TOTAL_SUPPLY) - amount
    );
    assert_eq!(token.balance_of(user), amount);
}

#[test]
fn test_pair_transfer_with_same_sender_and_recipient() {
    let (_env, token, owner, _factory_hash, _test) = deploy();
    let amount = 10.into();
    token.transfer(Sender(owner), owner, amount);
    assert_eq!(token.balance_of(owner), U256::from(INIT_TOTAL_SUPPLY));
}

#[test]
#[should_panic]
fn test_pair_transfer_too_much() {
    let (env, token, owner, _factory_hash, _test) = deploy();
    let user = env.next_user();
    let amount = U256::from(INIT_TOTAL_SUPPLY) + U256::one();
    token.transfer(Sender(owner), user, amount);
}

#[test]
fn test_pair_approve() {
    let (env, token, owner, _factory_hash, _test) = deploy();
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
    let (env, token, owner, factory_hash, test) = deploy();
    let user = env.next_user();
    let token0 = deploy_token0(&env);
    let token1 = deploy_token1(&env);
    let token0 = Key::Hash(token0.contract_hash());
    let token1 = Key::Hash(token1.contract_hash());
    let factory_hash = Key::Hash(factory_hash.contract_hash());
    token.initialize(Sender(owner), token0, token1, factory_hash);
    test.set_fee_to(Sender(owner), user, factory_hash);
    assert_eq!(token.factory_hash(), factory_hash);
    assert_eq!(token.token0(), token0);
    assert_eq!(token.token1(), token1);
}

#[test]
fn test_pair_set_treasury_fee_percent() {
    let (_env, token, owner, _factory_hash, _test) = deploy();
    assert_eq!(token.treasury_fee(), 3.into());
    let treasury_fee: U256 = 10.into();
    token.set_treasury_fee_percent(Sender(owner), treasury_fee);
    assert_eq!(token.treasury_fee(), treasury_fee);
    // treasuary fee cannot be more than 30
    let treasury_fee: U256 = 31.into();
    token.set_treasury_fee_percent(Sender(owner), treasury_fee);
    assert_eq!(token.treasury_fee(), 30.into());
    // treasuary fee cannot be less than 3
    let treasury_fee: U256 = 1.into();
    token.set_treasury_fee_percent(Sender(owner), treasury_fee);
    assert_eq!(token.treasury_fee(), 3.into());
}

#[test]
fn test_pair_skim() {
    let (env, token, owner, factory_hash, test) = deploy();
    let user = env.next_user();
    let token0 = deploy_token0(&env);
    let token1 = deploy_token1(&env);
    let token0 = Key::Hash(token0.contract_hash());
    let token1 = Key::Hash(token1.contract_hash());
    let factory_hash = Key::Hash(factory_hash.contract_hash());
    let amount0: U256 = 1000.into();
    let amount1: U256 = 1000.into();

    token.initialize(Sender(owner), token0, token1, factory_hash);
    assert_eq!(token.token0(), token0);
    assert_eq!(token.token1(), token1);
    assert_eq!(token.factory_hash(), factory_hash);

    test.mint_with_caller(
        Sender(owner),
        token0,
        Key::from(token.self_package_hash()),
        amount0,
    );
    test.mint_with_caller(
        Sender(owner),
        token1,
        Key::from(token.self_package_hash()),
        amount1,
    );
    token.skim(Sender(owner), user);
}
#[test]
fn test_pair_sync() {
    let (env, token, owner, factory_hash, test) = deploy();
    let user = env.next_user();
    let token0 = deploy_token0(&env);
    let token1 = deploy_token1(&env);
    let token0 = Key::Hash(token0.contract_hash());
    let token1 = Key::Hash(token1.contract_hash());
    let factory_hash = Key::Hash(factory_hash.contract_hash());
    let amount: U256 = 50.into();
    token.initialize(Sender(owner), token0, token1, factory_hash);
    assert_eq!(token.factory_hash(), factory_hash);
    assert_eq!(token.token0(), token0);
    assert_eq!(token.token1(), token1);
    test.mint_with_caller(
        Sender(owner),
        token0,
        Key::from(token.self_package_hash()),
        amount,
    );
    test.mint_with_caller(
        Sender(owner),
        token1,
        Key::from(token.self_package_hash()),
        amount,
    );
    token.sync(Sender(owner));
    assert_eq!(token.total_supply(), INIT_TOTAL_SUPPLY.into());
    assert_eq!(token.balance_of(owner), INIT_TOTAL_SUPPLY.into());
    assert_eq!(token.balance_of(user), 0.into());
    assert_eq!(token.reserve0(), 50.into());
    assert_eq!(token.reserve1(), 50.into());
}
#[test]
fn test_pair_swap() {
    let (env, token, owner, factory_hash, test) = deploy();
    let user = env.next_user();
    let token0 = deploy_token0(&env);
    let token1 = deploy_token1(&env);
    let token0 = Key::Hash(token0.contract_hash());
    let token1 = Key::Hash(token1.contract_hash());
    let factory_hash = Key::Hash(factory_hash.contract_hash());
    let amount0: U256 = 2000.into();
    let amount1: U256 = 2000.into();
    let amount: U256 = 1000.into();
    let amount2: U256 = 1000.into();
    let amount3: U256 = 40.into();
    let data: &str = "";

    token.initialize(Sender(owner), token0, token1, factory_hash);
    assert_eq!(token.token0(), token0);
    assert_eq!(token.token1(), token1);
    assert_eq!(token.factory_hash(), factory_hash);

    test.mint_with_caller(
        Sender(owner),
        token0,
        Key::from(token.self_package_hash()),
        amount0,
    );
    test.mint_with_caller(
        Sender(owner),
        token1,
        Key::from(token.self_package_hash()),
        amount1,
    );

    token.sync(Sender(owner));
    assert_eq!(token.reserve0(), 2000.into());
    assert_eq!(token.reserve1(), 2000.into());
    test.mint_with_caller(
        Sender(owner),
        token0,
        Key::from(token.self_package_hash()),
        amount,
    );
    test.mint_with_caller(
        Sender(owner),
        token1,
        Key::from(token.self_package_hash()),
        amount,
    );
    // token.erc20_mint(Sender(owner), token0, amount);
    // token.erc20_mint(Sender(owner), token1, amount);
    // assert_eq!(token.balance_of(token0), 100.into());
    // assert_eq!(token.balance_of(token1), 100.into());
    token.swap(Sender(owner), amount2, amount3, user, data);
}

#[test]
fn test_pair_transfer_from() {
    let (env, token, owner, _factory_hash, _test) = deploy();
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
    let (env, token, owner, _factory_hash, _test) = deploy();
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
    let (_, token, owner, factory_hash, _test) = deploy();
    token.constructor(
        Sender(owner),
        NAME,
        SYMBOL,
        DECIMALS,
        INIT_TOTAL_SUPPLY.into(),
        Key::from_formatted_str(
            "hash-0000000000000000000000000000000000000000000000000000000000000000",
        )
        .unwrap(),
        Key::Hash(factory_hash.contract_hash()),
    );
}
