use crate::alloc::string::ToString;
use crate::data::{self, get_all_pairs, Pairs, Whitelists};
use alloc::collections::BTreeMap;
use alloc::{string::String, vec::Vec};
use casper_contract::contract_api::runtime;
use casper_contract::contract_api::storage;
use casper_types::{runtime_args, ApiError, ContractPackageHash, Key, RuntimeArgs, URef, U256};
use contract_utils::{ContractContext, ContractStorage};

pub enum FACTORYEvent {
    PairCreated {
        token0: Key,
        token1: Key,
        pair: Key,
        all_pairs_length: U256,
    },
}
impl FACTORYEvent {
    pub fn type_name(&self) -> String {
        match self {
            FACTORYEvent::PairCreated {
                token0: _,
                token1: _,
                pair: _,
                all_pairs_length: _,
            } => "pair_created",
        }
        .to_string()
    }
}
#[repr(u16)]
pub enum Error {
    /// 65,559 for (UniswapV2 Factory Zero Address)
    UniswapV2FactoryZeroAddress = 23,
    /// 65,560 for (UniswapV2 Factory Pair Exists1)
    UniswapV2FactoryPairExists1 = 24,
    /// 65,561 for (UniswapV2 Factory Pair Exists2)
    UniswapV2FactoryPairExists2 = 25,
    /// 65,562 for (UniswapV2 Factory Forbidden1)
    UniswapV2FactoryForbidden1 = 26,
    /// 65,563 for (UniswapV2 Factory Forbidden2)
    UniswapV2FactoryForbidden2 = 27,
    /// 65,564 for (UniswapV2 Factory Identical Addresses)
    UniswapV2FactoryIdenticalAddresses = 28,
    /// 65,565 for (UniswapV2 Factory Not In White List)
    UniswapV2FactoryNotInWhiteList = 29,
    /// 65,566 for (UniswapV2 Factory Not Owner)
    UniswapV2FactoryNotOwner = 30,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}

pub trait FACTORY<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(
        &mut self,
        fee_to_setter: Key,
        all_pairs: Vec<Key>,
        contract_hash: Key,
        package_hash: Key,
    ) {
        data::set_fee_to_setter(fee_to_setter);
        data::set_owner(self.get_caller());
        data::set_all_pairs(all_pairs);
        data::set_hash(contract_hash);
        data::set_package_hash(package_hash);
        Pairs::init();
        Whitelists::init();
    }

    fn create_pair(&mut self, token_a: Key, token_b: Key, pair_hash: Key) {
        let white_lists: Whitelists = Whitelists::instance();
        let white_list_user: Key = white_lists.get(&self.get_caller());
        if white_list_user
            != Key::from_formatted_str(
                "account-hash-0000000000000000000000000000000000000000000000000000000000000000",
            )
            .unwrap()
        {
            if token_a == token_b {
                runtime::revert(Error::UniswapV2FactoryIdenticalAddresses);
            }
            let token0: Key;
            let token1: Key;
            let address_0: Key = Key::from_formatted_str(
                "hash-0000000000000000000000000000000000000000000000000000000000000000",
            )
            .unwrap();
            if token_a < token_b {
                token0 = token_a;
                token1 = token_b;
            } else {
                token0 = token_b;
                token1 = token_a;
            }
            // in before 0 address was hash-0000000000000000000000000000000000000000000000000000000000000000
            if token0 == address_0 {
                runtime::revert(Error::UniswapV2FactoryZeroAddress);
            }
            let pair_0_1_key: Key = self.get_pair(token0, token1);
            let pair_1_0_key: Key = self.get_pair(token1, token0);
            if pair_0_1_key != address_0 {
                runtime::revert(Error::UniswapV2FactoryPairExists1);
            }
            if pair_1_0_key != address_0 {
                runtime::revert(Error::UniswapV2FactoryPairExists2);
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
                runtime_args! {"token0" => token0, "token1" => token1, "factory_hash" => data::get_package_hash() },
            );

            // handling the pair creation by updating the storage
            self.set_pair(token0, token1, pair_hash);
            self.set_pair(token1, token0, pair_hash);
            let mut pairs: Vec<Key> = get_all_pairs();
            pairs.push(pair_hash);
            self.set_all_pairs(pairs);
            self.emit(&FACTORYEvent::PairCreated {
                token0: token0,
                token1: token1,
                pair: pair_hash,
                all_pairs_length: (get_all_pairs().len()).into(),
            });
        } else {
            runtime::revert(Error::UniswapV2FactoryNotInWhiteList);
        }
    }

    fn get_pair(&mut self, token0: Key, token1: Key) -> Key {
        Pairs::instance().get(&token0, &token1)
    }

    fn set_pair(&mut self, token0: Key, token1: Key, value: Key) {
        Pairs::instance().set(&token0, &token1, value);
    }

    fn set_fee_to(&mut self, fee_to: Key) {
        if self.get_caller() != self.get_fee_to_setter() {
            runtime::revert(Error::UniswapV2FactoryForbidden1);
        }
        data::set_fee_to(fee_to);
    }

    fn get_fee_to(&mut self) -> Key {
        data::get_fee_to()
    }

    fn set_fee_to_setter(&mut self, fee_to_setter: Key) {
        if self.get_caller() != self.get_fee_to_setter() {
            runtime::revert(Error::UniswapV2FactoryForbidden2);
        }
        data::set_fee_to_setter(fee_to_setter);
    }

    fn get_fee_to_setter(&mut self) -> Key {
        data::get_fee_to_setter()
    }

    fn set_all_pairs(&mut self, all_pairs: Vec<Key>) {
        data::set_all_pairs(all_pairs);
    }

    fn get_all_pairs(&mut self) -> Vec<Key> {
        data::get_all_pairs()
    }

    fn set_white_list(&mut self, white_list: Key, value: Key) {
        if self.get_caller() == data::get_owner() {
            Whitelists::instance().set(&white_list, value);
        } else {
            runtime::revert(Error::UniswapV2FactoryNotOwner);
        }
    }
    fn emit(&mut self, factory_event: &FACTORYEvent) {
        let mut events = Vec::new();
        let package = self.get_package_hash();
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
            let _: URef = storage::new_uref(event);
        }
    }

    fn get_package_hash(&mut self) -> ContractPackageHash {
        let package_hash_add_array = match data::get_package_hash() {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        ContractPackageHash::new(package_hash_add_array)
    }
}
