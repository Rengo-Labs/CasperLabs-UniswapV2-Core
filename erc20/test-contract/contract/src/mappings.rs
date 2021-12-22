use core::convert::TryInto;

use alloc::{format, string::String};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};

use casper_types::{
    bytesrepr::{FromBytes, ToBytes},
    CLTyped,
};

pub fn get_key<T: FromBytes + CLTyped + Default>(name: &str) -> T {
    match runtime::get_key(name) {
        None => Default::default(),
        Some(value) => {
            let key = value.try_into().unwrap_or_revert();
            storage::read(key).unwrap_or_revert().unwrap_or_revert()
        }
    }
}

pub fn set_key<T: ToBytes + CLTyped>(name: &str, value: T) {
    match runtime::get_key(name) {
        Some(key) => {
            let key_ref = key.try_into().unwrap_or_revert();
            storage::write(key_ref, value);
        }
        None => {
            let key = storage::new_uref(value).into();
            runtime::put_key(name, key);
        }
    }
}

pub fn self_hash_key() -> String {
    format!("self_hash")
}

pub fn self_package_key() -> String {
    format!("package_hash")
}

pub fn erc20_key() -> String {
    format!("erc20")
}

pub fn transfer_key() -> String {
    format!("transfer_result")
}

pub fn transfer_from_key() -> String {
    format!("transfer_from_result")
}

pub fn allowance() -> String {
    format!("allowance")
}

pub fn increase_allowance_key() -> String {
    format!("increase_allowance_result")
}

pub fn decrease_allowance_key() -> String {
    format!("decrease_allowance_result")
}
