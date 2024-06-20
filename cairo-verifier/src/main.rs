use clap::Parser;
use stark_platinum_prover::proof::options::{ProofOptions, SecurityLevel};
use cairo_verifier;

mod commands;

#[derive(Parser)]
struct Cli {
    proof_path: String,
}

fn main(){
    let proof_options = ProofOptions::new_secure(SecurityLevel::Conjecturable100Bits, 3);
    let args: commands::ProverArgs = commands::ProverArgs::parse();

    match args.entity {
        commands::ProverEntity::Verify(args) => {   
            cairo_verifier::verify_proof(&args.proof_path, proof_options);
        },
        commands::ProverEntity::Prove(args) => {
            cairo_verifier::prove(
                &args.trace_bin_path,
                &args.memory_bin_path,
                &args.output_path,
                &args.proof_path,
                &proof_options
            );
        }
    }
}
