use crate::wcspr_instance::*;
use casper_types::{account::AccountHash, ContractPackageHash, Key, U256, U512};
use test_env::{TestContract, TestEnv};

const DESTINATION_DEPOSIT: &str = "deposit";
const DESTINATION_WITHDRAW: &str = "withdraw";
pub const DESTINATION_GET_PURSE_BALANCE: &str = "get_purse_balance";
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
    let token: TestContract = WCSPRInstance::new(&env, NAME, owner, NAME, SYMBOL, DECIMALS);
    let proxy: TestContract = WCSPRInstance::proxy(&env, Key::Hash(token.contract_hash()), owner);
    // let proxy: TestContract = WCSPRInstance::new(&env, NAME, owner, NAME, SYMBOL, DECIMALS);
    let proxy2: TestContract = WCSPRInstance::proxy2(&env, Key::Hash(token.contract_hash()), owner);

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
    let (env, token, proxy, _, owner) = deploy();
    let depositor = env.next_user();
    let amount: U512 = 500.into();
    let wcspr_package_hash: ContractPackageHash = token.self_package_hash_result();

    // deposit with purse proxy
    let _purse_proxy: TestContract = deploy_purse_proxy(
        &env,
        depositor,
        amount,
        Key::from(wcspr_package_hash),
        DESTINATION_DEPOSIT,
    );

    // depositor balance
    let depositor_balance: U256 = token.balance_of(depositor);
    // get wcspr's purse balance through proxy contract 1 to verify deposit
    let wcspr_purse_balance: U512 = proxy.get_main_purse_balance(depositor);

    assert_eq!(wcspr_purse_balance, U512::from(depositor_balance.as_u128()));
    assert_eq!(wcspr_purse_balance, amount);
}

#[test]
#[should_panic]
fn test_wcspr_deposit_too_much() {
    let (env, token, proxy, _, owner) = deploy();
    let depositor = env.next_user();
    let amount: U512 = 500.into();
    let wcspr_package_hash: ContractPackageHash = token.self_package_hash_result();
    let wcspr_package_hash_key = Key::from(wcspr_package_hash);
    let proxy_package_hash: ContractPackageHash = proxy.package_hash_result();
    let proxy_package_hash_key = Key::from(proxy_package_hash);

    // get depositor current purse balance
    let _balance_check_proxy = deploy_purse_proxy(
        &env,
        depositor,
        0.into(),
        Key::from(proxy_package_hash_key),
        DESTINATION_GET_PURSE_BALANCE,
    );
    let depositor_purse_balance: U512 = proxy.result();
    assert_ne!(depositor_purse_balance, U512::from(0));

    // deposit with purse proxy - panic here
    let _purse_proxy: TestContract = deploy_purse_proxy(
        &env,
        depositor,
        depositor_purse_balance + 100,
        Key::from(wcspr_package_hash),
        DESTINATION_DEPOSIT,
    );
}

#[test]
#[should_panic]
fn test_wcspr_deposit_invalid_access_rights_purse() {
    let (env, token, proxy, _, owner) = deploy();
    let depositor = env.next_user();
    let amount: U512 = 500.into();
    let wcspr_package_hash: ContractPackageHash = token.self_package_hash_result();

    // deposit with purse proxy - Panic here
    let _purse_proxy: TestContract = deploy_invalid_purse_proxy(
        &env,
        depositor,
        amount,
        Key::from(wcspr_package_hash),
        DESTINATION_DEPOSIT,
    );
}

#[test]
fn test_wcspr_withdraw() {
    let (env, token, proxy, _, owner) = deploy();
    let depositor = env.next_user();
    let amount: U512 = 500.into();
    let wcspr_package_hash: ContractPackageHash = token.self_package_hash_result();
    let wcspr_package_hash_key = Key::from(wcspr_package_hash);
    // deposit with purse proxy
    let _deposit_purse_proxy: TestContract = deploy_purse_proxy(
        &env,
        depositor,
        amount,
        wcspr_package_hash_key,
        DESTINATION_DEPOSIT,
    );

    let depositor_balance: U256 = token.balance_of(depositor);
    let wcspr_purse_balance: U512 = proxy.get_main_purse_balance(depositor);

    // check deposit
    assert_eq!(wcspr_purse_balance, U512::from(depositor_balance.as_u128()));
    assert_eq!(wcspr_purse_balance, amount);

    // withdraw wih purse proxy
    let _withdraw_purse_proxy = deploy_purse_proxy(
        &env,
        depositor,
        amount / 2,
        wcspr_package_hash_key,
        DESTINATION_WITHDRAW,
    );

    let depositor_balance: U256 = token.balance_of(depositor);
    let wcspr_purse_balance: U512 = proxy.get_main_purse_balance(depositor);

    // check withdraw - depositor's WCSPR and wcspr purse balance are halved
    assert_eq!(depositor_balance, U256::from(amount.as_u128()) / 2);
    assert_eq!(wcspr_purse_balance, amount / 2);
    assert_eq!(wcspr_purse_balance, U512::from(depositor_balance.as_u128()));
}

#[test]
#[should_panic]
fn test_wcspr_deposit_more_than_u256_limit() {
    let (env, token, proxy, _, owner) = deploy();
    let depositor = env.next_user();
    let amount: U512 = U512::MAX;
    let wcspr_package_hash: ContractPackageHash = token.self_package_hash_result();
    let wcspr_package_hash_key = Key::from(wcspr_package_hash);
    // deposit with purse proxy
    let _deposit_purse_proxy: TestContract = deploy_purse_proxy(
        &env,
        depositor,
        amount,
        wcspr_package_hash_key,
        DESTINATION_DEPOSIT,
    );
}

#[test]
#[should_panic]
fn test_wcspr_withdraw_more_than_u256_limit() {
    let (env, token, proxy, _, owner) = deploy();
    let depositor = env.next_user();
    let amount: U512 = 500.into();
    let wcspr_package_hash: ContractPackageHash = token.self_package_hash_result();
    let wcspr_package_hash_key = Key::from(wcspr_package_hash);
    // deposit with purse proxy
    let _deposit_purse_proxy: TestContract = deploy_purse_proxy(
        &env,
        depositor,
        amount,
        wcspr_package_hash_key,
        DESTINATION_DEPOSIT,
    );

    let depositor_balance: U256 = token.balance_of(depositor);
    let wcspr_purse_balance: U512 = proxy.get_main_purse_balance(depositor);

    // check deposit
    assert_eq!(wcspr_purse_balance, U512::from(depositor_balance.as_u128()));
    assert_eq!(wcspr_purse_balance, amount);

    // withdraw wih purse proxy
    let _withdraw_purse_proxy = deploy_purse_proxy(
        &env,
        depositor,
        U512::MAX,
        wcspr_package_hash_key,
        DESTINATION_WITHDRAW,
    );
}

#[test]
#[should_panic]
fn test_wcspr_withdraw_too_much() {
    let (env, token, proxy, _, owner) = deploy();
    let depositor = env.next_user();
    let amount: U512 = 500.into();
    let wcspr_package_hash: ContractPackageHash = token.self_package_hash_result();
    let wcspr_package_hash_key = Key::from(wcspr_package_hash);
    // deposit with purse proxy
    let _deposit_purse_proxy: TestContract = deploy_purse_proxy(
        &env,
        depositor,
        amount,
        wcspr_package_hash_key,
        DESTINATION_DEPOSIT,
    );

    let depositor_balance: U256 = token.balance_of(depositor);
    let wcspr_purse_balance: U512 = proxy.get_main_purse_balance(depositor);

    // check deposit
    assert_eq!(wcspr_purse_balance, U512::from(depositor_balance.as_u128()));
    assert_eq!(wcspr_purse_balance, amount);

    // withdraw wih purse proxy - Panic here
    let _withdraw_purse_proxy = deploy_purse_proxy(
        &env,
        depositor,
        amount * 2,
        wcspr_package_hash_key,
        DESTINATION_WITHDRAW,
    );
}

#[test]
#[should_panic]
fn test_wcspr_withdraw_no_deposit() {
    let (env, token, proxy, _, owner) = deploy();
    let depositor = env.next_user();
    let amount: U512 = 500.into();
    let wcspr_package_hash: ContractPackageHash = token.self_package_hash_result();
    let wcspr_package_hash_key = Key::from(wcspr_package_hash);

    // no wcspr, no cspr, no deposit
    let wcspr_purse_balance: U512 = proxy.get_main_purse_balance(depositor);
    let depositor_balance: U256 = token.balance_of(depositor);
    assert_eq!(depositor_balance, U256::from(0));
    assert_eq!(wcspr_purse_balance, U512::from(0));

    // Panic here
    let _withdraw_purse_proxy = deploy_purse_proxy(
        &env,
        depositor,
        amount,
        wcspr_package_hash_key,
        DESTINATION_WITHDRAW,
    );
}

// #[test]
// fn test_wcspr_transfer() {
//     let (env, token, proxy, _, owner) = deploy();
//     let package_hash = proxy.package_hash_result();
//     let proxy_contract_hash = proxy.contract_hash_result();
//     let user = env.next_user();

//     let num = 200;
//     let amount: U512 = num.into();

//     // first deposit some amount and verify
//     proxy.deposit(owner, amount, Key::from(proxy_contract_hash));
//     let res: Result<(), u32> = proxy.deposit_result();
//     assert_eq!(token.balance_of(package_hash), U256::from(amount.as_u128()));
//     assert_eq!(res.is_ok(), true);
//     let transfer_amount: U256 = 1.into();

//     // transfer amount to user
//     proxy.transfer(owner, user, transfer_amount);
//     let _ret: Result<(), u32> = proxy.transfer_result();
//     assert_eq!(
//         token.balance_of(package_hash),
//         U256::from(amount.as_u128()) - transfer_amount
//     );
//     assert_eq!(token.balance_of(user), transfer_amount);
// }

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
    proxy.deposit(owner, amount, Key::from(proxy_contract_hash));
    let res: Result<(), u32> = proxy.deposit_result();
    assert_eq!(token.balance_of(package_hash), U256::from(amount.as_u128())); //+ U256::from(deposit_amount));
    assert_eq!(res.is_ok(), true);
    let transfer_amount: U256 = 201.into();

    // transfer amount to user
    proxy.transfer(owner, user, transfer_amount);
    let _ret: Result<(), u32> = proxy.transfer_result();
}

#[test]
fn test_wcspr_approve() {
    let (env, token, _proxy, _, owner) = deploy();
    let user = env.next_user();
    let amount = 10.into();
    token.approve(owner, user, amount);
    assert_eq!(token.balance_of(user), 0.into());
    assert_eq!(token.allowance(owner, user), amount);
    assert_eq!(token.allowance(user, owner), 0.into());
}

// #[test]
// fn test_wcspr_transfer_from() {
//     let (env, token, proxy, proxy2, owner) = deploy();

//     let recipient = env.next_user();
//     let proxy_contract_hash = proxy.contract_hash_result();
//     let package_hash = proxy.package_hash_result();
//     let package_hash2 = proxy2.package_hash_result();
//     let deposit_amount = 50;
//     let allowance = 10.into();
//     let amount: U256 = 5.into();

//     // proxy.increase_allowance(owner, recipient, allowance);
//     // first deposit some amount and verify
//     proxy.deposit(owner, deposit_amount.into(), Key::from(proxy_contract_hash));
//     let res: Result<(), u32> = proxy.deposit_result();
//     assert_eq!(token.balance_of(package_hash), deposit_amount.into()); //+ U256::from(deposit_amount));
//     assert_eq!(res.is_ok(), true);

//     proxy.approve(owner, package_hash2, allowance);
//     proxy.allowance_fn(owner, Key::from(package_hash), Key::from(package_hash2));
//     assert_eq!(proxy.allowance_res(), 10.into());
//     proxy2.transfer_from(owner, Key::from(package_hash), Key::from(recipient), amount);
//     assert_eq!(token.balance_of(recipient), amount);
//     assert_eq!(
//         token.balance_of(package_hash),
//         U256::from(deposit_amount) - amount
//     );
// }
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

    // proxy.increase_allowance(owner, recipient, allowance);
    // first deposit some amount and verify
    proxy.deposit(owner, deposit_amount.into(), Key::from(proxy_contract_hash));
    let res: Result<(), u32> = proxy.deposit_result();
    assert_eq!(token.balance_of(package_hash), deposit_amount.into()); //+ U256::from(deposit_amount));
    assert_eq!(res.is_ok(), true);

    proxy.approve(owner, package_hash2, allowance);
    proxy.allowance_fn(owner, Key::from(package_hash), Key::from(package_hash2));
    assert_eq!(proxy.allowance_res(), 10.into());
    proxy2.transfer_from(owner, Key::from(package_hash), Key::from(recipient), amount);
    assert_eq!(token.balance_of(recipient), amount);
    assert_eq!(
        token.balance_of(package_hash),
        U256::from(deposit_amount) - amount
    );
}

#[test]
fn test_wcspr_increase_allowance() {
    let (env, token, proxy, proxy2, owner) = deploy();
    let package_hash = proxy.package_hash_result();
    let package_hash2 = proxy2.package_hash_result();
    let amount: U256 = 100.into();

    proxy.increase_allowance(owner, package_hash2, amount);
    proxy.allowance_fn(owner, Key::from(package_hash), Key::from(package_hash2));
    assert_eq!(proxy.allowance_res(), 100.into());

    proxy.increase_allowance(owner, package_hash2, amount);
    proxy.allowance_fn(owner, Key::from(package_hash), Key::from(package_hash2));
    assert_eq!(proxy.allowance_res(), 200.into());
}

#[test]
fn test_wcspr_decrease_allowance() {
    let (env, token, proxy, proxy2, owner) = deploy();
    let package_hash = proxy.package_hash_result();
    let package_hash2 = proxy2.package_hash_result();
    let amount: U256 = 100.into();

    proxy.increase_allowance(owner, package_hash2, amount + amount);
    proxy.allowance_fn(owner, Key::from(package_hash), Key::from(package_hash2));
    assert_eq!(proxy.allowance_res(), 200.into());

    proxy.decrease_allowance(owner, package_hash2, amount);
    proxy.allowance_fn(owner, Key::from(package_hash), Key::from(package_hash2));
    assert_eq!(proxy.allowance_res(), 100.into());
}
