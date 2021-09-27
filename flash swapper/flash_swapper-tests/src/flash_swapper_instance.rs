use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use casper_types::{ContractHash, Key,ApiError, RuntimeArgs, U256, bytesrepr::ToBytes, runtime_args};
use test_env::{Sender, TestContract, TestEnv};
pub struct FlashSwapperInstance(TestContract);

use casper_contract::{
    contract_api::{runtime}
};

impl FlashSwapperInstance {
    pub fn new(
        env: &TestEnv,
        contract_name: &str,
        sender: Sender,
        wcspr: Key,
        dai: Key,
        uniswap_v2_factory: Key,
    ) -> FlashSwapperInstance {
        FlashSwapperInstance(TestContract::new(
            env,
            "flash-swapper.wasm",
            contract_name,
            sender,
            runtime_args! {
                "uniswap_v2_factory" => uniswap_v2_factory,
                "wcspr" => wcspr,
                "dai" => dai,
            },
        ))
    }

    pub fn mint_with_caller(&self, caller:Key, recipient: Key, amount: U256) {
        let caller_hash_add_array = match caller {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        let caller_hash_add = ContractHash::new(caller_hash_add_array);
        let _ret: () = runtime::call_contract(caller_hash_add,"mint",runtime_args!{"to" => recipient, "amount" => amount});

    }

    pub fn constructor(&self, sender: Sender, wcspr: Key, dai: Key, uniswap_v2_factory: Key) {
        self.0.call_contract(
            sender,
            "constructor",
            runtime_args! {
                "wcspr" => wcspr,
                "dai" => dai,
                "uniswap_v2_factory" => uniswap_v2_factory,
            },
        );
    }

    pub fn start_swap(&self, sender: Sender, token_borrow: Key, amount: U256, token_pay: Key, user_data: String) {
        self.0.call_contract(
            sender,
            "start_swap",
            runtime_args! {
                "token_borrow" => token_borrow,
                "amount" => amount,
                "token_pay" => token_pay,
                "user_data" => user_data,
            },
        );
    }

    pub fn uniswap_v2_call(&self, sender: Sender, _sender: Key, amount0: U256, amount1: U256, data: String) {
        self.0.call_contract(
            sender,
            "uniswap_v2_call",
            runtime_args! {
                "sender" => _sender,
                "amount0" => amount0,
                "amount1" => amount1,
                "data" => data
            },
        );
    }

    pub fn self_contract_hash(&self) -> Key {
        self.0.query_named_key(String::from("self_contract_hash"))
    }

    pub fn balance_pair(&self) -> U256 {
        self.0.query_named_key(String::from("pair_balance"))
    }
    
    pub fn amount0(&self) -> U256 {
        self.0.query_named_key(String::from("amount0"))
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
