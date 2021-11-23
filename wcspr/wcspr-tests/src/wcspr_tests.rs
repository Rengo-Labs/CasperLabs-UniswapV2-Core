use casper_engine_test_support::AccountHash;
use casper_types::{Key, U256};
use test_env::{Sender, TestContract, TestEnv};

use crate::wcspr_instance::WCSPRInstance;

const NAME: &str = "Wrapped_Casper";
const SYMBOL: &str = "WCSPR";
const DECIMALS: u8 = 10;
pub const DEPOSIT_TEST_RESULT_KEY_NAME: &str = "deposit_test_result";
pub const WITHDRAW_TEST_RESULT_KEY_NAME: &str = "withdraw_test_result";
pub const TRANSFER_TEST_RESULT_KEY_NAME: &str = "transfer_test_result";
pub const TRANSFER_FROM_TEST_RESULT_KEY_NAME: &str = "transfer_from_test_result";
pub const PACKAGE_HASH_KEY_NAME: &str = "package_hash";
pub const CONTRACT_HASH_KEY_NAME: &str = "contract_hash";
pub const WCSPR_HASH_KEY_NAME: &str = "wcspr_hash";

fn deploy() -> (TestEnv, WCSPRInstance, WCSPRInstance, AccountHash) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let token: TestContract = WCSPRInstance::new(&env, NAME, Sender(owner), NAME, SYMBOL, DECIMALS);
    let proxy: TestContract =
        WCSPRInstance::proxy(&env, Key::Hash(token.contract_hash()), Sender(owner));
    // let proxy: TestContract = WCSPRInstance::new(&env, NAME, Sender(owner), NAME, SYMBOL, DECIMALS);

    (
        env,
        WCSPRInstance::instance(token),
        WCSPRInstance::instance(proxy),
        owner,
    )
}

#[test]
fn test_wcspr_deploy() {
    let (env, token, _proxy, owner) = deploy();
    let user = env.next_user();
    assert_eq!(token.name(), NAME);
    assert_eq!(token.symbol(), SYMBOL);
    assert_eq!(token.balance_of(user), 0.into());
    assert_eq!(token.allowance(owner, user), 0.into());
    assert_eq!(token.allowance(user, owner), 0.into());
}

#[test]
fn test_wcspr_transfer() {
    let (env, token, proxy, owner) = deploy();
    let package_hash = proxy.package_hash_result();
    let proxy_balance: U256 = token.balance_of(package_hash);
    let user = env.next_user();
    let amount: U256 = 0.into();

    proxy.transfer(Sender(owner), user, amount);
    assert_eq!(token.balance_of(user), amount);
    assert_eq!(token.balance_of(package_hash), proxy_balance - amount);
}

#[test]
fn test_wcspr_transfer_with_same_sender_and_recipient() {
    let (_env, _token, proxy, owner) = deploy();
    let package_hash = proxy.package_hash_result();
    let amount: U256 = 10.into();

    proxy.transfer(Sender(owner), package_hash, amount);

    let ret: Result<(), u32> = proxy.transfer_result();

    match ret {
        Ok(()) => println!("Passed"),
        Err(e) => println!("Failed {}", e),
    }
}

#[test]
#[should_panic]
fn test_wcspr_transfer_too_much() {
    let (env, token, proxy, owner) = deploy();
    let user = env.next_user();
    let amount = U256::one();
    token.transfer(Sender(owner), user, amount);

    let ret: Result<(), u32> = proxy.transfer_result();

    match ret {
        Ok(()) => println!("Passed"),
        Err(e) => println!("Failed {}", e),
    }
}

#[test]
fn test_wcspr_approve() {
    let (env, token, _proxy, owner) = deploy();
    let user = env.next_user();
    let amount = 10.into();
    token.approve(Sender(owner), user, amount);
    assert_eq!(token.balance_of(user), 0.into());
    assert_eq!(token.allowance(owner, user), amount);
    assert_eq!(token.allowance(user, owner), 0.into());
}

#[test]
fn test_wcspr_transfer_from() {
    let (env, token, proxy, owner) = deploy();
    let package_hash = proxy.package_hash_result();

    let owner_balance = token.balance_of(owner);

    let recipient = env.next_user();
    let allowance: U256 = 10.into();
    let amount: U256 = 0.into();

    token.approve(Sender(owner), package_hash, allowance);
    proxy.transfer_from(Sender(owner), owner.into(), recipient, amount);

    assert_eq!(token.balance_of(recipient), amount);
    // assert_eq!(token.allowance(owner, package_hash), allowance - amount.into());
    assert_eq!(token.balance_of(owner), owner_balance - amount);

    let ret: Result<(), u32> = proxy.transfer_from_result();

    match ret {
        Ok(()) => println!("Passed"),
        Err(e) => println!("Failed {}", e),
    }
}

#[test]
#[should_panic]
fn test_wcspr_transfer_from_too_much() {
    let (env, token, proxy, owner) = deploy();
    let _spender = env.next_user();
    let recipient = env.next_user();
    let allowance = 10.into();
    let amount = 12.into();
    let package_hash = proxy.package_hash_result();

    token.approve(Sender(owner), package_hash, allowance);
    proxy.transfer_from(Sender(owner), owner.into(), recipient, amount);

    let ret: Result<(), u32> = proxy.transfer_from_result();

    match ret {
        Ok(()) => println!("Passed"),
        Err(e) => println!("Failed {}", e),
    }
}

#[test]
#[should_panic]
fn test_calling_construction() {
    let (_, token, _, owner) = deploy();
    token.constructor(Sender(owner), NAME, SYMBOL);
}
