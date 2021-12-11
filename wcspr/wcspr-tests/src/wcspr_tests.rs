use crate::wcspr_instance::WCSPRInstance;
use casper_engine_test_support::AccountHash;
use casper_types::{Key, U256, U512};
use test_env::{Sender, TestContract, TestEnv};

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

fn deploy() -> (
    TestEnv,
    WCSPRInstance,
    WCSPRInstance,
    WCSPRInstance,
    AccountHash,
) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let token: TestContract = WCSPRInstance::new(&env, NAME, Sender(owner), NAME, SYMBOL, DECIMALS);
    let proxy: TestContract =
        WCSPRInstance::proxy(&env, Key::Hash(token.contract_hash()), Sender(owner));
    // let proxy: TestContract = WCSPRInstance::new(&env, NAME, Sender(owner), NAME, SYMBOL, DECIMALS);
    let proxy2: TestContract =
        WCSPRInstance::proxy2(&env, Key::Hash(token.contract_hash()), Sender(owner));

    (
        env,
        WCSPRInstance::instance(token),
        WCSPRInstance::instance(proxy),
        WCSPRInstance::instance(proxy2),
        owner,
    )
}

#[test]
fn test_wcspr_deploy() {
    let (env, token, _proxy, _, owner) = deploy();
    let user = env.next_user();
    assert_eq!(token.name(), NAME);
    assert_eq!(token.symbol(), SYMBOL);
    assert_eq!(token.balance_of(user), 0.into());
    assert_eq!(token.allowance(owner, user), 0.into());
    assert_eq!(token.allowance(user, owner), 0.into());
}

#[test]
fn test_wcspr_deposit() {
    let (_env, token, proxy, _, owner) = deploy();
    let proxy_contract_hash = proxy.contract_hash_result();
    let _proxy_package_hash = proxy.package_hash_result();
    let package_hash = proxy.package_hash_result();
    let proxy_balance: U256 = token.balance_of(owner);
    let num = 200;
    let amount: U512 = num.into();
    //token.self_contract_hash_result()
    proxy.deposit(Sender(owner), amount, Key::from(proxy_contract_hash));
    let res: Result<(), u32> = proxy.deposit_result();

    assert_eq!(
        token.balance_of(package_hash),
        proxy_balance + U256::from(num)
    );
    assert_eq!(res.is_ok(), true);
}

#[test]
fn test_wcspr_deposit_zero_amount() {
    let (_env, token, proxy, _, owner) = deploy();
    let proxy_contract_hash = proxy.contract_hash_result();
    let proxy_package_hash = proxy.package_hash_result();
    let proxy_balance: U256 = token.balance_of(proxy_contract_hash);
    let num = 0;
    let amount: U512 = num.into();
    //token.self_contract_hash_result()
    proxy.deposit(Sender(owner), amount, Key::from(proxy_contract_hash));
    let res: Result<(), u32> = proxy.deposit_result();

    assert_eq!(
        token.balance_of(proxy_package_hash),
        proxy_balance + U256::from(num)
    );
    assert_eq!(res.is_err(), true);
}

#[test]
fn test_wcspr_withdraw() {
    let (_env, token, proxy, _, owner) = deploy();
    let proxy_package_hash = proxy.package_hash_result();
    let proxy_contract_hash = proxy.contract_hash_result();
    let deposit_amount = 10;
    let withdraw_amount = 5;
    let proxy_balance: U256 = token.balance_of(proxy_package_hash);
    // let amount: U512 = deposit_amount.into();

    // first deposit some amount and verify
    proxy.deposit(
        Sender(owner),
        deposit_amount.into(),
        Key::from(proxy_contract_hash),
    );
    let res: Result<(), u32> = proxy.deposit_result();
    assert_eq!(
        token.balance_of(proxy_package_hash),
        proxy_balance
            .checked_add(deposit_amount.into())
            .unwrap_or_default()
    ); //+ U256::from(deposit_amount));
    assert_eq!(res.is_ok(), true);

    // withdraw some amount from deposited amount and verify
    proxy.withdraw(Sender(owner), U512::from(withdraw_amount));
    let res: Result<(), u32> = proxy.withdraw_result();
    assert_eq!(res.is_ok(), true);
    let new_proxy_balance: U256 = U256::from(deposit_amount - withdraw_amount);
    assert_eq!(token.balance_of(proxy_package_hash), new_proxy_balance);
}

#[test]
fn test_wcspr_transfer() {
    let (env, token, proxy, _, owner) = deploy();
    let package_hash = proxy.package_hash_result();
    let proxy_contract_hash = proxy.contract_hash_result();
    let user = env.next_user();

    let num = 200;
    let amount: U512 = num.into();

    // first deposit some amount and verify
    proxy.deposit(Sender(owner), amount, Key::from(proxy_contract_hash));
    let res: Result<(), u32> = proxy.deposit_result();
    assert_eq!(token.balance_of(package_hash), U256::from(amount.as_u128()));
    assert_eq!(res.is_ok(), true);
    let transfer_amount: U256 = 1.into();

    // transfer amount to user
    proxy.transfer(Sender(owner), user, transfer_amount);
    let _ret: Result<(), u32> = proxy.transfer_result();
    assert_eq!(
        token.balance_of(package_hash),
        U256::from(amount.as_u128()) - transfer_amount
    );
    assert_eq!(token.balance_of(user), transfer_amount);
}

#[test]
#[should_panic]
fn test_wcspr_transfer_too_much() {
    let (env, token, proxy, _, owner) = deploy();
    let package_hash = proxy.package_hash_result();
    let proxy_contract_hash = proxy.contract_hash_result();
    let user = env.next_user();

    let num = 200;
    let amount: U512 = num.into();

    // first deposit some amount and verify
    proxy.deposit(Sender(owner), amount, Key::from(proxy_contract_hash));
    let res: Result<(), u32> = proxy.deposit_result();
    assert_eq!(token.balance_of(package_hash), U256::from(amount.as_u128())); //+ U256::from(deposit_amount));
    assert_eq!(res.is_ok(), true);
    let transfer_amount: U256 = 201.into();

    // transfer amount to user
    proxy.transfer(Sender(owner), user, transfer_amount);
    let _ret: Result<(), u32> = proxy.transfer_result();
}

#[test]
fn test_wcspr_approve() {
    let (env, token, _proxy, _, owner) = deploy();
    let user = env.next_user();
    let amount = 10.into();
    token.approve(Sender(owner), user, amount);
    assert_eq!(token.balance_of(user), 0.into());
    assert_eq!(token.allowance(owner, user), amount);
    assert_eq!(token.allowance(user, owner), 0.into());
}

#[test]
fn test_wcspr_transfer_from() {
    let (env, token, proxy, proxy2, owner) = deploy();

    let recipient = env.next_user();
    let proxy_contract_hash = proxy.contract_hash_result();
    let package_hash = proxy.package_hash_result();
    let package_hash2 = proxy2.package_hash_result();
    let deposit_amount = 50;
    let allowance = 10.into();
    let amount: U256 = 5.into();

    // proxy.increase_allowance(Sender(owner), recipient, allowance);
    // first deposit some amount and verify
    proxy.deposit(
        Sender(owner),
        deposit_amount.into(),
        Key::from(proxy_contract_hash),
    );
    let res: Result<(), u32> = proxy.deposit_result();
    assert_eq!(token.balance_of(package_hash), deposit_amount.into()); //+ U256::from(deposit_amount));
    assert_eq!(res.is_ok(), true);

    proxy.approve(Sender(owner), package_hash2, allowance);
    proxy.allowance_fn(
        Sender(owner),
        Key::from(package_hash),
        Key::from(package_hash2),
    );
    assert_eq!(proxy.allowance_res(), 10.into());
    proxy2.transfer_from(
        Sender(owner),
        Key::from(package_hash),
        Key::from(recipient),
        amount,
    );
    assert_eq!(token.balance_of(recipient), amount);
    assert_eq!(
        token.balance_of(package_hash),
        U256::from(deposit_amount) - amount
    );
}
#[test]
#[should_panic]
fn test_wcspr_transfer_from_too_much() {
    let (env, token, proxy, proxy2, owner) = deploy();

    let recipient = env.next_user();
    let proxy_contract_hash = proxy.contract_hash_result();
    let package_hash = proxy.package_hash_result();
    let package_hash2 = proxy2.package_hash_result();
    let deposit_amount = 50;
    let allowance = 10.into();
    let amount: U256 = 12.into();

    // proxy.increase_allowance(Sender(owner), recipient, allowance);
    // first deposit some amount and verify
    proxy.deposit(
        Sender(owner),
        deposit_amount.into(),
        Key::from(proxy_contract_hash),
    );
    let res: Result<(), u32> = proxy.deposit_result();
    assert_eq!(token.balance_of(package_hash), deposit_amount.into()); //+ U256::from(deposit_amount));
    assert_eq!(res.is_ok(), true);

    proxy.approve(Sender(owner), package_hash2, allowance);
    proxy.allowance_fn(
        Sender(owner),
        Key::from(package_hash),
        Key::from(package_hash2),
    );
    assert_eq!(proxy.allowance_res(), 10.into());
    proxy2.transfer_from(
        Sender(owner),
        Key::from(package_hash),
        Key::from(recipient),
        amount,
    );
    assert_eq!(token.balance_of(recipient), amount);
    assert_eq!(
        token.balance_of(package_hash),
        U256::from(deposit_amount) - amount
    );
}

#[test]
#[should_panic]
fn test_calling_construction() {
    let (_, token, _, _, owner) = deploy();
    token.constructor(Sender(owner), NAME, SYMBOL);
}
