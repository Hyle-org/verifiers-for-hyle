#![no_std]
extern crate alloc;

use serde::{Deserialize, Serialize};
use alloc::vec::Vec;

#[derive(Serialize, Deserialize, Debug)]

pub struct HyleOutput {
    pub initial_state: Vec<u8>,
    pub next_state: Vec<u8>,
}
