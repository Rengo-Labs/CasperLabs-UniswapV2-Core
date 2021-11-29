use casper_engine_test_support::AccountHash;
use casper_types::{U256, Key, U512, ContractHash};
use test_env::{Sender, TestEnv, TestContract};
use casper_contract::{contract_api::{system, account}};
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
    let proxy: TestContract = WCSPRInstance::proxy(&env, Key::Hash(token.contract_hash()), Sender(owner));
    // let proxy: TestContract = WCSPRInstance::new(&env, NAME, Sender(owner), NAME, SYMBOL, DECIMALS);

    (env,  WCSPRInstance::instance(token), WCSPRInstance::instance(proxy), owner)
}

#[test]
fn test_wcspr_deploy() {
    let (env, token, proxy, owner) = deploy();
    let user = env.next_user();
    assert_eq!(token.name(), NAME);
    assert_eq!(token.symbol(), SYMBOL);
    assert_eq!(token.balance_of(user), 0.into());
    assert_eq!(token.allowance(owner, user), 0.into());
    assert_eq!(token.allowance(user, owner), 0.into());
}

#[test]
fn test_wcspr_deposit(){
    let (env, token, proxy, owner) = deploy();
    let proxy_contract_hash = proxy.contract_hash_result();
    let proxy_package_hash = proxy.package_hash_result();
    let proxy_balance : U256= token.balance_of(proxy_contract_hash);
    let num = 200;
    let amount: U512 = num.into();
    //token.self_contract_hash_result()
    proxy.deposit(Sender(owner), amount, Key::from(proxy_contract_hash));
    let res: Result<(), u32>= proxy.deposit_result();

    assert_eq!(token.balance_of(proxy_package_hash), proxy_balance + U256::from(num));
    assert_eq!(res.is_ok(), true);
}

#[test]
fn test_wcspr_deposit_zero_amount(){
    let (env, token, proxy, owner) = deploy();
    let proxy_contract_hash = proxy.contract_hash_result();
    let proxy_package_hash = proxy.package_hash_result();
    let proxy_balance : U256= token.balance_of(proxy_contract_hash);
    let num = 0;
    let amount: U512 = num.into();
    //token.self_contract_hash_result()
    proxy.deposit(Sender(owner), amount, Key::from(proxy_contract_hash));
    let res: Result<(), u32>= proxy.deposit_result();

    assert_eq!(token.balance_of(proxy_package_hash), proxy_balance + U256::from(num));
    assert_eq!(res.is_err(), true);
}
#[test]
fn test_wcspr_withdraw(){
    let (env, token, proxy, owner) = deploy();
    let proxy_package_hash = proxy.package_hash_result();
    let proxy_contract_hash = proxy.contract_hash_result();
    let deposit_amount= 10;
    let withdraw_amount= 5;
    let proxy_balance : U256 = token.balance_of(proxy_package_hash);
    // let amount: U512 = deposit_amount.into();

    // first deposit some amount and verify
    proxy.deposit(Sender(owner), deposit_amount.into(), Key::from(proxy_contract_hash));
    let res: Result<(), u32>= proxy.deposit_result();
    assert_eq!(token.balance_of(proxy_package_hash), proxy_balance.checked_add(deposit_amount.into()).unwrap_or_default()); //+ U256::from(deposit_amount));
    assert_eq!(res.is_ok(), true);

    // withdraw some amount from deposited amount and verify
    proxy.withdraw(Sender(owner), Key::from(proxy_package_hash), U512::from(withdraw_amount), token.self_contract_hash_result());

    // assert_eq!(token.balance_of(proxy_package_hash), (proxy_balance.checked_add(U256::from(deposit_amount).checked_sub(withdraw_amount.into()).unwrap_or_default())).unwrap_or_default());
    // proxy.withdraw(Sender(owner), package_hash, amount);
    // let res: Result<(), u32>= proxy.withdraw_result();

    // assert_eq!(token.balance_of(package_hash), proxy_balance - U256::from(10));
}

#[test]
fn test_wcspr_transfer() {
    let (env, token, proxy, owner) = deploy();
    let package_hash = proxy.package_hash_result();
    let proxy_balance : U256= token.balance_of(package_hash);
    let user = env.next_user();
    let amount: U256 = 0.into();

    proxy.transfer(Sender(owner), user, amount);
    let ret: Result<(), u32> = proxy.transfer_result();

    assert_eq!(ret.is_err(), true);// sent amount is zero
    assert_eq!(token.balance_of(user), amount);
    assert_eq!(token.balance_of(package_hash), proxy_balance-amount);  
}

#[test]
fn test_wcspr_transfer_with_same_sender_and_recipient() {
    let (env, token, proxy, owner) = deploy();
    let package_hash = proxy.package_hash_result();
    let amount : U256= 10.into();


    proxy.transfer(Sender(owner), package_hash, amount);

    let ret: Result<(), u32> = proxy.transfer_result();
    // sent amount is zero
    assert_eq!(ret.is_err(), true);
}

#[test]
#[should_panic]
fn test_wcspr_transfer_too_much() {
    let (env, token, proxy, owner) = deploy();
    let user = env.next_user();
    let amount = U256::one();
    token.transfer(Sender(owner), user, amount);

    let ret: Result<(), u32> = proxy.transfer_result();
    assert_eq!(ret.is_err(), true);
}

#[test]
fn test_wcspr_approve() {
    let (env, token, proxy, owner) = deploy();
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
    let allowance :U256= 10.into();
    let amount:U256 = 0.into();

    token.approve(Sender(owner), package_hash, allowance);
    proxy.transfer_from(Sender(owner), owner.into(), recipient, amount);

    // assert_eq!(token.balance_of(recipient), amount);
    // assert_eq!(token.allowance(owner, package_hash), allowance - amount.into());
    // assert_eq!(token.balance_of(owner), owner_balance - amount);

    let ret: Result<(), u32> = proxy.transfer_from_result();
    assert_eq!(ret.is_err(), true);
}

#[test]
#[should_panic]
fn test_wcspr_transfer_from_too_much() {
    let (env, token, proxy, owner) = deploy();
    let spender = env.next_user();
    let recipient = env.next_user();
    let allowance = 10.into();
    let amount = 12.into();
    let package_hash= proxy.package_hash_result();

    token.approve(Sender(owner), package_hash, allowance);
    proxy.transfer_from(Sender(owner), owner.into(), recipient, amount);

    let ret: Result<(), u32> = proxy.transfer_from_result();
    assert_eq!(ret.is_err(), true);
}

#[test]
#[should_panic]
fn test_calling_construction() {
    let (_, token, _,owner) = deploy();
    token.constructor(Sender(owner), NAME, SYMBOL);
}
