use std::collections::BTreeMap;

use crate::data::WcsprEvents;
use casper_contract::{
    contract_api::{runtime, storage, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{ContractHash, ContractPackageHash, URef, U256, U512};
use casperlabs_contract_utils::{ContractContext, ContractStorage};
use casperlabs_erc20::{data::get_package_hash, Address, ERC20};
use common::{
    errors::Errors,
    functions::{get_purse, set_purse},
};
use num_traits::cast::AsPrimitive;

pub trait WCSPR<Storage: ContractStorage>: ContractContext<Storage> + ERC20<Storage> {
    fn init(&self, contract_hash: ContractHash, package_hash: ContractPackageHash, purse: URef) {
        ERC20::init(self, contract_hash, package_hash);
        set_purse(purse);
    }

    fn deposit(&self, amount: U512, purse: URef) -> Result<(), u32> {
        if amount.is_zero() {
            return Err(5); // Amount to transfer is 0
        }
        if amount > <casper_types::U256 as AsPrimitive<casper_types::U512>>::as_(U256::MAX) {
            runtime::revert(Errors::UniswapV2CoreWCSPROverFlow1);
        }
        if system::get_purse_balance(purse).unwrap_or_revert()
            > <casper_types::U256 as AsPrimitive<casper_types::U512>>::as_(U256::MAX)
        {
            runtime::revert(Errors::UniswapV2CoreWCSPROverFlow2);
        }
        // transfers native cspr from source purse to destination purse
        system::transfer_from_purse_to_purse(purse, get_purse(), amount, None).unwrap_or_revert();
        // mint wcspr for the caller
        self.mint(
            Address::from(self.get_caller()),
            <casper_types::U512 as AsPrimitive<casper_types::U256>>::as_(amount),
        )
        .unwrap_or_revert();
        self.emit(&WcsprEvents::Deposit { purse, amount });
        Ok(())
    }

    fn withdraw(&self, amount: U512, purse: URef) -> Result<(), u32> {
        if amount.is_zero() {
            return Err(5); // Amount to transfer is 0
        }
        if amount > <casper_types::U256 as AsPrimitive<casper_types::U512>>::as_(U256::MAX) {
            runtime::revert(Errors::UniswapV2CoreWCSPROverFlow3);
        }
        // transfer native cspr from purse to account
        system::transfer_from_purse_to_purse(get_purse(), purse, amount, None).unwrap_or_revert();
        // burn wcspr for the caller
        self.burn(
            Address::from(self.get_caller()),
            <casper_types::U512 as AsPrimitive<casper_types::U256>>::as_(amount),
        )
        .unwrap_or_revert();
        self.emit(&WcsprEvents::Withdraw { purse, amount });
        Ok(())
    }

    // Events
    fn emit(&self, wcspr_event: &WcsprEvents) {
        let mut events = Vec::new();
        let formatted_package_hash = get_package_hash().to_formatted_string();
        let package_hash_arr: Vec<&str> = formatted_package_hash.split('-').collect();
        let package_hash: String = package_hash_arr[1].to_string();
        match wcspr_event {
            WcsprEvents::Deposit { purse, amount } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert("event_type", wcspr_event.type_name());
                event.insert("purse", purse.to_string());
                event.insert("amount", amount.to_string());
                events.push(event);
            }
            WcsprEvents::Withdraw { purse, amount } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert("event_type", wcspr_event.type_name());
                event.insert("purse", purse.to_string());
                event.insert("amount", amount.to_string());
                events.push(event);
            }
        };
        for event in events {
            let _: URef = storage::new_uref(event);
        }
    }
}
