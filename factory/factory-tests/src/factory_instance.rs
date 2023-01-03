use tests_common::{account::AccountHash, deploys::deploy_factory, *};

pub struct FACTORYInstance(pub TestContract);

impl FACTORYInstance {
    pub fn new(
        env: &TestEnv,
        sender: AccountHash,
        fee_to_setter: Key,
        time: u64,
    ) -> FACTORYInstance {
        FACTORYInstance(deploy_factory(env, sender, fee_to_setter, time))
    }

    pub fn constructor<T: Into<Key>>(&self, sender: AccountHash, fee_to_setter: T, time: u64) {
        self.0.call_contract(
            sender,
            "constructor",
            runtime_args! {
                "fee_to_setter" => fee_to_setter.into(),
            },
            time,
        );
    }

    pub fn set_fee_to_setter<T: Into<Key>>(
        &self,
        sender: AccountHash,
        fee_to_setter: T,
        time: u64,
    ) {
        self.0.call_contract(
            sender,
            "set_fee_to_setter",
            runtime_args! {
                "fee_to_setter" => fee_to_setter.into(),
            },
            time,
        );
    }

    pub fn set_fee_to<T: Into<Key>>(&self, sender: AccountHash, fee_to: T, time: u64) {
        self.0.call_contract(
            sender,
            "set_fee_to",
            runtime_args! {
                "fee_to" => fee_to.into(),
            },
            time,
        );
    }

    pub fn create_pair<T: Into<Key>>(
        &self,
        sender: AccountHash,
        token_a: T,
        token_b: T,
        pair_hash: T,
        time: u64,
    ) {
        self.0.call_contract(
            sender,
            "create_pair",
            runtime_args! {
                "token_a" => token_a.into(),
                "token_b" => token_b.into(),
                "pair_hash" => pair_hash.into(),
            },
            time,
        );
    }

    pub fn remove_pair<T: Into<Key>>(&self, sender: AccountHash, pair_hash: T, time: u64) {
        self.0.call_contract(
            sender,
            "remove_pair",
            runtime_args! {
                "pair_hash" => pair_hash.into(),
            },
            time,
        );
    }

    pub fn set_white_list<T: Into<Key>>(&self, sender: AccountHash, white_list: T, time: u64) {
        self.0.call_contract(
            sender,
            "set_white_list",
            runtime_args! {
                "white_list" => white_list.into(),
            },
            time,
        );
    }

    pub fn get_white_lists<T: Into<Key>>(&self, account: T) -> (Key, Key) {
        self.0
            .query_dictionary("white_lists", helpers::key_to_str(&account.into()))
            .unwrap()
    }

    pub fn self_contract_hash(&self) -> ContractHash {
        self.0.query_named_key(String::from("self_contract_hash"))
    }

    pub fn contract_package_hash(&self) -> ContractPackageHash {
        self.0
            .query_named_key(String::from("contract_package_hash"))
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
            .query_dictionary("pairs", helpers::keys_to_str(&token0, &token1))
            .unwrap()
    }
}
