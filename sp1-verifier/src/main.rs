use std::env;
use std::fs;

use base64::prelude::*;

use sp1_sdk::{ProverClient, SP1Proof};

use hyle_contract::HyleOutput;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        3 | 5 => {}
        _ => {
            eprintln!("Usage: {} <image_id> <receipt_path> <initial_state> <next_state>", args[0]);
            eprintln!("Usage for stateless verifiation: {} <image_id> <receipt_path>", args[0]);
            std::process::exit(1);
        }
    }
    
    let elf: Vec<u8> = BASE64_STANDARD.decode(fs::read(&args[1]).expect("loading elf failed")).expect("ELF decoding failed");

    let mut proof = SP1Proof::load(&args[2]).expect("loading proof failed");

    if args.len() == 5 {
        // Stored as raw base64 values.
        let initial_state = BASE64_STANDARD.decode(&args[3]).expect("Invalid initial state string");
        let next_state = BASE64_STANDARD.decode(&args[4]).expect("Invalid next state string");

        let output: HyleOutput<()> = proof.public_values.read::<HyleOutput<()>>();

        if output.initial_state != initial_state.to_vec() {
            panic!("Initial state mismatch");
        }

        if output.next_state != next_state.to_vec() {
            panic!("Next state mismatch");
        }
    }

    // TODO: check that elf is correct ?
    let prover_client = ProverClient::new();
    let (_pk, vk) = prover_client.setup(&elf);
    
    prover_client.verify(&proof, &vk).expect("verification failed");
}
