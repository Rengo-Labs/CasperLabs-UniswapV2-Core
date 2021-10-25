use casper_engine_test_support::AccountHash;
use casper_types::U256;
use test_env::{Sender, TestEnv};

use crate::wcspr_instance::WCSPRInstance;

const NAME: &str = "Wrapped_Casper";
const SYMBOL: &str = "WCSPR";
const DECIMALS: u8 = 10;

fn deploy() -> (TestEnv, WCSPRInstance, AccountHash) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let token = WCSPRInstance::new(&env, NAME, Sender(owner), NAME, SYMBOL, DECIMALS);
    (env, token, owner)
}

#[test]
fn test_wcspr_deploy() {
    let (env, token, owner) = deploy();
    let user = env.next_user();
    assert_eq!(token.name(), NAME);
    assert_eq!(token.symbol(), SYMBOL);
    assert_eq!(token.balance_of(user), 0.into());
    assert_eq!(token.allowance(owner, user), 0.into());
    assert_eq!(token.allowance(user, owner), 0.into());
}

#[test]
fn test_wcspr_transfer() {
    let (env, token, owner) = deploy();
    let user = env.next_user();
    let amount = 0.into();
    token.transfer(Sender(owner), user, amount);
    assert_eq!(token.balance_of(user), amount);
}

#[test]
#[should_panic]
fn test_wcspr_transfer_too_much() {
    let (env, token, owner) = deploy();
    let user = env.next_user();
    let amount = U256::one();
    token.transfer(Sender(owner), user, amount);
}

#[test]
fn test_wcspr_approve() {
    let (env, token, owner) = deploy();
    let user = env.next_user();
    let amount = 10.into();
    token.approve(Sender(owner), user, amount);
    assert_eq!(token.balance_of(user), 0.into());
    assert_eq!(token.allowance(owner, user), amount);
    assert_eq!(token.allowance(user, owner), 0.into());
}

#[test]
fn test_wcspr_transfer_from() {
    let (env, token, owner) = deploy();
    let spender = env.next_user();
    let recipient = env.next_user();
    let allowance = 10.into();
    let amount = 0.into();
    token.approve(Sender(owner), spender, allowance);
    token.transfer_from(Sender(spender), owner, recipient, amount);
    assert_eq!(token.balance_of(spender), 0.into());
    assert_eq!(token.balance_of(recipient), amount);
    assert_eq!(token.allowance(owner, spender), allowance - amount);
}

#[test]
#[should_panic]
fn test_wcspr_transfer_from_too_much() {
    let (env, token, owner) = deploy();
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
    let (_, token, owner) = deploy();
    token.constructor(Sender(owner), NAME, SYMBOL);
}
