
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{Key};
use contract_utils::{get_key, set_key};

pub const SELF_CONTRACT_HASH: &str = "self_contract_hash";
pub const WCSPR: &str = "wcspr";
pub const DAI: &str = "dai";
pub const CSPR: &str = "cspr";
pub const PERMISSIONED_PAIR_ADDRESS: &str = "permissioned_pair_address";
pub const UNISWAP_V2_FACTROY: &str = "uniswap_v2_factory";



pub fn set_wcspr(wcspr: Key){
    set_key(WCSPR, wcspr);
}
pub fn get_wcspr()-> Key {
    get_key(WCSPR).unwrap_or_revert()
}

pub fn set_dai(dai: Key){
    set_key(DAI, dai);
}
pub fn get_dai()-> Key {
    get_key(DAI).unwrap_or_revert()
}

pub fn set_permissioned_pair_address(permissioned_pair_address: Key){
    set_key(PERMISSIONED_PAIR_ADDRESS, permissioned_pair_address);
}
pub fn get_permissioned_pair_address()-> Key {
    get_key(PERMISSIONED_PAIR_ADDRESS).unwrap_or_revert()
}



pub fn set_cspr(cspr: Key){
    set_key(CSPR, cspr);
}
pub fn get_cspr()-> Key {
    get_key(CSPR).unwrap_or_revert()
}

pub fn set_uniswap_v2_factory(uniswap_v2_factory: Key){
    set_key(UNISWAP_V2_FACTROY, uniswap_v2_factory)
}
pub fn get_uniswap_v2_factory()-> Key{
    get_key(UNISWAP_V2_FACTROY).unwrap_or_revert()
}

pub fn set_hash(contract_hash: Key){
    set_key(SELF_CONTRACT_HASH, contract_hash);
}

pub fn get_hash()-> Key {
    get_key(SELF_CONTRACT_HASH).unwrap_or_revert()
}
