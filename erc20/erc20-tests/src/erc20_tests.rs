use tests_common::{account::AccountHash, deploys::*, helpers::*, *};

fn deploy() -> (TestEnv, AccountHash, TestContract) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let token = deploy_erc20(
        &env,
        NAME,
        owner,
        NAME,
        SYMBOL,
        DECIMALS,
        INIT_TOTAL_SUPPLY,
        now(),
    );
    (env, owner, token)
}

#[test]
fn test_deploy() {
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
fn test_erc20_mint_burn() {
    let (_, owner, erc20) = deploy();
    let amount: U256 = 123_000_000_000u64.into();
    erc20.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Address::Account(owner),
            "amount" => amount
        },
        now(),
    );
    let ret: U256 = erc20.query(BALANCES, address_to_str(&Address::Account(owner)));
    assert_eq!(ret, amount);
    erc20.call_contract(
        owner,
        "burn",
        runtime_args! {
            "from" => Address::Account(owner),
            "amount" => amount
        },
        now(),
    );
    let ret: U256 = erc20.query(BALANCES, address_to_str(&Address::Account(owner)));
    assert_eq!(ret, 0.into());
}

#[test]
fn test_erc20_transfer() {
    let (env, owner, erc20) = deploy();
    let to = env.next_user();
    let amount: U256 = 123_000_000_000u64.into();
    erc20.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Address::Account(to),
            "amount" => amount
        },
        now(),
    );
    let ret: U256 = erc20.query(BALANCES, address_to_str(&Address::Account(to)));
    assert_eq!(ret, amount);
    let ret: U256 = erc20.query(BALANCES, address_to_str(&Address::Account(owner)));
    assert_eq!(ret, 0.into());
    erc20.call_contract(
        to,
        "transfer",
        runtime_args! {
            "recipient" => Address::Account(owner),
            "amount" => amount,
        },
        now(),
    );
    let ret: U256 = erc20.query(BALANCES, address_to_str(&Address::Account(to)));
    assert_eq!(ret, 0.into());
    let ret: U256 = erc20.query(BALANCES, address_to_str(&Address::Account(owner)));
    assert_eq!(ret, amount);
}

#[test]
fn test_erc20_approve_transfer_from() {
    let (env, owner, erc20) = deploy();
    let to = env.next_user();
    let tmp_user = env.next_user();
    let amount: U256 = 123_000_000_000u64.into();
    erc20.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Address::Account(to),
            "amount" => amount
        },
        now(),
    );
    let ret: U256 = erc20.query(BALANCES, address_to_str(&Address::Account(to)));
    assert_eq!(ret, amount);
    let ret: U256 = erc20.query(BALANCES, address_to_str(&Address::Account(owner)));
    assert_eq!(ret, 0.into());
    erc20.call_contract(
        to,
        "approve",
        runtime_args! {
            "spender" => Address::Account(tmp_user),
            "amount" => amount,
        },
        now(),
    );
    erc20.call_contract(
        tmp_user,
        "transfer_from",
        runtime_args! {
            "owner" => Address::Account(to),
            "recipient" => Address::Account(owner),
            "amount" => amount,
        },
        now(),
    );
    let ret: U256 = erc20.query(BALANCES, address_to_str(&Address::Account(to)));
    assert_eq!(ret, 0.into());
    let ret: U256 = erc20.query(BALANCES, address_to_str(&Address::Account(owner)));
    assert_eq!(ret, amount);
}

#[test]
fn test_erc20_increase_allowance_transfer_from() {
    let (env, owner, erc20) = deploy();
    let to = env.next_user();
    let tmp_user = env.next_user();
    let amount: U256 = 123_000_000_000u64.into();
    erc20.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Address::Account(to),
            "amount" => amount
        },
        now(),
    );
    let ret: U256 = erc20.query(BALANCES, address_to_str(&Address::Account(to)));
    assert_eq!(ret, amount);
    let ret: U256 = erc20.query(BALANCES, address_to_str(&Address::Account(owner)));
    assert_eq!(ret, 0.into());
    erc20.call_contract(
        to,
        "increase_allowance",
        runtime_args! {
            "spender" => Address::Account(tmp_user),
            "amount" => amount,
        },
        now(),
    );
    erc20.call_contract(
        tmp_user,
        "transfer_from",
        runtime_args! {
            "owner" => Address::Account(to),
            "recipient" => Address::Account(owner),
            "amount" => amount,
        },
        now(),
    );
    let ret: U256 = erc20.query(BALANCES, address_to_str(&Address::Account(to)));
    assert_eq!(ret, 0.into());
    let ret: U256 = erc20.query(BALANCES, address_to_str(&Address::Account(owner)));
    assert_eq!(ret, amount);
}

#[test]
#[should_panic]
fn test_erc20_decrease_allowance_transfer_from() {
    let (env, owner, erc20) = deploy();
    let to = env.next_user();
    let tmp_user = env.next_user();
    let amount: U256 = 123_000_000_000u64.into();
    erc20.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Address::Account(to),
            "amount" => amount
        },
        now(),
    );
    let ret: U256 = erc20.query(BALANCES, address_to_str(&Address::Account(to)));
    assert_eq!(ret, amount);
    let ret: U256 = erc20.query(BALANCES, address_to_str(&Address::Account(owner)));
    assert_eq!(ret, 0.into());
    erc20.call_contract(
        to,
        "decrease_allowance",
        runtime_args! {
            "spender" => Address::Account(tmp_user),
            "amount" => amount,
        },
        now(),
    );
    erc20.call_contract(
        tmp_user,
        "transfer_from",
        runtime_args! {
            "owner" => Address::Account(to),
            "recipient" => Address::Account(owner),
            "amount" => amount,
        },
        now(),
    );
    let ret: U256 = erc20.query(BALANCES, address_to_str(&Address::Account(to)));
    assert_eq!(ret, 0.into());
    let ret: U256 = erc20.query(BALANCES, address_to_str(&Address::Account(owner)));
    assert_eq!(ret, amount);
}
