#![no_main]
#![no_std]

extern crate alloc;

use alloc::{boxed::Box, collections::BTreeSet, format, string::String, vec};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, CLType, CLTyped, CLValue, ContractHash, EntryPoint, EntryPointAccess,
    EntryPointType, EntryPoints, Group, Key, Parameter, RuntimeArgs, URef, U128, U256,
};
use contract_utils::{ContractContext, OnChainContractStorage};
use hex::encode;
use pair::{self, PAIR};
use renvm_sig::keccak256;

#[derive(Default)]
struct Pair(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for Pair {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl PAIR<OnChainContractStorage> for Pair {}

impl Pair {
    fn constructor(
        &mut self,
        name: String,
        symbol: String,
        decimals: u8,
        initial_supply: U256,
        nonce: U256,
        domain_separator: String,
        permit_type_hash: String,
        contract_hash: ContractHash,
        reserve0: U128,
        reserve1: U128,
        block_timestamp_last: u64,
        price0_cumulative_last: U256,
        price1_cumulative_last: U256,
        k_last: U256,
        treasury_fee: U256,
        minimum_liquidity: U256,
        callee_contract_hash: Key,
        factory_hash: Key,
    ) {
        PAIR::init(
            self,
            name,
            symbol,
            decimals,
            domain_separator,
            permit_type_hash,
            Key::from(contract_hash),
            factory_hash,
            reserve0,
            reserve1,
            block_timestamp_last,
            price0_cumulative_last,
            price1_cumulative_last,
            k_last,
            treasury_fee,
            minimum_liquidity,
            callee_contract_hash,
        );
        PAIR::mint(self, self.get_caller(), initial_supply);
        PAIR::set_nonce(self, self.get_caller(), nonce);
    }
}

#[no_mangle]
fn constructor() {
    let name: String = runtime::get_named_arg("name");
    let symbol: String = runtime::get_named_arg("symbol");
    let decimals: u8 = runtime::get_named_arg("decimals");
    let initial_supply: U256 = runtime::get_named_arg("initial_supply");
    let nonce: U256 = runtime::get_named_arg("nonce");
    let domain_separator: String = runtime::get_named_arg("domain_separator");
    let permit_type_hash: String = runtime::get_named_arg("permit_type_hash");
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let reserve0: U128 = runtime::get_named_arg("reserve0");
    let reserve1: U128 = runtime::get_named_arg("reserve1");
    let block_timestamp_last: u64 = runtime::get_named_arg("block_timestamp_last");
    let price0_cumulative_last: U256 = runtime::get_named_arg("price0_cumulative_last");
    let price1_cumulative_last: U256 = runtime::get_named_arg("price1_cumulative_last");
    let k_last: U256 = runtime::get_named_arg("k_last"); // reserve0 * reserve1, as of immediately after the most recent liquidity event
    let treasury_fee: U256 = runtime::get_named_arg("treasury_fee");
    let minimum_liquidity: U256 = runtime::get_named_arg("minimum_liquidity");
    let callee_contract_hash: Key = runtime::get_named_arg("callee_contract_hash");
    let factory_hash: Key = runtime::get_named_arg("factory_hash");
    Pair::default().constructor(
        name,
        symbol,
        decimals,
        initial_supply,
        nonce,
        domain_separator,
        permit_type_hash,
        contract_hash,
        reserve0,
        reserve1,
        block_timestamp_last,
        price0_cumulative_last,
        price1_cumulative_last,
        k_last,
        treasury_fee,
        minimum_liquidity,
        callee_contract_hash,
        factory_hash,
    );
}

/// This function is to transfer tokens against the address that user provided
///
/// # Parameters
///
/// * `recipient` - A Key that holds the account address of the user
///
/// * `amount` - A U256 that holds the amount for approve
///

#[no_mangle]
fn transfer() {
    let recipient: Key = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");
    Pair::default().transfer(recipient, amount);
}

/// This function is to transfer tokens against the address that has been approved before by owner
///
/// # Parameters
///
/// * `owner` - A Key that holds the account address of the user
///  
/// * `recipient` - A Key that holds the account address of the user
///
/// * `amount` - A U256 that holds the amount for approve
///  

#[no_mangle]
fn transfer_from() {
    let owner: Key = runtime::get_named_arg("owner");
    let recipient: Key = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");
    Pair::default().transfer_from(owner, recipient, amount);
}

/// force balances to match reserves

#[no_mangle]
fn skim() {
    let to: Key = runtime::get_named_arg("to");
    Pair::default().skim(to);
}

/// force reserves to match balances

#[no_mangle]
fn sync() {
    Pair::default().sync();
}

/// this low-level function should be called from a contract which performs important safety checks

#[no_mangle]
fn swap() {
    let amount0_out: U256 = runtime::get_named_arg("amount0_out");
    let amount1_out: U256 = runtime::get_named_arg("amount1_out");
    let to: Key = runtime::get_named_arg("to");
    let data: String = runtime::get_named_arg("data");
    Pair::default().swap(amount0_out, amount1_out, to, data);
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

#[no_mangle]
fn permit() {
    let public_key: String = runtime::get_named_arg("public");
    let signature: String = runtime::get_named_arg("signature");
    let owner: Key = runtime::get_named_arg("owner");
    let spender: Key = runtime::get_named_arg("spender");
    let value: U256 = runtime::get_named_arg("value");
    let deadline: u64 = runtime::get_named_arg("deadline");
    Pair::default().permit(public_key, signature, owner, spender, value, deadline);
}

/// This function is to approve tokens against the address that user provided so the address can transfer on his behalf
///
/// # Parameters
///
/// * `spender` - A Key that holds the account address of the user
///  
/// * `amount` - A U256 that holds the value which is goin to approve
///

#[no_mangle]
fn approve() {
    let spender: Key = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");
    Pair::default().approve(spender, amount);
}

/// This function is to mint token against the address that user provided
///
/// # Parameters
///
/// * `to` - A Key that holds the account address of the user
///  

#[no_mangle]
fn mint() {
    let to: Key = runtime::get_named_arg("to");
    let liquidity: U256 = Pair::default().mint_helper(to);
    runtime::ret(CLValue::from_t(liquidity).unwrap_or_revert());
}

/// This function is to mint token against the address that user provided with the amount
///
/// # Parameters
///
/// * `to` - A Key that holds the account address of the user
///
/// * `amount` - A U256 that holds the value that is going to mint
///

#[no_mangle]
fn erc20_mint() {
    let to: Key = runtime::get_named_arg("to");
    let amount: U256 = runtime::get_named_arg("amount");
    Pair::default().mint(to, amount);
}

/// This function is to burn token against the address that user provided
///
/// # Parameters
///
/// * `from` - A Key that holds the account address of the user
///

#[no_mangle]
fn burn() {
    let to: Key = runtime::get_named_arg("to");
    let (amount0, amount1): (U256, U256) = Pair::default().burn_helper(to);
    runtime::ret(CLValue::from_t((amount0, amount1)).unwrap_or_revert());
}

/// This function is to get a balance of a owner provided by user
///
/// # Parameters
///
/// * `owner` - A Key that holds the account address of the owner against which user wants the Balance
///

#[no_mangle]
fn balance_of() {
    let owner: Key = runtime::get_named_arg("owner");
    let ret: U256 = Pair::default().balance_of(owner);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// This function is to get the reserves like Reserve0, Reserve1 and Block Time Stamp
///

#[no_mangle]
fn get_reserves() {
    let (reserve0, reserve1, block_timestamp_last): (U128, U128, u64) =
        Pair::default().get_reserves();
    runtime::ret(CLValue::from_t((reserve0, reserve1, block_timestamp_last)).unwrap_or_revert());
}

/// This function is to get a nonce of a owner provided by user
///
/// # Parameters
///
/// * `owner` - A Key that holds the account address of the owner against which user wants the Nonce
///

#[no_mangle]
fn nonce() {
    let owner: Key = runtime::get_named_arg("owner");
    let ret: U256 = Pair::default().nonce(owner);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// This function is to get a allowance of a owner and spender provided by user
///
/// # Parameters
///
/// * `owner` - A Key that holds the account address of the owner against which user wants the Allowance
///
/// * `spender` - A Key that holds the account address of the owner against which user wants the Allowance
///

#[no_mangle]
fn allowance() {
    let owner: Key = runtime::get_named_arg("owner");
    let spender: Key = runtime::get_named_arg("spender");
    let ret: U256 = Pair::default().allowance(owner, spender);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// This function is to get a Total Supply
///

#[no_mangle]
fn total_supply() {
    let ret: U256 = Pair::default().total_supply();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// This function is to get a Treasury Fee
///

#[no_mangle]
fn treasury_fee() {
    let ret: U256 = Pair::default().get_treasury_fee();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// This function is to fetch a Token0
///

#[no_mangle]
fn token0() {
    let ret: Key = Pair::default().get_token0();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// This function is to fetch a Token1
///

#[no_mangle]
fn token1() {
    let ret: Key = Pair::default().get_token1();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// This method will be called once by the factory at time of create_pair() method
///
/// This function is to Initialize Pair Contract with Token0 and Token1 and called in Factory Contract method create_pair()
///

#[no_mangle]
pub extern "C" fn initialize() {
    let token0: Key = runtime::get_named_arg("token0");
    let token1: Key = runtime::get_named_arg("token1");
    let factory_hash: Key = runtime::get_named_arg("factory_hash");

    Pair::default().initialize(token0, token1, factory_hash);
}

/// This function is to set a treasury_fee
///
/// # Parameters
///
/// * `treasury_fee` - A U256 that holds the value that is going to be a treasury_fee
///

#[no_mangle]
pub extern "C" fn set_treasury_fee_percent() {
    let treasury_fee: U256 = runtime::get_named_arg("treasury_fee");
    Pair::default().set_treasury_fee_percent(treasury_fee);
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("name", String::cl_type()),
            Parameter::new("symbol", String::cl_type()),
            Parameter::new("decimals", u8::cl_type()),
            Parameter::new("initial_supply", U256::cl_type()),
            Parameter::new("nonce", U256::cl_type()),
            Parameter::new("domain_separator", String::cl_type()),
            Parameter::new("permit_type_hash", String::cl_type()),
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("reserve0", U128::cl_type()),
            Parameter::new("reserve1", U128::cl_type()),
            Parameter::new("block_timestamp_last", u64::cl_type()),
            Parameter::new("price0_cumulative_last", U256::cl_type()),
            Parameter::new("price1_cumulative_last", U256::cl_type()),
            Parameter::new("k_last", U256::cl_type()), // reserve0 * reserve1, as of immediately after the most recent liquidity event
            Parameter::new("treasury_fee", U256::cl_type()),
            Parameter::new("minimum_liquidity", U256::cl_type()),
            Parameter::new("callee_contract_hash", Key::cl_type()),
            Parameter::new("factory_hash", Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "transfer",
        vec![
            Parameter::new("recipient", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "transfer_from",
        vec![
            Parameter::new("owner", Key::cl_type()),
            Parameter::new("recipient", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "swap",
        vec![
            Parameter::new("amount0_out", U256::cl_type()),
            Parameter::new("amount1_out", U256::cl_type()),
            Parameter::new("to", Key::cl_type()),
            Parameter::new("data", String::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "skim",
        vec![Parameter::new("to", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "sync",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "permit",
        vec![
            Parameter::new("public", String::cl_type()),
            Parameter::new("signature", String::cl_type()),
            Parameter::new("owner", Key::cl_type()),
            Parameter::new("spender", Key::cl_type()),
            Parameter::new("value", U256::cl_type()),
            Parameter::new("deadline", u64::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "approve",
        vec![
            Parameter::new("spender", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "balance_of",
        vec![Parameter::new("owner", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "nonce",
        vec![Parameter::new("owner", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "allowance",
        vec![
            Parameter::new("owner", Key::cl_type()),
            Parameter::new("spender", Key::cl_type()),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "total_supply",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "mint",
        vec![Parameter::new("to", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "burn",
        vec![Parameter::new("to", Key::cl_type())],
        CLType::Tuple2([Box::new(CLType::U256), Box::new(CLType::U256)]),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "treasury_fee",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "set_treasury_fee_percent",
        vec![Parameter::new("treasury_fee", U256::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "token0",
        vec![],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "token1",
        vec![],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "initialize",
        vec![
            Parameter::new("token0", Key::cl_type()),
            Parameter::new("token1", Key::cl_type()),
            Parameter::new("factory_hash", Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_reserves",
        vec![],
        CLType::Tuple3([
            Box::new(CLType::U128),
            Box::new(CLType::U128),
            Box::new(u64::cl_type()),
        ]),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "erc20_mint",
        vec![
            Parameter::new("to", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points
}

#[no_mangle]
fn call() {
    // Build new package with initial a first version of the contract.
    let (package_hash, access_token) = storage::create_contract_package_at_hash();
    let (contract_hash, _) =
        storage::add_contract_version(package_hash, get_entry_points(), Default::default());
    let name: String = runtime::get_named_arg("name");
    let symbol: String = runtime::get_named_arg("symbol");
    let decimals: u8 = runtime::get_named_arg("decimals");
    let initial_supply: U256 = runtime::get_named_arg("initial_supply");
    let nonce: U256 = 0.into();
    let callee_contract_hash: Key = runtime::get_named_arg("callee_contract_hash");
    let factory_hash: Key = runtime::get_named_arg("factory_hash");
    let eip_712_domain: &str =
        "EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)";
    let permit_type: &str =
        "Permit(address owner,address spender,uint256 value,uint256 nonce,uint256 deadline)";
    let chain_id: &str = "101";
    let eip_domain_hash = keccak256(eip_712_domain.as_bytes()); // to take a byte hash of EIP712Domain
    let name_hash = keccak256(name.as_bytes()); // to take a byte hash of name
    let one_hash = keccak256("1".as_bytes()); // to take a byte hash of "1"
    let eip_domain_hash = encode(eip_domain_hash); // to encode and convert eip_domain_hash into string
    let name_hash = encode(name_hash); // to encode and convert name_hash into string
    let one_hash = encode(one_hash); // to encode and convert one_hash into string
    let concatenated_data: String = format!(
        "{}{}{}{}{}",
        eip_domain_hash, name_hash, one_hash, chain_id, contract_hash
    ); //string contactination
    let domain_separator = keccak256(concatenated_data.as_bytes()); //to take a byte hash of concatenated Data
    let permit_type_hash = keccak256(permit_type.as_bytes()); // to take a byte hash of Permit Type
    let domain_separator = encode(domain_separator);
    let permit_type_hash = encode(permit_type_hash);
    let minimum_liquidity: U256 = (10 ^ 3).into();
    let reserve0: U128 = 0.into();
    let reserve1: U128 = 0.into();
    let block_timestamp_last: u64 = 0;
    let price0_cumulative_last: U256 = 0.into();
    let price1_cumulative_last: U256 = 0.into();
    let k_last: U256 = 0.into(); // reserve0 * reserve1, as of immediately after the most recent liquidity event
    let treasury_fee: U256 = 0.into();

    // Prepare constructor args
    let constructor_args = runtime_args! {
        "name" => name,
        "symbol" => symbol,
        "decimals" => decimals,
        "initial_supply" => initial_supply,
        "nonce" => nonce,
        "domain_separator" => domain_separator,
        "permit_type_hash" => permit_type_hash,
        "contract_hash" => contract_hash,
        "reserve0" => reserve0,
        "reserve1" => reserve1,
        "block_timestamp_last" => block_timestamp_last,
        "price0_cumulative_last" => price0_cumulative_last,
        "price1_cumulative_last" => price1_cumulative_last,
        "k_last" => k_last,
        "treasury_fee" => treasury_fee,
        "minimum_liquidity" => minimum_liquidity,
        "callee_contract_hash" => callee_contract_hash,
        "factory_hash" => factory_hash
    };

    // Add the constructor group to the package hash with a single URef.
    let constructor_access: URef =
        storage::create_contract_user_group(package_hash, "constructor", 1, Default::default())
            .unwrap_or_revert()
            .pop()
            .unwrap_or_revert();

    // Call the constructor entry point
    let _: () =
        runtime::call_versioned_contract(package_hash, None, "constructor", constructor_args);

    // Remove all URefs from the constructor group, so no one can call it for the second time.
    let mut urefs = BTreeSet::new();
    urefs.insert(constructor_access);
    storage::remove_contract_user_group_urefs(package_hash, "constructor", urefs)
        .unwrap_or_revert();

    // Store contract in the account's named keys.
    let contract_name: alloc::string::String = runtime::get_named_arg("contract_name");

    runtime::put_key(
        &format!("{}_package_hash", contract_name),
        package_hash.into(),
    );
    runtime::put_key(
        &format!("{}_package_hash_wrapped", contract_name),
        storage::new_uref(package_hash).into(),
    );
    runtime::put_key(
        &format!("{}_contract_hash", contract_name),
        contract_hash.into(),
    );
    runtime::put_key(
        &format!("{}_contract_hash_wrapped", contract_name),
        storage::new_uref(contract_hash).into(),
    );
    runtime::put_key(
        &format!("{}_package_access_token", contract_name),
        access_token.into(),
    );
}
