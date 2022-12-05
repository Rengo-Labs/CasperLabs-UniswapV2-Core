use crate::wcspr_instance::*;
use tests_common::{account::AccountHash, deploys::*, helpers::*, keys::*, *};

fn deploy() -> (TestEnv, AccountHash, TestContract) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let token = deploy_wcspr(
        &env,
        WRAPPED_CSPR.into(),
        owner,
        NAME.into(),
        SYMBOL.into(),
        DECIMALS,
        INIT_TOTAL_SUPPLY,
        now(),
    );
    (env, owner, token)
}

#[test]
fn test_wcspr_deploy() {
    let (_, _, token) = deploy();
    assert_eq!(NAME, token.query_named_key::<String>("name".into()));
    assert_eq!(SYMBOL, token.query_named_key::<String>("symbol".into()));
    assert_eq!(DECIMALS, token.query_named_key::<u8>("decimals".into()));
    assert_eq!(
        INIT_TOTAL_SUPPLY,
        token.query_named_key::<U256>("total_supply".into())
    );
}

#[test]
fn test_wcspr_deposit() {
    let (env, owner, token) = deploy();
    assert_eq!(
        token.query::<U256>(BALANCES, address_to_str(&Address::Account(owner))),
        0.into()
    );
    call(
        &env,
        owner,
        DEPOSIT,
        Key::Hash(token.package_hash()),
        AMOUNT_U512,
        now(),
    );
    assert_eq!(
        token.query::<U256>(BALANCES, address_to_str(&Address::Account(owner))),
        AMOUNT.into()
    );
}

#[test]
fn test_wcspr_withdraw() {
    let (env, owner, token) = deploy();
    assert_eq!(
        token.query::<U256>(BALANCES, address_to_str(&Address::Account(owner))),
        0.into()
    );
    call(
        &env,
        owner,
        DEPOSIT,
        Key::Hash(token.package_hash()),
        AMOUNT_U512,
        now(),
    );
    assert_eq!(
        token.query::<U256>(BALANCES, address_to_str(&Address::Account(owner))),
        AMOUNT.into()
    );
    call(
        &env,
        owner,
        WITHDRAW,
        Key::Hash(token.package_hash()),
        AMOUNT_U512,
        now(),
    );
    assert_eq!(
        token.query::<U256>(BALANCES, address_to_str(&Address::Account(owner))),
        0.into()
    );
}

#[test]
#[should_panic]
fn test_wcspr_withdraw_with_no_deposit() {
    let (env, owner, token) = deploy();
    assert_eq!(
        token.query::<U256>(BALANCES, address_to_str(&Address::Account(owner))),
        0.into()
    );
    call(
        &env,
        owner,
        WITHDRAW,
        Key::Hash(token.package_hash()),
        AMOUNT_U512,
        now(),
    );
}
