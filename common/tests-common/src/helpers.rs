use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use common::{bytesrepr::ToBytes, *};
use hex::encode;

pub const BALANCES: &str = "balances";
pub const ALLOWANCES: &str = "allowances";
pub const TREASURY_FEE: &str = "treasury_fee";

pub const NAME: &str = "ERC20";
pub const SYMBOL: &str = "ERC";
pub const DECIMALS: u8 = 9;
pub const INIT_TOTAL_SUPPLY: U256 = U256([0, 0, 0, 0]);
pub const AMOUNT: U256 = U256([100_000_000_000, 0, 0, 0]);
pub const AMOUNT_U512: U512 = U512([100_000_000_000, 0, 0, 0, 0, 0, 0, 0]);
pub const WRAPPED_CSPR: &str = "Wrapped CSPR";

pub fn address_to_str(owner: &Address) -> String {
    let preimage = owner.to_bytes().unwrap();
    base64::encode(&preimage)
}

pub fn addresses_to_str(owner: Address, spender: Address) -> String {
    let mut hasher = VarBlake2b::new(32).unwrap();
    hasher.update(owner.to_bytes().unwrap());
    hasher.update(spender.to_bytes().unwrap());

    let mut ret = [0u8; 32];
    hasher.finalize_variable(|hash| ret.clone_from_slice(hash));

    encode(ret)
}

pub fn key_to_str(key: &Key) -> String {
    match key {
        Key::Account(account) => account.to_string(),
        Key::Hash(package) => encode(package),
        _ => panic!("Unexpected key type"),
    }
}

pub fn keys_to_str(key_a: &Key, key_b: &Key) -> String {
    let mut hasher = VarBlake2b::new(32).unwrap();
    hasher.update(key_a.to_bytes().unwrap());
    hasher.update(key_b.to_bytes().unwrap());

    let mut ret = [0u8; 32];
    hasher.finalize_variable(|hash| ret.clone_from_slice(hash));

    encode(ret)
}
