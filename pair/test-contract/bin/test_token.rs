#![no_main]
#![no_std]

extern crate alloc;

use alloc::{collections::BTreeSet, format, string::String, vec};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, CLTyped, ContractHash, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints,
    Group, Key, Parameter, RuntimeArgs, URef, U256,
};
use contract_utils::{ContractContext, OnChainContractStorage};
use test::{self, TEST};

#[derive(Default)]
struct Test(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for Test {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl TEST<OnChainContractStorage> for Test {}

impl Test {
    fn constructor(&mut self, name: String, contract_hash: ContractHash) {
        TEST::init(self, name, Key::from(contract_hash));
    }
}

#[no_mangle]
fn constructor() {
    let name: String = runtime::get_named_arg("name");
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    Test::default().constructor(name, contract_hash);
}

#[no_mangle]
fn mint_with_caller() {
    let caller: Key = runtime::get_named_arg("caller");
    let to: Key = runtime::get_named_arg("to");
    let amount: U256 = runtime::get_named_arg("amount");
    Test::default().mint_with_caller(caller, to, amount);
}

#[no_mangle]
pub extern "C" fn set_fee_to() {
    let fee_to: Key = runtime::get_named_arg("fee_to");
    let factory_hash: Key = runtime::get_named_arg("factory_hash");
    Test::default().set_fee_to(fee_to, factory_hash);
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("name", String::cl_type()),
            Parameter::new("contract_hash", ContractHash::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "set_fee_to",
        vec![Parameter::new("fee_to", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "mint_with_caller",
        vec![
            Parameter::new("caller", Key::cl_type()),
            Parameter::new("to", Key::cl_type()),
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

    let name: &str = "TEST";
    // Prepare constructor args
    let constructor_args = runtime_args! {
        "name" => name,
        "contract_hash" => contract_hash
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
