use alloc::string::String;

use casper_contract::contract_api::runtime;
use casper_types::{runtime_args, ApiError, ContractHash, Key, RuntimeArgs, U256};
use contract_utils::set_key;
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
            "deposit",
            runtime_args! {"to" => recipient, "amount" => amount},
        );
    }

    fn pair_mint(&mut self, caller: Key, recipient: Key, amount: U256) {
        let caller_hash_add_array = match caller {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };

        let caller_hash_add = ContractHash::new(caller_hash_add_array);

        let _ret: () = runtime::call_contract(
            caller_hash_add,
            "erc20_mint",
            runtime_args! {"to" => recipient, "amount" => amount},
        );
    }

    fn balance(&mut self, token: Key, owner: Key) {
        let token_hash_add_array = match token {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };

        let token_hash_add = ContractHash::new(token_hash_add_array);

        let balance: U256 = runtime::call_contract(
            token_hash_add,
            "balance_of",
            runtime_args! {"owner" => owner},
        );
        set_key("Balance", balance);
    }

    fn token0(&mut self, pair: Key) {
        let pair_hash_add_array = match pair {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };

        let pair_hash_add = ContractHash::new(pair_hash_add_array);

        let token0: Key = runtime::call_contract(pair_hash_add, "token0", runtime_args! {});
        set_key("token0", token0);
    }

    fn token1(&mut self, pair: Key) {
        let pair_hash_add_array = match pair {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };

        let pair_hash_add = ContractHash::new(pair_hash_add_array);

        let token1: Key = runtime::call_contract(pair_hash_add, "token1", runtime_args! {});
        set_key("token1", token1);
    }
    //

    // FACTORY METHOD
    fn create_pair(&mut self, token_a: Key, token_b: Key, pair_hash: Key, factory_hash: Key) {
        let factory_hash_add_array = match factory_hash {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        let factory_hash_add = ContractHash::new(factory_hash_add_array);
        let _create_pair: () = runtime::call_contract(
            factory_hash_add,
            "create_pair",
            runtime_args! {
                "token_a" => token_a,
                "token_b" => token_b,
                "pair_hash" => pair_hash
            },
        );
    }

    fn sync(&mut self, pair_hash: Key) {
        let pair_hash_add_array = match pair_hash {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        let pair_hash_add = ContractHash::new(pair_hash_add_array);
        let _fee_to: () = runtime::call_contract(pair_hash_add, "sync", runtime_args! {});
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
