use std::{env, fs::File, io::Read};

use base64::prelude::*;

use sp1_core::utils::BabyBearPoseidon2;
use sp1_prover::SP1ReduceProof;
use sp1_sdk::{InnerSC, SP1Prover, SP1Proof, SP1VerifyingKey};

use hyle_contract::HyleOutput;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <b64_encoded_verification_key> <receipt_path>", args[0]);
        std::process::exit(1);
    }

    let vk_json = String::from_utf8(BASE64_STANDARD.decode(&args[1]).expect("vk decoding failed")).expect("Fail to cast vk to json string");

    let vk: SP1VerifyingKey = SP1VerifyingKey{
        vk: serde_json::from_str(&vk_json).unwrap()
    };
    // let mut proof = SP1Proof::load(&args[2]).expect("loading proof failed");

    // Load the shrinked proof.
    let mut file = File::open(&args[2]).unwrap();
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).unwrap();

    let shrinked_proof: SP1ReduceProof<BabyBearPoseidon2> = bincode::deserialize::<SP1ReduceProof<InnerSC>>(&bytes).unwrap();

    // TODO: check that vk is correct ?
    let prover = SP1Prover::new();
    // prover.verify(&proof, &vk).expect("verification failed");
    prover.verify_shrink(&shrinked_proof, &vk).expect("verification failed");

    let public_values_slice = shrinked_proof.proof.public_values;
    // pourquoi public values c'est un Vec<BabyBear> ... ?

    println!("{:?}", output);

    // Outputs to stdout for the caller to read.
    // let output: HyleOutput<()> = proof.public_values.read::<HyleOutput<()>>();
    // println!("{}", serde_json::to_string(&output).expect("Failed to serialize output"));

}
