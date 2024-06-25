use cairo_platinum_prover::{air::{generate_cairo_proof, verify_cairo_proof, PublicInputs}, cairo_mem::CairoMemory, execution_trace::build_main_trace, register_states::RegisterStates};
use hyle_contract::HyleOutput;
use stark_platinum_prover::proof::options::{ProofOptions, SecurityLevel};
use serde::{Deserialize, Serialize};
use lambdaworks_math::field::fields::fft_friendly::stark_252_prime_field::Stark252PrimeField;
use stark_platinum_prover::proof::stark::StarkProof;
use error::VerifierError;

pub mod error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    from: String,
    to: String,
    amount: u64,
}

pub fn verify_proof(proof_path: &String) -> Result<String, VerifierError>{
    let proof_options = ProofOptions::new_secure(SecurityLevel::Conjecturable100Bits, 3);
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


pub fn prove(trace_data: Vec<u8>, memory_data: Vec<u8>, output: &str) -> Result<Vec<u8>, VerifierError> {
    let proof_options = ProofOptions::new_secure(SecurityLevel::Conjecturable100Bits, 3);
    let Some((proof, pub_inputs)) = generate_proof_from_trace(
        &trace_data,
        &memory_data,
        &proof_options,
    ) else {
        return Err(VerifierError("Error generation prover args".to_string()));
    };
    let proof = write_proof(proof, pub_inputs, output);
    Ok(proof)
}

pub fn generate_proof_from_trace(
    trace_data: &Vec<u8>,
    memory_data: &Vec<u8>,
    proof_options: &ProofOptions,
) -> Option<(
    StarkProof<Stark252PrimeField, Stark252PrimeField>,
    PublicInputs,
)> {
    // ## Generating the prover args
    let register_states = RegisterStates::from_bytes_le(trace_data).expect("Cairo trace data incorrect");
    let memory = CairoMemory::from_bytes_le(memory_data).expect("Cairo memory data incorrect");

    // data length
    let data_len = 0_usize;
    let mut pub_inputs = PublicInputs::from_regs_and_mem(&register_states, &memory, data_len);


    let main_trace = build_main_trace(&register_states, &memory, &mut pub_inputs);


    // ## Prove
    let proof = match generate_cairo_proof(&main_trace, &pub_inputs, proof_options) {
        Ok(p) => p,
        Err(err) => {
            eprintln!("Error generating proof: {:?}", err);
            return None;
        }
    };

    Some((proof, pub_inputs))
}

fn write_proof(
    proof: StarkProof<Stark252PrimeField, Stark252PrimeField>,
    pub_inputs: PublicInputs,
    output: &str,
) -> Vec<u8> {
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
    let program_output: HyleOutput::<Event> = serde_json::from_str(&output).expect("JSON output was not well-formatted");
    let program_output_bytes: Vec<u8> =
        bincode::serde::encode_to_vec(&program_output, bincode::config::standard()).unwrap();
    bytes.extend(program_output_bytes);
    ///////////////////////
    bytes
}
