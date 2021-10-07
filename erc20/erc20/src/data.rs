use alloc::string::String;

use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{Key, U256};
use contract_utils::{get_key, set_key, Dict};

pub const BALANCES_DICT: &str = "balances";
pub const NONCES_DICT: &str = "nonces";
pub const ALLOWANCES_DICT: &str = "allowances";
pub const NAME: &str = "name";
pub const SYMBOL: &str = "symbol";
pub const DECIMALS: &str = "decimals";
pub const TOTAL_SUPPLY: &str = "total_supply";
pub const SELF_CONTRACT_HASH: &str = "self_contract_hash";
pub const DOMAIN_SEPARATOR: &str = "domain_separator";
pub const PERMIT_TYPE_HASH: &str = "permit_type_hash";

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

pub struct Nonces {
    dict: Dict,
}

impl Nonces {
    pub fn instance() -> Nonces {
        Nonces {
            dict: Dict::instance(NONCES_DICT),
        }
    }

    pub fn init() {
        Dict::init(NONCES_DICT)
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

pub fn decimals() -> u8 {
    get_key(DECIMALS).unwrap_or_revert()
}

pub fn set_decimals(decimals: u8) {
    set_key(DECIMALS, decimals);
}

pub fn total_supply() -> U256 {
    get_key(TOTAL_SUPPLY).unwrap_or_default()
}

pub fn set_total_supply(total_supply: U256) {
    set_key(TOTAL_SUPPLY, total_supply);
}

pub fn set_hash(contract_hash: Key) {
    set_key(SELF_CONTRACT_HASH, contract_hash);
}

pub fn get_hash() -> Key {
    get_key(SELF_CONTRACT_HASH).unwrap_or_revert()
}

pub fn set_domain_separator(domain_separator: String) {
    set_key(DOMAIN_SEPARATOR, domain_separator);
}

pub fn get_domain_separator() -> String {
    get_key(DOMAIN_SEPARATOR).unwrap_or_revert()
}

pub fn set_permit_type_hash(permit_type_hash: String) {
    set_key(PERMIT_TYPE_HASH, permit_type_hash);
}

pub fn get_permit_type_hash() -> String {
    get_key(PERMIT_TYPE_HASH).unwrap_or_revert()
}
