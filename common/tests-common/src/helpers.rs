use casper_types::{bytesrepr::ToBytes, U256, U512};
use casperlabs_erc20::Address;

pub const BALANCES: &str = "balances";
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
