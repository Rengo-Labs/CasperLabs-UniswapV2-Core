pub mod transfer_helper {

    use crate::data::{self};
    use casper_contract::{ contract_api::{runtime}};
    use casper_types::{ contracts::{ContractHash},Key, RuntimeArgs, runtime_args, U256};
    
    pub fn safe_transfer(token: Key, to: Key, value: U256)
    {
        let args: RuntimeArgs = runtime_args!{
            "recipient" => to,
            "amount" => value
        };
        let _:() = runtime::call_contract(ContractHash::from(token.into_hash().unwrap_or_default()), "transfer", args);
    }

    pub fn safe_transfer_from(token: Key, from: Key, to: Key, value: U256)
    {
        // Token must be approved for router to spend.
        let args: RuntimeArgs = runtime_args!{
            "owner" => from,
            "recipient" => to,
            "amount" => value
        };

        let _:() = runtime::call_contract(ContractHash::from(token.into_hash().unwrap_or_default()), "transfer_from", args);
    }

    /*
    pub fn safe_transfer_cspr(to: Key, value: U256)
    {

        /*
        // call deposit method from wcspr
        let args: RuntimeArgs = runtime_args!{
            "to" => to,
            "amount" => value
        };
        let wcspr_contract: ContractHash = data::wcspr();
        let () = runtime::call_contract(wcspr_contract, "deposit", args);
        */
    }
    */
}