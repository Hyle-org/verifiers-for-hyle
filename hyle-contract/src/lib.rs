#![no_std]

extern crate alloc;

use serde::{Deserialize, Serialize};
use alloc::{vec::Vec, string::String};

#[derive(Serialize, Deserialize, Debug)]
pub struct HyleInput<T> {
    pub initial_state: Vec<u8>,
    pub sender: String,
    pub caller: String,
    pub block_number: u64,
    pub block_time: u64,
    pub tx_hash: Vec<u8>,
    pub program_inputs: T
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HyleOutput<T> {
    pub version: u32,
    pub initial_state: Vec<u8>,
    pub next_state: Vec<u8>,
    pub sender: String,
    pub caller: String,
    pub block_number: u64,
    pub block_time: u64,
    pub tx_hash: Vec<u8>,
    pub program_outputs: T
}
