use clap::Parser;
use stark_platinum_prover::proof::options::{ProofOptions, SecurityLevel};
use crate::utils::error::VerifierError;

mod commands;
mod utils;

#[derive(Parser)]
struct Cli {
    proof_path: String,
}

fn main() -> Result<(), VerifierError> {
    let proof_options = ProofOptions::new_secure(SecurityLevel::Conjecturable100Bits, 3);
    let args: commands::ProverArgs = commands::ProverArgs::parse();

    let res = match args.entity {
        commands::ProverEntity::Verify(args) => {   
            utils::verify_proof(&args.proof_path, proof_options)
        },
        commands::ProverEntity::Prove(args) => {
            utils::prove(
                &args.trace_bin_path,
                &args.memory_bin_path,
                &args.output_path,
                &args.proof_path,
                &proof_options
            )
        }
    };
    match res {
        Result::Ok(output) => println!("{}", output),
        Result::Err(err) => {
            return Err(err);
        }
    };
    Ok(())
}
