use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{Key, U128, U256};
use casperlabs_contract_utils::{get_key, set_key};

const FACTORY: &str = "factory_hash";
const RESERVE0: &str = "reserve0";
const RESERVE1: &str = "reserve1";
const BLOCK_TIMESTAMP_LAST: &str = "block_timestamp_last";
const PRICE0_CUMULATIVE_LAST: &str = "price0_cumulative_last";
const PRICE1_CUMULATIVE_LAST: &str = "price1_cumulative_last";
const K_LAST: &str = "k_last";
const TREASURY_FEE: &str = "treasury_fee";
const MINIMUM_LIQUIDITY: &str = "minimum_liquidity";
const TOKEN0: &str = "token0";
const TOKEN1: &str = "token1";
const LIQUIDITY: &str = "liquidity";
const AMOUNT0: &str = "amount0";
const AMOUNT1: &str = "amount1";
const CALLEE_PACKAGE_HASH: &str = "callee_package_hash";
const LOCK: &str = "lock";

pub fn account_zero() -> Key {
    Key::from_formatted_str(
        "account-hash-0000000000000000000000000000000000000000000000000000000000000000",
    )
    .unwrap()
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
