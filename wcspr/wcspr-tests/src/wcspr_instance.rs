use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};

use casper_types::{
    bytesrepr::ToBytes, runtime_args, ContractPackageHash, Key, RuntimeArgs, URef, U256, U512,
};
use test_env::{Sender, TestContract, TestEnv};

pub const DEPOSIT_TEST_RESULT_KEY_NAME: &str = "deposit_test_result";
pub const WITHDRAW_TEST_RESULT_KEY_NAME: &str = "withdraw_test_result";
pub const TRANSFER_TEST_RESULT_KEY_NAME: &str = "transfer_test_result";
pub const TRANSFER_FROM_TEST_RESULT_KEY_NAME: &str = "transfer_from_test_result";
pub const PACKAGE_HASH_KEY_NAME: &str = "package_hash";
pub const CONTRACT_HASH_KEY_NAME: &str = "contract_hash";
pub const WCSPR_HASH_KEY_NAME: &str = "wcspr_hash";

pub struct WCSPRInstance(TestContract);
impl WCSPRInstance {
    pub fn new(
        env: &TestEnv,
        contract_name: &str,
        sender: Sender,
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

    pub fn proxy(env: &TestEnv, wcspr: Key, sender: Sender) -> TestContract {
        TestContract::new(
            env,
            "contract.wasm",
            "proxy_test",
            sender,
            runtime_args! {
                "wcspr" => wcspr
            },
        )
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

    pub fn withdraw<T: Into<Key>>(&self, sender: Sender, to: T, amount: U512) {
        self.0.call_contract(
            sender,
            "withdraw",
            runtime_args! {
                "amount"=>amount,
                "to"=>to.into()
            },
        );
    }

    pub fn deposit(&self, sender: Sender, amount: U512, purse: URef) {
        self.0.call_contract(
            sender,
            "deposit",
            runtime_args! {
                "amount"=>amount,
                "purse"=>purse
            },
        );
    }

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
