#![no_std]
#[macro_use]
extern crate alloc;

pub mod data;
mod factory;

pub use contract_utils;
pub use factory::FACTORY;

use alloc::{collections::BTreeMap, string::String};
use casper_types::U256;
