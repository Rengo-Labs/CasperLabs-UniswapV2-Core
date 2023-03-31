use crate::data::get_package_hash;
use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
};
use casper_contract::contract_api::storage;
use casper_types::{Key, U256};

pub enum ERC20Event {
    Approval {
        owner: Key,
        spender: Key,
        value: U256,
    },
    Transfer {
        from: Key,
        to: Key,
        value: U256,
    },
}

impl ERC20Event {
    pub fn type_name(&self) -> String {
        match self {
            ERC20Event::Approval {
                owner: _,
                spender: _,
                value: _,
            } => "approve",
            ERC20Event::Transfer {
                from: _,
                to: _,
                value: _,
            } => "transfer",
        }
        .to_string()
    }
}

pub fn emit(erc20_event: &ERC20Event) {
    match erc20_event {
        ERC20Event::Approval {
            owner,
            spender,
            value,
        } => {
            let mut event = BTreeMap::new();
            event.insert("contract_package_hash", get_package_hash().to_string());
            event.insert("event_type", erc20_event.type_name());
            event.insert("owner", owner.to_string());
            event.insert("spender", spender.to_string());
            event.insert("value", value.to_string());
            storage::new_uref(event)
        }
        ERC20Event::Transfer { from, to, value } => {
            let mut event = BTreeMap::new();
            event.insert("contract_package_hash", get_package_hash().to_string());
            event.insert("event_type", erc20_event.type_name());
            event.insert("from", from.to_string());
            event.insert("to", to.to_string());
            event.insert("value", value.to_string());
            storage::new_uref(event)
        }
    };
}
