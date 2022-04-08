#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;
use alloc::string::String;
use casper_contract::{
    contract_api::{account, runtime, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{runtime_args, ApiError, ContractPackageHash, Key, RuntimeArgs, URef, U512};

pub const DESTINATION_DEPOSIT: &str = "deposit";
pub const DESTINATION_WITHDRAW: &str = "withdraw";
pub const DESTINATION_GET_PURSE_BALANCE: &str = "get_purse_balance";
pub const AMOUNT_RUNTIME_ARG: &str = "amount";
pub const PURSE_RUNTIME_ARG: &str = "purse";
pub const TO_PURSE_RUNTIME_ARG: &str = "to_purse";

#[repr(u32)]
pub enum Error {
    Abort = 0,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}

#[no_mangle]
pub extern "C" fn call() {
    let destination_package_hash: Key = runtime::get_named_arg("destination_package_hash");
    let destination_entrypoint: String = runtime::get_named_arg("destination_entrypoint");

    let main_purse: URef = account::get_main_purse();

    let ret: Result<(), u32> = match destination_entrypoint.as_str() {
        DESTINATION_DEPOSIT => {
            let amount: U512 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG);
            let secondary_purse: URef = system::create_purse();
            system::transfer_from_purse_to_purse(main_purse, secondary_purse, amount, None)
                .unwrap_or_revert();

            runtime::call_versioned_contract(
                ContractPackageHash::from(destination_package_hash.into_hash().unwrap()),
                None,
                DESTINATION_DEPOSIT,
                runtime_args! {
                    AMOUNT_RUNTIME_ARG => amount,
                    PURSE_RUNTIME_ARG => secondary_purse
                },
            )
        }
        DESTINATION_WITHDRAW => {
            let amount: U512 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG);
            runtime::call_versioned_contract(
                ContractPackageHash::from(destination_package_hash.into_hash().unwrap()),
                None,
                DESTINATION_WITHDRAW,
                runtime_args! {
                    AMOUNT_RUNTIME_ARG => amount,
                    TO_PURSE_RUNTIME_ARG => main_purse
                },
            )
        }
        DESTINATION_GET_PURSE_BALANCE => {
            let () = runtime::call_versioned_contract(
                ContractPackageHash::from(destination_package_hash.into_hash().unwrap()),
                None,
                DESTINATION_GET_PURSE_BALANCE,
                runtime_args! {
                    PURSE_RUNTIME_ARG => main_purse
                },
            );
            Ok(())
        }
        _ => Err(Error::Abort as u32),
    };
    ret.unwrap_or_revert();
}
