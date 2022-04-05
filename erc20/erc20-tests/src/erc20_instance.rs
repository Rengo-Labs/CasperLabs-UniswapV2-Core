use std::collections::BTreeMap;

use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use casper_types::{
    account::AccountHash, bytesrepr::ToBytes, runtime_args, CLTyped, ContractPackageHash, Key,
    RuntimeArgs, U256,
};
use test_env::{TestContract, TestEnv};

pub type TokenId = U256;
pub type Meta = BTreeMap<String, String>;

pub struct ERC20Instance(TestContract);

impl ERC20Instance {
    pub fn instance(erc20: TestContract) -> ERC20Instance {
        ERC20Instance(erc20)
    }

    pub fn proxy(env: &TestEnv, erc20: Key, sender: AccountHash) -> TestContract {
        TestContract::new(
            env,
            "erc20-proxy-token.wasm",
            "proxy_test",
            sender,
            runtime_args! {
                "erc20" => erc20
            },
        )
    }
    pub fn proxy2(env: &TestEnv, erc20: Key, sender: AccountHash) -> TestContract {
        TestContract::new(
            env,
            "erc20-proxy-token.wasm",
            "proxy_test2",
            sender,
            runtime_args! {
                "erc20" => erc20
            },
        )
    }

    pub fn new(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        name: &str,
        symbol: &str,
        decimals: u8,
        supply: U256,
    ) -> TestContract {
        TestContract::new(
            env,
            "erc20-token.wasm",
            contract_name,
            sender,
            runtime_args! {
                "initial_supply" => supply,
                "name" => name,
                "symbol" => symbol,
                "decimals" => decimals
            },
        )
    }

    pub fn constructor(
        &self,
        sender: AccountHash,
        name: &str,
        symbol: &str,
        decimals: u8,
        initial_supply: U256,
    ) {
        self.0.call_contract(
            sender,
            "constructor",
            runtime_args! {
                "initial_supply" => initial_supply,
                "name" => name,
                "symbol" => symbol,
                "decimals" => decimals
            },
        );
    }
    // pub fn new(
    //     env: &TestEnv,
    //     contract_name: &str,
    //     sender: AccountHash,
    //     name: &str,
    //     symbol: &str,
    //     decimals: u8,
    //     initial_supply: U256,
    // ) -> ERC20Instance {
    //     ERC20Instance(TestContract::new(
    //         env,
    //         "erc20-token.wasm",
    //         contract_name,
    //         sender,
    //         runtime_args! {
    //             "name" => name,
    //             "symbol" => symbol,
    //             "initial_supply" => initial_supply,
    //             "decimals" => decimals,
    //         },
    //     ))
    // }

    pub fn transfer<T: Into<Key>>(&self, sender: AccountHash, recipient: T, amount: U256) {
        self.0.call_contract(
            sender,
            "transfer",
            runtime_args! {
                "recipient" => recipient.into(),
                "amount" => amount
            },
        );
    }

    pub fn transfer_from(&self, sender: AccountHash, owner: Key, recipient: Key, amount: U256) {
        self.0.call_contract(
            sender,
            "transfer_from",
            runtime_args! {
                "owner" => owner,
                "recipient" => recipient,
                "amount" => amount
            },
        );
    }

    pub fn approve<T: Into<Key>>(&self, sender: AccountHash, spender: T, amount: U256) {
        self.0.call_contract(
            sender,
            "approve",
            runtime_args! {
                "spender" => spender.into(),
                "amount" => amount
            },
        );
    }

    pub fn increase_allowance<T: Into<Key>>(&self, sender: AccountHash, spender: T, amount: U256) {
        self.0.call_contract(
            sender,
            "increase_allowance",
            runtime_args! {
                "spender" => spender.into(),
                "amount" => amount
            },
        );
    }

    pub fn allowance_fn(&self, sender: AccountHash, owner: Key, spender: Key) {
        self.0.call_contract(
            sender,
            "allowance",
            runtime_args! {
                "owner" => owner,
                "spender" => spender,
            },
        );
    }

    pub fn decrease_allowance<T: Into<Key>>(&self, sender: AccountHash, spender: T, amount: U256) {
        self.0.call_contract(
            sender,
            "decrease_allowance",
            runtime_args! {
                "spender" => spender.into(),
                "amount" => amount
            },
        );
    }

    pub fn mint<T: Into<Key>>(&self, sender: AccountHash, to: T, amount: U256) {
        self.0.call_contract(
            sender,
            "mint",
            runtime_args! {
                "to" => to.into(),
                "amount" => amount
            },
        );
    }
    pub fn burn<T: Into<Key>>(&self, sender: AccountHash, from: T, amount: U256) {
        self.0.call_contract(
            sender,
            "burn",
            runtime_args! {
                "from" => from.into(),
                "amount" => amount
            },
        );
    }

    pub fn balance_of<T: Into<Key>>(&self, account: T) -> U256 {
        self.0
            .query_dictionary("balances", key_to_str(&account.into()))
            .unwrap_or_default()
    }

    pub fn nonce<T: Into<Key>>(&self, account: T) -> U256 {
        self.0
            .query_dictionary("nonce", key_to_str(&account.into()))
            .unwrap_or_default()
    }

    pub fn allowance<T: Into<Key>>(&self, owner: T, spender: T) -> U256 {
        let owner: Key = owner.into();
        let spender: Key = spender.into();
        self.0
            .query_dictionary("allowances", keys_to_str(&owner, &spender))
            .unwrap_or_default()
    }
    pub fn allowance_package_hash<T: Into<Key>>(&self, owner: ContractPackageHash, spender: T) -> U256 {
        let owner: Key = owner.into();
        let spender: Key = spender.into();
        self.0
            .query_dictionary("allowances", keys_to_str(&owner, &spender))
            .unwrap_or_default()
    }
    

    pub fn name(&self) -> String {
        self.0.query_named_key(String::from("name"))
    }

    pub fn symbol(&self) -> String {
        self.0.query_named_key(String::from("symbol"))
    }

    pub fn decimals(&self) -> u8 {
        self.0.query_named_key(String::from("decimals"))
    }

    pub fn total_supply(&self) -> U256 {
        self.0.query_named_key(String::from("total_supply"))
    }

    pub fn contract_package_hash(&self) -> ContractPackageHash {
        self.0
            .query_named_key(String::from("contract_package_hash"))
    }
    pub fn contract_hash(&self) -> Key {
        self.0.query_named_key(String::from("self_contract_hash"))
    }

    // Result methods
    pub fn transfer_result(&self) -> Result<(), u32> {
        self.0.query_named_key("transfer_result".to_string())
    }

    pub fn package_hash_result(&self) -> ContractPackageHash {
        self.0.query_named_key("package_hash".to_string())
    }

    pub fn transfer_from_result(&self) -> Result<(), u32> {
        self.0.query_named_key("transfer_from_result".to_string())
    }
    pub fn allowance_res(&self) -> U256 {
        self.0.query_named_key("allowance".to_string())
    }

    pub fn increase_allowance_res(&self) -> Result<(), u32> {
        self.0.query_named_key("increase_allowance_result".to_string())
    }
    pub fn decrease_allowance_res(&self) -> Result<(), u32> {
        self.0.query_named_key("decrease_allowance_result".to_string())
    }

    pub fn meta(&self) -> Meta {
        self.0.query_named_key(String::from("meta"))
    }
}

pub fn key_to_str(key: &Key) -> String {
    match key {
        Key::Account(account) => account.to_string(),
        Key::Hash(package) => hex::encode(package),
        _ => panic!("Unexpected key type"),
    }
}

pub fn keys_to_str(key_a: &Key, key_b: &Key) -> String {
    let mut hasher = VarBlake2b::new(32).unwrap();
    hasher.update(key_a.to_bytes().unwrap());
    hasher.update(key_b.to_bytes().unwrap());
    let mut ret = [0u8; 32];
    hasher.finalize_variable(|hash| ret.clone_from_slice(hash));
    hex::encode(ret)
}

pub fn key_and_value_to_str<T: CLTyped + ToBytes>(key: &Key, value: &T) -> String {
    let mut hasher = VarBlake2b::new(32).unwrap();
    hasher.update(key.to_bytes().unwrap());
    hasher.update(value.to_bytes().unwrap());
    let mut ret = [0u8; 32];
    hasher.finalize_variable(|hash| ret.clone_from_slice(hash));
    hex::encode(ret)
}
