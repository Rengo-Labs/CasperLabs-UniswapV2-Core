use std::collections::BTreeMap;

use crate::data::*;
use crate::events::PAIREvent;
use casperlabs_erc20::{data::*, Address, ERC20};
use common::{
    contract_api::{runtime, storage},
    errors::Errors,
    functions::account_zero_address,
    unwrap_or_revert::UnwrapOrRevert,
    *,
};
use cryptoxide::ed25519;

pub trait PAIR<Storage: ContractStorage>: ContractContext<Storage> + ERC20<Storage> {
    #[allow(clippy::too_many_arguments)]
    fn init(
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
        set_reserve0(reserve0);
        set_reserve1(reserve1);
        set_block_timestamp_last(block_timestamp_last);
        set_price0_cumulative_last(price0_cumulative_last);
        set_price1_cumulative_last(price1_cumulative_last);
        set_k_last(k_last);
        set_treasury_fee(treasury_fee);
        set_minimum_liquidity(minimum_liquidity);
        set_callee_package_hash(callee_package_hash);
        set_factory_hash(factory_hash);
        set_lock(lock);
        ERC20::init(self, contract_hash, package_hash);
    }

    fn skim(&self, to: Key) {
        if get_lock() != 0 {
            //UniswapV2: Locked
            runtime::revert(Errors::UniswapV2CorePairLocked1);
        }
        set_lock(1);
        let balance0: U256 = runtime::call_versioned_contract(
            get_token0().into_hash().unwrap_or_revert().into(),
            None,
            "balance_of",
            runtime_args! {
                "owner" => Address::Contract(get_package_hash())
            },
        );
        let balance1: U256 = runtime::call_versioned_contract(
            get_token1().into_hash().unwrap_or_revert().into(),
            None,
            "balance_of",
            runtime_args! {
                "owner" => Address::Contract(get_package_hash())
            },
        );
        let amount0: U256 = balance0
            .checked_sub(get_reserve0().as_u128().into())
            .unwrap_or_revert_with(Errors::UniswapV2CorePairUnderFlow1);
        () = runtime::call_versioned_contract(
            get_token0().into_hash().unwrap_or_revert().into(),
            None,
            "transfer",
            runtime_args! {
                "recipient" => Address::from(to),
                "amount" => amount0
            },
        );
        let amount1: U256 = balance1
            .checked_sub(get_reserve1().as_u128().into())
            .unwrap_or_revert_with(Errors::UniswapV2CorePairUnderFlow2);
        () = runtime::call_versioned_contract(
            get_token1().into_hash().unwrap_or_revert().into(),
            None,
            "transfer",
            runtime_args! {
                "recipient" => Address::from(to),
                "amount" => amount1
            },
        );
        set_lock(0);
    }

    fn sync(&self) {
        if get_lock() != 0 {
            //UniswapV2: Locked
            runtime::revert(Errors::UniswapV2CorePairLocked2);
        }
        set_lock(1);
        let balance0: U256 = runtime::call_versioned_contract(
            get_token0().into_hash().unwrap_or_revert().into(),
            None,
            "balance_of",
            runtime_args! {
                "owner" => Address::Contract(get_package_hash())
            },
        );
        let balance1: U256 = runtime::call_versioned_contract(
            get_token0().into_hash().unwrap_or_revert().into(),
            None,
            "balance_of",
            runtime_args! {
                "owner" => Address::Contract(get_package_hash())
            },
        );
        self.update(balance0, balance1, get_reserve0(), get_reserve1());
        set_lock(0);
    }

    fn swap(&self, amount0_out: U256, amount1_out: U256, to: Key, _data: String) {
        if get_lock() != 0 {
            //UniswapV2: Locked
            runtime::revert(Errors::UniswapV2CorePairLocked3);
        }
        set_lock(1);
        if amount0_out > 0.into() || amount1_out > 0.into() {
            let (reserve0, reserve1, _) = self.get_reserves(); // gas savings
            if amount0_out < U256::from(reserve0.as_u128())
                && amount1_out < U256::from(reserve1.as_u128())
            {
                if to != get_token0() && to != get_token1() {
                    if amount0_out > 0.into() {
                        // optimistically transfer tokens
                        () = runtime::call_versioned_contract(
                            get_token0().into_hash().unwrap_or_revert().into(),
                            None,
                            "transfer",
                            runtime_args! {
                                "recipient" => Address::from(to),
                                "amount" => amount0_out
                            },
                        );
                    }
                    if amount1_out > 0.into() {
                        // optimistically transfer tokens
                        () = runtime::call_versioned_contract(
                            get_token1().into_hash().unwrap_or_revert().into(),
                            None,
                            "transfer",
                            runtime_args! {
                                "recipient" => Address::from(to),
                                "amount" => amount1_out
                            },
                        );
                    }
                    // FLASH SWAPPER DISABLED : HALBORN ISSUE #19
                    // if !data.is_empty() {
                    //     () = runtime::call_versioned_contract(
                    //         to.into_hash().unwrap_or_revert().into(),
                    //         None,
                    //         "uniswap_v2_call",
                    //         runtime_args! {
                    //             "sender" => get_callee_package_hash(),
                    //             "amount0" => amount0_out,
                    //             "amount1" => amount1_out,
                    //             "data" => data
                    //         },
                    //     );
                    // }
                    let balance0: U256 = runtime::call_versioned_contract(
                        get_token0().into_hash().unwrap_or_revert().into(),
                        None,
                        "balance_of",
                        runtime_args! {
                            "owner" => Address::Contract(get_package_hash())
                        },
                    );
                    let balance1: U256 = runtime::call_versioned_contract(
                        get_token1().into_hash().unwrap_or_revert().into(),
                        None,
                        "balance_of",
                        runtime_args! {
                            "owner" => Address::Contract(get_package_hash())
                        },
                    );
                    let mut amount0_in: U256 = 0.into();
                    let mut amount1_in: U256 = 0.into();
                    if balance0 > U256::from(reserve0.as_u128()) - amount0_out {
                        amount0_in = balance0 - (U256::from(reserve0.as_u128()) - amount0_out)
                    }
                    if balance1 > U256::from(reserve1.as_u128()) - amount1_out {
                        amount1_in = balance1 - (U256::from(reserve1.as_u128()) - amount1_out);
                    }
                    if amount0_in > 0.into() || amount1_in > 0.into() {
                        let amount_1000: U256 = 1000.into();
                        let amount_3: U256 = 3.into();
                        let balance0_adjusted: U256 =
                            (balance0.checked_mul(amount_1000).unwrap_or_revert_with(
                                Errors::UniswapV2CorePairMultiplicationOverFlow1,
                            ))
                            .checked_sub(amount0_in.checked_mul(amount_3).unwrap_or_revert_with(
                                Errors::UniswapV2CorePairMultiplicationOverFlow2,
                            ))
                            .unwrap_or_revert_with(Errors::UniswapV2CorePairUnderFlow3);
                        let balance1_adjusted: U256 =
                            (balance1.checked_mul(amount_1000).unwrap_or_revert_with(
                                Errors::UniswapV2CorePairMultiplicationOverFlow3,
                            ))
                            .checked_sub(amount1_in.checked_mul(amount_3).unwrap_or_revert_with(
                                Errors::UniswapV2CorePairMultiplicationOverFlow4,
                            ))
                            .unwrap_or_revert_with(Errors::UniswapV2CorePairUnderFlow4);
                        let reserve0_conversion: U256 = U256::from(reserve0.as_u128());
                        let reserve1_conversion: U256 = U256::from(reserve1.as_u128());
                        let base: i32 = 1000;
                        let reserve_multiply: U256 = (base.pow(2)).into();
                        if (balance0_adjusted
                            .checked_mul(balance1_adjusted)
                            .unwrap_or_revert_with(
                                Errors::UniswapV2CorePairMultiplicationOverFlow5,
                            ))
                            >= (reserve0_conversion
                                .checked_mul(reserve1_conversion)
                                .unwrap_or_revert_with(
                                    Errors::UniswapV2CorePairMultiplicationOverFlow6,
                                )
                                .checked_mul(reserve_multiply)
                                .unwrap_or_revert_with(
                                    Errors::UniswapV2CorePairMultiplicationOverFlow7,
                                ))
                        {
                            self.update(balance0, balance1, reserve0, reserve1);
                            self.emit(&PAIREvent::Swap {
                                sender: self.get_caller(),
                                amount0_in,
                                amount1_in,
                                amount0_out,
                                amount1_out,
                                to,
                                from: self.get_caller(),
                                pair: Key::from(get_package_hash()),
                            });
                        } else {
                            //UniswapV2: K
                            runtime::revert(Errors::UniswapV2CorePairInsufficientConvertedBalance);
                        }
                    } else {
                        //UniswapV2: INSUFFICIENT_INPUT_AMOUNT
                        runtime::revert(Errors::UniswapV2CorePairInsufficientInputAmount);
                    }
                } else {
                    //UniswapV2: INVALID_TO
                    runtime::revert(Errors::UniswapV2CorePairInvalidTo);
                }
            } else {
                //UniswapV2: INSUFFICIENT_LIQUIDITY
                runtime::revert(Errors::UniswapV2CorePairInsufficientLiquidity);
            }
        } else {
            //UniswapV2: INSUFFICIENT_OUTPUT_AMOUNT
            runtime::revert(Errors::UniswapV2CorePairInsufficientOutputAmount);
        }
        set_lock(0);
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

    fn ecrecover(
        &self,
        public_key: String,
        signature: String,
        digest: [u8; 32],
        owner: Key,
    ) -> bool {
        let public_key_without_spaces: String = public_key.split_whitespace().collect();
        let public_key_string: Vec<&str> = public_key_without_spaces.split(',').collect();
        let mut public_key_vec: Vec<u8> = Vec::new();
        let mut public_counter: usize = 0;
        while public_counter < 32 {
            public_key_vec.push(public_key_string[public_counter].parse::<u8>().unwrap());
            public_counter = public_counter
                .checked_add(1)
                .unwrap_or_revert_with(Errors::UniswapV2CorePairOverFlow1);
        }
        let signature_without_spaces: String = signature.split_whitespace().collect();
        let signature_string: Vec<&str> = signature_without_spaces.split(',').collect();
        let mut signature_vec: Vec<u8> = Vec::new();
        let mut signature_counter: usize = 0;
        while signature_counter < 64 {
            signature_vec.push(signature_string[signature_counter].parse::<u8>().unwrap());
            signature_counter = signature_counter
                .checked_add(1)
                .unwrap_or_revert_with(Errors::UniswapV2CorePairOverFlow2);
        }
        let result: bool = ed25519::verify(&digest, &public_key_vec, &signature_vec);
        let verify_key: String = format!("{}{}", "VERIFY", owner);
        set_key(&verify_key, result);
        result
    }

    fn set_treasury_fee_percent(&self, treasury_fee: U256) {
        if treasury_fee < 30.into() && treasury_fee > 3.into() {
            set_treasury_fee(treasury_fee);
        } else if treasury_fee >= 30.into() {
            set_treasury_fee(30.into());
        } else {
            set_treasury_fee(3.into());
        }
    }

    #[allow(unused_assignments)]
    fn mint(&self, to: Key) -> U256 {
        let (reserve0, reserve1, _block_timestamp_last) = self.get_reserves(); // gas savings
        let balance0: U256 = runtime::call_versioned_contract(
            get_token0().into_hash().unwrap_or_revert().into(),
            None,
            "balance_of",
            runtime_args! {
                "owner" => Address::Contract(get_package_hash())
            },
        );
        let balance1: U256 = runtime::call_versioned_contract(
            get_token1().into_hash().unwrap_or_revert().into(),
            None,
            "balance_of",
            runtime_args! {
                "owner" => Address::Contract(get_package_hash())
            },
        );
        let amount0: U256 = balance0
            .checked_sub(U256::from(reserve0.as_u128()))
            .unwrap_or_revert_with(Errors::UniswapV2CorePairUnderFlow5);
        let amount1: U256 = balance1
            .checked_sub(U256::from(reserve1.as_u128()))
            .unwrap_or_revert_with(Errors::UniswapV2CorePairUnderFlow6);
        let fee_on: bool = self.mint_fee(reserve0, reserve1);
        let mut liquidity: U256 = 0.into();
        if self.total_supply() == 0.into() {
            liquidity = self
                .sqrt(
                    amount0
                        .checked_mul(amount1)
                        .unwrap_or_revert_with(Errors::UniswapV2CorePairMultiplicationOverFlow8),
                )
                .checked_sub(get_minimum_liquidity())
                .unwrap_or_revert_with(Errors::UniswapV2CorePairUnderFlow7);
            ERC20::mint(
                self,
                Address::from(account_zero_address()),
                get_minimum_liquidity(),
            )
            .unwrap_or_revert();
        } else {
            let x: U256 = (amount0
                .checked_mul(self.total_supply())
                .unwrap_or_revert_with(Errors::UniswapV2CorePairMultiplicationOverFlow9))
                / U256::from(reserve0.as_u128());
            let y: U256 = (amount1
                .checked_mul(self.total_supply())
                .unwrap_or_revert_with(Errors::UniswapV2CorePairMultiplicationOverFlow10))
                / U256::from(reserve1.as_u128());
            liquidity = self.min(x, y);
        }
        if liquidity > 0.into() {
            ERC20::mint(self, Address::from(to), liquidity).unwrap_or_revert();
            self.update(balance0, balance1, reserve0, reserve1);
            if fee_on {
                let k_last: U256 = U256::from(
                    (reserve0
                        .checked_mul(reserve1)
                        .unwrap_or_revert_with(Errors::UniswapV2CorePairMultiplicationOverFlow11))
                    .as_u128(),
                ); // reserve0 and reserve1 are up-to-date
                set_k_last(k_last);
            }
            set_liquidity(liquidity); // return liquidity
            self.emit(&PAIREvent::Mint {
                sender: self.get_caller(),
                amount0,
                amount1,
                pair: Key::from(get_package_hash()),
            });
            liquidity // return liquidity
        } else {
            //UniswapV2: INSUFFICIENT_LIQUIDITY_MINTED
            runtime::revert(Errors::UniswapV2CorePairInsufficientLiquidityMinted);
        }
    }

    fn burn(&self, to: Key) -> (U256, U256) {
        let (reserve0, reserve1, _block_timestamp_last) = self.get_reserves(); // gas savings
        let balance0: U256 = runtime::call_versioned_contract(
            get_token0().into_hash().unwrap_or_revert().into(),
            None,
            "balance_of",
            runtime_args! {
                "owner" => Address::Contract(get_package_hash())
            },
        );
        let balance1: U256 = runtime::call_versioned_contract(
            get_token1().into_hash().unwrap_or_revert().into(),
            None,
            "balance_of",
            runtime_args! {
                "owner" => Address::Contract(get_package_hash())
            },
        );
        let liquidity: U256 = self.balance_of(Address::Contract(get_package_hash()));
        let fee_on: bool = self.mint_fee(reserve0, reserve1);
        let amount0: U256 = (liquidity
            .checked_mul(balance0)
            .unwrap_or_revert_with(Errors::UniswapV2CorePairMultiplicationOverFlow12))
            / self.total_supply();
        let amount1: U256 = (liquidity
            .checked_mul(balance1)
            .unwrap_or_revert_with(Errors::UniswapV2CorePairMultiplicationOverFlow13))
            / self.total_supply();
        if amount0 > 0.into() && amount1 > 0.into() {
            ERC20::burn(self, Address::Contract(get_package_hash()), liquidity).unwrap_or_revert();
            () = runtime::call_versioned_contract(
                get_token0().into_hash().unwrap_or_revert().into(),
                None,
                "transfer",
                runtime_args! {
                    "recipient" => Address::from(to),
                    "amount" => amount0
                },
            );
            () = runtime::call_versioned_contract(
                get_token1().into_hash().unwrap_or_revert().into(),
                None,
                "transfer",
                runtime_args! {
                    "recipient" => Address::from(to),
                    "amount" => amount1
                },
            );
            let balance0: U256 = runtime::call_versioned_contract(
                get_token0().into_hash().unwrap_or_revert().into(),
                None,
                "balance_of",
                runtime_args! {
                    "owner" => Address::Contract(get_package_hash())
                },
            );
            let balance1: U256 = runtime::call_versioned_contract(
                get_token1().into_hash().unwrap_or_revert().into(),
                None,
                "balance_of",
                runtime_args! {
                    "owner" => Address::Contract(get_package_hash())
                },
            );
            self.update(balance0, balance1, reserve0, reserve1);
            if fee_on {
                let k_last: U256 = U256::from(
                    (reserve0
                        .checked_mul(reserve1)
                        .unwrap_or_revert_with(Errors::UniswapV2CorePairMultiplicationOverFlow14))
                    .as_u128(),
                ); // reserve0 and reserve1 are up-to-date
                set_k_last(k_last);
            }
            set_amount0(amount0);
            set_amount1(amount1);
            self.emit(&PAIREvent::Burn {
                sender: self.get_caller(),
                amount0,
                amount1,
                to,
                pair: Key::from(get_package_hash()),
            });
            (amount0, amount1)
        } else {
            //UniswapV2: INSUFFICIENT_LIQUIDITY_BURNED
            runtime::revert(Errors::UniswapV2CorePairInsufficientLiquidityBurned);
        }
    }

    // if fee is on, mint liquidity equivalent to 1/6th of the growth in sqrt(k)
    fn mint_fee(&self, reserve0: U128, reserve1: U128) -> bool {
        let fee_to: Key = runtime::call_versioned_contract(
            get_factory_hash().into_hash().unwrap_or_revert().into(),
            None,
            "fee_to",
            runtime_args! {},
        );
        let mut fee_on: bool = false;
        if fee_to != account_zero_address() {
            fee_on = true;
        }
        let k_last: U256 = get_k_last(); // gas savings
        let treasury_fee: U256 = get_treasury_fee();
        if fee_on {
            if k_last != 0.into() {
                let mul_val: U256 = U256::from(
                    (reserve1
                        .checked_mul(reserve0)
                        .unwrap_or_revert_with(Errors::UniswapV2CorePairMultiplicationOverFlow15))
                    .as_u128(),
                );
                let root_k: U256 = self.sqrt(mul_val);
                let root_k_last: U256 = self.sqrt(k_last);
                if root_k > root_k_last {
                    let subtracted_root_k: U256 = root_k
                        .checked_sub(root_k_last)
                        .unwrap_or_revert_with(Errors::UniswapV2CorePairUnderFlow8);
                    let numerator: U256 = self
                        .total_supply()
                        .checked_mul(subtracted_root_k)
                        .unwrap_or_revert_with(Errors::UniswapV2CorePairMultiplicationOverFlow16);
                    let denominator: U256 = (root_k
                        .checked_mul(treasury_fee)
                        .unwrap_or_revert_with(Errors::UniswapV2CorePairMultiplicationOverFlow17))
                    .checked_add(root_k_last)
                    .unwrap_or_revert_with(Errors::UniswapV2CorePairOverFlow3);
                    if denominator > U256::from(0) {
                        let liquidity: U256 = numerator / denominator;
                        if liquidity > 0.into() {
                            ERC20::mint(self, Address::from(fee_to), liquidity).unwrap_or_revert();
                        }
                    } else {
                        //UniswapV2: DENOMINATOR IS ZERO
                        runtime::revert(Errors::UniswapV2CorePairDenominatorIsZero);
                    }
                }
            }
        } else if k_last != 0.into() {
            set_k_last(0.into());
        }
        fee_on
    }

    fn initialize(&self, token0: Key, token1: Key, factory_hash: Key) {
        if factory_hash == get_factory_hash() {
            set_token0(token0);
            set_token1(token1);
        } else {
            //(UniswapV2: FORBIDDEN)
            runtime::revert(Errors::UniswapV2CorePairForbidden);
        }
    }

    fn get_reserves(&self) -> (U128, U128, u64) {
        (get_reserve0(), get_reserve1(), get_block_timestamp_last())
    }

    fn sqrt(&self, y: U256) -> U256 {
        let mut z: U256 = 0.into();
        if y > 3.into() {
            z = y;
            let mut x: U256 = (y
                .checked_div(U256::from(2))
                .unwrap_or_revert_with(Errors::UniswapV2CorePairDivisionOverFlow1))
            .checked_add(U256::from(1))
            .unwrap_or_revert_with(Errors::UniswapV2CorePairOverFlow4);
            while x < z {
                z = x;
                x = ((y
                    .checked_div(x)
                    .unwrap_or_revert_with(Errors::UniswapV2CorePairDivisionOverFlow2))
                .checked_add(x)
                .unwrap_or_revert_with(Errors::UniswapV2CorePairOverFlow5))
                .checked_div(U256::from(2))
                .unwrap_or_revert_with(Errors::UniswapV2CorePairDivisionOverFlow3);
            }
        } else if y != 0.into() {
            z = 1.into();
        }
        z
    }

    fn min(&self, x: U256, y: U256) -> U256 {
        if x < y {
            x
        } else {
            y
        }
    }

    /// encode a U128 as a U256
    fn encode(&self, y: U128) -> U256 {
        let q128: U256 = (2 ^ 128).into();
        let y_u256: U256 = U256::from(y.as_u128());
        let z: U256 = y_u256 * q128; // never overflows
        z
    }

    /// divide a U256 by a U128, returning a U256
    fn uqdiv(&self, x: U256, y: U128) -> U256 {
        let y_u256: U256 = U256::from(y.as_u128());
        let z: U256 = x / y_u256;
        z
    }

    /// encode_uqdiv
    fn encode_uqdiv(
        &self,
        encode_reserve: U128,
        uqdiv_reserve: U128,
        mut general_price_cumulative_last: U256,
        time_elapsed: u64,
    ) -> U256 {
        let encode_result: U256 = self.encode(encode_reserve);
        let uqdive_result: U256 = self.uqdiv(encode_result, uqdiv_reserve);
        general_price_cumulative_last += uqdive_result * time_elapsed;
        general_price_cumulative_last
    }

    fn update(&self, balance0: U256, balance1: U256, reserve0: U128, reserve1: U128) {
        let one: U128 = 1.into();
        let overflow_check: U256 = U256::from(
            ((U128::MAX)
                .checked_sub(one)
                .unwrap_or_revert_with(Errors::UniswapV2CorePairUnderFlow9))
            .as_u128(),
        );
        if balance0 <= overflow_check && balance1 <= overflow_check {
            let block_timestamp: u64 = runtime::get_blocktime().into();
            let block_timestamp_last: u64 = get_block_timestamp_last();
            let time_elapsed: u64 = block_timestamp - block_timestamp_last; // overflow is desired
            if time_elapsed > 0 && reserve0 != 0.into() && reserve1 != 0.into() {
                let price0_cumulative_last: U256 = get_price0_cumulative_last();
                let price1_cumulative_last: U256 = get_price1_cumulative_last();
                let price0_cumulative_last_result: U256 =
                    self.encode_uqdiv(reserve1, reserve0, price0_cumulative_last, time_elapsed);
                set_price0_cumulative_last(price0_cumulative_last_result);
                let price1_cumulative_last_result: U256 =
                    self.encode_uqdiv(reserve0, reserve1, price1_cumulative_last, time_elapsed);
                set_price1_cumulative_last(price1_cumulative_last_result);
            }
            let reserve0_conversion: U128 = U128::from(balance0.as_u128());
            let reserve1_conversion: U128 = U128::from(balance1.as_u128());
            set_reserve0(reserve0_conversion);
            set_reserve1(reserve1_conversion);
            set_block_timestamp_last(block_timestamp);
            self.emit(&PAIREvent::Sync {
                reserve0: reserve0_conversion,
                reserve1: reserve1_conversion,
                pair: Key::from(get_package_hash()),
            });
        } else {
            //UniswapV2: OVERFLOW
            runtime::revert(Errors::UniswapV2CorePairOverFlow6);
        }
    }

    fn emit(&self, pair_event: &PAIREvent) {
        let mut events = Vec::new();
        let formatted_package_hash = get_package_hash().to_formatted_string();
        let package_hash_arr: Vec<&str> = formatted_package_hash.split('-').collect();
        let package_hash: String = package_hash_arr[1].to_string();
        match pair_event {
            PAIREvent::Mint {
                sender,
                amount0,
                amount1,
                pair,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert("event_type", pair_event.type_name());
                event.insert("sender", sender.to_string());
                event.insert("amount0", amount0.to_string());
                event.insert("amount1", amount1.to_string());
                event.insert("pair", pair.to_string());
                events.push(event);
            }
            PAIREvent::Burn {
                sender,
                amount0,
                amount1,
                to,
                pair,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert("event_type", pair_event.type_name());
                event.insert("sender", sender.to_string());
                event.insert("amount0", amount0.to_string());
                event.insert("amount1", amount1.to_string());
                event.insert("to", to.to_string());
                event.insert("pair", pair.to_string());
                events.push(event);
            }
            PAIREvent::Swap {
                sender,
                amount0_in,
                amount1_in,
                amount0_out,
                amount1_out,
                to,
                from,
                pair,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert("event_type", pair_event.type_name());
                event.insert("sender", sender.to_string());
                event.insert("amount0In", amount0_in.to_string());
                event.insert("amount1In", amount1_in.to_string());
                event.insert("amount0Out", amount0_out.to_string());
                event.insert("amount1Out", amount1_out.to_string());
                event.insert("to", to.to_string());
                event.insert("from", from.to_string());
                event.insert("pair", pair.to_string());
                events.push(event);
            }
            PAIREvent::Sync {
                reserve0,
                reserve1,
                pair,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert("event_type", pair_event.type_name());
                event.insert("reserve0", reserve0.to_string());
                event.insert("reserve1", reserve1.to_string());
                event.insert("pair", pair.to_string());
                events.push(event);
            }
        };
        for event in events {
            storage::new_uref(event);
        }
    }
}
