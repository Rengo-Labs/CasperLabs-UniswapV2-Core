use crate::data::*;
use common::{
    contract_api::{runtime, storage},
    errors::Errors,
    functions::*,
    *,
};
use std::collections::BTreeMap;

pub trait FACTORY<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(
        &self,
        fee_to_setter: Key,
        all_pairs: Vec<Key>,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
    ) {
        set_fee_to_setter(fee_to_setter);
        set_owner(self.get_caller());
        set_all_pairs(all_pairs);
        set_contract_hash(contract_hash);
        set_package_hash(package_hash);
        Pairs::init();
        Whitelists::init();
    }

    fn create_pair(&self, token_a: Key, token_b: Key, pair_hash: Key) {
        let white_lists: Whitelists = Whitelists::instance();
        let white_list_user: Key = white_lists.get(&self.get_caller());
        if white_list_user != account_zero_address() {
            if token_a == token_b {
                runtime::revert(Errors::UniswapV2FactoryIdenticalAddresses);
            }
            let token0: Key;
            let token1: Key;
            if token_a < token_b {
                token0 = token_a;
                token1 = token_b;
            } else {
                token0 = token_b;
                token1 = token_a;
            }
            // in before 0 address was hash-0000000000000000000000000000000000000000000000000000000000000000
            if token0 == zero_address() {
                runtime::revert(Errors::UniswapV2FactoryZeroAddress);
            }
            let pair_0_1_key: Key = self.get_pair(token0, token1);
            let pair_1_0_key: Key = self.get_pair(token1, token0);
            if pair_0_1_key != zero_address() {
                runtime::revert(Errors::UniswapV2FactoryPairExists1);
            }
            if pair_1_0_key != zero_address() {
                runtime::revert(Errors::UniswapV2FactoryPairExists2);
            }
            //convert Key to ContractPackageHash
            let pair_hash_add_array = match pair_hash {
                Key::Hash(package) => package,
                _ => runtime::revert(ApiError::UnexpectedKeyVariant),
            };
            let pair_package_hash = ContractPackageHash::new(pair_hash_add_array);
            let _ret: () = runtime::call_versioned_contract(
                pair_package_hash,
                None,
                "initialize",
                runtime_args! {"token0" => token0, "token1" => token1, "factory_hash" => Key::from(get_package_hash()) },
            );

            // handling the pair creation by updating the storage
            self.set_pair(token0, token1, pair_hash);
            self.set_pair(token1, token0, pair_hash);
            let mut pairs: Vec<Key> = get_all_pairs();
            pairs.push(pair_hash);
            set_all_pairs(pairs);
            self.emit(&FACTORYEvent::PairCreated {
                token0,
                token1,
                pair: pair_hash,
                all_pairs_length: (get_all_pairs().len()).into(),
            });
        } else {
            runtime::revert(Errors::UniswapV2FactoryNotInWhiteList);
        }
    }

    fn get_pair(&self, token0: Key, token1: Key) -> Key {
        Pairs::instance().get(&token0, &token1)
    }

    fn set_pair(&self, token0: Key, token1: Key, value: Key) {
        Pairs::instance().set(&token0, &token1, value);
    }

    fn set_fee_to(&self, fee_to: Key) {
        if self.get_caller() != get_fee_to_setter() {
            runtime::revert(Errors::UniswapV2FactoryForbidden1);
        }
        set_fee_to(fee_to);
    }

    fn set_fee_to_setter(&self, fee_to_setter: Key) {
        if self.get_caller() != get_fee_to_setter() {
            runtime::revert(Errors::UniswapV2FactoryForbidden2);
        }
        set_fee_to_setter(fee_to_setter);
    }

    fn set_white_list(&self, white_list: Key, value: Key) {
        if self.get_caller() == get_owner() {
            Whitelists::instance().set(&white_list, value);
        } else {
            runtime::revert(Errors::UniswapV2FactoryNotOwner);
        }
    }

    fn emit(&self, factory_event: &FACTORYEvent) {
        let mut events = Vec::new();
        let package = get_package_hash();
        match factory_event {
            FACTORYEvent::PairCreated {
                token0,
                token1,
                pair,
                all_pairs_length,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", factory_event.type_name());
                event.insert("token0", token0.to_string());
                event.insert("token1", token1.to_string());
                event.insert("pair", pair.to_string());
                event.insert("all_pairs_length", all_pairs_length.to_string());
                events.push(event);
            }
        };

        for event in events {
            storage::new_uref(event);
        }
    }
}
