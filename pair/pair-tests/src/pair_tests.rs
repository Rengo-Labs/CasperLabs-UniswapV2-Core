use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs, U128, U256};
use casperlabs_erc20::Address;
use casperlabs_test_env::{now, TestContract, TestEnv};
use tests_common::{deploys::*, helpers::*};

fn deploy() -> (TestEnv, AccountHash, TestContract, TestContract) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let factory_contract = deploy_factory(&env, owner, Key::Account(owner), now());
    let wcspr = deploy_wcspr(
        &env,
        owner,
        "Wrapped Cspr".into(),
        "WCSPR".into(),
        9,
        0.into(),
        now(),
    );
    let dai = deploy_wcspr(
        &env,
        owner,
        "dai token".into(),
        "DAI".into(),
        9,
        0.into(),
        now(),
    );
    let callee_contract = deploy_flashswapper(
        &env,
        owner,
        Key::Hash(wcspr.package_hash()),
        Key::Hash(dai.package_hash()),
        Key::Hash(factory_contract.package_hash()),
        now(),
    );
    let token = deploy_pair(
        &env,
        owner,
        NAME,
        SYMBOL,
        DECIMALS,
        AMOUNT,
        Key::Hash(callee_contract.package_hash()),
        Key::Hash(factory_contract.package_hash()),
        now(),
    );
    (env, owner, token, factory_contract)
}

fn initialize(
    env: &TestEnv,
    owner: AccountHash,
    token: &TestContract,
    factory: &TestContract,
) -> (TestContract, TestContract) {
    let token0 = deploy_token0(
        env,
        owner,
        "Token0".into(),
        "TK-0".into(),
        9,
        0.into(),
        now(),
    );
    let token1 = deploy_token1(
        env,
        owner,
        "Token1".into(),
        "TK-1".into(),
        9,
        0.into(),
        now(),
    );
    token.call_contract(
        owner,
        "initialize",
        runtime_args! {
            "token0" => Key::Hash(token0.package_hash()),
            "token1" => Key::Hash(token1.package_hash()),
            "factory_hash" => Key::Hash(factory.package_hash())
        },
        now(),
    );
    assert_eq!(
        Key::Hash(token0.package_hash()),
        token.query_named_key("token0".into())
    );
    assert_eq!(
        Key::Hash(token1.package_hash()),
        token.query_named_key("token1".into())
    );
    assert_eq!(
        Key::Hash(factory.package_hash()),
        token.query_named_key("factory_hash".into())
    );
    token0.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Address::Contract(token.package_hash().into()),
            "amount" => AMOUNT
        },
        now(),
    );
    token1.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Address::Contract(token.package_hash().into()),
            "amount" => AMOUNT
        },
        now(),
    );
    (token0, token1)
}

#[test]
fn test_pair_deploy() {
    let (_, _, token, _) = deploy();
    assert_eq!(NAME, token.query_named_key::<String>("name".into()));
    assert_eq!(SYMBOL, token.query_named_key::<String>("symbol".into()));
    assert_eq!(DECIMALS, token.query_named_key::<u8>("decimals".into()));
    assert_eq!(AMOUNT, token.query_named_key::<U256>("total_supply".into()));
}

#[test]
fn test_pair_transfer() {
    let (env, owner, token, _) = deploy();
    let to = env.next_user();
    let ret: U256 = token.query(BALANCES, address_to_str(&Address::Account(owner)));
    assert_eq!(ret, AMOUNT);
    token.call_contract(
        owner,
        "transfer",
        runtime_args! {
            "recipient" => Address::Account(to),
            "amount" => AMOUNT,
        },
        now(),
    );
    let ret: U256 = token.query(BALANCES, address_to_str(&Address::Account(to)));
    assert_eq!(ret, AMOUNT);
    let ret: U256 = token.query(BALANCES, address_to_str(&Address::Account(owner)));
    assert_eq!(ret, 0.into());
}

#[test]
fn test_pair_initialize() {
    let (env, owner, token, factory) = deploy();
    initialize(&env, owner, &token, &factory);
}

#[test]
fn test_pair_set_treasury_fee_percent() {
    let (_, owner, token, _) = deploy();
    assert_eq!(token.query_named_key::<U256>(TREASURY_FEE.into()), 3.into());
    token.call_contract(
        owner,
        "set_treasury_fee_percent",
        runtime_args! {
            TREASURY_FEE => U256::from(10)
        },
        now(),
    );
    assert_eq!(
        token.query_named_key::<U256>(TREASURY_FEE.into()),
        10.into()
    );
}

#[test]
#[should_panic]
fn test_pair_set_treasury_fee_percent_cannot_be_more_than_30() {
    let (_, owner, token, _) = deploy();
    assert_eq!(token.query_named_key::<U256>(TREASURY_FEE.into()), 3.into());
    token.call_contract(
        owner,
        "set_treasury_fee_percent",
        runtime_args! {
            TREASURY_FEE => U256::from(31)
        },
        now(),
    );
    assert_eq!(
        token.query_named_key::<U256>(TREASURY_FEE.into()),
        10.into()
    );
}

#[test]
#[should_panic]
fn test_pair_set_treasury_fee_percent_cannot_be_less_than_3() {
    let (_, owner, token, _) = deploy();
    assert_eq!(token.query_named_key::<U256>(TREASURY_FEE.into()), 3.into());
    token.call_contract(
        owner,
        "set_treasury_fee_percent",
        runtime_args! {
            TREASURY_FEE => U256::from(2)
        },
        now(),
    );
    assert_eq!(
        token.query_named_key::<U256>(TREASURY_FEE.into()),
        10.into()
    );
}

#[test]
fn test_pair_skim() {
    let (env, owner, token, factory) = deploy();
    let ret = initialize(&env, owner, &token, &factory);
    let user = env.next_user();
    token.call_contract(
        owner,
        "skim",
        runtime_args! {
            "to" => Key::Account(user)
        },
        now(),
    );
    assert_eq!(
        ret.0
            .query::<U256>(BALANCES, address_to_str(&Address::Account(user))),
        100_000_000_000u64.into()
    );
    assert_eq!(
        ret.1
            .query::<U256>(BALANCES, address_to_str(&Address::Account(user))),
        100_000_000_000u64.into()
    );
}

#[test]
fn test_pair_mint() {
    let (env, owner, token, factory) = deploy();
    let ret = initialize(&env, owner, &token, &factory);
    token.call_contract(owner, "sync", runtime_args! {}, now());
    ret.0.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Address::Contract(token.package_hash().into()),
            "amount" => AMOUNT
        },
        now(),
    );
    ret.1.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Address::Contract(token.package_hash().into()),
            "amount" => AMOUNT
        },
        now(),
    );
    token.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Key::Hash(token.package_hash())
        },
        now(),
    );
    assert_eq!(
        token.query::<U256>(
            BALANCES,
            address_to_str(&Address::Contract(token.package_hash().into()))
        ),
        100000000000u64.into()
    );
}

#[test]
fn test_pair_burn() {
    let (env, owner, token, factory) = deploy();
    let ret = initialize(&env, owner, &token, &factory);
    token.call_contract(owner, "sync", runtime_args! {}, now());
    ret.0.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Address::Contract(token.package_hash().into()),
            "amount" => AMOUNT
        },
        now(),
    );
    ret.1.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Address::Contract(token.package_hash().into()),
            "amount" => AMOUNT
        },
        now(),
    );
    token.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Key::Hash(token.package_hash())
        },
        now(),
    );
    assert_eq!(
        token.query::<U256>(
            BALANCES,
            address_to_str(&Address::Contract(token.package_hash().into()))
        ),
        100000000000u64.into()
    );
    token.call_contract(
        owner,
        "burn",
        runtime_args! {
            "to" => Key::Hash(token.package_hash())
        },
        now(),
    );
    assert_eq!(
        token.query::<U256>(
            BALANCES,
            address_to_str(&Address::Contract(token.package_hash().into()))
        ),
        0.into()
    );
}

#[test]
fn test_pair_sync() {
    let (env, owner, token, factory) = deploy();
    initialize(&env, owner, &token, &factory);
    token.call_contract(owner, "sync", runtime_args! {}, now());
    assert_eq!(
        token.query_named_key::<U128>("reserve0".into()),
        100_000_000_000u64.into()
    );
    assert_eq!(
        token.query_named_key::<U128>("reserve1".into()),
        100_000_000_000u64.into()
    );
}

#[test]
fn test_pair_swap() {
    let (env, owner, token, factory) = deploy();
    let amount0_out: U256 = 50_000_000_000u64.into();
    let amount1_out: U256 = 20_000_000_000u64.into();
    let data: &str = "";
    let ret = initialize(&env, owner, &token, &factory);
    token.call_contract(owner, "sync", runtime_args! {}, now());
    ret.0.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Address::Contract(token.package_hash().into()),
            "amount" => U256::from(100_000_000_000u64)
        },
        now(),
    );
    ret.1.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Address::Contract(token.package_hash().into()),
            "amount" => U256::from(100_000_000_000u64)
        },
        now(),
    );
    token.call_contract(
        owner,
        "swap",
        runtime_args! {
            "amount0_out" => amount0_out,
            "amount1_out" => amount1_out,
            "to" => Key::Hash(token.package_hash()),
            "data" => data
        },
        now(),
    );
    assert_eq!(
        ret.0.query::<U256>(
            BALANCES,
            address_to_str(&Address::Contract(token.package_hash().into()))
        ),
        200_000_000_000u64.into()
    );
}
