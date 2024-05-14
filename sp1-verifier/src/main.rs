use std::env;
use std::fs;

use base64::prelude::*;

use sp1_sdk::{ProverClient, SP1Proof};

use hyle_contract::HyleOutput;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 5 {
        eprintln!("Usage: {} <elf_path> <proof_path> <initial_state> <next_state>", args[0]);
        std::process::exit(1);
    }
    
    let elf: Vec<u8> = BASE64_STANDARD.decode(fs::read(&args[1]).expect("loading elf failed")).expect("ELF decoding failed");
    
    let initial_state = BASE64_STANDARD.decode(&args[3]).expect("Invalid initial state string");
    let next_state = BASE64_STANDARD.decode(&args[4]).expect("Invalid next state string");

    let mut proof = SP1Proof::load(&args[2]).expect("loading proof failed");

    perform_verification(&elf, &mut proof, &initial_state, &next_state);
}

fn perform_verification(
    elf: &[u8],
    proof: &mut SP1Proof,
    initial_state: &[u8],
    next_state: &[u8],
) {
    let first_var_committed = &proof.public_values.read::<Vec<u8>>();
    let second_var_committed = &proof.public_values.read::<Vec<u8>>();


    let output: HyleOutput = HyleOutput{
        initial_state: first_var_committed.clone(),
        next_state: second_var_committed.clone()
    };

    if output.initial_state != initial_state.to_vec() {
        panic!("Initial state mismatch");
    }

    if output.next_state != next_state.to_vec() {
        panic!("Next state mismatch");
    }

    // TODO: check that elf is correct ?
    let prover_client = ProverClient::new();
    let (_pk, vk) = prover_client.setup(elf);
    
    prover_client.verify(&proof, &vk).expect("verification failed");

    println!("output: {:?} > {:?}", output.initial_state, output.next_state);
}