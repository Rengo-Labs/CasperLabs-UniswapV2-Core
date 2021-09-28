use alloc::string::String;

use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::Key;
use contract_utils::{get_key, set_key};

pub const NAME: &str = "name";
pub const SELF_CONTRACT_HASH: &str = "self_contract_hash";

pub fn name() -> String {
    get_key(NAME).unwrap_or_revert()
}

pub fn set_name(name: String) {
    set_key(NAME, name);
}

pub fn set_hash(contract_hash: Key) {
    set_key(SELF_CONTRACT_HASH, contract_hash);
}

pub fn get_hash() -> Key {
    get_key(SELF_CONTRACT_HASH).unwrap_or_revert()
}
