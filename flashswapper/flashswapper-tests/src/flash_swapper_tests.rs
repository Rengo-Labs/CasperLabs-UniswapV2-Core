use crate::flash_swapper_instance::FlashSwapperInstance;
use tests_common::{account::AccountHash, deploys::*, functions::zero_address, helpers::*, *};

fn deploy_flash_swapper() -> (
    TestEnv,
    FlashSwapperInstance,
    AccountHash,
    TestContract,
    TestContract,
    TestContract,
    TestContract,
) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let factory = deploy_factory(&env, owner, Key::Account(owner), now());
    let wcspr = deploy_wcspr(
        &env,
        "WCSPR-1",
        owner,
        WRAPPED_CSPR.into(),
        "WCSPR".into(),
        9,
        0.into(),
        now(),
    );
    let dai = deploy_wcspr(
        &env,
        "WCSPR-2",
        owner,
        "Dai Token".into(),
        "DAI".into(),
        9,
        0.into(),
        now(),
    );
    let btc = deploy_wcspr(
        &env,
        "WCSPR-3",
        owner,
        "Bitcoin".into(),
        "BTC".into(),
        9,
        0.into(),
        now(),
    );
    let flash_swapper = FlashSwapperInstance::new(
        &env,
        "flash_swapper",
        owner,
        Key::Hash(wcspr.contract_hash()),
        Key::Hash(dai.contract_hash()),
        Key::Hash(factory.contract_hash()),
        now(),
    );
    (env, flash_swapper, owner, factory, wcspr, dai, btc)
}

#[test]
fn test_flash_swapper_deploy() {
    let (_, flash_swapper, _, _, _, _, _) = deploy_flash_swapper();
    let self_hash: ContractHash = flash_swapper.self_contract_hash();
    assert_ne!(self_hash, zero_address().into_hash().unwrap().into());
}
// todo:
// will be done later when purses are supported in test cases

// #[test]
// fn test_start_swap_with_simple_flash_loan() {
//     let (env, flash_swapper, owner, factory, wcspr, dai, _, test) = deploy_flash_swapper();
//     let pair = deploy_pair(&env, &factory, flash_swapper.self_contract_hash());
//     let amount: U256 = 500.into();
//     test.create_pair(
//         owner,
//         Key::Hash(dai.contract_hash()),
//         Key::Hash(wcspr.contract_hash()),
//         Key::Hash(pair.contract_hash()),
//         Key::Hash(factory.contract_hash()),
//     );
//     test.token0(owner, Key::Hash(pair.contract_hash()));
//     test.token1(owner, Key::Hash(pair.contract_hash()));
//     test.pair_mint(
//         owner,
//         Key::Hash(pair.contract_hash()),
//         test.get_token0(),
//         amount,
//     );
//     test.pair_mint(
//         owner,
//         Key::Hash(pair.contract_hash()),
//         test.get_token1(),
//         amount,
//     );
//     test.sync(owner, Key::Hash(pair.contract_hash()));
//     flash_swapper.start_swap(
//         owner,
//         Key::Hash(wcspr.contract_hash()),
//         100.into(),
//         Key::Hash(wcspr.contract_hash()),
//         "User Data".into(),
//     );
// }

// #[test]
// fn test_start_swap_with_simple_flash_swap() {
//     let (env, flash_swapper, owner, factory, wcspr, dai, _, test) = deploy_flash_swapper();
//     let pair = deploy_pair(&env, &factory, flash_swapper.self_contract_hash());
//     test.create_pair(
//         owner,
//         Key::Hash(dai.contract_hash()),
//         Key::Hash(wcspr.contract_hash()),
//         Key::Hash(pair.contract_hash()),
//         Key::Hash(factory.contract_hash()),
//     );
//     let amount: U256 = 500.into();
//     test.token0(owner, Key::Hash(pair.contract_hash()));
//     test.token1(owner, Key::Hash(pair.contract_hash()));
//     test.pair_mint(
//         owner,
//         Key::Hash(pair.contract_hash()),
//         test.get_token0(),
//         amount,
//     );
//     test.pair_mint(
//         owner,
//         Key::Hash(pair.contract_hash()),
//         test.get_token1(),
//         amount,
//     );
//     test.sync(owner, Key::Hash(pair.contract_hash()));
//     test.pair_mint(
//         owner,
//         Key::Hash(pair.contract_hash()),
//         test.get_token0(),
//         amount,
//     );
//     test.pair_mint(
//         owner,
//         Key::Hash(pair.contract_hash()),
//         test.get_token1(),
//         amount,
//     );
//     flash_swapper.start_swap(
//         owner,
//         Key::Hash(dai.contract_hash()),
//         100.into(),
//         Key::from_formatted_str(
//             "hash-0000000000000000000000000000000000000000000000000000000000000000",
//         )
//         .unwrap(),
//         "User Data".into(),
//     );
// }

// #[test]
// fn test_start_swap_with_traingular_flash_swap() {
//     let (env, flash_swapper, owner, factory, wcspr, dai, btc, test) = deploy_flash_swapper();
//     let pair = deploy_pair(&env, &factory, flash_swapper.self_contract_hash());
//     let amount: U256 = 500.into();
//     test.create_pair(
//         owner,
//         Key::Hash(btc.contract_hash()),
//         Key::Hash(wcspr.contract_hash()),
//         Key::Hash(pair.contract_hash()),
//         Key::Hash(factory.contract_hash()),
//     );
//     test.token0(owner, Key::Hash(pair.contract_hash()));
//     test.token1(owner, Key::Hash(pair.contract_hash()));
//     test.pair_mint(
//         owner,
//         Key::Hash(pair.contract_hash()),
//         test.get_token0(),
//         amount,
//     );
//     test.pair_mint(
//         owner,
//         Key::Hash(pair.contract_hash()),
//         test.get_token1(),
//         amount,
//     );
//     test.create_pair(
//         owner,
//         Key::Hash(dai.contract_hash()),
//         Key::Hash(wcspr.contract_hash()),
//         Key::Hash(pair.contract_hash()),
//         Key::Hash(factory.contract_hash()),
//     );
//     test.token0(owner, Key::Hash(pair.contract_hash()));
//     test.token1(owner, Key::Hash(pair.contract_hash()));
//     test.pair_mint(
//         owner,
//         Key::Hash(pair.contract_hash()),
//         test.get_token0(),
//         amount,
//     );
//     test.pair_mint(
//         owner,
//         Key::Hash(pair.contract_hash()),
//         test.get_token1(),
//         amount,
//     );
//     test.pair_mint(
//         owner,
//         Key::Hash(pair.contract_hash()),
//         Key::Hash(dai.contract_hash()),
//         amount,
//     );
//     test.pair_mint(
//         owner,
//         Key::Hash(pair.contract_hash()),
//         Key::Hash(btc.contract_hash()),
//         amount,
//     );
//     test.pair_mint(
//         owner,
//         Key::Hash(pair.contract_hash()),
//         Key::Hash(wcspr.contract_hash()),
//         amount,
//     );
//     test.sync(owner, Key::Hash(pair.contract_hash()));
//     flash_swapper.start_swap(
//         owner,
//         Key::Hash(dai.contract_hash()),
//         10.into(),
//         Key::Hash(btc.contract_hash()),
//         "User Data".into(),
//     );
// }

#[test]
#[should_panic]
fn test_calling_construction() {
    let (_env, flash_swapper, owner, factory, wcspr, dai, _) = deploy_flash_swapper();
    flash_swapper.constructor(
        owner,
        Key::Hash(wcspr.contract_hash()),
        Key::Hash(dai.contract_hash()),
        Key::Hash(factory.contract_hash()),
        now(),
    );
}
