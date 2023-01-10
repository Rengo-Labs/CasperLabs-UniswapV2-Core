use crate::data::*;
use common::{
    contract_api::{runtime, storage},
    errors::Errors,
    functions::*,
    unwrap_or_revert::UnwrapOrRevert,
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
        let (white_list_user, _) = white_lists.get(&self.get_caller());
        if white_list_user == account_zero_address() || white_list_user == zero_address() {
            runtime::revert(Errors::UniswapV2FactoryNotInWhiteList1);
        }
        if token_a == token_b {
            runtime::revert(Errors::UniswapV2FactoryIdenticalAddresses1);
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
            runtime::revert(Errors::UniswapV2FactoryZeroAddress1);
        }
        let pair_0_1_key: Key = self.get_pair(token0, token1);
        let pair_1_0_key: Key = self.get_pair(token1, token0);
        if pair_0_1_key != zero_address() {
            runtime::revert(Errors::UniswapV2FactoryPairExists1);
        }
        if pair_1_0_key != zero_address() {
            runtime::revert(Errors::UniswapV2FactoryPairExists2);
        }
        runtime::call_versioned_contract::<()>(
            pair_hash.into_hash().unwrap_or_revert().into(),
            None,
            "initialize",
            runtime_args! {
                "token0" => token0,
                "token1" => token1
            },
        );
        // handling the pair creation by updating the storage
        self.set_pair(token0, token1, pair_hash);
        self.set_pair(token1, token0, pair_hash);
        let mut pairs: Vec<Key> = get_all_pairs();
        pairs.push(pair_hash);
        set_all_pairs(pairs);
        Whitelists::instance().set(&self.get_caller(), self.get_caller(), pair_hash);
        self.emit(&FACTORYEvent::PairCreated {
            token0,
            token1,
            pair: pair_hash,
            all_pairs_length: (get_all_pairs().len()).into(),
        });
    }

    fn remove_pair(&self, pair_hash: Key) {
        let white_lists: Whitelists = Whitelists::instance();
        let (white_list_user, whitelist_pair) = white_lists.get(&self.get_caller());
        if white_list_user == zero_address() || white_list_user == account_zero_address() {
            runtime::revert(Errors::UniswapV2FactoryNotInWhiteList2);
        }
        if whitelist_pair != pair_hash {
            runtime::revert(Errors::UniswapV2FactoryWhiteListPairMismatch);
        }
        let token_a: Key = runtime::call_versioned_contract(
            pair_hash.into_hash().unwrap_or_revert().into(),
            None,
            "token0",
            runtime_args! {},
        );
        let token_b: Key = runtime::call_versioned_contract(
            pair_hash.into_hash().unwrap_or_revert().into(),
            None,
            "token1",
            runtime_args! {},
        );
        if token_a == token_b {
            runtime::revert(Errors::UniswapV2FactoryIdenticalAddresses2);
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
            runtime::revert(Errors::UniswapV2FactoryZeroAddress2);
        }
        let pair_0_1_key: Key = self.get_pair(token0, token1);
        let pair_1_0_key: Key = self.get_pair(token1, token0);
        if pair_0_1_key == zero_address() {
            runtime::revert(Errors::UniswapV2FactoryNoPairExists1);
        }
        if pair_1_0_key == zero_address() {
            runtime::revert(Errors::UniswapV2FactoryNoPairExists2);
        }
        runtime::call_versioned_contract::<()>(
            pair_hash.into_hash().unwrap_or_revert().into(),
            None,
            "deinitialize",
            runtime_args! {},
        );
        // handling the pair creation by updating the storage
        self.set_pair(token0, token1, zero_address());
        self.set_pair(token1, token0, zero_address());
        let mut pairs: Vec<Key> = get_all_pairs();
        let index = pairs
            .iter()
            .position(|&val| val == pair_hash)
            .unwrap_or_revert();
        pairs.swap_remove(index);
        set_all_pairs(pairs);
        Whitelists::instance().set(&self.get_caller(), self.get_caller(), zero_address());
        self.emit(&FACTORYEvent::PairRemoved {
            token0,
            token1,
            pair: pair_hash,
            all_pairs_length: (get_all_pairs().len()).into(),
        });
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
        if self.get_caller() != get_owner() {
            runtime::revert(Errors::UniswapV2FactoryNotOwner);
        }
        Whitelists::instance().set(&white_list, value, zero_address());
    }

    fn emit(&self, factory_event: &FACTORYEvent) {
        match factory_event {
            FACTORYEvent::PairCreated {
                token0,
                token1,
                pair,
                all_pairs_length,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", factory_event.type_name());
                event.insert("token0", token0.to_string());
                event.insert("token1", token1.to_string());
                event.insert("pair", pair.to_string());
                event.insert("all_pairs_length", all_pairs_length.to_string());
                storage::new_uref(event);
            }
            FACTORYEvent::PairRemoved {
                token0,
                token1,
                pair,
                all_pairs_length,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", factory_event.type_name());
                event.insert("token0", token0.to_string());
                event.insert("token1", token1.to_string());
                event.insert("pair", pair.to_string());
                event.insert("all_pairs_length", all_pairs_length.to_string());
                storage::new_uref(event);
            }
        };
    }
}
