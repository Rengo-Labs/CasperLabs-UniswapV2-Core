#[repr(u16)]
pub enum ErrorCodes {
    Abort = 35,
    TimedOut,
}

pub mod uniswapv2_contract_methods {

    // FACTORY Contract Methods
    pub const FACTORY_GET_PAIR: &str = "get_pair";
    pub const FACTORY_CREATE_PAIR: &str = "create_pair";

    // Library Contract Methods
    pub const LIBRARY_GET_RESERVES: &str = "get_reserves";
    pub const LIBRARY_QUOTE: &str = "quote";
    pub const LIBRARY_PAIR_FOR: &str = "pair_for";
    pub const LIBRARY_SAFE_TRANSFER_FROM: &str = "safe_transfer_from";
    pub const LIBRARY_SORT_TOKENS: &str = "sort_tokens";
    pub const LIBRARY_GET_AMOUNTS_OUT: &str = "get_amounts_out";
    pub const LIBRARY_GET_AMOUNTS_IN: &str = "get_amounts_in";
    pub const LIBRARY_GET_AMOUNT_OUT: &str = "get_amount_out";
    pub const LIBRARY_GET_AMOUNT_IN: &str = "get_amount_in";

    // Pair Contract Methods
    pub const PAIR_MINT: &str = "mint";
    pub const PAIR_TRANSFER_FROM: &str = "transfer_from";
    pub const PAIR_BURN: &str = "burn";
    pub const PAIR_PERMIT: &str = "permit";
    pub const PAIR_SWAP: &str = "swap";

    // IWETH Contract methods
    pub const WCSPR_DEPOSIT: &str = "deposit";
    pub const WCSPR_TRANSFER: &str = "transfer";
    pub const WCSPR_TRANSFER_FROM: &str = "transfer_from";
    pub const WCSPR_WITHDRAW: &str = "withdraw";
}
