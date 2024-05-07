use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]

pub struct HyleOutput {
    pub initial_state: Vec<u8>,
    pub next_state: Vec<u8>,
}
