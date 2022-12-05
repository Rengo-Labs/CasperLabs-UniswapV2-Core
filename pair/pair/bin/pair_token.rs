#![no_main]

use casperlabs_erc20::{Address, ERC20};
use common::{
    contract_api::{runtime, storage},
    runtime_args,
    unwrap_or_revert::UnwrapOrRevert,
    *,
};
use pair::{
    self,
    data::{get_token0, get_token1, get_treasury_fee},
    PAIR,
};
use std::collections::BTreeSet;

#[derive(Default)]
struct Pair(OnChainContractStorage);
impl Pair {
    #[allow(clippy::too_many_arguments)]
    fn constructor(
        &self,
        reserve0: U128,
        reserve1: U128,
        block_timestamp_last: u64,
        price0_cumulative_last: U256,
        price1_cumulative_last: U256,
        k_last: U256,
        treasury_fee: U256,
        minimum_liquidity: U256,
        callee_package_hash: Key,
        factory_hash: Key,
        lock: u64,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
    ) {
        PAIR::init(
            self,
            reserve0,
            reserve1,
            block_timestamp_last,
            price0_cumulative_last,
            price1_cumulative_last,
            k_last,
            treasury_fee,
            minimum_liquidity,
            callee_package_hash,
            factory_hash,
            lock,
            contract_hash,
            package_hash,
        );
    }
}

impl ContractContext<OnChainContractStorage> for Pair {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}
impl PAIR<OnChainContractStorage> for Pair {}
impl ERC20<OnChainContractStorage> for Pair {}

#[no_mangle]
fn constructor() {
    let reserve0: U128 = runtime::get_named_arg("reserve0");
    let reserve1: U128 = runtime::get_named_arg("reserve1");
    let block_timestamp_last: u64 = runtime::get_named_arg("block_timestamp_last");
    let price0_cumulative_last: U256 = runtime::get_named_arg("price0_cumulative_last");
    let price1_cumulative_last: U256 = runtime::get_named_arg("price1_cumulative_last");
    let k_last: U256 = runtime::get_named_arg("k_last"); // reserve0 * reserve1, as of immediately after the most recent liquidity event
    let treasury_fee: U256 = runtime::get_named_arg("treasury_fee");
    let minimum_liquidity: U256 = runtime::get_named_arg("minimum_liquidity");
    let callee_package_hash: Key = runtime::get_named_arg("callee_package_hash");
    let factory_hash: Key = runtime::get_named_arg("factory_hash");
    let lock: u64 = runtime::get_named_arg("lock");
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    Pair::default().constructor(
        reserve0,
        reserve1,
        block_timestamp_last,
        price0_cumulative_last,
        price1_cumulative_last,
        k_last,
        treasury_fee,
        minimum_liquidity,
        callee_package_hash,
        factory_hash,
        lock,
        contract_hash,
        package_hash,
    );
}

/// This function is to return the Name of contract
#[no_mangle]
fn name() {
    runtime::ret(CLValue::from_t(Pair::default().name()).unwrap_or_revert());
}

/// This function is to return the Symbol of contract
#[no_mangle]
fn symbol() {
    runtime::ret(CLValue::from_t(Pair::default().symbol()).unwrap_or_revert());
}

/// This function is to return the Decimals of contract
#[no_mangle]
fn decimals() {
    runtime::ret(CLValue::from_t(Pair::default().decimals()).unwrap_or_revert());
}

/// This function is to return the Total Supply of the contract
#[no_mangle]
fn total_supply() {
    runtime::ret(CLValue::from_t(Pair::default().total_supply()).unwrap_or_revert());
}

/// This function is to return the Balance  of owner against the address that user provided
/// # Parameters
/// * `owner` - Address that holds the account address of the user against which user wants to get balance
#[no_mangle]
fn balance_of() {
    let owner: Address = runtime::get_named_arg("owner");
    runtime::ret(CLValue::from_t(Pair::default().balance_of(owner)).unwrap_or_revert());
}

/// This function is to return the Allowance of owner and spender that user provided
/// # Parameters
/// * `owner` - Address that holds the account address of the user
/// * `spender` - Address that holds the account address of the user
#[no_mangle]
fn allowance() {
    let owner: Address = runtime::get_named_arg("owner");
    let spender: Address = runtime::get_named_arg("spender");
    runtime::ret(CLValue::from_t(Pair::default().allowance(owner, spender)).unwrap_or_revert());
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
    Pair::default()
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
    Pair::default()
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
    Pair::default().approve(spender, amount).unwrap_or_revert();
}

/// This function is to transfer tokens against the address that user provided
/// # Parameters
/// * `recipient` - Address that holds the account address of the user
/// * `amount` - A U256 that holds the amount for transfer
#[no_mangle]
fn transfer() {
    let recipient: Address = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");
    Pair::default()
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
    Pair::default()
        .transfer_from(owner, recipient, amount)
        .unwrap_or_revert();
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

/// This function is to mint token against the address that user provided
/// # Parameters
/// * `to` - A Key that holds the account address of the user
#[no_mangle]
fn mint() {
    let to: Key = runtime::get_named_arg("to");
    let liquidity: U256 = PAIR::mint(&Pair::default(), to);
    runtime::ret(CLValue::from_t(liquidity).unwrap_or_revert());
}

/// This function is to burn token against the address that user provided
/// # Parameters
/// * `from` - A Key that holds the account address of the user
#[no_mangle]
fn burn() {
    let to: Key = runtime::get_named_arg("to");
    let (amount0, amount1): (U256, U256) = PAIR::burn(&Pair::default(), to);
    runtime::ret(CLValue::from_t((amount0, amount1)).unwrap_or_revert());
}

/// This function is to get the reserves like Reserve0, Reserve1 and Block Time Stamp
#[no_mangle]
fn get_reserves() {
    let (reserve0, reserve1, block_timestamp_last): (U128, U128, u64) =
        Pair::default().get_reserves();
    runtime::ret(CLValue::from_t((reserve0, reserve1, block_timestamp_last)).unwrap_or_revert());
}

/// This function is to get a Treasury Fee
#[no_mangle]
fn treasury_fee() {
    runtime::ret(CLValue::from_t(get_treasury_fee()).unwrap_or_revert());
}

/// This function is to set a treasury_fee
/// # Parameters
/// * `treasury_fee` - A U256 that holds the value that is going to be a treasury_fee
#[no_mangle]
fn set_treasury_fee_percent() {
    let treasury_fee: U256 = runtime::get_named_arg("treasury_fee");
    Pair::default().set_treasury_fee_percent(treasury_fee);
}

/// This function is to fetch a Token0
#[no_mangle]
fn token0() {
    runtime::ret(CLValue::from_t(get_token0()).unwrap_or_revert());
}

/// This function is to fetch a Token1
#[no_mangle]
fn token1() {
    runtime::ret(CLValue::from_t(get_token1()).unwrap_or_revert());
}

/// This method will be called once by the factory at time of create_pair() method
/// This function is to Initialize Pair Contract with Token0 and Token1 and called in Factory Contract method create_pair()
#[no_mangle]
fn initialize() {
    let token0: Key = runtime::get_named_arg("token0");
    let token1: Key = runtime::get_named_arg("token1");
    let factory_hash: Key = runtime::get_named_arg("factory_hash");

    Pair::default().initialize(token0, token1, factory_hash);
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("reserve0", U128::cl_type()),
            Parameter::new("reserve1", U128::cl_type()),
            Parameter::new("block_timestamp_last", u64::cl_type()),
            Parameter::new("price0_cumulative_last", U256::cl_type()),
            Parameter::new("price1_cumulative_last", U256::cl_type()),
            Parameter::new("k_last", U256::cl_type()),
            Parameter::new("treasury_fee", U256::cl_type()),
            Parameter::new("minimum_liquidity", U256::cl_type()),
            Parameter::new("callee_package_hash", Key::cl_type()),
            Parameter::new("factory_hash", Key::cl_type()),
            Parameter::new("lock", u64::cl_type()),
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
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
    entry_points
}

#[no_mangle]
fn call() {
    // Store contract in the account's named keys. Contract name must be same for all new versions of the contracts
    let contract_name: String = runtime::get_named_arg("contract_name");

    // If this is the first deployment
    if !runtime::has_key(&format!("{}_package_hash", contract_name)) {
        let name: String = runtime::get_named_arg("name");
        let symbol: String = runtime::get_named_arg("symbol");
        let decimals: u8 = runtime::get_named_arg("decimals");
        let initial_supply: U256 = runtime::get_named_arg("initial_supply");
        let callee_package_hash: Key = runtime::get_named_arg("callee_package_hash");
        let factory_hash: Key = runtime::get_named_arg("factory_hash");

        // Build new package with initial a first version of the contract.
        let (package_hash, access_token) = storage::create_contract_package_at_hash();
        let (contract_hash, _) = storage::add_contract_version(
            package_hash,
            get_entry_points(),
            Pair::default()
                .named_keys(name, symbol, decimals, initial_supply)
                .unwrap_or_revert(),
        );

        let base: i32 = 10;
        let minimum_liquidity: U256 = (base.pow(3)).into();
        let reserve0: U128 = 0.into();
        let reserve1: U128 = 0.into();
        let block_timestamp_last: u64 = 0;
        let price0_cumulative_last: U256 = 0.into();
        let price1_cumulative_last: U256 = 0.into();
        let k_last: U256 = 0.into(); // reserve0 * reserve1, as of immediately after the most recent liquidity event
        let treasury_fee: U256 = 3.into();
        let lock: u64 = 0;

        // Prepare constructor args
        let constructor_args = runtime_args! {
            "reserve0" => reserve0,
            "reserve1" => reserve1,
            "block_timestamp_last" => block_timestamp_last,
            "price0_cumulative_last" => price0_cumulative_last,
            "price1_cumulative_last" => price1_cumulative_last,
            "k_last" => k_last,
            "treasury_fee" => treasury_fee,
            "minimum_liquidity" => minimum_liquidity,
            "callee_package_hash" => callee_package_hash,
            "factory_hash" => factory_hash,
            "lock" => lock,
            "contract_hash" => contract_hash,
            "package_hash" => package_hash
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
