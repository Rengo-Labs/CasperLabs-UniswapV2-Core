use alloc::string::String;
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{Key, U256, URef};
use contract_utils::{get_key, set_key, Dict};
//use casper_contract::{value::account::PurseId ,contract_api::{runtime,system}, unwrap_or_revert::UnwrapOrRevert};

pub const BALANCES_DICT: &str = "balances";
pub const ALLOWANCES_DICT: &str = "allowances";
pub const NAME: &str = "name";
pub const SYMBOL: &str = "symbol";
pub const SELF_CONTRACT_HASH: &str = "self_contract_hash";
pub const MAIN_PURSE: &str = "main_purse";

pub struct Balances {
    dict: Dict,
}

impl Balances {
    pub fn instance() -> Balances {
        Balances {
            dict: Dict::instance(BALANCES_DICT),
        }
    }

    pub fn init() {
        Dict::init(BALANCES_DICT)
    }

    pub fn get(&self, owner: &Key) -> U256 {
        self.dict.get_by_key(owner).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, value: U256) {
        self.dict.set_by_key(owner, value);
    }
}

pub struct Allowances {
    dict: Dict,
}

impl Allowances {
    pub fn instance() -> Allowances {
        Allowances {
            dict: Dict::instance(ALLOWANCES_DICT),
        }
    }

    pub fn init() {
        Dict::init(ALLOWANCES_DICT)
    }

    pub fn get(&self, owner: &Key, spender: &Key) -> U256 {
        self.dict.get_by_keys((owner, spender)).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, spender: &Key, value: U256) {
        self.dict.set_by_keys((owner, spender), value);
    }
}

pub fn name() -> String {
    get_key(NAME).unwrap_or_revert()
}

pub fn set_name(name: String) {
    set_key(NAME, name);
}

pub fn symbol() -> String {
    get_key(SYMBOL).unwrap_or_revert()
}

pub fn set_symbol(symbol: String) {
    set_key(SYMBOL, symbol);
}

pub fn set_hash(contract_hash: Key) {
    set_key(SELF_CONTRACT_HASH, contract_hash);
}

pub fn get_hash() -> Key {
    get_key(SELF_CONTRACT_HASH).unwrap_or_revert()
}

pub fn set_main_purse(purse: URef) {
    set_key(MAIN_PURSE, Key::from(purse));
}


pub fn get_main_purse() -> URef {
    let contract_main_purse_key: Key = get_key(MAIN_PURSE).unwrap_or_revert();
    let contract_main_purse = contract_main_purse_key.as_uref().unwrap_or_revert();
    *contract_main_purse
}
