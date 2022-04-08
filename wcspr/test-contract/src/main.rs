#![no_std]
#![no_main]

// #[cfg(not(target_arch = "wasm32"))]
// compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;
use alloc::{collections::BTreeSet, format, vec};

use casper_contract::{
    contract_api::{account, runtime, storage, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::{ContractHash, ContractPackageHash},
    runtime_args, CLTyped, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Group, Key,
    Parameter, RuntimeArgs, URef, U256, U512,
};

pub mod constants;
use constants::*;

pub mod utils;
use utils::*;

// ================================== Test Endpoints ================================== //
#[no_mangle]
fn deposit() {
    let amount: U512 = runtime::get_named_arg("amount");
    let purse: URef = runtime::get_named_arg("purse");
    // let purse: URef = account::get_main_purse();
    // let purse: URef = get_key(&SELF_PURSE_KEY_NAME);
    let ret: Result<(), u32> = runtime::call_contract(
        get_key(&WCSPR_HASH_KEY_NAME),
        DEPOSIT_ENTRY_POINT_NAME,
        runtime_args! {
            PURSE_RUNTIME_ARG_NAME=> purse,
            AMOUNT_RUNTIME_ARG_NAME=> amount
        },
    );

    set_key(DEPOSIT_TEST_RESULT_KEY_NAME, ret);
}

#[no_mangle]
fn deposit_session() {
    let amount: U512 = runtime::get_named_arg("amount");
    let proxy_hash: Key = runtime::get_named_arg("proxy_hash");
    let purse: URef = account::get_main_purse();

    let () = runtime::call_contract(
        _create_hash_from_key(proxy_hash),
        DEPOSIT_ENTRY_POINT_NAME,
        runtime_args! {
            PURSE_RUNTIME_ARG_NAME=> purse,
            AMOUNT_RUNTIME_ARG_NAME=> amount
        },
    );
    // set_key(DEPOSIT_TEST_RESULT_KEY_NAME, ret);
}
#[no_mangle]
fn withdraw() {
    //let to: Key = runtime::get_named_arg("to");
    let to: URef = system::create_purse();
    let amount: U512 = runtime::get_named_arg("amount");
    // let wcspr_hash: Key = runtime::get_named_arg("wcspr_hash");
    let wcspr_hash: ContractHash = get_key(&WCSPR_HASH_KEY_NAME);
    let ret: Result<(), u32> = runtime::call_contract(
        // _create_hash_from_key(wcspr_hash),
        wcspr_hash,
        WITHDRAW_ENTRY_POINT_NAME,
        runtime_args! {
            TO_PURSE_RUNTIME_ARG_NAME => to,
            AMOUNT_RUNTIME_ARG_NAME=> amount
        },
    );
    set_key(WITHDRAW_TEST_RESULT_KEY_NAME, ret);
}

#[no_mangle]
fn transfer() {
    let recipient: Key = runtime::get_named_arg(RECIPIENT_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);
    let wcspr_hash: ContractHash = get_key(&WCSPR_HASH_KEY_NAME);

    let res: Result<(), u32> = runtime::call_contract(
        wcspr_hash,
        TRANSFER_ENTRY_POINT_NAME,
        runtime_args! {
            RECIPIENT_RUNTIME_ARG_NAME => recipient,
            AMOUNT_RUNTIME_ARG_NAME => amount
        },
    );
    set_key(TRANSFER_TEST_RESULT_KEY_NAME, res);
}

#[no_mangle]
fn transfer_from() {
    let owner: Key = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
    let recipient: Key = runtime::get_named_arg(RECIPIENT_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);
    let wcspr_hash: ContractHash = get_key(&WCSPR_HASH_KEY_NAME);

    let res: Result<(), u32> = runtime::call_contract(
        wcspr_hash,
        TRANSFER_FROM_ENTRY_POINT_NAME,
        runtime_args! {
            OWNER_RUNTIME_ARG_NAME => owner,
            RECIPIENT_RUNTIME_ARG_NAME => recipient,
            AMOUNT_RUNTIME_ARG_NAME => amount
        },
    );
    set_key(TRANSFER_FROM_TEST_RESULT_KEY_NAME, res);
}

#[no_mangle]
fn allowance() {
    let wcspr_hash: ContractHash = get_key(&WCSPR_HASH_KEY_NAME);
    let owner: Key = runtime::get_named_arg("owner");
    let spender: Key = runtime::get_named_arg("spender");
    let args: RuntimeArgs = runtime_args! {
        "owner" => owner,
        "spender" => spender,
    };

    let ret: U256 = runtime::call_contract(wcspr_hash, "allowance", args);
    set_key(ALLOWANCE, ret);
}

#[no_mangle]
fn increase_allowance() {
    let wcspr_hash: ContractHash = get_key(&WCSPR_HASH_KEY_NAME);

    let spender: Key = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");
    let args: RuntimeArgs = runtime_args! {
        "spender" => spender,
        "amount" => amount,
    };

    let ret: Result<(), u32> = runtime::call_contract(wcspr_hash, "increase_allowance", args);
    set_key("result", ret);
}

#[no_mangle]
fn decrease_allowance() {
    let wcspr_hash: ContractHash = get_key(&WCSPR_HASH_KEY_NAME);

    let spender: Key = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");
    let args: RuntimeArgs = runtime_args! {
        "spender" => spender,
        "amount" => amount,
    };

    let ret: Result<(), u32> = runtime::call_contract(wcspr_hash, "decrease_allowance", args);
    set_key("result", ret);
}

#[no_mangle]
fn approve() {
    let wcspr_hash: ContractHash = get_key(&WCSPR_HASH_KEY_NAME);
    let spender: Key = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");
    let args: RuntimeArgs = runtime_args! {
        "spender" => spender,
        "amount" => amount,
    };

    let _ret: () = runtime::call_contract(wcspr_hash, "approve", args);
}

#[no_mangle]
fn get_main_purse() {
    let wcspr_hash: ContractHash = get_key(&WCSPR_HASH_KEY_NAME);
    let ret: URef = runtime::call_contract(wcspr_hash, "get_main_purse", runtime_args! {});
    set_key("result", ret);
}

#[no_mangle]
fn get_main_purse_balance() {
    let wcspr_hash: ContractHash = get_key(&WCSPR_HASH_KEY_NAME);
    let ret: U512 = runtime::call_contract(wcspr_hash, "get_main_purse_balance", runtime_args! {});
    set_key("result", ret);
}

// get balance of any purse
#[no_mangle]
fn get_purse_balance() {
    let main_purse: URef = runtime::get_named_arg("purse");
    let ret: U512 = system::get_purse_balance(main_purse).unwrap_or_revert();
    set_key("result", ret);
}

// ================================== Helper functions ============================ //
fn _create_hash_from_key(key: Key) -> ContractHash {
    ContractHash::from(key.into_hash().unwrap_or_default())
}

// ================================ Test Contract Construction =========================== //
fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "get_main_purse",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_main_purse_balance",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_purse_balance",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("wcspr", Key::cl_type()),
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "transfer",
        vec![
            Parameter::new("recipient", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "transfer_from",
        vec![
            Parameter::new("owner", Key::cl_type()),
            Parameter::new("recipient", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "deposit",
        vec![
            Parameter::new("amount", U512::cl_type()),
            Parameter::new("purse", URef::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "deposit_session",
        vec![
            Parameter::new("amount", U512::cl_type()),
            // Parameter::new("purse", URef::cl_type()),
            Parameter::new("proxy_hash", Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Session,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "allowance",
        vec![
            Parameter::new("owner", Key::cl_type()),
            Parameter::new("spender", Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "withdraw",
        vec![
            //Parameter::new("to", Key::cl_type()),             // no need to pass 'to'. Purse will be created here
            Parameter::new("amount", U512::cl_type()),
            // Parameter::new("wcspr_hash", Key::cl_type())
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "increase_allowance",
        vec![
            Parameter::new("spender", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "decrease_allowance",
        vec![
            Parameter::new("spender", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "approve",
        vec![
            Parameter::new("spender", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points
}

#[no_mangle]
fn constructor() {
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    let wcspr_hash: Key = runtime::get_named_arg("wcspr");
    set_key(
        &WCSPR_HASH_KEY_NAME,
        ContractHash::from(wcspr_hash.into_hash().unwrap_or_default()),
    );
    set_key(&CONTRACT_HASH_KEY_NAME, contract_hash);
    set_key(&PACKAGE_HASH_KEY_NAME, package_hash);
}

// All session code must have a `call` entrypoint.
#[no_mangle]
pub extern "C" fn call() {
    // Build new package with initial a first version of the contract.
    let (package_hash, access_token) = storage::create_contract_package_at_hash();
    let (contract_hash, _): (ContractHash, _) =
        storage::add_contract_version(package_hash, get_entry_points(), Default::default());

    let wcspr_hash: Key = runtime::get_named_arg("wcspr");

    // Get parameters and pass it to the constructors
    // Prepare constructor args
    let constructor_args = runtime_args! {
        CONTRACT_HASH_RUNTIME_ARG_NAME => contract_hash,
        PACKAGE_HASH_RUNTIME_ARG_NAME => package_hash,
        WCSPR_HASH_RUNTIME_ARG_NAME => wcspr_hash,
    };

    // Add the constructor group to the package hash with a single URef.
    let constructor_access: URef =
        storage::create_contract_user_group(package_hash, "constructor", 1, Default::default())
            .unwrap_or_revert()
            .pop()
            .unwrap_or_revert();

    // Call the constructor entry point
    let _: () =
        runtime::call_versioned_contract(package_hash, None, "constructor", constructor_args);

    // Remove all URefs from the constructor group, so no one can call it for the second time.
    let mut urefs = BTreeSet::new();
    urefs.insert(constructor_access);
    storage::remove_contract_user_group_urefs(package_hash, "constructor", urefs)
        .unwrap_or_revert();

    // Store contract in the account's named keys.
    let contract_name: alloc::string::String = runtime::get_named_arg("contract_name");

    runtime::put_key(
        &format!("{}_package_hash", contract_name),
        package_hash.into(),
    );
    runtime::put_key(
        &format!("{}_package_hash_wrapped", contract_name),
        storage::new_uref(package_hash).into(),
    );
    runtime::put_key(
        &format!("{}_contract_hash", contract_name),
        contract_hash.into(),
    );
    runtime::put_key(
        &format!("{}_contract_hash_wrapped", contract_name),
        storage::new_uref(contract_hash).into(),
    );
    runtime::put_key(
        &format!("{}_package_access_token", contract_name),
        access_token.into(),
    );
}
