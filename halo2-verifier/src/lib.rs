use halo2_base::{
    gates::{
        circuit::{builder::BaseCircuitBuilder, BaseCircuitParams, CircuitBuilderStage},
        flex_gate::MultiPhaseThreadBreakPoints,
    },
    halo2_proofs::{
        halo2curves::bn256::{Bn256, Fr, G1Affine},
        plonk::{verify_proof, Circuit, VerifyingKey},
        poly::{
            commitment::{Params, ParamsProver},
            kzg::{
                commitment::{KZGCommitmentScheme, ParamsKZG},
                multiopen::VerifierSHPLONK,
                strategy::SingleStrategy,
            },
        },
        SerdeFormat,
    },
    utils::fs::gen_srs,
    AssignedValue,
};
use serde::de::DeserializeOwned;
use snark_verifier_sdk::{
    halo2::{read_snark, PoseidonTranscript},
    NativeLoader,
};
use std::{
    env::var,
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
    time::Instant,
};

pub struct CircuitScaffold<T, F> {
    f: F,
    private_inputs: T,
}

/// A generic verifier that takes in a function, degree k, lookup bits, and data path.
pub fn verify<T: DeserializeOwned>(
    f: impl FnOnce(&mut BaseCircuitBuilder<Fr>, T, &mut Vec<AssignedValue<Fr>>),
    name: String,
    k: u32,
    lookup_bits: usize,
    data_path: PathBuf,
) {
    let private_inputs: T = serde_json::from_reader(
        File::open(&data_path)
            .unwrap_or_else(|e| panic!("Input file not found at {data_path:?}. {e:?}")),
    )
    .expect("Input file should be a valid JSON file");

    let vk_path = data_path.join(PathBuf::from(format!("{name}.vk")));
    let params = gen_srs(k);
    let precircuit = CircuitScaffold { f, private_inputs };
    let mut circuit =
        precircuit.create_circuit(CircuitBuilderStage::Keygen, None, &params, lookup_bits);
    let vk = custom_read_vk(vk_path, &circuit);
    let snark_path = data_path.join(PathBuf::from(format!("{name}.snark")));
    let snark = read_snark(&snark_path)
        .unwrap_or_else(|e| panic!("Snark not found at {snark_path:?}. {e:?}"));

    let verifier_params = params.verifier_params();
    let strategy = SingleStrategy::new(&params);
    let mut transcript = PoseidonTranscript::<NativeLoader, &[u8]>::new::<0>(&snark.proof[..]);
    let instance = &snark.instances[0][..];
    let start = Instant::now();
    verify_proof::<
        KZGCommitmentScheme<Bn256>,
        VerifierSHPLONK<'_, Bn256>,
        _,
        _,
        SingleStrategy<'_, Bn256>,
    >(
        verifier_params,
        &vk,
        strategy,
        &[&[instance]],
        &mut transcript,
    )
    .unwrap();
    let verification_time = start.elapsed();
    println!("Snark verified successfully in {:?}", verification_time);
    circuit.clear();
}

impl<T, F> CircuitScaffold<T, F>
where
    F: FnOnce(&mut BaseCircuitBuilder<Fr>, T, &mut Vec<AssignedValue<Fr>>),
{
    /// Creates a Halo2 circuit from the given function.
    fn create_circuit(
        self,
        stage: CircuitBuilderStage,
        pinning: Option<(BaseCircuitParams, MultiPhaseThreadBreakPoints)>,
        params: &ParamsKZG<Bn256>,
        lookup_bits: usize,
    ) -> BaseCircuitBuilder<Fr> {
        let mut builder = BaseCircuitBuilder::from_stage(stage);
        if let Some((params, break_points)) = pinning {
            builder.set_params(params);
            builder.set_break_points(break_points);
        } else {
            let k = params.k() as usize;

            // we initiate a "thread builder". This is what keeps track of the execution trace of our program. If not in proving mode, it also keeps track of the ZK constraints.
            builder.set_k(k);
            builder.set_lookup_bits(lookup_bits);
            builder.set_instance_columns(1);
        };

        // builder.main(phase) gets a default "main" thread for the given phase. For most purposes we only need to think about phase 0
        // we need a 64-bit number as input in this case
        // while `some_algorithm_in_zk` was written generically for any field `F`, in practice we use the scalar field of the BN254 curve because that's what the proving system backend uses
        let mut assigned_instances = vec![];
        (self.f)(&mut builder, self.private_inputs, &mut assigned_instances);
        if !assigned_instances.is_empty() {
            assert_eq!(
                builder.assigned_instances.len(),
                1,
                "num_instance_columns != 1"
            );
            builder.assigned_instances[0] = assigned_instances;
        }

        if !stage.witness_gen_only() {
            // now `builder` contains the execution trace, and we are ready to actually create the circuit
            // minimum rows is the number of rows used for blinding factors. This depends on the circuit itself, but we can guess the number and change it if something breaks (default 9 usually works)
            let minimum_rows = var("MINIMUM_ROWS")
                .unwrap_or_else(|_| "20".to_string())
                .parse()
                .unwrap();
            builder.calculate_params(Some(minimum_rows));
        }

        builder
    }
}

fn custom_read_vk<C, P>(fname: P, circuit: &C) -> VerifyingKey<G1Affine>
where
    C: Circuit<Fr>,
    P: AsRef<Path>,
{
    let f = File::open(&fname)
        .unwrap_or_else(|e| panic!("Failed to open file: {:?}: {e:?}", fname.as_ref()));
    let mut bufreader = BufReader::new(f);
    VerifyingKey::read::<_, C>(&mut bufreader, SerdeFormat::RawBytes, circuit.params())
        .expect("Could not read vkey")
}
