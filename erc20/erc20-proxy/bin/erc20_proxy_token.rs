#![no_main]
#![no_std]

#[macro_use]
extern crate alloc;

use alloc::{collections::BTreeSet, format};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, CLTyped, ContractHash, ContractPackageHash, EntryPoint, EntryPointAccess,
    EntryPointType, EntryPoints, Group, Key, Parameter, RuntimeArgs, URef, U256,
};

pub mod mappings;

#[no_mangle]
fn constructor() {
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    let erc20: Key = runtime::get_named_arg("erc20");

    mappings::set_key(&mappings::self_hash_key(), contract_hash);
    mappings::set_key(&mappings::self_package_key(), package_hash);
    mappings::set_key(
        &mappings::erc20_key(),
        ContractHash::from(erc20.into_hash().unwrap_or_default()),
    );
}

#[no_mangle]
fn transfer() {
    let erc20_address: ContractHash = mappings::get_key(&mappings::erc20_key());

    let recipient: Key = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");

    let args: RuntimeArgs = runtime_args! {
        "recipient" => recipient,
        "amount" => amount,
    };

    let ret: Result<(), u32> = runtime::call_contract(erc20_address, "transfer", args);
    mappings::set_key(&mappings::transfer_key(), ret);
}

#[no_mangle]
fn approve() {
    let erc20_address: ContractHash = mappings::get_key(&mappings::erc20_key());
    let spender: Key = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");
    let args: RuntimeArgs = runtime_args! {
        "spender" => spender,
        "amount" => amount,
    };

    let _ret: () = runtime::call_contract(erc20_address, "approve", args);
}

#[no_mangle]
fn allowance() {
    let erc20_address: ContractHash = mappings::get_key(&mappings::erc20_key());
    let owner: Key = runtime::get_named_arg("owner");
    let spender: Key = runtime::get_named_arg("spender");
    let args: RuntimeArgs = runtime_args! {
        "owner" => owner,
        "spender" => spender,
    };

    let ret: U256 = runtime::call_contract(erc20_address, "allowance", args);
    mappings::set_key(&mappings::allowance(), ret);
}

#[no_mangle]
fn transfer_from() {
    let erc20_address: ContractHash = mappings::get_key(&mappings::erc20_key());

    let owner: Key = runtime::get_named_arg("owner");
    let recipient: Key = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");

    let args: RuntimeArgs = runtime_args! {
        "owner" => owner,
        "recipient" => recipient,
        "amount" => amount,
    };

    let ret: Result<(), u32> = runtime::call_contract(erc20_address, "transfer_from", args);
    mappings::set_key(&mappings::transfer_from_key(), ret);
}

#[no_mangle]
fn increase_allowance() {
    let erc20_address: ContractHash = mappings::get_key(&mappings::erc20_key());

    let spender: Key = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");
    let args: RuntimeArgs = runtime_args! {
        "spender" => spender,
        "amount" => amount,
    };

    let ret: Result<(), u32> = runtime::call_contract(erc20_address, "increase_allowance", args);
    mappings::set_key(&mappings::increase_allowance_key(), ret);
}

#[no_mangle]
fn decrease_allowance() {
    let erc20_address: ContractHash = mappings::get_key(&mappings::erc20_key());

    let spender: Key = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");
    let args: RuntimeArgs = runtime_args! {
        "spender" => spender,
        "amount" => amount,
    };

    let ret: Result<(), u32> = runtime::call_contract(erc20_address, "decrease_allowance", args);
    mappings::set_key(&mappings::decrease_allowance_key(), ret);
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
            Parameter::new("erc20", Key::cl_type()),
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
fn call() {
    // Build new package with initial a first version of the contract.
    let (package_hash, access_token) = storage::create_contract_package_at_hash();
    let (contract_hash, _) =
        storage::add_contract_version(package_hash, get_entry_points(), Default::default());
    let erc20: Key = runtime::get_named_arg("erc20");

    // Prepare constructor args
    let constructor_args = runtime_args! {
        "contract_hash" => contract_hash,
        "package_hash" => package_hash,
        "erc20" => erc20
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
