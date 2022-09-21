use alloc::string::String;

use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{ContractPackageHash, Key, U128, U256};
use contract_utils::{get_key, set_key, Dict};

pub const BALANCES_DICT: &str = "balances";
pub const NONCES_DICT: &str = "nonces";
pub const ALLOWANCES_DICT: &str = "allowances";
pub const NAME: &str = "name";
pub const SYMBOL: &str = "symbol";
pub const DECIMALS: &str = "decimals";
pub const TOTAL_SUPPLY: &str = "total_supply";
pub const SELF_CONTRACT_HASH: &str = "self_contract_hash";
pub const SELF_PACKAGE_HASH: &str = "self_package_hash";
pub const DOMAIN_SEPARATOR: &str = "domain_separator";
pub const FACTORY: &str = "factory_hash";
pub const RESERVE0: &str = "reserve0";
pub const RESERVE1: &str = "reserve1";
pub const BLOCK_TIMESTAMP_LAST: &str = "block_timestamp_last";
pub const PRICE0_CUMULATIVE_LAST: &str = "price0_cumulative_last";
pub const PRICE1_CUMULATIVE_LAST: &str = "price1_cumulative_last";
pub const K_LAST: &str = "k_last";
pub const TREASURY_FEE: &str = "treasury_fee";
pub const MINIMUM_LIQUIDITY: &str = "minimum_liquidity";
pub const TOKEN0: &str = "token0";
pub const TOKEN1: &str = "token1";
pub const LIQUIDITY: &str = "liquidity";
pub const AMOUNT0: &str = "amount0";
pub const AMOUNT1: &str = "amount1";
pub const CALLEE_PACKAGE_HASH: &str = "callee_package_hash";
pub const LOCK: &str = "lock";

pub struct Balances {
    dict: Dict,
}

impl Balances {
    pub fn instance() -> Balances {
        Balances {
            dict: Dict::instance(BALANCES_DICT),
        }
    }

    pub fn init() {
        Dict::init(BALANCES_DICT)
    }

    pub fn get(&self, owner: &Key) -> U256 {
        self.dict.get_by_key(owner).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, value: U256) {
        self.dict.set_by_key(owner, value);
    }
}

pub struct Nonces {
    dict: Dict,
}

impl Nonces {
    pub fn instance() -> Nonces {
        Nonces {
            dict: Dict::instance(NONCES_DICT),
        }
    }

    pub fn init() {
        Dict::init(NONCES_DICT)
    }

    pub fn get(&self, owner: &Key) -> U256 {
        self.dict.get_by_key(owner).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, value: U256) {
        self.dict.set_by_key(owner, value);
    }
}

pub struct Allowances {
    dict: Dict,
}

impl Allowances {
    pub fn instance() -> Allowances {
        Allowances {
            dict: Dict::instance(ALLOWANCES_DICT),
        }
    }

    pub fn init() {
        Dict::init(ALLOWANCES_DICT)
    }

    pub fn get(&self, owner: &Key, spender: &Key) -> U256 {
        self.dict.get_by_keys((owner, spender)).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, spender: &Key, value: U256) {
        self.dict.set_by_keys((owner, spender), value);
    }
}

pub fn name() -> String {
    get_key(NAME).unwrap_or_revert()
}

pub fn set_name(name: String) {
    set_key(NAME, name);
}

pub fn symbol() -> String {
    get_key(SYMBOL).unwrap_or_revert()
}

pub fn set_symbol(symbol: String) {
    set_key(SYMBOL, symbol);
}

pub fn decimals() -> u8 {
    get_key(DECIMALS).unwrap_or_revert()
}

pub fn set_decimals(decimals: u8) {
    set_key(DECIMALS, decimals);
}

pub fn total_supply() -> U256 {
    get_key(TOTAL_SUPPLY).unwrap_or_default()
}

pub fn set_total_supply(total_supply: U256) {
    set_key(TOTAL_SUPPLY, total_supply);
}

pub fn set_hash(contract_hash: Key) {
    set_key(SELF_CONTRACT_HASH, contract_hash);
}

pub fn get_hash() -> Key {
    get_key(SELF_CONTRACT_HASH).unwrap_or_revert()
}
pub fn set_package_hash(package_hash: ContractPackageHash) {
    set_key(SELF_PACKAGE_HASH, package_hash);
}

pub fn get_package_hash() -> ContractPackageHash {
    get_key(SELF_PACKAGE_HASH).unwrap_or_revert()
}

pub fn set_factory_hash(factory_hash: Key) {
    set_key(FACTORY, factory_hash);
}

pub fn get_factory_hash() -> Key {
    get_key(FACTORY).unwrap_or_revert()
}

pub fn set_token0(token0: Key) {
    set_key(TOKEN0, token0);
}

pub fn get_token0() -> Key {
    get_key(TOKEN0).unwrap_or_revert()
}

pub fn set_token1(token1: Key) {
    set_key(TOKEN1, token1);
}

pub fn get_token1() -> Key {
    get_key(TOKEN1).unwrap_or_revert()
}

pub fn set_domain_separator(domain_separator: String) {
    set_key(DOMAIN_SEPARATOR, domain_separator);
}

pub fn get_domain_separator() -> String {
    get_key(DOMAIN_SEPARATOR).unwrap_or_revert()
}

pub fn set_reserve0(reserve0: U128) {
    set_key(RESERVE0, reserve0);
}

pub fn get_reserve0() -> U128 {
    get_key(RESERVE0).unwrap_or_revert()
}

pub fn set_reserve1(reserve1: U128) {
    set_key(RESERVE1, reserve1);
}

pub fn get_reserve1() -> U128 {
    get_key(RESERVE1).unwrap_or_revert()
}

pub fn set_block_timestamp_last(block_timestamp_last: u64) {
    set_key(BLOCK_TIMESTAMP_LAST, block_timestamp_last);
}

pub fn get_block_timestamp_last() -> u64 {
    get_key(BLOCK_TIMESTAMP_LAST).unwrap_or_revert()
}

pub fn set_price0_cumulative_last(price0_cumulative_last: U256) {
    set_key(PRICE0_CUMULATIVE_LAST, price0_cumulative_last);
}

pub fn get_price0_cumulative_last() -> U256 {
    get_key(PRICE0_CUMULATIVE_LAST).unwrap_or_revert()
}

pub fn set_price1_cumulative_last(price1_cumulative_last: U256) {
    set_key(PRICE1_CUMULATIVE_LAST, price1_cumulative_last);
}

pub fn get_price1_cumulative_last() -> U256 {
    get_key(PRICE1_CUMULATIVE_LAST).unwrap_or_revert()
}

pub fn set_k_last(k_last: U256) {
    set_key(K_LAST, k_last);
}

pub fn get_k_last() -> U256 {
    get_key(K_LAST).unwrap_or_revert()
}

pub fn set_treasury_fee(treasury_fee: U256) {
    set_key(TREASURY_FEE, treasury_fee);
}

pub fn get_treasury_fee() -> U256 {
    get_key(TREASURY_FEE).unwrap_or_revert()
}

pub fn set_minimum_liquidity(minimum_liquidity: U256) {
    set_key(MINIMUM_LIQUIDITY, minimum_liquidity);
}

pub fn get_minimum_liquidity() -> U256 {
    get_key(MINIMUM_LIQUIDITY).unwrap_or_revert()
}

pub fn set_liquidity(liquidity: U256) {
    set_key(LIQUIDITY, liquidity);
}

pub fn get_liquidity() -> U256 {
    get_key(LIQUIDITY).unwrap_or_revert()
}

pub fn set_amount0(amount0: U256) {
    set_key(AMOUNT0, amount0);
}

pub fn get_amount0() -> U256 {
    get_key(AMOUNT0).unwrap_or_revert()
}

pub fn set_amount1(amount1: U256) {
    set_key(AMOUNT1, amount1);
}

pub fn get_amount1() -> U256 {
    get_key(AMOUNT1).unwrap_or_revert()
}

pub fn set_callee_package_hash(callee_package_hash: Key) {
    set_key(CALLEE_PACKAGE_HASH, callee_package_hash);
}

pub fn get_callee_package_hash() -> Key {
    get_key(CALLEE_PACKAGE_HASH).unwrap_or_revert()
}

pub fn set_lock(lock: u64) {
    set_key(LOCK, lock);
}

pub fn get_lock() -> u64 {
    get_key(LOCK).unwrap_or_revert()
}
