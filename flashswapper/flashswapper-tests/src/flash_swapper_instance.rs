use tests_common::{account::AccountHash, contract_api::runtime, *};

pub struct FlashSwapperInstance(TestContract);

impl FlashSwapperInstance {
    pub fn new(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        wcspr: Key,
        dai: Key,
        uniswap_v2_factory: Key,
        time: u64,
    ) -> FlashSwapperInstance {
        FlashSwapperInstance(TestContract::new(
            env,
            "flashswapper-token.wasm",
            contract_name,
            sender,
            runtime_args! {
                "uniswap_v2_factory" => uniswap_v2_factory,
                "wcspr" => wcspr,
                "dai" => dai,
            },
            time,
        ))
    }

    pub fn mint_with_caller(&self, caller: Key, recipient: Key, amount: U256) {
        let caller_hash_add_array = match caller {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        let caller_hash_add = ContractHash::new(caller_hash_add_array);
        let _ret: () = runtime::call_contract(
            caller_hash_add,
            "mint",
            runtime_args! {"to" => recipient, "amount" => amount},
        );
    }

    pub fn constructor(
        &self,
        sender: AccountHash,
        wcspr: Key,
        dai: Key,
        uniswap_v2_factory: Key,
        time: u64,
    ) {
        self.0.call_contract(
            sender,
            "constructor",
            runtime_args! {
                "wcspr" => wcspr,
                "dai" => dai,
                "uniswap_v2_factory" => uniswap_v2_factory,
            },
            time,
        );
    }

    pub fn start_swap(
        &self,
        sender: AccountHash,
        token_borrow: Key,
        amount: U256,
        token_pay: Key,
        user_data: String,
        time: u64,
    ) {
        self.0.call_contract(
            sender,
            "start_swap",
            runtime_args! {
                "token_borrow" => token_borrow,
                "amount" => amount,
                "token_pay" => token_pay,
                "user_data" => user_data,
            },
            time,
        );
    }

    pub fn uniswap_v2_call(
        &self,
        sender: AccountHash,
        _sender: Key,
        amount0: U256,
        amount1: U256,
        data: String,
        time: u64,
    ) {
        self.0.call_contract(
            sender,
            "uniswap_v2_call",
            runtime_args! {
                "sender" => _sender,
                "amount0" => amount0,
                "amount1" => amount1,
                "data" => data
            },
            time,
        );
    }

    pub fn self_contract_hash(&self) -> ContractHash {
        self.0.query_named_key(String::from("self_contract_hash"))
    }

    pub fn balance_pair(&self) -> U256 {
        self.0.query_named_key(String::from("pair_balance"))
    }

    pub fn amount0(&self) -> U256 {
        self.0.query_named_key(String::from("amount0"))
    }
}
