#![no_main]

use common::{
    contract_api::{account, runtime, system},
    keys::*,
    unwrap_or_revert::UnwrapOrRevert,
    *,
};

fn purse(amount: U512) -> URef {
    let main_purse: URef = account::get_main_purse();
    let secondary_purse: URef = system::create_purse();
    system::transfer_from_purse_to_purse(main_purse, secondary_purse, amount, None)
        .unwrap_or_revert();
    secondary_purse
}

#[no_mangle]
fn call() {
    let entrypoint: String = runtime::get_named_arg(ENTRYPOINT);
    let package_hash: Key = runtime::get_named_arg(PACKAGE_HASH);
    match entrypoint.as_str() {
        DEPOSIT => {
            let amount: U512 = runtime::get_named_arg("amount");
            let () = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                DEPOSIT,
                runtime_args! {
                    "purse" => purse(amount),
                    "amount" => amount
                },
            );
        }
        WITHDRAW => {
            let amount: U512 = runtime::get_named_arg("amount");
            let () = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                WITHDRAW,
                runtime_args! {
                    "purse" => purse(amount),
                    "amount" => amount
                },
            );
        }
        _ => runtime::revert(ApiError::UnexpectedKeyVariant),
    };
}
