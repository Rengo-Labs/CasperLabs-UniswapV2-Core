use casper_contract::contract_api::runtime;
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{ApiError, Key, URef};
use contract_utils::{get_key, set_key};

pub const SELF_CONTRACT_HASH: &str = "self_contract_hash";
pub const WCSPR: &str = "wcspr";
pub const DAI: &str = "dai";
pub const BTC: &str = "btc";
pub const CSPR: &str = "cspr";
pub const PERMISSIONED_PAIR_ADDRESS: &str = "permissioned_pair_address";
pub const UNISWAP_V2_FACTROY: &str = "uniswap_v2_factory";
pub const UNISWAP_V2_PAIR: &str = "uniswap_v2_pair";
pub const SELF_PURSE: &str = "self_purse";
pub const CONTRACT_PACKAGE_HASH: &str = "contract_package_hash";

#[repr(u16)]
pub enum ErrorCodes {
    Abort = 35,
}

pub fn set_wcspr(wcspr: Key) {
    set_key(WCSPR, wcspr);
}

pub fn get_wcspr() -> Key {
    get_key(WCSPR).unwrap_or_revert()
}

pub fn set_dai(dai: Key) {
    set_key(DAI, dai);
}

pub fn get_dai() -> Key {
    get_key(DAI).unwrap_or_revert()
}

pub fn set_permissioned_pair_address(permissioned_pair_address: Key) {
    set_key(PERMISSIONED_PAIR_ADDRESS, permissioned_pair_address);
}

pub fn get_permissioned_pair_address() -> Key {
    get_key(PERMISSIONED_PAIR_ADDRESS).unwrap_or_revert()
}

pub fn set_cspr(cspr: Key) {
    set_key(CSPR, cspr);
}

pub fn get_cspr() -> Key {
    get_key(CSPR).unwrap_or_revert()
}

pub fn set_uniswap_v2_factory(uniswap_v2_factory: Key) {
    set_key(UNISWAP_V2_FACTROY, uniswap_v2_factory)
}

pub fn get_uniswap_v2_factory() -> Key {
    get_key(UNISWAP_V2_FACTROY).unwrap_or_revert()
}

pub fn set_uniswap_v2_pair(uniswap_v2_pair: Key) {
    set_key(UNISWAP_V2_PAIR, uniswap_v2_pair)
}

pub fn get_uniswap_v2_pair() -> Key {
    get_key(UNISWAP_V2_PAIR).unwrap_or_revert()
}

pub fn set_hash(contract_hash: Key) {
    set_key(SELF_CONTRACT_HASH, contract_hash);
}

pub fn get_hash() -> Key {
    get_key(SELF_CONTRACT_HASH).unwrap_or_revert()
}

// contract purse
pub fn set_self_purse(purse: URef) {
    runtime::put_key(&SELF_PURSE, purse.into());
}

pub fn get_self_purse() -> URef {
    let destination_purse_key = runtime::get_key(&SELF_PURSE).unwrap_or_revert();

    match destination_purse_key.as_uref() {
        Some(uref) => *uref,
        None => runtime::revert(ApiError::User(ErrorCodes::Abort as u16)),
    }
}

pub fn set_package_hash(package_hash: Key) {
    set_key(CONTRACT_PACKAGE_HASH, package_hash);
}

pub fn get_package_hash() -> Key {
    get_key(CONTRACT_PACKAGE_HASH).unwrap_or_revert()
}
