use alloc::{format, string::String, vec::Vec};

use casper_contract::{
    contract_api::{runtime}
};
use casper_types::{Key, U256, BlockTime,ApiError};
use contract_utils::{ContractContext, ContractStorage};


use crate::data::{self, Allowances, Balances, Nonces};//};//
use casper_types::system::mint::Error as MintError;
// use casper_types::system::handle_payment::Error as PosError;
use contract_utils::{set_key};

use renvm_sig::keccak256;
use renvm_sig::hash_message;
use cryptoxide::ed25519;

/// Enum for FailureCode, It represents codes for different smart contract errors.
#[repr(u16)]
pub enum FailureCode {

      /// 65,536 for (UniswapV2: EXPIRED)
      Zero = 0, 
      /// 65,537 for (signature verification failed)
      Two
}

pub trait ERC20<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(&mut self, name: String, symbol: String, decimals: u8, domain_separator: String, permit_type_hash: String, contract_hash: Key ) {
        data::set_name(name);
        data::set_symbol(symbol);
        data::set_decimals(decimals);
        data::set_domain_separator(domain_separator);
        data::set_permit_type_hash(permit_type_hash);
        data::set_hash(contract_hash);
        Nonces::init();
        Balances::init();
        Allowances::init();
    }

    fn balance_of(&mut self, owner: Key) -> U256 {
        Balances::instance().get(&owner)
    }
    fn nonce(&mut self, owner: Key) -> U256 {
        Nonces::instance().get(&owner)
    }

    fn transfer(&mut self, recipient: Key, amount: U256) {
        self.make_transfer(self.get_caller(), recipient, amount);
    }

    fn approve(&mut self, spender: Key, amount: U256) {
        Allowances::instance().set(&self.get_caller(), &spender, amount);
    }

    fn allowance(&mut self, owner: Key, spender: Key) -> U256 {
        Allowances::instance().get(&owner, &spender)
    }

    fn transfer_from(&mut self, owner: Key, recipient: Key, amount: U256) {
        let allowances = Allowances::instance();
        let spender = self.get_caller();
        let spender_allowance = allowances.get(&owner, &spender);
        allowances.set(&owner, &spender, spender_allowance - amount);
        self.make_transfer(owner, recipient, amount);
    }

    /// Given the subscription details, generate eip-191 standard hash.
    /// # Parameters
    ///
    /// * `data` - A string slice that holds the meta transaction data
    ///
    /// * `from` - An Accounthash that holds the account address of the subscriber/signer
    /// 
    fn get_subscription_hash(&mut self,data:String,owner:Key) -> [u8;32]
    {
        let get_eip191_standard_hash=hash_message(data);
        let get_eip191_standard_hash_string =hex::encode(get_eip191_standard_hash);
        let subcription_hash_key=format!("{}{}","SUBSCRIPTION_HASH",owner);
        set_key(&subcription_hash_key, get_eip191_standard_hash_string);
        return get_eip191_standard_hash;
    }

    /// This function is to get subcription signer and verify if it is equal
    /// to the signer public key or not. 
    /// 
    /// # Parameters
    ///
    /// * `public_key` - A string slice that holds the public key of the meta transaction signer,  Subscriber have to get it from running cryptoxide project externally.
    ///
    /// * `signature` - A string slice that holds the signature of the meta transaction,  Subscriber have to get it from running cryptoxide project externally.
    /// 
    /// * `get_eip191_standard_hash` - A u8 array that holds the eip-191 standard subcription hash of the meta transaction
    /// 
    /// * `from` - An Accounthash that holds the account address of the subscriber/signer
    /// 
    fn get_subscription_signer_and_verification(&mut self,public_key:String,signature:String,get_eip191_standard_hash:[u8;32],owner:Key) -> bool
    {
        let public_key_without_spaces:String=public_key.split_whitespace().collect();

        let public_key_string: Vec<&str>=public_key_without_spaces.split(',').collect();
    
        let mut public_key_vec:Vec<u8> = Vec::new();
        let mut public_counter=0;
        while public_counter<32{
            public_key_vec.push(public_key_string[public_counter].parse::<u8>().unwrap());
            public_counter=public_counter+1;
        }

        let signature_without_spaces:String=signature.split_whitespace().collect();

        let signature_string: Vec<&str>=signature_without_spaces.split(',').collect();

        let mut signature_vec:Vec<u8> = Vec::new();
        let mut signature_counter=0;
        while signature_counter<64{
            signature_vec.push(signature_string[signature_counter].parse::<u8>().unwrap());
            signature_counter=signature_counter+1;
        }

        let result:bool=ed25519::verify(&get_eip191_standard_hash,&public_key_vec,&signature_vec);
        let verify_key=format!("{}{}","VERIFY",owner);
        set_key(&verify_key, result);
        return result;
    }

    /// This function is to get meta transaction signer and verify if it is equal
    /// to the signer public key or not then call approve. 
    /// 
    /// # Parameters
    ///
    /// * `public_key` - A string slice that holds the public key of the meta transaction signer,  Subscriber have to get it from running cryptoxide project externally.
    ///
    /// * `signature` - A string slice that holds the signature of the meta transaction,  Subscriber have to get it from running cryptoxide project externally.
    /// 
    /// * `owner` - A Key that holds the account address of the owner
    /// 
    /// * `spender` - A Key that holds the account address of the spender
    ///  
    /// * `value` - A U256 that holds the value
    ///  
    /// * `deadeline` - A u64 that holds the deadline limit
    /// 
    fn permit(&mut self, public_key: String, signature: String, owner: Key, spender: Key, value : U256, deadline: u64) {
     
        let domain_separator:String= data::get_domain_separator();
        let permit_type_hash:String= data::get_permit_type_hash();
        let nonce:U256=self.nonce(Key::from(self.get_caller()));

        let deadline_into_blocktime=BlockTime::new(deadline);
        let blocktime =runtime::get_blocktime();

        if deadline_into_blocktime >= blocktime{
    
            let data:String = format!("{}{}{}{}{}{}",permit_type_hash,owner,spender,value,nonce,deadline);
            let hash=keccak256(data.as_bytes());
        
            let hash_string =hex::encode(hash);
    
            let final_data:String = format!("{}{}",domain_separator,hash_string);
    
            let eip191_standard_hash:[u8;32]=self.get_subscription_hash(final_data,Key::from(self.get_caller()));
        
            self.set_nonce(Key::from(self.get_caller()),1.into());

            let result:bool=self.get_subscription_signer_and_verification(public_key,signature,eip191_standard_hash,Key::from(self.get_caller()));
            
            if result==true{
                self.approve(spender,value);
            }
            else{

                //signature verification failed
                runtime::revert(ApiError::User(FailureCode::Two as u16));
            }

        }
        else{

            //deadline is equal to or greater than blocktime
            runtime::revert(ApiError::User(FailureCode::Zero as u16));
        }
             
    }

    fn mint(&mut self, recipient: Key, amount: U256) {
        let balances = Balances::instance();
        let balance = balances.get(&recipient);
        balances.set(&recipient, balance + amount);

        data::set_total_supply(data::total_supply() + amount);
    }
    fn burn(&mut self, recipient: Key, amount: U256) {
        let balances = Balances::instance();
        let balance = balances.get(&recipient);
        if balance >= amount {
            balances.set(&recipient, balance - amount);
            data::set_total_supply(data::total_supply() - amount);
        }else {
            runtime::revert(MintError::InsufficientFunds)
            // PosError::InsufficientPaymentForAmountSpent
        }
        
    }
    fn set_nonce(&mut self, recipient: Key, amount: U256) {
        let nonces = Nonces::instance();
        let nonce = nonces.get(&recipient);
        nonces.set(&recipient, nonce + amount);
    }

    fn make_transfer(&mut self, sender: Key, recipient: Key, amount: U256) {
        let balances = Balances::instance();

        let sender_balance = balances.get(&sender);
        let recipient_balance = balances.get(&recipient);

        balances.set(&sender, sender_balance - amount);
        balances.set(&recipient, recipient_balance + amount);
    }

    fn total_supply(&mut self) -> U256 {
        data::total_supply()
    }
    fn name(&mut self) -> String {
        data::name()
    }
    fn symbol(&mut self) -> String {
        data::symbol()
    }
}
