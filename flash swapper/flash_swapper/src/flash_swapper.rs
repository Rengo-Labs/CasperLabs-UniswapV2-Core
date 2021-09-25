use alloc::{format, string::String,vec::Vec};

use casper_contract::{contract_api::{runtime::{self, call_contract}}, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{ApiError, ContractHash,Key, RuntimeArgs, U256, runtime_args};
use contract_utils::{ContractContext, ContractStorage};

use crate::data::{self};

use contract_utils::{get_key};

/// Enum for FailureCode, It represents codes for different smart contract errors.
#[repr(u16)]
enum FailureCode {

      /// 65,536 for (Requested pair is not available)
      Zero= 0,
      /// 65,537 for (Requested borrow token is not available) 
      One,  
      //  65,538 for (Requested pay token is not available)
      Two,
      //  65,539 for (_amount is too big)
      Three

}

#[repr(u16)]
pub enum Error {
    UniswapV2ZeroAddress = 0,
    UniswapV2PairExists = 1,
    UniswapV2PermissionedPairAccess = 2,
    UniswapV2InvalidContractAddress = 3,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}

pub trait FLASHSWAPPER<Storage: ContractStorage>: ContractContext<Storage> {

    fn init(&mut self,wcspr: Key, dai: Key, uniswap_v2_factory: Key, contract_hash: Key ) {
        data::set_wcspr(wcspr);
        data::set_cspr(Key::from_formatted_str("hash-0000000000000000000000000000000000000000000000000000000000000000").unwrap());
        data::set_dai(dai);
        data::set_uniswap_v2_factory(uniswap_v2_factory);
        data::set_hash(contract_hash);
    }    

    fn start_swap(&mut self, _token_borrow: Key, _amount: U256, _token_pay: Key, _user_data: String) {

        let mut is_borrowing_cspr: bool=false;
        let mut is_paying_cspr: bool=false;
        let mut token_borrow: Key = _token_borrow;//btc
        let mut token_pay: Key = _token_pay;// dai
        let cspr = data::get_cspr();
        let wcspr = data::get_wcspr();

        if token_borrow == cspr {
            is_borrowing_cspr = true;
            token_borrow = wcspr; // we'll borrow wcspr from UniswapV2 but then unwrap it for the user
        }
        if token_pay == cspr {
            is_paying_cspr = true;
            token_pay = wcspr; // we'll wrap the user's cspr before sending it back to UniswapV2
        }
        if token_borrow == token_pay {
            self.simple_flash_loan(token_borrow, _amount, is_borrowing_cspr, is_paying_cspr, _user_data);
        } else if token_borrow == wcspr || token_pay == wcspr {
            self.simple_flash_swap(token_borrow, _amount, token_pay, is_borrowing_cspr, is_paying_cspr, _user_data);
        } else {
            self.traingular_flash_swap(token_borrow, _amount, token_pay, _user_data);
        }

    }

    fn uniswap_v2_call(&mut self, _sender: Key, _amount0: U256, _amount1: U256, _data: String) {
        // access control
        
        let permissioned_pair_address = data::get_permissioned_pair_address();
        if self.get_caller() == permissioned_pair_address{
            runtime::revert(Error::UniswapV2PermissionedPairAccess);
        }
        if _sender != data::get_hash(){
            runtime::revert(Error::UniswapV2InvalidContractAddress);
        }
                
        let decoded_data_without_commas: Vec<&str> = _data.split(',').collect();
        let _token_borrow_string = format!(
            "{}{}",
            "hash-",
            decoded_data_without_commas[1]
        );
        let _token_pay_string = format!(
            "{}{}",
            "hash-",
            decoded_data_without_commas[3]
        );

        let _swap_type: &str = decoded_data_without_commas[0];
        let _token_borrow :Key= Key::from_formatted_str(&_token_borrow_string).unwrap(); // ????
        let _amount: U256 = decoded_data_without_commas[2].parse().unwrap();
        let _token_pay :Key= Key::from_formatted_str(&_token_pay_string).unwrap();
        let _is_borrowing_cspr: bool = decoded_data_without_commas[4].parse().unwrap();
        let _is_paying_cspr: bool = decoded_data_without_commas[5].parse().unwrap();
        let _triangle_data: &str = decoded_data_without_commas[6];
        let _user_data: &str = decoded_data_without_commas[7];

        if _swap_type == "simple_loan" {
            self.simple_flash_loan_execute(_token_borrow, _amount,self.get_caller(), _is_borrowing_cspr, _is_paying_cspr, _user_data.into());
        } else if _swap_type == "simple_swap" {
            self.simple_flash_swap_execute(_token_borrow, _amount, _token_pay, self.get_caller(), _is_borrowing_cspr, _is_paying_cspr, _user_data.into());
        } else {
            self.traingular_flash_swap_execute(_token_borrow, _amount, _token_pay, _triangle_data.into(), _user_data.into());
        }
       
    }

    // @notice This function is used when the user repays with the same token they borrowed
    // @dev This initiates the flash borrow. See `simpleFlashLoanExecute` for the code that executes after the borrow.
    fn simple_flash_loan(&mut self, _token_borrow: Key, _amount: U256, _is_borrowing_cspr: bool, _is_paying_cspr: bool, _data: String) {
        
        let mut other_token =  data::get_dai();
        let wcspr = data::get_wcspr();
        let uniswap_v2_factory = data::get_uniswap_v2_factory();
        if _token_borrow != wcspr {
            other_token = wcspr;
        }
        
        let uniswap_v2_factory_hash_add_array = match uniswap_v2_factory {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };

        let uniswap_v2_factory_hash_add = ContractHash::new(uniswap_v2_factory_hash_add_array);

        let permissioned_pair_address: Key = call_contract(uniswap_v2_factory_hash_add, "get_pair", runtime_args!{"token0" => _token_borrow, "token1"  => other_token });
        data::set_permissioned_pair_address(permissioned_pair_address);
        let pair_address: Key = data::get_permissioned_pair_address();
        // in before 0 address was hash-0000000000000000000000000000000000000000000000000000000000000000
        if pair_address == Key::from_formatted_str("hash-0000000000000000000000000000000000000000000000000000000000000000").unwrap() {
            runtime::revert(Error::UniswapV2ZeroAddress);
        }
        let pair_address_hash_add_array = match pair_address {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        let pair_address_hash_add = ContractHash::new(pair_address_hash_add_array);

        let token0: Key = call_contract(pair_address_hash_add, "token0", RuntimeArgs::new());
        let token1: Key = call_contract(pair_address_hash_add, "token1", RuntimeArgs::new());
        // assert_eq!(token0,_token_borrow);
        let amount0_out: U256;
        let amount1_out: U256;
        if _token_borrow == token0 {
            amount0_out = _amount;
        }
        else{
            amount0_out = 0.into();
        }
        if _token_borrow == token1 {
            amount1_out = _amount;
        }
        else{
            amount1_out = 0.into();
        }
        
        let _token_borrow_hash_add_array = match _token_borrow {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };

        let _token_borrow_hash_add = ContractHash::new(_token_borrow_hash_add_array);

        let _token_borrow_str = _token_borrow_hash_add.to_formatted_string();
        let _token_borrow_vec: Vec<&str> = _token_borrow_str.split('-').collect();
        let _token_borrow_hash = _token_borrow_vec[1];
        let data: String=format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}","simple_loan",",",_token_borrow_hash,",",_amount,",",_token_borrow_hash,",",_is_borrowing_cspr,",",_is_paying_cspr,",","",",",_data);
        let _ret:() = runtime::call_contract(pair_address_hash_add, "swap", runtime_args!{"amount0_out" => amount0_out, "amount1_out"  => amount1_out, "to" => data::get_hash(), "data" => data });
    }    


    // @notice This is the code that is executed after `simpleFlashLoan` initiated the flash-borrow
    // @dev When this code executes, this contract will hold the flash-borrowed _amount of _token_borrow
    fn simple_flash_loan_execute(&mut self, _token_borrow: Key, _amount: U256, _pair_address: Key, _is_borrowing_cspr: bool, _is_paying_cspr: bool, _user_data: String){

        let wcspr = data::get_wcspr();

        let wcspr_hash_add_array = match wcspr {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };

        let wcspr_hash_add = ContractHash::new(wcspr_hash_add_array);
        let cspr = data::get_cspr();
        if _is_borrowing_cspr {
            let () = call_contract(wcspr_hash_add, "withdraw", runtime_args!{"from" => data::get_hash(), "amount" => _amount});
        }

        let fee: U256 = ((_amount * 3) / 997) +  1;
        let amount_to_repay: U256 = _amount + fee;
        let token_borrowed: Key;
        let token_to_repay: Key;

        if _is_borrowing_cspr {
            token_borrowed = cspr;
        } else {
            token_borrowed =_token_borrow;
        }
        if _is_paying_cspr {
            token_to_repay = cspr;
        } else {
            token_to_repay = _token_borrow;
        }
        
        // do whatever the user wants
        self.execute(token_borrowed, _amount, token_to_repay, amount_to_repay, _user_data);

        // payback the loan
        // wrap the cspr if necessary

        if _is_paying_cspr {
           let () = call_contract(wcspr_hash_add, "deposit", runtime_args!{"to" => data::get_hash(),"amount" => amount_to_repay});
        }
 
        let _token_borrow_hash_add_array = match _token_borrow {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };

        let _token_borrow_hash_add = ContractHash::new(_token_borrow_hash_add_array);

        let () = call_contract(_token_borrow_hash_add, "transfer", runtime_args!{"recipient"=>_pair_address ,"amount" => amount_to_repay});
    }

    /// @notice This function is used when either the _tokenBorrow or _tokenPay is wcspr or cspr
    /// @dev Since ~all tokens trade against wcspr (if they trade at all), we can use a single UniswapV2 pair to
    /// flash-borrow and repay with the requested tokens.
    /// @dev This initiates the flash borrow. See `simpleFlashSwapExecute` for the code that executes after the borrow.
    /// 
    fn simple_flash_swap( &mut self, token_borrow:Key, amount:U256 , token_pay:Key, is_borrowing_cspr:bool , is_paying_cspr:bool , user_data: String ) 
    {
        let uniswap_v2_factory_address:Key=data::get_uniswap_v2_factory();

        //convert Key to ContractHash
        let uniswap_v2_factory_address_hash_add_array = match uniswap_v2_factory_address {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        let uniswap_v2_factory_contract_hash = ContractHash::new(uniswap_v2_factory_address_hash_add_array);

        let token_borrow_token_pay_pair_address:Key=runtime::call_contract(uniswap_v2_factory_contract_hash,"get_pair",runtime_args!{"token0" => token_borrow,"token1" => token_pay});
        data::set_permissioned_pair_address(token_borrow_token_pay_pair_address);
        let pair_address:Key = token_borrow_token_pay_pair_address; // gas efficiency

        let address_0 = Key::from_formatted_str("hash-0000000000000000000000000000000000000000000000000000000000000000").unwrap();
        // assert_ne!(address_0,pair_address);
        if pair_address != address_0 {

            //convert Key to ContractHash
            let pair_address_hash_add_array = match pair_address {
                Key::Hash(package) => package,
                _ => runtime::revert(ApiError::UnexpectedKeyVariant),
            };
    
            let pair_address_hash_add = ContractHash::new(pair_address_hash_add_array);
    

            let token0:Key = runtime::call_contract(pair_address_hash_add,"token0",runtime_args!{});
            let token1:Key = runtime::call_contract(pair_address_hash_add,"token1",runtime_args!{});
            
            let amount0_out: U256;
            let amount1_out: U256;

            if token_borrow == token0{
                amount0_out = amount;
            }else{
                amount0_out = 0.into();
            }
            if token_borrow == token1{
                amount1_out = amount;
            }else{
                amount1_out = 0.into();
            }

            let _token_borrow_hash_add_array = match token_borrow {
                Key::Hash(package) => package,
                _ => runtime::revert(ApiError::UnexpectedKeyVariant),
            };
    
            let _token_borrow_hash_add = ContractHash::new(_token_borrow_hash_add_array);
    
            let _token_borrow_str = _token_borrow_hash_add.to_formatted_string();
            let _token_borrow_vec: Vec<&str> = _token_borrow_str.split('-').collect();
            let _token_borrow_hash = _token_borrow_vec[1];

            let _token_pay_hash_add_array = match token_pay {
                Key::Hash(package) => package,
                _ => runtime::revert(ApiError::UnexpectedKeyVariant),
            };
    
            let _token_pay_hash_add = ContractHash::new(_token_pay_hash_add_array);
    
            let _token_pay_str = _token_pay_hash_add.to_formatted_string();
            let _token_pay_vec: Vec<&str> = _token_pay_str.split('-').collect();
            let _token_pay_hash = _token_pay_vec[1];
            
            let data: String = format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}","simple_swap",",",_token_borrow_hash,",",amount,",",_token_pay_hash,",",is_borrowing_cspr,",",is_paying_cspr,",","",",",user_data);
            let _ret:() = runtime::call_contract(pair_address_hash_add, "swap", runtime_args!{"amount0_out" => amount0_out, "amount1_out"  => amount1_out, "to" => data::get_hash(), "data" => data });
        } else {
            // requested pair is not available
            runtime::revert(ApiError::User(FailureCode::Zero as u16));
        }
        
    }

    /// @notice This is the code that is executed after `simpleFlashSwap` initiated the flash-borrow
    /// @dev When this code executes, this contract will hold the flash-borrowed _amount of _tokenBorrow

    fn simple_flash_swap_execute( &mut self, token_borrow:Key, amount:U256 , token_pay:Key, _pair_address:Key, is_borrowing_cspr:bool , is_paying_cspr:bool , _user_data: String)
    {
        // unwrap wcspr if necessary

        let wcspr_address:Key=data::get_wcspr();

        //convert Key to ContractHash
        let wcspr_address_hash_add_array = match wcspr_address {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        let wcspr_contract_hash = ContractHash::new(wcspr_address_hash_add_array);

        if is_borrowing_cspr == true {
            let _withdraw_result: ()=runtime::call_contract(wcspr_contract_hash,"withdraw",runtime_args!{"from" => data::get_hash(),"amount" => amount});
        }
        // compute the amount of _tokenPay that needs to be repaid

        let permissioned_pair_address:Key = get_key("permissioned_pair_address").unwrap_or_revert(); // gas efficiency

        let pair_address:Key = permissioned_pair_address; // gas efficiency

        //convert Key to ContractHash
        let token_borrow_address_hash_add_array = match token_borrow {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        let token_borrow_contract_hash = ContractHash::new(token_borrow_address_hash_add_array);

        let pair_balance_token_borrow:U256=runtime::call_contract(token_borrow_contract_hash,"balance_of",runtime_args!{"owner" => pair_address});

        //convert Key to ContractHash
        let token_pay_address_hash_add_array = match token_pay {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        let token_pay_contract_hash = ContractHash::new(token_pay_address_hash_add_array);

        let pair_balance_token_pay:U256=runtime::call_contract(token_pay_contract_hash,"balance_of",runtime_args!{"owner" => pair_address});

        let amount_1000:U256=1000.into();
        let amount_997:U256=997.into();
        let amount_1:U256=1.into();

        let amount_to_repay:U256 = ((amount_1000 * pair_balance_token_pay * amount) / (amount_997 * pair_balance_token_borrow)) + amount_1;

        // get the orignal tokens the user requested
        let mut _token_borrowed=Key::from_formatted_str("hash-0000000000000000000000000000000000000000000000000000000000000000").unwrap();
        let mut _token_to_repay=Key::from_formatted_str("hash-0000000000000000000000000000000000000000000000000000000000000000").unwrap();
        let cspr:Key=data::get_cspr();

        if is_borrowing_cspr == true {
            _token_borrowed=cspr;
        }
        else {
            _token_borrowed=token_borrow;
        }
        if is_paying_cspr == true {
            _token_to_repay=cspr;
        }
        else {
            _token_to_repay=token_pay;
        }

        // do whatever the user wants
        self.execute(_token_borrowed, amount, _token_to_repay, amount_to_repay, _user_data);

        // payback loan
        // wrap cspr if necessary

        if is_paying_cspr==true {
            let _deposit_result:() =runtime::call_contract(wcspr_contract_hash,"deposit",runtime_args!{"to" => data::get_hash(),"amount" => amount_to_repay});
        }

        let () = runtime::call_contract(token_pay_contract_hash,"transfer",runtime_args!{"recipient" => _pair_address, "amount" => amount_to_repay});
    }
    
    /// @notice This function is used when neither the _tokenBorrow nor the _tokenPay is wcspr
    /// @dev Since it is unlikely that the _tokenBorrow/_tokenPay pair has more liquidaity than the _tokenBorrow/wcspr and
    ///     _tokenPay/wcspr pairs, we do a triangular swap here. That is, we flash borrow wcspr from the _tokenPay/wcspr pair,
    ///     Then we swap that borrowed wcspr for the desired _tokenBorrow via the _tokenBorrow/wcspr pair. And finally,
    ///     we pay back the original flash-borrow using _tokenPay.
    /// @dev This initiates the flash borrow. See `traingularFlashSwapExecute` for the code that executes after the borrow.
    /// 
    fn traingular_flash_swap(&mut self, token_borrow:Key, amount:U256 , token_pay:Key, user_data: String)
    {
        
        let uniswap_v2_factory_address:Key=data::get_uniswap_v2_factory();

        // convert Key to ContractHash
        let uniswap_v2_factory_address_hash_add_array = match uniswap_v2_factory_address {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        let uniswap_v2_factory_contract_hash = ContractHash::new(uniswap_v2_factory_address_hash_add_array);

        let wcspr:Key=data::get_wcspr();
        let borrow_pair_address:Key=runtime::call_contract(uniswap_v2_factory_contract_hash,"get_pair",runtime_args!{"token0" => token_borrow,"token1" => wcspr});
    
        let address_0:Key = Key::from_formatted_str("hash-0000000000000000000000000000000000000000000000000000000000000000").unwrap();

        if borrow_pair_address != address_0
        {
            let permissioned_pair_address:Key=runtime::call_contract(uniswap_v2_factory_contract_hash,"get_pair",runtime_args!{"token0" => token_pay,"token1" => wcspr});
            data::set_permissioned_pair_address(permissioned_pair_address);
            let pay_pair_address:Key = permissioned_pair_address; // gas efficiency
          
            if pay_pair_address != address_0
            {
                // STEP 1: Compute how much wcspr will be needed to get _amount of _tokenBorrow out of the _tokenBorrow/wcspr pool

                //convert Key to ContractHash
                let token_borrow_address_hash_add_array = match token_borrow {
                    Key::Hash(package) => package,
                    _ => runtime::revert(ApiError::UnexpectedKeyVariant),
                };
                let token_borrow_contract_hash = ContractHash::new(token_borrow_address_hash_add_array);
                let pair_balance_token_borrow_before:U256=runtime::call_contract(token_borrow_contract_hash,"balance_of",runtime_args!{"owner" => borrow_pair_address});

                if pair_balance_token_borrow_before >= amount 
                {
 
                    let pair_balance_token_borrow_after:U256 = pair_balance_token_borrow_before - amount;

                    //convert Key to ContractHash
                    let wcspr_address_hash_add_array = match wcspr {
                        Key::Hash(package) => package,
                        _ => runtime::revert(ApiError::UnexpectedKeyVariant),
                    };
                    let wcspr_contract_hash = ContractHash::new(wcspr_address_hash_add_array);
                    let pair_balance_wcspr:U256=runtime::call_contract(wcspr_contract_hash,"balance_of",runtime_args!{"owner" => borrow_pair_address});

                    let amount_1000:U256=1000.into();
                    let amount_997:U256=997.into();
                    let amount_1:U256=1.into();
            
                    let amount_of_wcspr:U256 = ((amount_1000 * pair_balance_wcspr * amount) / (amount_997 * pair_balance_token_borrow_after)) + amount_1;
        //             // using a helper function here to avoid "stack too deep" :(
                    self.traingular_flash_swap_helper(token_borrow, amount, token_pay, borrow_pair_address, pay_pair_address, amount_of_wcspr, user_data);
                }
                else {
                    // _amount is too big
                    runtime::revert(ApiError::User(FailureCode::Three as u16));
                }
               
            }
            else {
                // Requested pay token is not available
                runtime::revert(ApiError::User(FailureCode::Two as u16));
            }
        }
        else {
            // Requested borrow token is not available
            runtime::revert(ApiError::User(FailureCode::One as u16));
        }
    }

    /// @notice Helper function for `traingularFlashSwap` to avoid `stack too deep` errors
    /// 
    fn traingular_flash_swap_helper(&mut self,token_borrow:Key,amount:U256, token_pay:Key, borrow_pair_address:Key , pay_pair_address:Key,amount_of_wcspr:U256, user_data:String)
    {
        //convert Key to ContractHash
        let pay_pair_address_hash_add_array = match pay_pair_address {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        let pay_pair_contract_hash = ContractHash::new(pay_pair_address_hash_add_array);

        // // Step 2: Flash-borrow _amountOfwcspr wcspr from the _tokenPay/wcspr pool
        let token0:Key=runtime::call_contract(pay_pair_contract_hash,"token0",runtime_args!{});
        let token1:Key=runtime::call_contract(pay_pair_contract_hash,"token1",runtime_args!{});

        let mut amount0_out:U256=0.into();
        let mut amount1_out:U256=0.into();
        let wcspr:Key=data::get_wcspr();

        if wcspr == token0{
            amount0_out = amount_of_wcspr;
        }

        if wcspr == token1{
            amount1_out = amount_of_wcspr;
        }
        let _token_borrow_hash_add_array = match token_borrow {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };

        let _token_borrow_hash_add = ContractHash::new(_token_borrow_hash_add_array);

        let _token_borrow_str = _token_borrow_hash_add.to_formatted_string();
        let _token_borrow_vec: Vec<&str> = _token_borrow_str.split('-').collect();
        let _token_borrow_hash = _token_borrow_vec[1];

        let _token_pay_hash_add_array = match token_pay {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };

        let _token_pay_hash_add = ContractHash::new(_token_pay_hash_add_array);

        let _token_pay_str = _token_pay_hash_add.to_formatted_string();
        let _token_pay_vec: Vec<&str> = _token_pay_str.split('-').collect();
        let _token_pay_hash = _token_pay_vec[1];

        let _borrow_pair_hash_add_array = match borrow_pair_address {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };

        let _borrow_pair_hash_add = ContractHash::new(_borrow_pair_hash_add_array);

        let _borrow_pair_str = _borrow_pair_hash_add.to_formatted_string();
        let _borrow_pair_vec: Vec<&str> = _borrow_pair_str.split('-').collect();
        let _borrow_pair_hash = _borrow_pair_vec[1];

        let triangle_data :String = format!("{}{}{}",_borrow_pair_hash,".", amount_of_wcspr);
        let data:String = format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}","triangular_swap",",", _token_borrow_hash,",", amount,",", _token_pay_hash,",", false,",", false,",", triangle_data,",", user_data);

        let _result:()=runtime::call_contract(pay_pair_contract_hash,"swap",runtime_args!{"amount0_out" => amount0_out,"amount1_out" => amount1_out,"to" => data::get_hash(),"data" => data});
        
    }    

    /// @notice This is the code that is executed after `traingularFlashSwap` initiated the flash-borrow
    /// @dev When this code executes, this contract will hold the amount of wcspr we need in order to get _amount
    ///     _tokenBorrow from the _tokenBorrow/wcspr pair.
    fn traingular_flash_swap_execute(&mut self, token_borrow:Key, amount:U256 , token_pay:Key, triangle_data:String, user_data:String)
    {

        // decode _triangleData
        let decoded_data_without_fullstop: Vec<&str> = triangle_data.split('.').collect();
        let borrow_pair_address_string = format!(
            "{}{}",
            "hash-",
            decoded_data_without_fullstop[0]
        );

        let borrow_pair_address :Key= Key::from_formatted_str(&borrow_pair_address_string).unwrap();
        let amount_of_wcspr: U256 = decoded_data_without_fullstop[1].parse().unwrap();
     
        //convert Key to ContractHash
        let borrow_pair_address_hash_add_array = match borrow_pair_address {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        let borrow_pair_contract_hash = ContractHash::new(borrow_pair_address_hash_add_array);

        // Step 3: Using a normal swap, trade that wcspr for _tokenBorrow
        let token0:Key = runtime::call_contract(borrow_pair_contract_hash,"token0",runtime_args!{});
        let token1:Key = runtime::call_contract(borrow_pair_contract_hash,"token1",runtime_args!{});
        let amount0_out: U256;
        let amount1_out: U256;


        if token_borrow == token0{
            amount0_out = amount;
        } else {
            amount0_out = 0.into();
        }

        if token_borrow == token1{
            amount1_out = amount;
        } else {
            amount1_out = 0.into();
        }

        // send our flash-borrowed wcspr to the pair
        let wcspr:Key=data::get_wcspr();

        //convert Key to ContractHash
        let wcspr_address_hash_add_array = match wcspr {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        let wcspr_contract_hash = ContractHash::new(wcspr_address_hash_add_array);
        let () = runtime::call_contract(wcspr_contract_hash,"transfer",runtime_args!{"recipient" => borrow_pair_address,"amount" => amount_of_wcspr});

        let flash_swapper_address:Key=data::get_hash();
        let _result: () = runtime::call_contract(borrow_pair_contract_hash,"swap",runtime_args!{"amount0_out" => amount0_out,"amount1_out" => amount1_out,"to" => flash_swapper_address,"data" => user_data.clone()});
        
        // compute the amount of _tokenPay that needs to be repaid
        let permissioned_pair_address:Key=get_key("permissioned_pair_address").unwrap_or_revert();
        let pay_pair_address:Key = permissioned_pair_address; // gas efficiency

        let pair_balance_wcspr:U256=runtime::call_contract(wcspr_contract_hash,"balance_of",runtime_args!{"owner" => pay_pair_address});

        //convert Key to ContractHash
        let token_pay_address_hash_add_array = match token_pay {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        let token_pay_contract_hash = ContractHash::new(token_pay_address_hash_add_array);
        let pair_balance_token_pay:U256=runtime::call_contract(token_pay_contract_hash,"balance_of",runtime_args!{"owner" => pay_pair_address});

        let amount_1000:U256=1000.into();
        let amount_997:U256=997.into();
        let amount_1:U256=1.into();

        let amount_to_repay:U256 = ((amount_1000 * pair_balance_token_pay * amount_of_wcspr) / (amount_997 * pair_balance_wcspr)) + amount_1;

        // Step 4: Do whatever the user wants (arb, liqudiation, etc)
        self.execute( token_borrow, amount, token_pay, amount_to_repay, user_data);

        // Step 5: Pay back the flash-borrow to the _tokenPay/wcspr pool
        let () = runtime::call_contract(token_pay_contract_hash,"transfer",runtime_args!{"recipient" => pay_pair_address,"amount" => amount_to_repay});
    }
    
    // @notice This is where the user's custom logic goes
    // @dev When this function executes, this contract will hold _amount of _token_borrow
    // @dev It is important that, by the end of the execution of this function, this contract holds the necessary
    //     amount of the original _token_pay needed to pay back the flash-loan.
    // @dev Paying back the flash-loan happens automatically by the calling function -- do not pay back the loan in this function
    // @dev If you entered `hash-0000000000000000000000000000000000000000000000000000000000000000` for _token_pay when you called `flashSwap`, then make sure this contract holds _amount cspr before this
    //     finishes executing
    // @dev User will override this function on the inheriting contract
    fn execute(&mut self,_token_borrow: Key, _amount: U256, _token_pay: Key, _amount_to_repay: U256, _user_data: String){

    }
}    