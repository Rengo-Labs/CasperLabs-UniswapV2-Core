use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use casper_types::{bytesrepr::ToBytes, runtime_args, Key, RuntimeArgs, U256};
use test_env::{Sender, TestContract, TestEnv};

pub struct WCSPRInstance(TestContract);

impl WCSPRInstance {
    pub fn new(
        env: &TestEnv,
        contract_name: &str,
        sender: Sender,
        name: &str,
        symbol: &str,
    ) -> WCSPRInstance {
        WCSPRInstance(TestContract::new(
            env,
            "wcspr-token.wasm",
            contract_name,
            sender,
            runtime_args! {
                "name" => name,
                "symbol" => symbol,
            },
        ))
    }

    pub fn constructor(&self, sender: Sender, name: &str, symbol: &str) {
        self.0.call_contract(
            sender,
            "constructor",
            runtime_args! {
                "name" => name,
                "symbol" => symbol,
            },
        );
    }

    pub fn transfer<T: Into<Key>>(&self, sender: Sender, recipient: T, amount: U256) {
        self.0.call_contract(
            sender,
            "transfer",
            runtime_args! {
                "recipient" => recipient.into(),
                "amount" => amount
            },
        );
    }

    pub fn transfer_from<T: Into<Key>>(
        &self,
        sender: Sender,
        owner: T,
        recipient: T,
        amount: U256,
    ) {
        self.0.call_contract(
            sender,
            "transfer_from",
            runtime_args! {
                "owner" => owner.into(),
                "recipient" => recipient.into(),
                "amount" => amount
            },
        );
    }

    pub fn approve<T: Into<Key>>(&self, sender: Sender, spender: T, amount: U256) {
        self.0.call_contract(
            sender,
            "approve",
            runtime_args! {
                "spender" => spender.into(),
                "amount" => amount
            },
        );
    }

    pub fn balance_of<T: Into<Key>>(&self, account: T) -> U256 {
        self.0
            .query_dictionary("balances", key_to_str(&account.into()))
            .unwrap_or_default()
    }

    pub fn allowance<T: Into<Key>>(&self, owner: T, spender: T) -> U256 {
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
