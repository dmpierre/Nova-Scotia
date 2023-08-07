use std::{collections::HashMap, io::Write, env::current_dir, time::{Instant, Duration}};
use ff::PrimeField;
use nova_scotia::{
    circom::reader::load_r1cs, create_public_params, create_recursive_circuit, FileLocation, F1,
    G2,
    utils::println_constraints_summary,
};
use nova_snark::traits::Group;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
struct EffSig {
    start_pub_input: [String; 7],
    signatures: Vec<[String; 7]>,
}

fn bench(iteration_count: usize, per_iteration_count: usize) -> (Duration, Duration) {

    let root = current_dir().unwrap();
    let circuit_file = root.join("examples/ecdsa-sigs/ecdsa_sigs_test.r1cs");
    let r1cs = load_r1cs(&FileLocation::PathBuf(circuit_file));
    let witness_generator_wasm = root.join("examples/ecdsa-sigs/ecdsa_sigs_test_js/ecdsa_sigs_test.wasm");

    let sigs: EffSig = serde_json::from_str(include_str!(
        "ecdsa-sigs/sigs/out/sig_ecdsa_batch_sample.json"
    ))
    .unwrap();

    let start_public_input = vec![
        F1::from_str_vartime(&sigs.start_pub_input[0]).unwrap(),
        F1::from_str_vartime(&sigs.start_pub_input[1]).unwrap(),
        F1::from_str_vartime(&sigs.start_pub_input[2]).unwrap(),
        F1::from_str_vartime(&sigs.start_pub_input[3]).unwrap(),
        F1::from_str_vartime(&sigs.start_pub_input[4]).unwrap(),
        F1::from_str_vartime(&sigs.start_pub_input[5]).unwrap(),
        F1::from_str_vartime(&sigs.start_pub_input[6]).unwrap(),
    ];

    let mut private_inputs = Vec::new();

    for i in 0..iteration_count {
        let mut private_input = HashMap::new();
        private_input.insert(
            "signatures".to_string(),
            json!(
                sigs.signatures
                    [i * per_iteration_count..i * per_iteration_count + per_iteration_count]
            ),
        );
        private_inputs.push(private_input);
    }

    let pp = create_public_params(r1cs.clone());

    println_constraints_summary(&pp);

    println!("Creating a RecursiveSNARK...");
    let start = Instant::now();
    let recursive_snark = create_recursive_circuit(
        FileLocation::PathBuf(witness_generator_wasm),
        r1cs,
        private_inputs,
        start_public_input.clone(),
        &pp,
    )
    .unwrap();
    let prover_time = start.elapsed();
    println!("RecursiveSNARK creation took {:?}", start.elapsed());

    let z0_secondary = vec![<G2 as Group>::Scalar::zero()];

    // verify the recursive SNARK
    println!("Verifying a RecursiveSNARK...");
    let start = Instant::now();
    let res = recursive_snark.verify(
        &pp,
        iteration_count,
        start_public_input.clone(),
        z0_secondary.clone(),
    );
    println!(
        "RecursiveSNARK::verify: {:?}, took {:?}",
        res,
        start.elapsed()
    );
    let verifier_time = start.elapsed();
    assert!(res.is_ok());
    (prover_time, verifier_time)
}

fn main() {
    println!("Starting benchmark...");
    let mut file = std::fs::File::create("examples/ecdsa-sigs/benchmark.csv").unwrap();
    file.write_all(b"iteration_count,per_iteration_count,prover_time,verifier_time\n")
        .unwrap();

    for i in vec![10, 20, 50, 100] {
        let j = 300 / i;
        // run bash script
        std::process::Command::new("bash")
            .arg("examples/ecdsa-sigs/compile.sh")
            .arg(i.to_string())
            .output()
            .expect("failed to execute process");

        let (prover_time, verifier_time) = bench(j, i);
        file.write_all(format!("{},{},{:?},{:?}\n", j, i, prover_time, verifier_time).as_bytes())
            .unwrap();
    }
}
