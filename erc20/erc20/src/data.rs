use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};
use casper_contract::{
    contract_api::{runtime::get_call_stack, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{system::CallStackElement, ContractPackageHash, Key, URef, U256};
use contract_utils::{get_key, key_to_str, set_key, Dict};

use crate::event::ERC20Event;

const BALANCES_DICT: &str = "balances";
pub const NONCES_DICT: &str = "nonces";
pub const ALLOWANCES_DICT: &str = "allowances";

pub const NAME: &str = "name";
// pub const META: &str = "meta";
pub const SYMBOL: &str = "symbol";
pub const DECIMALS: &str = "decimals";
pub const TOTAL_SUPPLY: &str = "total_supply";
pub const SELF_CONTRACT_HASH: &str = "self_contract_hash";
pub const DOMAIN_SEPARATOR: &str = "domain_separator";
pub const PERMIT_TYPE_HASH: &str = "permit_type_hash";
pub const CONTRACT_PACKAGE_HASH: &str = "contract_package_hash";

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
        self.dict.get(&key_to_str(owner)).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, value: U256) {
        self.dict.set(&key_to_str(owner), value);
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
        self.dict.get(&key_to_str(owner)).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, value: U256) {
        self.dict.set(&key_to_str(owner), value);
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
pub fn set_hash(contract_hash: Key) {
    set_key(SELF_CONTRACT_HASH, contract_hash);
}

pub fn get_hash() -> Key {
    get_key(SELF_CONTRACT_HASH).unwrap_or_revert()
}
pub fn set_package_hash(package_hash: ContractPackageHash) {
    set_key(CONTRACT_PACKAGE_HASH, package_hash);
}

pub fn get_package_hash() -> ContractPackageHash {
    get_key(CONTRACT_PACKAGE_HASH).unwrap_or_revert()
}

pub fn contract_package_hash() -> ContractPackageHash {
    let call_stacks = get_call_stack();
    let last_entry = call_stacks.last().unwrap_or_revert();
    let package_hash: Option<ContractPackageHash> = match last_entry {
        CallStackElement::StoredContract {
            contract_package_hash,
            contract_hash: _,
        } => Some(*contract_package_hash),
        _ => None,
    };
    package_hash.unwrap_or_revert()
}

pub fn emit(event: &ERC20Event) {
    let mut events = Vec::new();
    let package = contract_package_hash();
    match event {
        ERC20Event::Mint {
            recipient,
            token_ids,
        } => {
            for token_id in token_ids {
                let mut param = BTreeMap::new();
                param.insert(CONTRACT_PACKAGE_HASH, package.to_string());
                param.insert("event_type", "erc20_mint_remove_one".to_string());
                param.insert("recipient", recipient.to_string());
                param.insert("token_id", token_id.to_string());
                events.push(param);
            }
        }
        ERC20Event::Burn { owner, token_ids } => {
            for token_id in token_ids {
                let mut param = BTreeMap::new();
                param.insert(CONTRACT_PACKAGE_HASH, package.to_string());
                param.insert("event_type", "erc20_burn_remove_one".to_string());
                param.insert("owner", owner.to_string());
                param.insert("token_id", token_id.to_string());
                events.push(param);
            }
        }
        ERC20Event::Approve {
            owner,
            spender,
            token_ids,
        } => {
            for token_id in token_ids {
                let mut param = BTreeMap::new();
                param.insert(CONTRACT_PACKAGE_HASH, package.to_string());
                param.insert("event_type", "erc20_approve_token".to_string());
                param.insert("owner", owner.to_string());
                param.insert("spender", spender.to_string());
                param.insert("token_id", token_id.to_string());
                events.push(param);
            }
        }
        ERC20Event::Transfer {
            sender,
            recipient,
            token_ids,
        } => {
            for token_id in token_ids {
                let mut param = BTreeMap::new();
                param.insert(CONTRACT_PACKAGE_HASH, package.to_string());
                param.insert("event_type", "erc20_transfer_token".to_string());
                param.insert("sender", sender.to_string());
                param.insert("recipient", recipient.to_string());
                param.insert("token_id", token_id.to_string());
                events.push(param);
            }
        }
        ERC20Event::MetadataUpdate { token_id } => {
            let mut param = BTreeMap::new();
            param.insert(CONTRACT_PACKAGE_HASH, package.to_string());
            param.insert("event_type", "erc20_metadata_update".to_string());
            param.insert("token_id", token_id.to_string());
            events.push(param);
        }
    };
    for param in events {
        let _: URef = storage::new_uref(param);
    }
}
