use crate::alloc::string::ToString;
use crate::data::{self, Error, WcsprEvents};
use alloc::string::String;
use alloc::{collections::BTreeMap, vec::Vec};
use casper_contract::{
    contract_api::{runtime, storage, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{ContractHash, ContractPackageHash, URef, U256, U512};
use casperlabs_contract_utils::{ContractContext, ContractStorage};
use casperlabs_erc20::data::{get_package_hash, set_contract_hash, set_package_hash};
use casperlabs_erc20::{Address, ERC20};
use num_traits::cast::AsPrimitive;

pub trait WCSPR<Storage: ContractStorage>: ContractContext<Storage> + ERC20<Storage> {
    fn init(&self, contract_hash: ContractHash, package_hash: ContractPackageHash, purse: URef) {
        set_contract_hash(contract_hash);
        set_package_hash(package_hash);
        data::set_self_purse(purse);
    }

    fn deposit(&mut self, amount: U512, purse: URef) -> Result<(), u32> {
        if amount.is_zero() {
            return Err(5); // Amount to transfer is 0
        }
        if amount
            > U512::from(<casper_types::U256 as AsPrimitive<casper_types::U512>>::as_(U256::MAX))
        {
            runtime::revert(Error::UniswapV2CoreWCSPROverFlow1);
        }
        if system::get_purse_balance(purse).unwrap_or_revert()
            > U512::from(<casper_types::U256 as AsPrimitive<casper_types::U512>>::as_(U256::MAX))
        {
            runtime::revert(Error::UniswapV2CoreWCSPROverFlow2);
        }
        // transfers native cspr from source purse to destination purse
        let _ = system::transfer_from_purse_to_purse(purse, data::get_self_purse(), amount, None)
            .unwrap_or_revert();
        // mint wcspr for the caller
        self.mint(
            Address::from(self.get_caller()),
            U256::from(<casper_types::U512 as AsPrimitive<casper_types::U256>>::as_(amount)),
        )
        .unwrap_or_revert();
        self.emit(&WcsprEvents::Deposit { purse, amount });
        Ok(())
    }

    fn withdraw(&mut self, amount: U512, purse: URef) -> Result<(), u32> {
        if amount.is_zero() {
            return Err(5); // Amount to transfer is 0
        }
        if amount
            > U512::from(<casper_types::U256 as AsPrimitive<casper_types::U512>>::as_(U256::MAX))
        {
            runtime::revert(Error::UniswapV2CoreWCSPROverFlow3);
        }
        // transfer native cspr from purse to account
        system::transfer_from_purse_to_purse(data::get_self_purse(), purse, amount, None)
            .unwrap_or_revert();
        // burn wcspr for the caller
        self.burn(
            Address::from(self.get_caller()),
            U256::from(<casper_types::U512 as AsPrimitive<casper_types::U256>>::as_(amount)),
        )
        .unwrap_or_revert();
        self.emit(&WcsprEvents::Withdraw { purse, amount });
        Ok(())
    }

    // Events
    fn emit(&mut self, wcspr_event: &WcsprEvents) {
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
