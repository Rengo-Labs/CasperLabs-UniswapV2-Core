use std::collections::BTreeMap;

use crate::data::WcsprEvents;
use common::{
    contract_api::{runtime, storage, system},
    errors::Errors,
    functions::{get_purse, set_purse, u256_to_u512, u512_to_u256},
    unwrap_or_revert::UnwrapOrRevert,
    *,
};
use uniswap_erc20::{data::get_package_hash, Address, ERC20};

pub trait WCSPR<Storage: ContractStorage>: ContractContext<Storage> + ERC20<Storage> {
    fn init(&self, contract_hash: ContractHash, package_hash: ContractPackageHash) {
        ERC20::init(self, contract_hash, package_hash);
        
        let purse: URef = system::create_purse();

        set_purse(purse);
    }

    fn deposit(&self, amount: U512, purse: URef) -> Result<(), u32> {
        if amount.is_zero() {
            return Err(5); // Amount to transfer is 0
        }
        if amount > u256_to_u512(U256::MAX) {
            runtime::revert(Errors::UniswapV2CoreWCSPROverFlow1);
        }
        if system::get_purse_balance(purse).unwrap_or_revert() > u256_to_u512(U256::MAX) {
            runtime::revert(Errors::UniswapV2CoreWCSPROverFlow2);
        }
        // transfers native cspr from source purse to destination purse
        system::transfer_from_purse_to_purse(purse, get_purse().into_add(), amount, None).unwrap_or_revert();
        // mint wcspr for the caller
        self.mint(Address::from(self.get_caller()), u512_to_u256(amount))
            .unwrap_or_revert();
        self.emit(&WcsprEvents::Deposit { purse, amount });
        Ok(())
    }

    fn withdraw(&self, amount: U512, purse: URef) -> Result<(), u32> {
        if amount.is_zero() {
            return Err(5); // Amount to transfer is 0
        }
        if amount > u256_to_u512(U256::MAX) {
            runtime::revert(Errors::UniswapV2CoreWCSPROverFlow3);
        }
        // transfer native cspr from purse to account
        system::transfer_from_purse_to_purse(get_purse(), purse, amount, None).unwrap_or_revert();
        // burn wcspr for the caller
        self.burn(Address::from(self.get_caller()), u512_to_u256(amount))
            .unwrap_or_revert();
        self.emit(&WcsprEvents::Withdraw { purse, amount });
        Ok(())
    }

    // Events
    fn emit(&self, wcspr_event: &WcsprEvents) {
        match wcspr_event {
            WcsprEvents::Deposit { purse, amount } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", wcspr_event.type_name());
                event.insert("purse", purse.to_string());
                event.insert("amount", amount.to_string());
                storage::new_uref(event);
            }
            WcsprEvents::Withdraw { purse, amount } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", wcspr_event.type_name());
                event.insert("purse", purse.to_string());
                event.insert("amount", amount.to_string());
                storage::new_uref(event);
            }
        };
    }
}
