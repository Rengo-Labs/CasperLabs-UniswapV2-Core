use alloc::{vec::Vec};

use casper_contract::{
    contract_api::{runtime},
};
use casper_types::{
    runtime_args, Key, RuntimeArgs, ContractHash,ApiError
};

use contract_utils::{ContractContext, ContractStorage};

use crate::data::{self, Pairs, get_all_pairs};

#[repr(u16)]
pub enum Error {
    UniswapV2ZeroAddress = 0,
    UniswapV2PairExists = 1,
    UniswapV2Forbidden = 2,
    UniswapV2IdenticalAddresses = 3,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}

pub trait FACTORY<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(&mut self, fee_to_setter:Key, all_pairs:Vec<Key>, contract_hash: Key ) {
        data::set_fee_to_setter(fee_to_setter);
        data::set_all_pairs(all_pairs);
        data::set_hash(contract_hash);
        Pairs::init();
    }
    fn create_pair(&mut self,token_a: Key,token_b: Key,pair_hash: Key) {

        if token_a == token_b {
            runtime::revert(Error::UniswapV2IdenticalAddresses);
        }
        let token0: Key;
        let token1: Key;
        let address_0 = Key::from_formatted_str("hash-0000000000000000000000000000000000000000000000000000000000000000").unwrap();

        if token_a < token_b {
            token0 = token_a;
            token1 = token_b;
        } else {
            token0 = token_b;
            token1 = token_a;
        }

        // in before 0 address was hash-0
        if token0  == address_0 {
            runtime::revert(Error::UniswapV2ZeroAddress);
        }

        let pair_0_1_key:Key = self.get_pair(token0,token1);
        let pair_1_0_key:Key = self.get_pair(token1,token0);

        if pair_0_1_key != address_0 {
            runtime::revert(Error::UniswapV2PairExists);
        }
        if pair_1_0_key != address_0 {
            runtime::revert(Error::UniswapV2PairExists);
        }
        //convert Key to ContractHash
        let pair_hash_add_array = match pair_hash {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        let pair_contract_hash = ContractHash::new(pair_hash_add_array);

        let _ret: () = runtime::call_contract(pair_contract_hash, "initialize", runtime_args!{"token0" => token0, "token1" => token1, "factory_hash" => data::get_hash() });
        // handling the pair creation by updating the storage       
        self.set_pair(token0,token1,pair_hash);
        self.set_pair(token1,token0,pair_hash);

        let mut pairs: Vec<Key> = get_all_pairs();
        pairs.push(pair_hash);
        self.set_all_pairs(pairs);
        
    }

    fn get_pair(&mut self, token0: Key, token1: Key) -> Key {
        Pairs::instance().get(&token0, &token1)
    }

    fn set_pair(&mut self, token0: Key, token1: Key, value:Key) {
        Pairs::instance().set(&token0, &token1, value);
    }

    fn set_fee_to(&mut self, fee_to: Key) {

        if runtime::get_caller() != self.get_fee_to_setter().into_account().unwrap_or_default() {
            runtime::revert(Error::UniswapV2Forbidden);
        }
        data::set_fee_to(fee_to);
    }

    fn get_fee_to(&mut self) -> Key {
        data::get_fee_to()
    }

    fn set_fee_to_setter(&mut self, fee_to_setter: Key) {
        if runtime::get_caller() != self.get_fee_to_setter().into_account().unwrap_or_default() {
            runtime::revert(Error::UniswapV2Forbidden);
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
}
