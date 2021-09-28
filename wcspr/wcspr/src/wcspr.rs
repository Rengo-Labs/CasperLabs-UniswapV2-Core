use crate::data::{self, Allowances, Balances};
use alloc::string::String;
use casper_contract::contract_api::runtime;
use casper_types::system::mint::Error as MintError;
use casper_types::{Key, U256};
use contract_utils::{ContractContext, ContractStorage};

pub trait WCSPR<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(&mut self, name: String, symbol: String, decimals: u8, contract_hash: Key) {
        data::set_name(name);
        data::set_symbol(symbol);
        data::set_decimals(decimals);
        data::set_hash(contract_hash);
        Balances::init();
        Allowances::init();
    }

    fn balance_of(&mut self, owner: Key) -> U256 {
        Balances::instance().get(&owner)
    }

    fn transfer(&mut self, recipient: Key, amount: U256) {
        self.make_transfer(self.get_caller(), recipient, amount);
    }

    fn approve(&mut self, spender: Key, amount: U256) {
        Allowances::instance().set(&self.get_caller(), &spender, amount);
    }

    fn allowance(&mut self, owner: Key, spender: Key) -> U256 {
        Allowances::instance().get(&owner, &spender)
    }

    fn transfer_from(&mut self, owner: Key, recipient: Key, amount: U256) {
        let allowances = Allowances::instance();
        let spender = self.get_caller();
        let spender_allowance = allowances.get(&owner, &spender);
        allowances.set(&owner, &spender, spender_allowance - amount);
        self.make_transfer(owner, recipient, amount);
    }

    fn deposit(&mut self, recipient: Key, amount: U256) {
        let balances = Balances::instance();
        let balance = balances.get(&recipient);
        balances.set(&recipient, balance + amount);
        data::set_total_supply(data::total_supply() + amount);
    }

    fn withdraw(&mut self, recipient: Key, amount: U256) {
        let balances = Balances::instance();
        let balance = balances.get(&recipient);
        if balance >= amount {
            balances.set(&recipient, balance - amount);
            data::set_total_supply(data::total_supply() - amount);
        } else {
            runtime::revert(MintError::InsufficientFunds)
        }
    }

    fn make_transfer(&mut self, sender: Key, recipient: Key, amount: U256) {
        let balances = Balances::instance();
        let sender_balance = balances.get(&sender);
        let recipient_balance = balances.get(&recipient);
        balances.set(&sender, sender_balance - amount);
        balances.set(&recipient, recipient_balance + amount);
    }

    fn total_supply(&mut self) -> U256 {
        data::total_supply()
    }

    fn name(&mut self) -> String {
        data::name()
    }

    fn symbol(&mut self) -> String {
        data::symbol()
    }
}
