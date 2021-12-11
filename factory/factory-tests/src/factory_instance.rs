use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use casper_types::{bytesrepr::ToBytes, runtime_args, Key, RuntimeArgs};
use test_env::{Sender, TestContract, TestEnv};

pub struct FACTORYInstance(pub TestContract);

impl FACTORYInstance {
    pub fn new<T: Into<Key>>(
        env: &TestEnv,
        contract_name: &str,
        sender: Sender,
        fee_to_setter: T,
    ) -> FACTORYInstance {
        FACTORYInstance(TestContract::new(
            env,
            "factory.wasm",
            contract_name,
            sender,
            runtime_args! {
                "fee_to_setter" => fee_to_setter.into(),
            },
        ))
    }

    pub fn constructor<T: Into<Key>>(&self, sender: Sender, fee_to_setter: T) {
        self.0.call_contract(
            sender,
            "constructor",
            runtime_args! {
                "fee_to_setter" => fee_to_setter.into(),
            },
        );
    }

    pub fn set_fee_to_setter<T: Into<Key>>(&self, sender: Sender, fee_to_setter: T) {
        self.0.call_contract(
            sender,
            "set_fee_to_setter",
            runtime_args! {
                "fee_to_setter" => fee_to_setter.into(),
            },
        );
    }

    pub fn set_fee_to<T: Into<Key>>(&self, sender: Sender, fee_to: T) {
        self.0.call_contract(
            sender,
            "set_fee_to",
            runtime_args! {
                "fee_to" => fee_to.into(),
            },
        );
    }

    pub fn create_pair<T: Into<Key>>(&self, sender: Sender, token_a: T, token_b: T, pair_hash: T) {
        self.0.call_contract(
            sender,
            "create_pair",
            runtime_args! {
            "token_a" => token_a.into(),
            "token_b" => token_b.into(),
            "pair_hash" => pair_hash.into(),
            },
        );
    }

    pub fn set_white_list<T: Into<Key>>(&self, sender: Sender, white_list: T) {
        self.0.call_contract(
            sender,
            "set_white_list",
            runtime_args! {
                "white_list" => white_list.into(),
            },
        );
    }

    pub fn get_white_lists<T: Into<Key>>(&self, account: T) -> Key {
        self.0
            .query_dictionary("white_lists", key_to_str(&account.into()))
            .unwrap()
    }

    pub fn self_contract_hash(&self) -> Key {
        self.0.query_named_key(String::from("self_contract_hash"))
    }

    pub fn fee_to(&self) -> Key {
        self.0.query_named_key(String::from("fee_to"))
    }

    pub fn fee_to_setter(&self) -> Key {
        self.0.query_named_key(String::from("fee_to_setter"))
    }

    pub fn all_pairs(&self) -> Vec<Key> {
        self.0.query_named_key(String::from("all_pairs"))
    }

    pub fn get_pair<T: Into<Key>>(&self, token0: T, token1: T) -> Key {
        let token0: Key = token0.into();
        let token1: Key = token1.into();
        self.0
            .query_dictionary("pairs", keys_to_str(&token0, &token1))
            .unwrap()
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
