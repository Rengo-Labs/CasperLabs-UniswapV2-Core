#![no_std]

extern crate alloc;

pub mod data;
mod errors;
mod events;
mod pair;

pub use pair::PAIR;
