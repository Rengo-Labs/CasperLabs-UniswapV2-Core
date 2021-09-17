use alloc::{format, string::String, vec::Vec};

use casper_contract::{
    contract_api::{runtime}
};
use casper_types::{Key, U256, BlockTime, ApiError, ContractHash};
use contract_utils::{ContractContext, ContractStorage};


use crate::data::{self, Allowances, Balances, Nonces};//};//
use casper_types::system::mint::Error as MintError;
// use casper_types::system::handle_payment::Error as PosError;
use contract_utils::{set_key};

use renvm_sig::keccak256;
use cryptoxide::ed25519;
use hex::encode;


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

    /// This function is to get signer and verify if it is equal
    /// to the signer public key or not. 
    /// 
    /// # Parameters
    ///
    /// * `public_key` - A string slice that holds the public key of the meta transaction signer
    ///
    /// * `signature` - A string slice that holds the signature of the meta transaction
    /// 
    /// * `digest` - A u8 array that holds the digest
    /// 
    /// * `owner` - An Accounthash that holds the account address of the signer
    /// 
    fn ecrecover(&mut self,public_key:String,signature:String,digest:[u8;32],owner:Key) -> bool
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

        let result:bool=ed25519::verify(&digest,&public_key_vec,&signature_vec);
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
    
            let encode_packed:String = format!("{}{}{}","\x19\x01",domain_separator,hash_string);
    
            let digest=keccak256(encode_packed.as_bytes());
            let digest_string =hex::encode(digest);
            let digest_key=format!("{}{}","digest_",Key::from(self.get_caller()));
            set_key(&digest_key, digest_string);
        
            self.set_nonce(Key::from(self.get_caller()),1.into());

            let result:bool=self.ecrecover(public_key,signature,digest,Key::from(self.get_caller()));
            
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

    fn get_permit_type_and_domain_separator(&mut self, name: &str, contract_hash: ContractHash) -> (String,String){
    let eip_712_domain : &str="EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)";
    let permit_type: &str="Permit(address owner,address spender,uint256 value,uint256 nonce,uint256 deadline)";
    let chain_id : &str="101";
    let eip_domain_hash=keccak256(eip_712_domain.as_bytes());// to take a byte hash of EIP712Domain
    let name_hash=keccak256(name.as_bytes());// to take a byte hash of name
    let one_hash=keccak256("1".as_bytes());// to take a byte hash of "1"
    let eip_domain_hash = encode(eip_domain_hash);// to encode and convert eip_domain_hash into string
    let name_hash = encode(name_hash);// to encode and convert name_hash into string
    let one_hash = encode(one_hash);// to encode and convert one_hash into string
    let concatenated_data:String = format!("{}{}{}{}{}",eip_domain_hash,name_hash,one_hash,chain_id,contract_hash);//string contactination
    let domain_separator=keccak256(concatenated_data.as_bytes());//to take a byte hash of concatenated Data
    let permit_type_hash=keccak256(permit_type.as_bytes());// to take a byte hash of Permit Type
    let domain_separator=encode(domain_separator);
    let permit_type_hash=encode(permit_type_hash);
    (domain_separator, permit_type_hash)
    }
}
