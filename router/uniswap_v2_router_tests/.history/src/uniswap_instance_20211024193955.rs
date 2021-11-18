use casper_types::{runtime_args, ContractHash, ContractPackageHash, Key, RuntimeArgs, U256};
use test_env::{Sender, TestContract, TestEnv};

use cryptoxide::ed25519;
use renvm_sig::hash_message;
use renvm_sig::keccak256;

pub struct UniswapInstance(TestContract);

impl UniswapInstance {
    pub fn new(env: &TestEnv, router_address: Key, sender: Sender) -> UniswapInstance {
        UniswapInstance(TestContract::new(
            env,
            "test_contract.wasm",
            "RouterTest",
            sender,
            runtime_args! {
                "router_address" => router_address
                // contract_name is passed seperately, so we don't need to pass it here.
            },
        ))
    }

    pub fn constructor(
        &self,
        sender: Sender,
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

    pub fn add_liquidity(
        &self,
        sender: Sender,
        token_a: Key,
        token_b: Key,
        amount_a_desired: U256,
        amount_b_desired: U256,
        amount_a_min: U256,
        amount_b_min: U256,
        to: Key,
        deadline: U256,
        pair: Option<Key>
    ) {
        self.0.call_contract(
            sender,
            "add_liquidity",
            runtime_args! {
                "token_a" => token_a,
                "token_b" => token_b,
                "amount_a_desired" => amount_a_desired,
                "amount_b_desired" => amount_b_desired,
                "amount_a_min" => amount_a_min,
                "amount_b_min" => amount_b_min,
                "to" => to,
                "deadline" => deadline,
                "pair" => pair
            },
        );
    }

    pub fn add_liquidity_cspr(
        &self,
        sender: Sender,
        token: Key,
        amount_token_desired: U256,
        amount_cspr_desired: U256,
        amount_token_min: U256,
        amount_cspr_min: U256,
        to: Key,
        deadline: U256,
        pair: Option<Key>,
        router: Key,
        test_package_hash: Key
    ) {
        self.0.call_contract(
            sender,
            "add_liquidity_cspr",
            runtime_args! {
                "token" => token,
                "amount_token_desired" => amount_token_desired,
                "amount_cspr_desired" => amount_cspr_desired,
                "amount_token_min" => amount_token_min,
                "amount_cspr_min" => amount_cspr_min,
                "to" => to,
                "deadline" => deadline,
                "pair" => pair,
                "router_hash" => router,
                "self_package_hash" => test_package_hash
            },
        );
    }

    pub fn remove_liquidity(
        &self,
        sender: Sender,
        token_a: Key,
        token_b: Key,
        liquidity: U256,
        amount_a_min: U256,
        amount_b_min: U256,
        to: Key,
        deadline: U256,
    ) {
        self.0.call_contract(
            sender,
            "remove_liquidity",
            runtime_args! {
                "token_a" => token_a,
                "token_b" => token_b,
                "liquidity" => liquidity,
                "amount_a_min" => amount_a_min,
                "amount_b_min" => amount_b_min,
                "to" => to,
                "deadline" => deadline
            },
        );
    }

    pub fn remove_liquidity_cspr(
        &self,
        sender: Sender,
        token: Key,
        liquidity: U256,
        amount_token_min: U256,
        amount_cspr_min: U256,
        to: Key,
        deadline: U256,
    ) {
        self.0.call_contract(
            sender,
            "remove_liquidity_cspr",
            runtime_args! {
                "token" => token,
                "liquidity" => liquidity,
                "amount_token_min" => amount_token_min,
                "amount_cspr_min" => amount_cspr_min,
                "to" => to,
                "deadline" => deadline
            },
        );
    }

    pub fn remove_liquidity_with_permit(
        &self,
        sender: Sender,
        token_a: Key,
        token_b: Key,
        liquidity: U256,
        amount_a_min: U256,
        amount_b_min: U256,
        to: Key,
        deadline: U256,
        approve_max: bool,
        public_key: String,
        signature: String,
    ) {
        self.0.call_contract(
            sender,
            "remove_liquidity_with_permit",
            runtime_args! {
                "token_a" => token_a,
                "token_b" => token_b,
                "liquidity" => liquidity,
                "amount_a_min" => amount_a_min,
                "amount_b_min" => amount_b_min,
                "to" => to,
                "deadline" => deadline,
                "approve_max" => approve_max,
                "public_key" => public_key,
                "signature" => signature
            },
        );
    }

    pub fn remove_liquidity_cspr_with_permit(
        &self,
        sender: Sender,
        token: Key,
        liquidity: U256,
        amount_token_min: U256,
        amount_cspr_min: U256,
        to: Key,
        deadline: U256,
        approve_max: bool,
        public_key: String,
        signature: String,
    ) {
        self.0.call_contract(
            sender,
            "remove_liquidity_cspr_with_permit",
            runtime_args! {
                "token" => token,
                "liquidity" => liquidity,
                "amount_token_min" => amount_token_min,
                "amount_cspr_min" => amount_cspr_min,
                "to" => to,
                "deadline" => deadline,
                "approve_max" => approve_max,
                "public_key" => public_key,
                "signature" => signature
            },
        );
    }

    pub fn swap_exact_tokens_for_tokens(
        &self,
        sender: Sender,
        amount_in: U256,
        amount_out_min: U256,
        path: Vec<Key>,
        to: Key,
        deadline: U256,
    ) {
        self.0.call_contract(
            sender,
            "swap_exact_tokens_for_tokens",
            runtime_args! {
                "amount_in" => amount_in,
                "amount_out_min" => amount_out_min,
                "path" => path,
                "to" => to,
                "deadline" => deadline
            },
        );
    }

    pub fn swap_tokens_for_exact_tokens(
        &self,
        sender: Sender,
        amount_out: U256,
        amount_in_max: U256,
        path: Vec<Key>,
        to: Key,
        deadline: U256,
    ) {
        self.0.call_contract(
            sender,
            "swap_tokens_for_exact_tokens",
            runtime_args! {
                "amount_out" => amount_out,
                "amount_in_max" => amount_in_max,
                "path" => path,
                "to" => to,
                "deadline" => deadline
            },
        );
    }

    pub fn swap_exact_cspr_for_tokens(
        &self,
        sender: Sender,
        amount_out_min: U256,
        amount_in: U256,
        path: Vec<Key>,
        to: Key,
        deadline: U256,
    ) {
        self.0.call_contract(
            sender,
            "swap_exact_cspr_for_tokens",
            runtime_args! {
                "amount_out_min" => amount_out_min,
                "amount_in" => amount_in,
                "path" => path,
                "to" => to,
                "deadline" => deadline
            },
        );
    }

    pub fn swap_tokens_for_exact_cspr(
        &self,
        sender: Sender,
        amount_out: U256,
        amount_in_max: U256,
        path: Vec<Key>,
        to: Key,
        deadline: U256,
    ) {
        self.0.call_contract(
            sender,
            "swap_tokens_for_exact_cspr",
            runtime_args! {
                "amount_out" => amount_out,
                "amount_in_max" => amount_in_max,
                "path" => path,
                "to" => to,
                "deadline" => deadline
            },
        );
    }

    pub fn swap_exact_tokens_for_cspr(
        &self,
        sender: Sender,
        amount_in: U256,
        amount_out_min: U256,
        path: Vec<Key>,
        to: Key,
        deadline: U256,
    ) {
        self.0.call_contract(
            sender,
            "swap_exact_tokens_for_cspr",
            runtime_args! {
                "amount_in" => amount_in,
                "amount_out_min" => amount_out_min,
                "path" => path,
                "to" => to,
                "deadline" => deadline
            },
        );
    }

    pub fn swap_cspr_for_exact_tokens(
        &self,
        sender: Sender,
        amount_out: U256,
        amount_in_max: U256,
        path: Vec<Key>,
        to: Key,
        deadline: U256,
    ) {
        self.0.call_contract(
            sender,
            "swap_cspr_for_exact_tokens",
            runtime_args! {
                "amount_in_max" => amount_in_max,
                "amount_out" => amount_out,
                "path" => path,
                "to" => to,
                "deadline" => deadline
            },
        );
    }

    pub fn approve(&self, token: &TestContract, sender: Sender, spender: Key, amount: U256) {
        token.call_contract(
            sender,
            "approve",
            runtime_args! {
                "spender" => spender,
                "amount" => amount
            },
        );
    }

    pub fn balance_of<T: Into<Key>>(&self, token: &TestContract, account: T) -> U256 {
        token
            .query_dictionary("balances", key_to_str(&account.into()))
            .unwrap_or_default()
    }

    pub fn test_contract_package_hash(&self) -> Key {
        let package_hash: ContractPackageHash = self.0.query_named_key("package_hash".to_string());
        Key::from(package_hash)
    }

    pub fn test_contract_hash(&self) -> Key {
        let contract_hash: ContractHash = self.0.query_named_key("self_hash".to_string());
        Key::from(contract_hash)
    }
 

    /*
    pub fn uniswap_contract_address(&self) -> Key {
        let self_hash: ContractHash = self.0.query_named_key("self_hash".to_string());
        Key::from(self_hash)
    }

    pub fn uniswap_contract_package_hash(&self) -> Key {
        let package: ContractPackageHash = self.0.query_named_key("package_hash".to_string());
        package.into()
    }

    pub fn uniswap_router_address(&self) -> Key {
        let router_hash: ContractHash = self.0.query_named_key("router_hash".to_string());
        Key::from(router_hash)
    }

    pub fn uniswap_pair_address(&self) -> ContractHash {
        self.0.query_named_key(String::from("pair_hash"))
    }
    */
    pub fn calculate_signature(&self, data: &String, domainseparator: &String) -> (String, String) {
        let hash = keccak256(data.as_bytes());
        let hashstring = hex::encode(hash);
        let data2: String = format!("{}{}", domainseparator, hashstring);
        let geteip191standard_hash = hash_message(data2);

        let secret = "MC4CAQAwBQYDK2VwBCIEIPPGVic1+UO0UJJJRTHaBkpH/05oaDQacEinXQnKoaIu".as_bytes();
        let public = ed25519::to_public(secret);
        let signature = ed25519::signature_extended(&geteip191standard_hash, &secret);

        let signature = signature.to_vec();
        let public = public.to_vec();

        let signature_str = format!("{:?}", &signature);
        let public_str = format!("{:?}", &public);

        let mut signature_str = signature_str.replace("[", "");
        signature_str = signature_str.replace("]", "");

        let mut public_str = public_str.replace("[", "");
        public_str = public_str.replace("]", "");

        (signature_str, public_str)
    }

    // Result methods
    pub fn add_liquidity_result(&self) -> (U256, U256, U256) {
        let (amount_a, amount_b, liquidity): (U256, U256, U256) =
            self.0.query_named_key("add_liquidity_result".to_string());
        (amount_a, amount_b, liquidity)
    }

    pub fn add_liquidity_cspr_result(&self) -> (U256, U256, U256) {
        let (amount_token, amount_cspr, liquidity): (U256, U256, U256) = self
            .0
            .query_named_key("add_liquidity_cspr_result".to_string());
        (amount_token, amount_cspr, liquidity)
    }

    pub fn remove_liquidity_result(&self) -> (U256, U256) {
        let (amount_a, amount_b): (U256, U256) = self
            .0
            .query_named_key("remove_liquidity_result".to_string());
        (amount_a, amount_b)
    }

    pub fn remove_liquidity_cspr_result(&self) -> (U256, U256) {
        let (amount_token, amount_cspr): (U256, U256) = self
            .0
            .query_named_key("remove_liquidity_cspr_result".to_string());
        (amount_token, amount_cspr)
    }

    pub fn remove_liquidity_with_permit_result(&self) -> (U256, U256) {
        let (amount_a, amount_b): (U256, U256) = self
            .0
            .query_named_key("remove_liquidity_with_permit_result".to_string());
        (amount_a, amount_b)
    }

    pub fn remove_liquidity_cspr_with_permit_result(&self) -> (U256, U256) {
        let (amount_a, amount_b): (U256, U256) = self
            .0
            .query_named_key("remove_liquidity_cspr_with_permit_result".to_string());
        (amount_a, amount_b)
    }
}

pub fn key_to_str(key: &Key) -> String {
    match key {
        Key::Account(account) => account.to_string(),
        Key::Hash(package) => hex::encode(package),
        _ => panic!("Unexpected key type"),
    }
}
