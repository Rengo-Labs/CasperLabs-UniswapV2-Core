use crate::wcspr_instance::*;
use casper_types::{account::AccountHash, Key, U256};
use casperlabs_erc20::Address;
use casperlabs_test_env::{now, TestContract, TestEnv};

const NAME: &str = "Wrapped_Casper";
const SYMBOL: &str = "WCSPR";
const DECIMALS: u8 = 10;
const INIT_TOTAL_SUPPLY: U256 = U256([0, 0, 0, 0]);

const DEPOSIT: &str = "deposit";
const WITHDRAW: &str = "withdraw";
const BALANCES: &str = "balances";

fn deploy() -> (TestEnv, AccountHash, TestContract) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let token = deploy_wcspr(&env, NAME, owner, NAME, SYMBOL, DECIMALS, now());
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
    let amount = 500;
    call(
        &env,
        owner,
        DEPOSIT,
        Key::Hash(token.package_hash()),
        amount.into(),
        now(),
    );
    assert_eq!(
        token.query::<U256>(BALANCES, address_to_str(&Address::Account(owner))),
        amount.into()
    );
}

#[test]
fn test_wcspr_withdraw() {
    let (env, owner, token) = deploy();
    assert_eq!(
        token.query::<U256>(BALANCES, address_to_str(&Address::Account(owner))),
        0.into()
    );
    let amount = 500;
    call(
        &env,
        owner,
        DEPOSIT,
        Key::Hash(token.package_hash()),
        amount.into(),
        now(),
    );
    assert_eq!(
        token.query::<U256>(BALANCES, address_to_str(&Address::Account(owner))),
        amount.into()
    );
    call(
        &env,
        owner,
        WITHDRAW,
        Key::Hash(token.package_hash()),
        amount.into(),
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
    let amount = 500;
    call(
        &env,
        owner,
        WITHDRAW,
        Key::Hash(token.package_hash()),
        amount.into(),
        now(),
    );
}
