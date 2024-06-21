use cairo_platinum_prover::air::{generate_cairo_proof, verify_cairo_proof, PublicInputs};
use hyle_contract::HyleOutput;
use stark_platinum_prover::proof::options::ProofOptions;
use serde::{Deserialize, Serialize};
use std::fs;
use cairo_platinum_prover::runner::run::generate_prover_args_from_trace;
use lambdaworks_math::field::fields::fft_friendly::stark_252_prime_field::Stark252PrimeField;
use stark_platinum_prover::proof::stark::StarkProof;
use std::time::Instant;
use wasm_bindgen::prelude::*;
use error::VerifierError;

pub mod error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    from: String,
    to: String,
    amount: u64,
}

pub fn verify_proof(proof_path: &String, proof_options: ProofOptions) -> Result<String, VerifierError>{
    let Ok(program_content) = std::fs::read(proof_path) else {
        return Err(VerifierError(format!("Error opening {} file", proof_path)));
    };
    let mut bytes = program_content.as_slice();
    if bytes.len() < 8 {
        return Err(VerifierError(format!("Error reading proof from file: {}", proof_path)));
    }

    // Proof len was stored as an u32, 4u8 needs to be read
    let proof_len = u32::from_le_bytes(bytes[0..4].try_into().unwrap()) as usize;

    bytes = &bytes[4..];
    if bytes.len() < proof_len {
        return Err(VerifierError(format!("Error reading proof from file: {}", proof_path)));
    }
    
    let Ok((proof, _)) = bincode::serde::decode_from_slice(
        &bytes[0..proof_len],
        bincode::config::standard(),
    ) else {
        return Err(VerifierError(format!("Error reading proof from file: {}", proof_path)));
    };
    
    // PublicInputs len was stored as an u32, 4u8 needs to be read
    let pub_inputs_len = u32::from_le_bytes(bytes[proof_len..proof_len+4].try_into().unwrap()) as usize;
    let pub_inputs_bytes = &bytes[proof_len+4..proof_len+4+pub_inputs_len];
    
    let Ok((pub_inputs, _)) =
    bincode::serde::decode_from_slice(pub_inputs_bytes, bincode::config::standard())
    else {
        return Err(VerifierError(format!("Error reading proof from file: {}", proof_path)));
    };
    let program_output_bytes = &bytes[proof_len+4+pub_inputs_len..];

    let Ok((program_output, _)) =
        bincode::serde::decode_from_slice::<HyleOutput<Event>, _>(program_output_bytes, bincode::config::standard())
    else {
        return Err(VerifierError(format!("Error reading proof from file: {}", proof_path)));
    };

    if verify_cairo_proof(&proof, &pub_inputs, &proof_options){
        return Ok(serde_json::to_string(&program_output)?);
    }
    else {
        return Err(VerifierError(format!("Error reading proof from file: {}", proof_path)));
    }
}

#[wasm_bindgen]
pub fn prove(trace_bin_path: &str, memory_bin_path: &str, output_path: &str, proof_path: &str, proof_options: &ProofOptions) -> Result<String, VerifierError> {
    let Some((proof, pub_inputs)) = generate_proof_from_trace(
        trace_bin_path,
        memory_bin_path,
        proof_options,
    ) else {
        return Err(VerifierError("Error generation prover args".to_string()));
    };

    write_proof(proof, pub_inputs, output_path, proof_path);
    Ok("Trace and memory successfully proved".to_string())
}

fn generate_proof_from_trace(
    trace_bin_path: &str,
    memory_bin_path: &str,
    proof_options: &ProofOptions,
) -> Option<(
    StarkProof<Stark252PrimeField, Stark252PrimeField>,
    PublicInputs,
)> {
    // ## Generating the prover args
    let timer = Instant::now();
    let Ok((main_trace, pub_inputs)) =
        generate_prover_args_from_trace(trace_bin_path, memory_bin_path)
    else {
        eprintln!("Eroutput_pathror generating prover args");
        return None;
    };
    println!("  Time spent: {:?} \n", timer.elapsed());

    // ## Prove
    let timer = Instant::now();
    println!("Making proof ...");
    let proof = match generate_cairo_proof(&main_trace, &pub_inputs, proof_options) {
        Ok(p) => p,
        Err(err) => {
            eprintln!("Error generating proof: {:?}", err);
            return None;
        }
    };
    println!("  Time spent in proving: {:?} \n", timer.elapsed());

    Some((proof, pub_inputs))
}

fn write_proof(
    proof: StarkProof<Stark252PrimeField, Stark252PrimeField>,
    pub_inputs: PublicInputs,
    output_path: &str,
    proof_path: &str,
) {
    let mut bytes = vec![];
    let proof_bytes: Vec<u8> =
        bincode::serde::encode_to_vec(proof, bincode::config::standard()).unwrap();

    let pub_inputs_bytes: Vec<u8> =
        bincode::serde::encode_to_vec(&pub_inputs, bincode::config::standard()).unwrap();    

    // This should be reworked
    // Public inputs shouldn't be stored in the proof if the verifier wants to check them

    // An u32 is enough for storing proofs up to 32 GiB
    // They shouldn't exceed the order of kbs
    // Reading an usize leads to problem in WASM (32 bit vs 64 bit architecture)

    bytes.extend((proof_bytes.len() as u32).to_le_bytes());
    bytes.extend(proof_bytes);
    bytes.extend((pub_inputs_bytes.len() as u32).to_le_bytes());
    bytes.extend(pub_inputs_bytes);

    ///// HYLE CUSTOM /////
    // Basically adding the program output to the proof
    let program_output_str: String = fs::read_to_string(&output_path).expect("Failed to read output file");
    let program_output: HyleOutput::<Event> = serde_json::from_str(&program_output_str).expect("JSON output was not well-formatted");
    let program_output_bytes: Vec<u8> =
        bincode::serde::encode_to_vec(&program_output, bincode::config::standard()).unwrap();
    
    bytes.extend(program_output_bytes);
    ///////////////////////

    let Ok(()) = std::fs::write(&proof_path, bytes) else {
        eprintln!("Error writing proof to file: {}", &proof_path);
        return;
    };


    println!("Proof written to {}", proof_path);
}
