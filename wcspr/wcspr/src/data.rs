use crate::alloc::string::ToString;
use alloc::string::String;
use casper_contract::contract_api::runtime;
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{ApiError, URef, U512};

#[repr(u16)]
pub enum Error {
    UniswapV2CoreWCSPROverFlow1 = 15,
    /// 65,552 for (UniswapV2 Core WCSPR OverFlow6)
    UniswapV2CoreWCSPROverFlow2 = 16,
    /// 65,553 for (UniswapV2 Core WCSPR OverFlow7)
    UniswapV2CoreWCSPROverFlow3 = 17,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}

pub enum WcsprEvents {
    Deposit { purse: URef, amount: U512 },
    Withdraw { purse: URef, amount: U512 },
}

impl WcsprEvents {
    pub fn type_name(&self) -> String {
        match self {
            WcsprEvents::Deposit {
                purse: _,
                amount: _,
            } => "deposit",
            WcsprEvents::Withdraw {
                purse: _,
                amount: _,
            } => "withdraw",
        }
        .to_string()
    }
}

pub const SELF_PURSE: &str = "self_purse";

pub fn set_self_purse(purse: URef) {
    runtime::put_key(SELF_PURSE, purse.into());
}

pub fn get_self_purse() -> URef {
    match runtime::get_key(SELF_PURSE).unwrap_or_revert().as_uref() {
        Some(uref) => *uref,
        None => runtime::revert(ApiError::InvalidPurse),
    }
}
