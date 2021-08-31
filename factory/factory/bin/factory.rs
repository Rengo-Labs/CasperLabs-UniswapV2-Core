#![no_main]
#![no_std]

extern crate alloc;

use alloc::{collections::BTreeSet, format, vec, vec::Vec};
use alloc::prelude::v1::Box;
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, CLTyped, CLValue, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints,
    Group, Key, Parameter, RuntimeArgs, URef, U256, ContractHash,CLType
};
use contract_utils::{ContractContext, OnChainContractStorage};
use factory::{self, FACTORY};

#[derive(Default)]
struct Factory(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for Factory {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl FACTORY<OnChainContractStorage> for Factory {}

impl Factory {
    fn constructor(&mut self, fee_to: Key, fee_to_setter: Key, all_pairs: Vec<Key>, contract_hash: ContractHash) {
        FACTORY::init(self, fee_to, fee_to_setter, all_pairs, Key::from(contract_hash));
    }
}

#[no_mangle]
fn constructor() {
    let fee_to: Key = runtime::get_named_arg("fee_to");
    let fee_to_setter: Key = runtime::get_named_arg("fee_to_setter");
    let all_pairs: Vec<Key> = runtime::get_named_arg("all_pairs");
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    Factory::default().constructor(fee_to, fee_to_setter, all_pairs, contract_hash);
}



#[no_mangle]
fn fee_to() {
    let ret: Key = Factory::default().get_fee_to();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn fee_to_setter() {
    let ret: Key = Factory::default().get_fee_to_setter();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn all_pairs() {
    let ret: Vec<Key> = Factory::default().get_all_pairs();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn all_pairs_length() {
    let ret: Vec<Key> = Factory::default().get_all_pairs();
    runtime::ret(CLValue::from_t(U256::from(ret.len())).unwrap_or_revert());
}

#[no_mangle]
fn set_fee_to() {
    let fee_to: Key = runtime::get_named_arg("fee_to");
    Factory::default().set_fee_to(fee_to);
    
}

#[no_mangle]
fn set_fee_to_setter() {
    let fee_to_setter: Key = runtime::get_named_arg("fee_to_setter");
    Factory::default().set_fee_to_setter(fee_to_setter);
}

#[no_mangle]
fn create_pair() {
    let token_a: Key = runtime::get_named_arg("token_a");
    let token_b: Key = runtime::get_named_arg("token_b");
    Factory::default().create_pair(token_a,token_b);
}

#[no_mangle]
fn pair() {
    let token0: Key = runtime::get_named_arg("token0");
    let token1: Key = runtime::get_named_arg("token1");
    let ret: Key = Factory::default().pair(token0, token1);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("fee_to", Key::cl_type()),
            Parameter::new("fee_to_setter", Key::cl_type()),
            Parameter::new("all_pairs", CLType::List(Box::new(Key::cl_type()))),
            Parameter::new("contract_hash", ContractHash::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "create_pair",
        vec![
            Parameter::new("token_a", Key::cl_type()),
            Parameter::new("token_b", Key::cl_type()),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "pair",
        vec![
            Parameter::new("token0", Key::cl_type()),
            Parameter::new("token1", Key::cl_type()),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "fee_to",
        vec![],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "fee_to_setter",
        vec![],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "all_pairs",
        vec![],
        CLType::List(Box::new(Key::cl_type())),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "all_pairs_length",
        vec![],
        CLType::U256,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "set_fee_to",
        vec![Parameter::new("fee_to", Key::cl_type())],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "set_fee_to_setter",
        vec![Parameter::new("fee_to_setter", Key::cl_type())],
        CLType::Unit,
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

    let fee_to: Key = runtime::get_named_arg("fee_to");
    let fee_to_setter: Key = runtime::get_named_arg("fee_to_setter");
    let all_pairs: Vec<Key> = Vec::new();
   
    // Prepare constructor args
    let constructor_args = runtime_args! {
        "fee_to" => fee_to,
        "fee_to_setter" => fee_to_setter,
        "all_pairs" => all_pairs,
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
