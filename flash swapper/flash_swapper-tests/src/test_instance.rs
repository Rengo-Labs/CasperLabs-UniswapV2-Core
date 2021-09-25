use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use casper_types::{bytesrepr::ToBytes, runtime_args, Key, RuntimeArgs, U256};
use test_env::{Sender, TestContract, TestEnv};

pub struct TESTInstance(TestContract);

impl TESTInstance {
    pub fn new(
        env: &TestEnv,
        contract_name: &str,
        sender: Sender,
        name: &str,

    ) -> TESTInstance {
        TESTInstance(TestContract::new(
            env,
            "test-token.wasm",
            contract_name,
            sender,
            runtime_args! {
                "name" => name,
            },
        ))
    }

    pub fn constructor(
        &self,
        sender: Sender,
        name: &str,
    ) {
        self.0.call_contract(
            sender,
            "constructor",
            runtime_args! {
                "name" => name,
            },
        );
    }

  
    
    pub fn mint_with_caller<T: Into<Key>>(&self, sender: Sender, caller: T, to: Key, amount: U256) {
        self.0.call_contract(
            sender,
            "mint_with_caller",
            runtime_args! {
                "caller" => caller.into(),
                "to" => to,
                "amount" => amount
            },
        );
    }
    pub fn pair_mint<T: Into<Key>>(&self, sender: Sender, caller: T, to: Key, amount: U256) {
        self.0.call_contract(
            sender,
            "pair_mint",
            runtime_args! {
                "caller" => caller.into(),
                "to" => to,
                "amount" => amount
            },
        );
    }
    pub fn balance<T: Into<Key>>(&self, sender: Sender, token: T, owner: Key) {
        self.0.call_contract(
            sender,
            "balance",
            runtime_args! {
                "token" => token.into(),
                "owner" => owner,
            },
        );
    }
    pub fn token0<T: Into<Key>>(&self, sender: Sender, pair: T) {
        self.0.call_contract(
            sender,
            "token0",
            runtime_args! {
                "pair" => pair.into(),
            },
        );
    }
    pub fn token1<T: Into<Key>>(&self, sender: Sender, pair: T) {
        self.0.call_contract(
            sender,
            "token1",
            runtime_args! {
                "pair" => pair.into(),
            },
        );
    }
    pub fn get_token0(&self) -> Key{
        self.0.query_named_key(String::from("token0"))
    }
    pub fn get_token1(&self) -> Key{
        self.0.query_named_key(String::from("token1"))
    }
    pub fn balance_of(&self) -> U256 {
        self.0.query_named_key(String::from("Balance"))
    }
    pub fn name(&self) -> String {
        self.0.query_named_key(String::from("name"))
    }
    pub fn get_fee_to(&self) -> Key {
        self.0.query_named_key(String::from("fee_to"))
    }
    pub fn self_contract_hash(&self) -> Key {
        self.0.query_named_key(String::from("self_contract_hash"))
    }
    // Factory Method
    pub fn set_fee_to<T: Into<Key>>(&self, sender: Sender, fee_to: T, factory_hash: Key) {
        self.0.call_contract(
            sender,
            "set_fee_to",
            runtime_args! {
                "fee_to" => fee_to.into(),
                "factory_hash" => factory_hash
            },
        );
    }

    // Factory Method
    pub fn create_pair(&self, sender: Sender, token_a: Key, token_b: Key, pair_hash: Key, factory_hash: Key) {
        
        self.0.call_contract(
            sender,
            "create_pair",
            runtime_args! {
                "token_a" => token_a,
                "token_b" => token_b,
                "pair_hash" => pair_hash,
                "factory_hash" => factory_hash
            },
        );
    }
    // Pair Method
    pub fn sync(&self, sender: Sender, pair_hash: Key) {
        
        self.0.call_contract(
            sender,
            "sync",
            runtime_args! {
                "pair_hash" => pair_hash,
            },
        );
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

