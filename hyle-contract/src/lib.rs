#![no_std]

extern crate alloc;

use serde::{Deserialize, Serialize};
use alloc::{vec::Vec, string::String};

// This is intended as the "typical" input a Hyle smart contract would receive.
// Note that there is actually no requirement to match this structure.
#[derive(Serialize, Deserialize, Debug)]
pub struct HyleInput<T> {
    pub initial_state: Vec<u8>,
    pub origin: String,
    pub caller: String,
    pub block_number: u64,
    pub block_time: u64,
    pub tx_hash: Vec<u8>,
    pub program_inputs: T
}

// This struct should be used as the output (public witness) of a Hyle smart contract.
// The protocol enforces constraints on the non-generic fields.
// See the documentation for details.
#[derive(Serialize, Deserialize, Debug)]
pub struct HyleOutput<T> {
    pub version: u32,
    pub initial_state: Vec<u8>,
    pub next_state: Vec<u8>,
    pub origin: String,
    pub caller: String,
    pub block_number: u64,
    pub block_time: u64,
    pub tx_hash: Vec<u8>,
    pub program_outputs: T
}
