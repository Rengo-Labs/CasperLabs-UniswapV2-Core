use alloc::string::String;

use casper_contract::contract_api::runtime;
use casper_types::{runtime_args, ApiError, ContractHash, Key, RuntimeArgs, U256};
use contract_utils::{ContractContext, ContractStorage};

use crate::data::{self};

pub trait TEST<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(&mut self, name: String, contract_hash: Key) {
        data::set_name(name);
        data::set_hash(contract_hash);
    }

    fn mint_with_caller(&mut self, caller: Key, recipient: Key, amount: U256) {
        let caller_hash_add_array = match caller {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };

        let caller_hash_add = ContractHash::new(caller_hash_add_array);

        let _ret: () = runtime::call_contract(
            caller_hash_add,
            "mint",
            runtime_args! {"to" => recipient, "amount" => amount},
        );
    }

    fn set_fee_to(&mut self, fee_to: Key, factory_hash: Key) {
        let factory_hash_add_array = match factory_hash {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        let factory_hash_add = ContractHash::new(factory_hash_add_array);
        let _fee_to: () = runtime::call_contract(
            factory_hash_add,
            "set_fee_to",
            runtime_args! {"fee_to" => fee_to},
        );
    }
}
