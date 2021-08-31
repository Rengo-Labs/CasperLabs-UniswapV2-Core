
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{Key};
use contract_utils::{get_key, set_key};

pub const SELF_CONTRACT_HASH: &str = "self_contract_hash";
pub const WETH: &str = "weth";
pub const DAI: &str = "dai";
pub const ETH: &str = "eth";
pub const PERMISSIONED_PAIR_ADDRESS: &str = "permissioned_pair_address";
pub const UNISWAP_V2_FACTROY: &str = "uniswap_v2_factory";



pub fn set_weth(weth: Key){
    set_key(WETH, weth);
}
pub fn get_weth()-> Key {
    get_key(WETH).unwrap_or_revert()
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



pub fn set_eth(eth: Key){
    set_key(ETH, eth);
}
pub fn get_eth()-> Key {
    get_key(ETH).unwrap_or_revert()
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
