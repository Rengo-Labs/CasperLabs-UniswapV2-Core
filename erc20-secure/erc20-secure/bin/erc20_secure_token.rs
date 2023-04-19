#![no_main]

use casperlabs_ownable::OWNABLE;
use common::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
    *,
};
use erc20_secure_crate::{Address, ERC20};
use std::collections::BTreeSet;
#[derive(Default)]
struct Token(OnChainContractStorage);
impl Token {
    fn constructor(&mut self, contract_hash: ContractHash, package_hash: ContractPackageHash) {
        ERC20::init(self, contract_hash, package_hash);
    }
}

impl ERC20<OnChainContractStorage> for Token {}
impl OWNABLE<OnChainContractStorage> for Token {}
impl ContractContext<OnChainContractStorage> for Token {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

#[no_mangle]
fn constructor() {
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    Token::default().constructor(contract_hash, package_hash)
}
#[no_mangle]
fn owner() {
    let ret: Key = OWNABLE::owner(&Token::default());
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn is_owner() {
    let ret: bool = OWNABLE::is_owner(&Token::default());
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn renounce_ownership() {
    OWNABLE::renounce_ownership(&mut Token::default());
}
#[no_mangle]
fn transfer_ownership() {
    let new_owner: Key = runtime::get_named_arg("new_owner");
    OWNABLE::transfer_ownership(&mut Token::default(), new_owner);
}

/// This function is to return the Name of contract
#[no_mangle]
fn name() {
    runtime::ret(CLValue::from_t(Token::default().name()).unwrap_or_revert());
}

/// This function is to return the Symbol of contract
#[no_mangle]
fn symbol() {
    runtime::ret(CLValue::from_t(Token::default().symbol()).unwrap_or_revert());
}

/// This function is to return the Decimals of contract
#[no_mangle]
fn decimals() {
    runtime::ret(CLValue::from_t(Token::default().decimals()).unwrap_or_revert());
}

/// This function is to return the Total Supply of the contract
#[no_mangle]
fn total_supply() {
    runtime::ret(CLValue::from_t(Token::default().total_supply()).unwrap_or_revert());
}

/// This function is to return the Balance  of owner against the address that user provided
/// # Parameters
/// * `owner` - Address that holds the account address of the user against which user wants to get balance
#[no_mangle]
fn balance_of() {
    let owner: Address = runtime::get_named_arg("owner");
    runtime::ret(CLValue::from_t(Token::default().balance_of(owner)).unwrap_or_revert());
}

/// This function is to return the Allowance of owner and spender that user provided
/// # Parameters
/// * `owner` - Address that holds the account address of the user
/// * `spender` - Address that holds the account address of the user
#[no_mangle]
fn allowance() {
    let owner: Address = runtime::get_named_arg("owner");
    let spender: Address = runtime::get_named_arg("spender");
    runtime::ret(CLValue::from_t(Token::default().allowance(owner, spender)).unwrap_or_revert());
}

/// NOTE: Custom function
/// This function is to increase approval in the safe way, avoid front running
/// # Parameters
/// * `spender` - Address that holds the account address of the spender
/// * `amount` - Amount of approval to be increased
#[no_mangle]
fn increase_allowance() {
    let spender: Address = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");
    Token::default()
        .increase_allowance(spender, amount)
        .unwrap_or_revert();
}

/// NOTE: Custom function
/// This function is to decrease approval in the safe way, avoid front running
/// # Parameters
/// * `spender` - Address that holds the account address of the spender
/// * `amount` - Amount of approval to be decreased
#[no_mangle]
fn decrease_allowance() {
    let spender: Address = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");
    Token::default()
        .decrease_allowance(spender, amount)
        .unwrap_or_revert();
}

/// This function is to approve tokens against the address that user provided
/// # Parameters
/// * `spender` - Address that holds the account address of the user
/// * `amount` - A U256 that holds the amount for approve
/// **Recommendation:**
/// The exploit is mitigated through use of functions that increase/decrease the allowance relative to its current value, such as `increaseAllowance()` and `decreaseAllowance()`.
/// Pending community agreement on an ERC standard that would protect against this exploit, we recommend that developers of applications dependent on approve() / transferFrom()
/// should keep in mind that they have to set allowance to 0 first and verify if it was used before setting the new value.
/// **Note:**  Teams who decide to wait for such a standard should make these
/// recommendations to app developers who work with their token contract.
#[no_mangle]
fn approve() {
    let spender: Address = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");
    Token::default().approve(spender, amount).unwrap_or_revert();
}

/// This function is to transfer tokens against the address that user provided
/// # Parameters
/// * `recipient` - Address that holds the account address of the user
/// * `amount` - A U256 that holds the amount for transfer
#[no_mangle]
fn transfer() {
    let recipient: Address = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");
    Token::default()
        .transfer(recipient, amount)
        .unwrap_or_revert();
}

/// This function is to transfer tokens against the address that has been approved before by owner
/// # Parameters
/// * `owner` - Address that holds the account address of the user
/// * `recipient` - Address that holds the account address of the user
/// * `amount` - A U256 that holds the amount for transfer
/// **Recommendation:**
/// The exploit is mitigated through use of functions that increase/decrease the allowance relative to its current value, such as `increaseAllowance()` and `decreaseAllowance()`.
/// Pending community agreement on an ERC standard that would protect against this exploit, we recommend that developers of applications dependent on approve() / transferFrom()
/// should keep in mind that they have to set allowance to 0 first and verify if it was used before setting the new value.
/// **Note:**  Teams who decide to wait for such a standard should make these
/// recommendations to app developers who work with their token contract.
#[no_mangle]
fn transfer_from() {
    let owner: Address = runtime::get_named_arg("owner");
    let recipient: Address = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");
    Token::default()
        .transfer_from(owner, recipient, amount)
        .unwrap_or_revert();
}

/// This function is to mint token against the address that user provided
/// # Parameters
/// * `to` - Address that holds the account address of the user
/// * `amount` - A U256 that holds the amount for mint
#[no_mangle]
fn mint() {
    let to: Address = runtime::get_named_arg("to");
    let amount: U256 = runtime::get_named_arg("amount");
    Token::default().mint(to, amount).unwrap_or_revert();
}

/// This function is to burn token against the address that user provided
/// # Parameters
/// * `from` - Address that holds the account address of the user
/// * `amount` - A U256 that holds the amount for burn
#[no_mangle]
fn burn() {
    let from: Address = runtime::get_named_arg("from");
    let amount: U256 = runtime::get_named_arg("amount");
    Token::default().burn(from, amount).unwrap_or_revert();
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "owner",
        vec![],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "is_owner",
        vec![],
        bool::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "renounce_ownership",
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "transfer_ownership",
        vec![Parameter::new("new_owner", Key::cl_type())],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "name",
        vec![],
        CLType::String,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "symbol",
        vec![],
        CLType::String,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "decimals",
        vec![],
        CLType::U8,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "total_supply",
        vec![],
        CLType::U256,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "balance_of",
        vec![Parameter::new("owner", Address::cl_type())],
        CLType::U256,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "allowance",
        vec![
            Parameter::new("owner", Address::cl_type()),
            Parameter::new("spender", Address::cl_type()),
        ],
        CLType::U256,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "increase_allowance",
        vec![
            Parameter::new("spender", Address::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "decrease_allowance",
        vec![
            Parameter::new("spender", Address::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "approve",
        vec![
            Parameter::new("spender", Address::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "transfer",
        vec![
            Parameter::new("recipient", Address::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "transfer_from",
        vec![
            Parameter::new("owner", Address::cl_type()),
            Parameter::new("recipient", Address::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "mint",
        vec![
            Parameter::new("to", Address::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "burn",
        vec![
            Parameter::new("from", Address::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points
}

#[no_mangle]
fn call() {
    // Contract name must be same for all new versions of the contracts
    let contract_name: String = runtime::get_named_arg("contract_name");

    // If this is the first deployment
    if !runtime::has_key(&format!("{}_package_hash", contract_name)) {
        // Read arguments for the constructor call.
        let name: String = runtime::get_named_arg("name");
        let symbol: String = runtime::get_named_arg("symbol");
        let decimals: u8 = runtime::get_named_arg("decimals");
        let initial_supply: U256 = runtime::get_named_arg("initial_supply");

        // Build new package with initial a first version of the contract.
        let (package_hash, access_token) = storage::create_contract_package_at_hash();
        let (contract_hash, _) = storage::add_contract_version(
            package_hash,
            get_entry_points(),
            Token::default()
                .named_keys(name, symbol, decimals, initial_supply, package_hash)
                .unwrap_or_revert(),
        );

        // Prepare constructor args
        let constructor_args = runtime_args! {
            "contract_hash" => contract_hash,
            "package_hash"=> package_hash
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
    } else {
        // this is a contract upgrade

        let package_hash: ContractPackageHash =
            runtime::get_key(&format!("{}_package_hash", contract_name))
                .unwrap_or_revert()
                .into_hash()
                .unwrap()
                .into();

        let (contract_hash, _): (ContractHash, _) =
            storage::add_contract_version(package_hash, get_entry_points(), Default::default());

        // update contract hash
        runtime::put_key(
            &format!("{}_contract_hash", contract_name),
            contract_hash.into(),
        );
        runtime::put_key(
            &format!("{}_contract_hash_wrapped", contract_name),
            storage::new_uref(contract_hash).into(),
        );
    }
}
