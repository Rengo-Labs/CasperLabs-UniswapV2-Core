use casper_types::{account::AccountHash, Key, U256};
use test_env::{TestContract, TestEnv};

use crate::erc20_instance::ERC20Instance;

const NAME: &str = "ERC20";
const SYMBOL: &str = "ERC";
const DECIMALS: u8 = 8;
const INIT_TOTAL_SUPPLY: u64 = 1000;

fn deploy() -> (
    TestEnv,
    ERC20Instance,
    AccountHash,
    ERC20Instance,
    ERC20Instance,
) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let token: TestContract = ERC20Instance::new(
        &env,
        NAME,
        owner,
        NAME,
        SYMBOL,
        DECIMALS,
        INIT_TOTAL_SUPPLY.into(),
    );
    let test_contract: TestContract =
        ERC20Instance::proxy(&env, Key::Hash(token.contract_hash()), owner);
    let test_contract2: TestContract =
        ERC20Instance::proxy2(&env, Key::Hash(token.contract_hash()), owner);
    (
        env,
        ERC20Instance::instance(token),
        owner,
        ERC20Instance::instance(test_contract),
        ERC20Instance::instance(test_contract2),
    )
}

#[test]
fn test_deploy() {
    let (env, token, owner, _, _) = deploy();
    let user = env.next_user();
    assert_eq!(token.name(), NAME);
    assert_eq!(token.symbol(), SYMBOL);
    // assert_eq!(token.meta(), meta::contract_meta());
    assert_eq!(
        token.total_supply(),
        (INIT_TOTAL_SUPPLY + INIT_TOTAL_SUPPLY).into()
    );
    assert_eq!(token.decimals(), DECIMALS);
    assert_eq!(token.balance_of(owner), INIT_TOTAL_SUPPLY.into());
    assert_eq!(token.balance_of(user), 0.into());
    assert_eq!(token.allowance(owner, user), 0.into());
    assert_eq!(token.allowance(user, owner), 0.into());
}

#[test]
fn test_erc20_approve() {
    let (env, token, owner, _, _) = deploy();
    let user = env.next_user();
    let amount = 10.into();
    token.approve(owner, user, amount);
    assert_eq!(token.balance_of(owner), INIT_TOTAL_SUPPLY.into());
    assert_eq!(token.balance_of(user), 0.into());
    assert_eq!(token.allowance(owner, user), amount);
    assert_eq!(token.allowance(user, owner), 0.into());
}

#[test]
fn test_erc20_mint() {
    let (env, token, owner, _, _) = deploy();
    let user = env.next_user();
    let amount = 10.into();
    token.mint(owner, user, amount);
    assert_eq!(token.balance_of(owner), INIT_TOTAL_SUPPLY.into());
    assert_eq!(token.balance_of(user), amount);
    assert_eq!(token.balance_of(user), 10.into());
}

#[test]
fn test_erc20_burn() {
    let (env, token, owner, _, _) = deploy();
    let user = env.next_user();
    let amount = 10.into();
    assert_eq!(token.balance_of(owner), U256::from(INIT_TOTAL_SUPPLY));
    token.burn(owner, owner, amount);
    assert_eq!(
        token.balance_of(owner),
        U256::from(INIT_TOTAL_SUPPLY) - amount
    );
    assert_eq!(token.balance_of(user), 0.into());
}

#[test]
fn test_erc20_transfer() {
    let (env, token, owner, proxy, _proxy2) = deploy();
    let package_hash = proxy.package_hash_result();
    let user = env.next_user();
    let amount: U256 = 100.into();

    // TRASNFER CALL IN PROXY USES:- runtime::call_contract() so transfer is being done from proxy to a recipient

    // Minting to proxy contract as it is the intermediate caller to transfer
    token.mint(owner, package_hash, amount);

    assert_eq!(token.balance_of(package_hash), amount);
    assert_eq!(token.balance_of(user), U256::from(0));

    // // Transfering to user from the proxy contract
    proxy.transfer(owner, user, amount);

    assert_eq!(token.balance_of(package_hash), U256::from(0));
    assert_eq!(token.balance_of(user), amount);

    let ret: Result<(), u32> = proxy.transfer_result();

    match ret {
        Ok(()) => {}
        Err(e) => assert!(false, "Transfer Failed ERROR:{}", e),
    }
}

#[test]
#[should_panic]
fn test_erc20_transfer_with_same_sender_and_recipient() {
    let (env, token, owner, proxy, _proxy2) = deploy();
    let package_hash = proxy.package_hash_result();
    let user = env.next_user();
    let amount: U256 = 100.into();

    // TRASNFER CALL IN PROXY USES:- runtime::call_contract() so transfer is being done from proxy to a recipient

    // Minting to proxy contract as it is the intermediate caller to transfer
    token.mint(owner, package_hash, amount);

    assert_eq!(token.balance_of(package_hash), amount);
    assert_eq!(token.balance_of(user), U256::from(0));
    assert_eq!(token.balance_of(owner), 1000.into());

    // Transfering to user from the proxy contract
    proxy.transfer(owner, package_hash, amount);

    assert_eq!(token.balance_of(package_hash), U256::from(100));

    assert_eq!(token.balance_of(owner), U256::from(1000));

    let ret: Result<(), u32> = proxy.transfer_result();

    match ret {
        Ok(()) => {}
        Err(e) => assert!(false, "Transfer Failed ERROR:{}", e),
    }
}

#[test]
fn test_erc20_transfer_from() {
    let (env, token, owner, proxy, proxy2) = deploy();
    let package_hash = proxy.package_hash_result();
    let package_hash2 = proxy2.package_hash_result();
    let recipient = env.next_user();
    let user = env.next_user();
    let mint_amount = 100.into();
    let allowance = 10.into();
    let amount: U256 = 1.into();
    // Minting to proxy contract as it is the intermediate caller to transfer
    token.mint(owner, package_hash, mint_amount);

    proxy.approve(owner, package_hash2, allowance);
    assert_eq!(token.balance_of(owner), 1000.into());

    proxy.allowance_fn(owner, Key::from(package_hash), Key::from(package_hash2));
    assert_eq!(proxy.allowance_res(), 10.into());

    proxy2.transfer_from(owner, package_hash.into(), user.into(), amount);

    assert_eq!(token.nonce(owner), 0.into());
    assert_eq!(token.nonce(recipient), 0.into());
    assert_eq!(token.balance_of(owner), 1000.into());
    assert_eq!(token.balance_of(user), amount);

    let ret: Result<(), u32> = proxy2.transfer_from_result();

    match ret {
        Ok(()) => {}
        Err(e) => assert!(false, "Transfer Failed ERROR:{}", e),
    }
}

#[test]
#[should_panic]
fn test_erc20_transfer_from_too_much() {
    let (env, token, owner, proxy, proxy2) = deploy();
    let package_hash = proxy.package_hash_result();
    let package_hash2 = proxy2.package_hash_result();
    let user = env.next_user();
    let mint_amount = 100.into();
    let allowance = 10.into();
    let amount: U256 = 12.into();
    // Minting to proxy contract as it is the intermediate caller to transfer
    token.mint(owner, package_hash, mint_amount);

    proxy.approve(owner, package_hash2, allowance);
    assert_eq!(token.balance_of(owner), 1000.into());

    proxy.allowance_fn(owner, Key::from(package_hash), Key::from(package_hash2));
    assert_eq!(proxy.allowance_res(), 10.into());

    proxy2.transfer_from(owner, package_hash.into(), user.into(), amount);
}

#[test]
fn test_erc20_increase_allowance() {
    let (_, token, owner, proxy, proxy2) = deploy();
    let package_hash = proxy.package_hash_result();
    let package_hash2 = proxy2.package_hash_result();
    let amount: U256 = 100.into();

    proxy.increase_allowance(owner, package_hash2, amount);
    assert_eq!(token.balance_of(owner), 1000.into());

    proxy.allowance_fn(owner, Key::from(package_hash), Key::from(package_hash2));
    assert_eq!(proxy.allowance_res(), 100.into());

    proxy.increase_allowance(owner, package_hash2, amount);
    assert_eq!(token.balance_of(owner), 1000.into());

    proxy.allowance_fn(owner, Key::from(package_hash), Key::from(package_hash2));
    assert_eq!(proxy.allowance_res(), 200.into());

    let ret: Result<(), u32> = proxy.increase_allowance_res();

    match ret {
        Ok(()) => {}
        Err(e) => assert!(false, "Increase Allowance Failed ERROR:{}", e),
    }
}

#[test]
fn test_erc20_decrease_allowance() {
    let (_, token, owner, proxy, proxy2) = deploy();
    let package_hash = proxy.package_hash_result();
    let package_hash2 = proxy2.package_hash_result();
    let amount: U256 = 100.into();

    proxy.increase_allowance(owner, package_hash2, amount + amount);

    proxy.allowance_fn(owner, Key::from(package_hash), Key::from(package_hash2));
    assert_eq!(proxy.allowance_res(), 200.into());

    proxy.decrease_allowance(owner, package_hash2, amount);
    assert_eq!(token.balance_of(owner), 1000.into());

    proxy.allowance_fn(owner, Key::from(package_hash), Key::from(package_hash2));
    assert_eq!(proxy.allowance_res(), 100.into());

    let ret: Result<(), u32> = proxy.decrease_allowance_res();

    match ret {
        Ok(()) => {}
        Err(e) => assert!(false, "Decrease Allowance Failed ERROR:{}", e),
    }
}
