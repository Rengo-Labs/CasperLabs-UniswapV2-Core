use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use casper_types::{
    bytesrepr::ToBytes, runtime_args, ContractHash, ContractPackageHash, Key, RuntimeArgs, U256,
    U512, account::AccountHash
};
use test_env::{TestContract, TestEnv};

// pub mod constants;
use crate::constants::*;

pub struct WCSPRInstance(TestContract);
impl WCSPRInstance {
    pub fn new(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        name: &str,
        symbol: &str,
        decimals: u8,
    ) -> TestContract {
        TestContract::new(
            env,
            "wcspr-token.wasm",
            contract_name,
            sender,
            runtime_args! {
                "name" => name,
                "symbol" => symbol,
                "decimals"=>decimals
            },
        )
    }

    pub fn instance(contract: TestContract) -> WCSPRInstance {
        WCSPRInstance(contract)
    }

    pub fn proxy(env: &TestEnv, wcspr: Key, sender: AccountHash) -> TestContract {
        TestContract::new(
            env,
            "wcspr-test.wasm",
            "proxy_test",
            sender,
            runtime_args! {
                "wcspr" => wcspr
            },
        )
    }
    pub fn proxy2(env: &TestEnv, wcspr: Key, sender: AccountHash) -> TestContract {
        TestContract::new(
            env,
            "wcspr-test2.wasm",
            "proxy_test2",
            sender,
            runtime_args! {
                "wcspr" => wcspr
            },
        )
    }

    pub fn constructor(&self, sender: AccountHash, name: &str, symbol: &str) {
        self.0.call_contract(
            sender,
            "constructor",
            runtime_args! {
                "name" => name,
                "symbol" => symbol,
            },
        );
    }

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

    pub fn withdraw(&self, sender: AccountHash, amount: U512) {
        self.0.call_contract(
            sender,
            "withdraw",
            runtime_args! {
                "amount"=>amount,
                //"to"=>to.into(),
                // "wcspr_hash"=>wcspr_hash
            },
        );
    }

    pub fn deposit(&self, sender: AccountHash, amount: U512, proxy: Key) {
        self.0.call_contract(
            sender,
            "deposit_session",
            runtime_args! {
                "amount"=>amount,
                "proxy_hash"=>proxy
            },
        );
    }

    // pub fn deposit(&self, sender: AccountHash, amount:U512, purse: URef) {
    //     self.0.call_contract(sender,"deposit", runtime_args!{
    //         "amount"=>amount,
    //         "purse"=>purse
    //     });
    // }

    pub fn name(&self) -> String {
        self.0.query_named_key(String::from("name"))
    }

    pub fn symbol(&self) -> String {
        self.0.query_named_key(String::from("symbol"))
    }

    // Result methods
    pub fn transfer_result(&self) -> Result<(), u32> {
        self.0
            .query_named_key(TRANSFER_TEST_RESULT_KEY_NAME.to_string())
    }

    pub fn package_hash_result(&self) -> ContractPackageHash {
        self.0.query_named_key(PACKAGE_HASH_KEY_NAME.to_string())
    }

    pub fn contract_hash_result(&self) -> ContractHash {
        self.0.query_named_key(CONTRACT_HASH_KEY_NAME.to_string())
    }

    pub fn self_contract_hash_result(&self) -> Key {
        self.0
            .query_named_key(SELF_CONTRACT_HASH_KEY_NAME.to_string())
    }

    pub fn transfer_from_result(&self) -> Result<(), u32> {
        self.0
            .query_named_key(TRANSFER_FROM_TEST_RESULT_KEY_NAME.to_string())
    }

    pub fn deposit_result(&self) -> Result<(), u32> {
        self.0
            .query_named_key(DEPOSIT_TEST_RESULT_KEY_NAME.to_string())
    }

    pub fn withdraw_result(&self) -> Result<(), u32> {
        self.0
            .query_named_key(WITHDRAW_TEST_RESULT_KEY_NAME.to_string())
    }
    pub fn allowance_res(&self) -> U256 {
        self.0.query_named_key("allowance".to_string())
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
