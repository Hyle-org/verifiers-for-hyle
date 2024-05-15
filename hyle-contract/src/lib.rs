#![no_std]

extern crate alloc;

use serde::{Deserialize, Serialize};
use alloc::vec::Vec;

#[derive(Serialize, Deserialize, Debug)]
pub struct HyleInput<T> {
    pub block_number: u32,
    pub block_time: u32,
    pub caller: Vec<u8>,
    pub tx_hash: Vec<u8>,
    pub initial_state: u32,
    pub program_inputs: Option<T>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HyleOutput<T> {
    pub block_number: u32,
    pub block_time: u32,
    pub caller: Vec<u8>,
    pub tx_hash: Vec<u8>,
    pub initial_state: Vec<u8>,
    pub next_state: Vec<u8>,
    pub program_outputs: Option<T>
}
