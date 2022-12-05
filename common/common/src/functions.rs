use crate::keys::*;
use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{ApiError, Key, URef, U256, U512};
use num_traits::AsPrimitive;

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

pub fn u256_to_u512(u256: U256) -> U512 {
    <casper_types::U256 as AsPrimitive<casper_types::U512>>::as_(u256)
}

pub fn u512_to_u256(u512: U512) -> U256 {
    <casper_types::U512 as AsPrimitive<casper_types::U256>>::as_(u512)
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
