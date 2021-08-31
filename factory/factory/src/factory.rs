use alloc::{collections::BTreeSet, format, string::String, vec, vec::Vec};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, CLTyped, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints,
    Group, Key, Parameter, RuntimeArgs, URef, U256, ContractHash,ApiError
};

use contract_utils::{ContractContext, ContractStorage};


use crate::data::{self, Pairs,get_all_pairs};

use renvm_sig::keccak256;
use hex::encode;


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
    fn init(&mut self,fee_to:Key, fee_to_setter:Key, all_pairs:Vec<Key>, contract_hash: Key ) {
        data::set_fee_to(fee_to);
        data::set_fee_to_setter(fee_to_setter);
        data::set_all_pairs(all_pairs);
        data::set_hash(contract_hash);
        Pairs::init();
    }
    fn get_entry_points(&mut self) -> EntryPoints {
        let mut entry_points = EntryPoints::new();
        entry_points.add_entry_point(EntryPoint::new(
            "constructor",
            vec![
                Parameter::new("name", String::cl_type()),
                Parameter::new("symbol", String::cl_type()),
                Parameter::new("decimals", u8::cl_type()),
                Parameter::new("initial_supply", U256::cl_type()),
                Parameter::new("nonce", U256::cl_type()),
                Parameter::new("domain_separator", String::cl_type()),
                Parameter::new("permit_type_hash", String::cl_type()),
                Parameter::new("contract_hash", ContractHash::cl_type()),
            ],
            <()>::cl_type(),
            EntryPointAccess::Groups(vec![Group::new("constructor")]),
            EntryPointType::Contract,
        ));
        entry_points.add_entry_point(EntryPoint::new(
            "transfer",
            vec![
                Parameter::new("recipient", Key::cl_type()),
                Parameter::new("amount", U256::cl_type()),
            ],
            <()>::cl_type(),
            EntryPointAccess::Public,
            EntryPointType::Contract,
        ));
        entry_points.add_entry_point(EntryPoint::new(
            "transfer_from",
            vec![
                Parameter::new("owner", Key::cl_type()),
                Parameter::new("recipient", Key::cl_type()),
                Parameter::new("amount", U256::cl_type()),
            ],
            <()>::cl_type(),
            EntryPointAccess::Public,
            EntryPointType::Contract,
        ));
        entry_points.add_entry_point(EntryPoint::new(
            "skim",
            vec![
                Parameter::new("to", Key::cl_type()),
            ],
            <()>::cl_type(),
            EntryPointAccess::Public,
            EntryPointType::Contract,
        ));
        entry_points.add_entry_point(EntryPoint::new(
            "sync",
            vec![],
            <()>::cl_type(),
            EntryPointAccess::Public,
            EntryPointType::Contract,
        ));
        entry_points.add_entry_point(EntryPoint::new(
            "swap",
            vec![
                Parameter::new("amount0_out", U256::cl_type()),
                Parameter::new("amount1_out", U256::cl_type()),
                Parameter::new("to", Key::cl_type()),
                Parameter::new("data", String::cl_type())
            ],
            <()>::cl_type(),
            EntryPointAccess::Public,
            EntryPointType::Contract,
        ));
        entry_points.add_entry_point(EntryPoint::new(
            "permit",
            vec![
                Parameter::new("public", String::cl_type()),
                Parameter::new("signature", String::cl_type()),
                Parameter::new("owner", Key::cl_type()),
                Parameter::new("spender", Key::cl_type()),
                Parameter::new("value", U256::cl_type()),
                Parameter::new("deadline", u64::cl_type()),
            ],
            <()>::cl_type(),
            EntryPointAccess::Public,
            EntryPointType::Contract,
        ));
        entry_points.add_entry_point(EntryPoint::new(
            "approve",
            vec![
                Parameter::new("spender", Key::cl_type()),
                Parameter::new("amount", U256::cl_type()),
            ],
            <()>::cl_type(),
            EntryPointAccess::Public,
            EntryPointType::Contract,
        ));
    
        entry_points.add_entry_point(EntryPoint::new(
            "balance_of",
            vec![Parameter::new("owner", Key::cl_type())],
            U256::cl_type(),
            EntryPointAccess::Public,
            EntryPointType::Contract,
        ));
        entry_points.add_entry_point(EntryPoint::new(
            "nonce",
            vec![Parameter::new("owner", Key::cl_type())],
            U256::cl_type(),
            EntryPointAccess::Public,
            EntryPointType::Contract,
        ));
        entry_points.add_entry_point(EntryPoint::new(
            "allowance",
            vec![
                Parameter::new("owner", Key::cl_type()),
                Parameter::new("spender", Key::cl_type()),
            ],
            U256::cl_type(),
            EntryPointAccess::Public,
            EntryPointType::Contract,
        ));
        entry_points.add_entry_point(EntryPoint::new(
            "total_supply",
            vec![],
            U256::cl_type(),
            EntryPointAccess::Public,
            EntryPointType::Contract,
        ));
        entry_points.add_entry_point(EntryPoint::new(
            "mint",
            vec![
                Parameter::new("to", Key::cl_type()),
                Parameter::new("amount", U256::cl_type()),
            ],
            <()>::cl_type(),
            EntryPointAccess::Public,
            EntryPointType::Contract,
        ));
        entry_points.add_entry_point(EntryPoint::new(
            "burn",
            vec![
                Parameter::new("from", Key::cl_type()),
                Parameter::new("amount", U256::cl_type()),
            ],
            <()>::cl_type(),
            EntryPointAccess::Public,
            EntryPointType::Contract,
        ));
        entry_points
    }
    
    fn create_pair(&mut self,token_a:Key,token_b:Key) {

        if token_a == token_b {
            runtime::revert(Error::UniswapV2IdenticalAddresses);
        }

        let mut token0=Key::from_formatted_str("hash-0000000000000000000000000000000000000000000000000000000000000000").unwrap();
        let mut token1=Key::from_formatted_str("hash-0000000000000000000000000000000000000000000000000000000000000000").unwrap();
        let address_0=Key::from_formatted_str("hash-0000000000000000000000000000000000000000000000000000000000000000").unwrap();

        if token_a < token_b 
        {
            token0=token_a;
            token1=token_b;

        } else {
      
            token0=token_b;
            token1=token_a;
        };

        // in before 0 address was 0x0
        if token0  == address_0 {
            runtime::revert(Error::UniswapV2ZeroAddress);
        }

        let pair_0_1_key:Key= self.pair(token0,token1);
        let pair_1_0_key:Key= self.pair(token1,token0);

        if pair_0_1_key != address_0 {
            runtime::revert(Error::UniswapV2PairExists);
        }
        if pair_1_0_key != address_0 {
            runtime::revert(Error::UniswapV2PairExists);
        }
        //convert Key to ContractHash
        let token0_hash_add_array = match token0 {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        let token0_contract_hash = ContractHash::new(token0_hash_add_array);

        //convert Key to ContractHash
        let token1_hash_add_array = match token1 {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        let token1_contract_hash = ContractHash::new(token1_hash_add_array);

        let token0_name: String = runtime::call_contract(token0_contract_hash, "name", RuntimeArgs::new());
        let token1_name: String = runtime::call_contract(token1_contract_hash, "name", RuntimeArgs::new());
        let token0_symbol: String = runtime::call_contract(token0_contract_hash, "symbol", RuntimeArgs::new());
        let token1_symbol: String = runtime::call_contract(token1_contract_hash, "symbol", RuntimeArgs::new());

        let (package_hash, access_token) = storage::create_contract_package_at_hash();
        let (contract_hash, _) =
            storage::add_contract_version(package_hash, self.get_entry_points(), Default::default());

        let name: String = format!("{}{}{}",token0_name,"_",token1_name);
        let symbol: String = format!("{}{}{}",token0_symbol,"_",token1_symbol);
        let decimals: u8 = 18;
        let initial_supply: U256 = 1000.into();
        let nonce: U256 = 0.into();

        let eip_712_domain : &str="EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)";
        let permit_type: &str="Permit(address owner,address spender,uint256 value,uint256 nonce,uint256 deadline)";
        let chain_id : &str="101";

        let eip_domain_hash=keccak256(eip_712_domain.as_bytes());// to take a byte hash of EIP712Domain
        let name_hash=keccak256(name.as_bytes());// to take a byte hash of name
        let one_hash=keccak256("1".as_bytes());// to take a byte hash of "1"

        let eip_domain_hash = encode(eip_domain_hash);// to encode and convert eip_domain_hash into string
        let name_hash = encode(name_hash);// to encode and convert name_hash into string
        let one_hash = encode(one_hash);// to encode and convert one_hash into string
        let concatenated_data:String = format!("{}{}{}{}{}",eip_domain_hash,name_hash,one_hash,chain_id,contract_hash);//string contactination
        let domain_separator=keccak256(concatenated_data.as_bytes());//to take a byte hash of concatenated Data
        let permit_type_hash=keccak256(permit_type.as_bytes());// to take a byte hash of Permit Type

        let domain_separator=encode(domain_separator);
        let permit_type_hash=encode(permit_type_hash);


        // Prepare constructor args
        let constructor_args = runtime_args! {
            "name" => name,
            "symbol" => symbol,
            "decimals" => decimals,
            "initial_supply" => initial_supply,
            "nonce" => nonce,
            "domain_separator" => domain_separator,
            "permit_type_hash" => permit_type_hash,
            "contract_hash" => contract_hash

        };

        // Add the constructor group to the package hash with a single URef.
        let constructor_access: URef =
            storage::create_contract_user_group(package_hash, "constructor", 1, Default::default())
                .unwrap_or_revert()
                .pop()
                .unwrap_or_revert();

        // Call the constructor entry point
        let _: () =
            runtime::call_versioned_contract(package_hash, None, "constructor", constructor_args);

        // Remove all URefs from the constructor group, so no one can call it for the second time.
        let mut urefs = BTreeSet::new();
        urefs.insert(constructor_access);
        storage::remove_contract_user_group_urefs(package_hash, "constructor", urefs)
            .unwrap_or_revert();

        // Store contract in the account's named keys.
        let contract_name: &str = "pair_contract";

        runtime::put_key(
            &format!("{}_package_hash", contract_name),
            package_hash.into(),
        );
        runtime::put_key(
            &format!("{}_package_hash_wrapped", contract_name),
            storage::new_uref(package_hash).into(),
        );
        runtime::put_key(
            &format!("{}_contract_hash", contract_name),
            contract_hash.into(),
        );
        runtime::put_key(
            &format!("{}_contract_hash_wrapped", contract_name),
            storage::new_uref(contract_hash).into(),
        );
        runtime::put_key(
            &format!("{}_package_access_token", contract_name),
            access_token.into(),
        );

        let contract_hash_key=Key::from(contract_hash);

        // handling the pair creation by updating the storage
       
        self.set_pair(token0,token1,contract_hash_key);
        self.set_pair(token1,token0,contract_hash_key);

        let mut pairs: Vec<Key> = get_all_pairs();
        pairs.push(contract_hash_key);
        self.set_all_pairs(pairs);
        
    }

    fn pair(&mut self, token0: Key, token1: Key) -> Key {
        Pairs::instance().get(&token0, &token1)
    }

    fn set_pair(&mut self, token0: Key, token1: Key, value:Key) {
        Pairs::instance().set(&token0, &token1, value);
    }

    fn set_fee_to(&mut self, fee_to: Key) {
        if self.get_caller() != self.get_fee_to_setter() {
        runtime::revert(Error::UniswapV2Forbidden);
        }
        data::set_fee_to(fee_to);
    }

    fn get_fee_to(&mut self) -> Key {
        data::get_fee_to()
    }

    fn set_fee_to_setter(&mut self, fee_to_setter: Key) {
        if self.get_caller() != self.get_fee_to_setter() {
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
