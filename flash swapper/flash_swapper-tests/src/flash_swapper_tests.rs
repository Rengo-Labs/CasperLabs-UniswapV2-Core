use casper_engine_test_support::AccountHash;
use casper_types::{runtime_args, Key, RuntimeArgs, U256};
use test_env::{Sender, TestContract, TestEnv};

use crate::flash_swapper_instance::FlashSwapperInstance;
use crate::test_instance::TESTInstance;

fn deploy_factory(env: &TestEnv) -> TestContract {
    // deploy factory contract
    let owner_factory = env.next_user();
    let factory = TestContract::new(
        &env,
        "factory.wasm",
        "factory",
        Sender(owner_factory),
        runtime_args! {
            "fee_to_setter" => Key::from(owner_factory)
            // contract_name is passed seperately, so we don't need to pass it here.
        },
    );
    factory
}

fn deploy_wcspr(env: &TestEnv) -> TestContract {
    // deploy wcspr contract
    let decimals: u8 = 18;
    let init_total_supply: U256 = 1000.into();
    let owner_wcspr = env.next_user();
    let wcspr = TestContract::new(
        &env,
        "wcspr-token.wasm",
        "wcspr",
        Sender(owner_wcspr),
        runtime_args! {
            "initial_supply" => init_total_supply,
            "name" => "Wrapper Casper",
            "symbol" => "WCSPR",
            "decimals" => decimals
        },
    );
    wcspr
}

fn deploy_pair(env: &TestEnv, factory: &TestContract, calle: Key) -> TestContract {
    // deploy wcspr contract
    let decimals: u8 = 18;
    let init_total_supply: U256 = 1000.into();
    let owner_pair = env.next_user();
    let pair = TestContract::new(
        &env,
        "pair-token.wasm",
        "pair",
        Sender(owner_pair),
        runtime_args! {
            "initial_supply" => init_total_supply,
            "name" => "ERC20",
            "symbol" => "ERC",
            "decimals" => decimals,
            "callee_contract_hash" => calle,
            "factory_hash" => Key::Hash(factory.contract_hash())
        },
    );
    pair
}

fn deploy_flash_swapper() -> (
    TestEnv,
    FlashSwapperInstance,
    AccountHash,
    TestContract,
    TestContract,
    TestContract,
    TestContract,
    TESTInstance,
) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let factory = deploy_factory(&env);
    let wcspr = deploy_wcspr(&env);
    let dai = deploy_wcspr(&env);
    let btc = deploy_wcspr(&env);
    let flash_swapper = FlashSwapperInstance::new(
        &env,
        "flash_swapper",
        Sender(owner),
        Key::Hash(wcspr.contract_hash()),
        Key::Hash(dai.contract_hash()),
        Key::Hash(factory.contract_hash()),
    );
    let test = TESTInstance::new(&env, "TEST", Sender(owner), "TEST");
    (env, flash_swapper, owner, factory, wcspr, dai, btc, test)
}

#[test]
fn test_flash_swapper_deploy() {
    let (_, flash_swapper, _, _, _, _, _, _) = deploy_flash_swapper();
    let self_hash: Key = flash_swapper.self_contract_hash();
    let zero_addr: Key = Key::from_formatted_str(
        "hash-0000000000000000000000000000000000000000000000000000000000000000",
    )
    .unwrap();
    assert_ne!(self_hash, zero_addr);
}

#[test]
fn test_start_swap_with_simple_flash_loan() {
    let (env, flash_swapper, owner, factory, wcspr, dai, _, test) = deploy_flash_swapper();
    let pair = deploy_pair(&env, &factory, flash_swapper.self_contract_hash());
    let amount: U256 = 500.into();
    test.create_pair(
        Sender(owner),
        Key::Hash(dai.contract_hash()),
        Key::Hash(wcspr.contract_hash()),
        Key::Hash(pair.contract_hash()),
        Key::Hash(factory.contract_hash()),
    );
    test.token0(Sender(owner), Key::Hash(pair.contract_hash()));
    test.token1(Sender(owner), Key::Hash(pair.contract_hash()));
    test.pair_mint(
        Sender(owner),
        Key::Hash(pair.contract_hash()),
        test.get_token0(),
        amount,
    );
    test.pair_mint(
        Sender(owner),
        Key::Hash(pair.contract_hash()),
        test.get_token1(),
        amount,
    );
    test.sync(Sender(owner), Key::Hash(pair.contract_hash()));
    flash_swapper.start_swap(
        Sender(owner),
        Key::Hash(wcspr.contract_hash()),
        100.into(),
        Key::Hash(wcspr.contract_hash()),
        "User Data".into(),
    );
}

#[test]
fn test_start_swap_with_simple_flash_swap() {
    let (env, flash_swapper, owner, factory, wcspr, dai, _, test) = deploy_flash_swapper();
    let pair = deploy_pair(&env, &factory, flash_swapper.self_contract_hash());
    test.create_pair(
        Sender(owner),
        Key::Hash(dai.contract_hash()),
        Key::Hash(wcspr.contract_hash()),
        Key::Hash(pair.contract_hash()),
        Key::Hash(factory.contract_hash()),
    );
    let amount: U256 = 500.into();
    test.token0(Sender(owner), Key::Hash(pair.contract_hash()));
    test.token1(Sender(owner), Key::Hash(pair.contract_hash()));
    test.pair_mint(
        Sender(owner),
        Key::Hash(pair.contract_hash()),
        test.get_token0(),
        amount,
    );
    test.pair_mint(
        Sender(owner),
        Key::Hash(pair.contract_hash()),
        test.get_token1(),
        amount,
    );
    test.sync(Sender(owner), Key::Hash(pair.contract_hash()));
    test.pair_mint(
        Sender(owner),
        Key::Hash(pair.contract_hash()),
        test.get_token0(),
        amount,
    );
    test.pair_mint(
        Sender(owner),
        Key::Hash(pair.contract_hash()),
        test.get_token1(),
        amount,
    );
    flash_swapper.start_swap(
        Sender(owner),
        Key::Hash(dai.contract_hash()),
        100.into(),
        Key::from_formatted_str(
            "hash-0000000000000000000000000000000000000000000000000000000000000000",
        )
        .unwrap(),
        "User Data".into(),
    );
}

#[test]
fn test_start_swap_with_traingular_flash_swap() {
    let (env, flash_swapper, owner, factory, wcspr, dai, btc, test) = deploy_flash_swapper();
    let pair = deploy_pair(&env, &factory, flash_swapper.self_contract_hash());
    let amount: U256 = 500.into();
    test.create_pair(
        Sender(owner),
        Key::Hash(btc.contract_hash()),
        Key::Hash(wcspr.contract_hash()),
        Key::Hash(pair.contract_hash()),
        Key::Hash(factory.contract_hash()),
    );
    test.token0(Sender(owner), Key::Hash(pair.contract_hash()));
    test.token1(Sender(owner), Key::Hash(pair.contract_hash()));
    test.pair_mint(
        Sender(owner),
        Key::Hash(pair.contract_hash()),
        test.get_token0(),
        amount,
    );
    test.pair_mint(
        Sender(owner),
        Key::Hash(pair.contract_hash()),
        test.get_token1(),
        amount,
    );
    test.create_pair(
        Sender(owner),
        Key::Hash(dai.contract_hash()),
        Key::Hash(wcspr.contract_hash()),
        Key::Hash(pair.contract_hash()),
        Key::Hash(factory.contract_hash()),
    );
    test.token0(Sender(owner), Key::Hash(pair.contract_hash()));
    test.token1(Sender(owner), Key::Hash(pair.contract_hash()));
    test.pair_mint(
        Sender(owner),
        Key::Hash(pair.contract_hash()),
        test.get_token0(),
        amount,
    );
    test.pair_mint(
        Sender(owner),
        Key::Hash(pair.contract_hash()),
        test.get_token1(),
        amount,
    );
    test.pair_mint(
        Sender(owner),
        Key::Hash(pair.contract_hash()),
        Key::Hash(dai.contract_hash()),
        amount,
    );
    test.pair_mint(
        Sender(owner),
        Key::Hash(pair.contract_hash()),
        Key::Hash(btc.contract_hash()),
        amount,
    );
    test.pair_mint(
        Sender(owner),
        Key::Hash(pair.contract_hash()),
        Key::Hash(wcspr.contract_hash()),
        amount,
    );
    test.sync(Sender(owner), Key::Hash(pair.contract_hash()));
    flash_swapper.start_swap(
        Sender(owner),
        Key::Hash(dai.contract_hash()),
        10.into(),
        Key::Hash(btc.contract_hash()),
        "User Data".into(),
    );
}

#[test]
#[should_panic]
fn test_calling_construction() {
    let (_env, flash_swapper, owner, factory, wcspr, dai, _, _) = deploy_flash_swapper();
    flash_swapper.constructor(
        Sender(owner),
        Key::Hash(wcspr.contract_hash()),
        Key::Hash(dai.contract_hash()),
        Key::Hash(factory.contract_hash()),
    );
}
