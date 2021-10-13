use crate::data::{self, Allowances, Balances};
use alloc::string::String;
use casper_contract::{contract_api::{system}, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{Key, U256, U512, URef};
use contract_utils::{ContractContext, ContractStorage};

pub trait WCSPR<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(&mut self, name: String, symbol: String, contract_hash: Key) {
        data::set_name(name);
        data::set_symbol(symbol);
        data::set_hash(contract_hash);
        Balances::init();
        Allowances::init();

        data::set_main_purse(system::create_purse());
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

    fn deposit(&mut self, to: Key, purse: URef) 
    {
        let cspr_amount: U512 = system::get_purse_balance(purse).unwrap_or_revert();            // get amount of cspr from purse received
        let cspr_amount_u256: U256 = U256::from(cspr_amount.as_u128());                         // convert amount to U256
        let contract_main_purse: URef = data::get_main_purse();                                 // get this contract's purse

        // save received cspr
        let _ = system::transfer_from_purse_to_purse(purse, contract_main_purse, cspr_amount, None);    // transfers native cspr from source purse to destination purse

        // mint wcspr for the 'to' account
        let balances = Balances::instance();
        let balance = balances.get(&to);
        balances.set(&to, balance + cspr_amount_u256);
    }

    fn withdraw(&mut self, recipient: Key, amount: U512) {

        let balances = Balances::instance();
        let balance = balances.get(&recipient);                         // get balance of the receipent
        let cspr_amount_u256: U256 = U256::from(amount.as_u128());      // convert U512 to U256

        let contract_main_purse = data::get_main_purse();
        let main_purse_balance : U512 = system::get_purse_balance(contract_main_purse).unwrap_or_revert();


        if balance >= cspr_amount_u256 && amount <= main_purse_balance.into() {
            system::transfer_from_purse_to_account(                     // transfer native cspr from purse to account
                contract_main_purse, 
                recipient.into_account().unwrap_or_revert(), 
                amount, 
                None
            ).unwrap_or_revert();

            balances.set(&recipient, balance - cspr_amount_u256)
        }
    }

    fn make_transfer(&mut self, sender: Key, recipient: Key, amount: U256) {
        let balances = Balances::instance();
        let sender_balance = balances.get(&sender);
        let recipient_balance = balances.get(&recipient);
        balances.set(&sender, sender_balance - amount);
        balances.set(&recipient, recipient_balance + amount);
    }

    fn name(&mut self) -> String {
        data::name()
    }

    fn symbol(&mut self) -> String {
        data::symbol()
    }
}
