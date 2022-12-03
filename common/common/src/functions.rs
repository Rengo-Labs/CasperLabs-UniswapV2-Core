use crate::keys::*;
use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{ApiError, ContractHash, ContractPackageHash, Key, URef};
use casperlabs_contract_utils::{get_key, set_key};

pub fn zero_address() -> Key {
    Key::from_formatted_str("hash-0000000000000000000000000000000000000000000000000000000000000000")
        .unwrap()
}

pub fn account_zero_address() -> Key {
    Key::from_formatted_str(
        "account-hash-0000000000000000000000000000000000000000000000000000000000000000",
    )
    .unwrap()
}

pub fn block_timestamp() -> u64 {
    runtime::get_blocktime().into()
}

pub fn set_contract_hash(contract_hash: ContractHash) {
    set_key(SELF_CONTRACT_HASH, contract_hash);
}

pub fn get_contract_hash() -> ContractHash {
    get_key(SELF_CONTRACT_HASH).unwrap_or_default()
}

pub fn set_package_hash(package_hash: ContractPackageHash) {
    set_key(CONTRACT_PACKAGE_HASH, package_hash);
}

pub fn get_package_hash() -> ContractPackageHash {
    get_key(CONTRACT_PACKAGE_HASH).unwrap_or_default()
}

pub fn set_purse(purse: URef) {
    runtime::put_key(PURSE, purse.into());
}

pub fn get_purse() -> URef {
    match runtime::get_key(PURSE).unwrap_or_revert().as_uref() {
        Some(uref) => *uref,
        None => runtime::revert(ApiError::PurseNotCreated),
    }
}
