use crate::factory_instance::FACTORYInstance;
use tests_common::{
    account::AccountHash,
    deploys::{deploy_erc20, deploy_wcspr},
    helpers::*,
    *,
};

fn deploy() -> (TestEnv, FACTORYInstance, AccountHash, TestContract) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let _env_pair = TestEnv::new();
    let token = FACTORYInstance::new(&env, owner, Key::Account(owner), now());
    let wcspr = deploy_wcspr(
        &env,
        "WCSPR-1".into(),
        owner,
        WRAPPED_CSPR.into(),
        "WCSPR".into(),
        9,
        0.into(),
        now(),
    );
    let dai = deploy_erc20(&env, "Dai-1", owner, "Dai token", "DAI", 9, 0.into(), now());
    let callee_contract = TestContract::new(
        &env,
        "flashswapper-token.wasm",
        "flash_swapper",
        owner,
        runtime_args! {
            "wcspr" => Key::Hash(wcspr.package_hash()),
            "dai" => Key::Hash(dai.package_hash()),
            "uniswap_v2_factory" => Key::from(token.contract_package_hash())
        },
        now(),
    );
    let pair_contract = TestContract::new(
        &env,
        "pair-token.wasm",
        "Pair",
        owner,
        runtime_args! {
            "name" => NAME,
            "symbol" => SYMBOL,
            "decimals" => DECIMALS,
            "initial_supply" => INIT_TOTAL_SUPPLY,
            "callee_package_hash" => Key::Hash(callee_contract.package_hash()),
            "factory_hash" =>  Key::from(token.contract_package_hash())
        },
        now(),
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
    token.set_fee_to_setter(owner, user, now());
    assert_eq!(token.fee_to_setter(), Key::Account(user));
}

#[test]
fn test_factory_set_fee_to() {
    let (env, token, owner, _pair_hash) = deploy();
    let user = env.next_user();
    assert_eq!(token.fee_to_setter(), Key::Account(owner));
    token.set_fee_to(owner, user, now());
    assert_eq!(token.fee_to_setter(), Key::Account(owner));
    assert_eq!(token.fee_to(), Key::Account(user));
}

#[test]
fn test_factory_create_pair() {
    let (env, token, owner, pair_hash) = deploy();
    assert_eq!(token.fee_to_setter(), Key::Account(owner));
    let token0 = deploy_erc20(
        &env,
        "Token-1",
        owner,
        "Token 1",
        "TK-1",
        9,
        0.into(),
        now(),
    );
    let token1 = deploy_erc20(
        &env,
        "Token-2",
        owner,
        "Token 2",
        "TK-2",
        9,
        0.into(),
        now(),
    );
    let token0 = Key::Hash(token0.package_hash());
    let token1 = Key::Hash(token1.package_hash());
    let pair_hash = Key::Hash(pair_hash.package_hash());
    let user = env.next_user();
    token.set_white_list(owner, Key::Account(user), now());
    assert_eq!(
        token.get_white_lists(Key::Account(user)),
        Key::Account(user)
    );
    token.create_pair(user, token0, token1, pair_hash, now());
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
    token.set_white_list(owner, Key::Account(user), now());

    assert_eq!(
        token.get_white_lists(Key::Account(user)),
        Key::Account(user)
    );
    token.set_white_list(owner, Key::Account(owner), now());
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
    token.set_white_list(user, Key::Account(user), now());
}

#[test]
#[should_panic]
fn test_calling_construction() {
    let (env, token, owner, _pair_hash) = deploy();
    let user = env.next_user();
    token.constructor(owner, user, now());
}
