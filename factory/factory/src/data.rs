use alloc::{vec::Vec};
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{Key};
use contract_utils::{get_key, set_key, Dict};

pub const PAIRS_DICT: &str = "pairs";
pub const SELF_CONTRACT_HASH: &str = "self_contract_hash";
pub const FEE_TO: &str = "fee_to";
pub const FEE_TO_SETTER: &str = "fee_to_setter";
pub const ALL_PAIRS: &str = "all_pairs";
pub const TOKEN0: &str = "token0";
pub const TOKEN1: &str = "token1";

pub struct Pairs {
    dict: Dict,
}

impl Pairs {
    pub fn instance() -> Pairs {
        Pairs {
            dict: Dict::instance(PAIRS_DICT),
        }
    }

    pub fn init() {
        Dict::init(PAIRS_DICT)
    }

    pub fn get(&self, token0: &Key, token1: &Key) -> Key {
        match self.dict.get_by_keys((token0, token1)){
            Some(pair) => pair,
            None => Key::from_formatted_str("hash-0000000000000000000000000000000000000000000000000000000000000000").unwrap()
        }
    }

    pub fn set(&self, token0: &Key, token1: &Key, value: Key) {
        self.dict.set_by_keys((token0, token1), value);
    }
}

pub fn set_hash(contract_hash: Key){
    set_key(SELF_CONTRACT_HASH, contract_hash);
}

pub fn get_hash()-> Key {
    get_key(SELF_CONTRACT_HASH).unwrap_or_revert()
}

pub fn set_fee_to(fee_to: Key){
    set_key(FEE_TO, fee_to);
}

pub fn get_fee_to() -> Key {
    match get_key(FEE_TO){
        Some(fee_to) => fee_to,
        None => Key::from_formatted_str("account-hash-0000000000000000000000000000000000000000000000000000000000000000").unwrap()
    }
}

pub fn set_fee_to_setter(fee_to_setter: Key){
    set_key(FEE_TO_SETTER, fee_to_setter);
}

pub fn get_fee_to_setter() -> Key {
    match get_key(FEE_TO_SETTER){
        Some(fee_to_setter) => fee_to_setter,
        None => Key::from_formatted_str("account-hash-0000000000000000000000000000000000000000000000000000000000000000").unwrap()
    }
}

pub fn set_all_pairs(all_pairs: Vec<Key>) {
    set_key(ALL_PAIRS,all_pairs);
}

pub fn get_all_pairs() -> Vec<Key> {
    get_key(ALL_PAIRS).unwrap_or_revert()
}