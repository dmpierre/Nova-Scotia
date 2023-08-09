use std::{collections::HashMap, env::current_dir, time::Instant};

use ff::PrimeField;
use nova_scotia::{
    circom::reader::load_r1cs, create_public_params, create_recursive_circuit, FileLocation, F1,
    G2
};
use nova_snark::traits::Group;

use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct Game {
    board: Vec<Vec<Vec<Vec<u8>>>>,
    dense_weights: Vec<Vec<String>>,
    dense_bias: Vec<Vec<String>>,
    dense_1_weights: Vec<Vec<String>>,
    dense_1_bias: Vec<Vec<String>>,
    dense_2_weights: Vec<Vec<String>>,
    dense_2_bias: Vec<Vec<String>>,
    dense_3_weights: Vec<Vec<String>>,
    dense_3_bias: Vec<Vec<String>>,
    dense_4_weights: Vec<Vec<String>>,
    dense_4_bias: Vec<Vec<String>>,
    step_in: Vec<String>,
    turn: Vec<u8>,
    pathElementsCurrentLeafPlayer: Vec<Vec<String>>,
    pathIndicesCurrentLeafPlayer: Vec<Vec<u8>>,
    belowLeafPlayer: Vec<u8>,
    pathElementsBelowLeafPlayer: Vec<Vec<String>>,
    pathIndicesBelowLeafPlayer: Vec<Vec<u8>>,
    updatedRootFromPlayerPlay: Vec<String>,
    pathElementsUpdatedRootFromPlayer: Vec<Vec<String>>,
    agentMoveRowHelper: Vec<u8>,
    playerPlayedIndex: Vec<u8>,
    pathElementsCurrentLeafAgent: Vec<Vec<String>>,
    pathIndicesCurrentLeafAgent: Vec<Vec<u8>>,
    belowLeafAgent: Vec<u8>,
    pathElementsBelowLeafAgent: Vec<Vec<String>>,
    pathIndicesBelowLeafAgent: Vec<Vec<u8>>,
    updatedRootFromAgentPlay: Vec<String>,
    pathElementsUpdatedRootFromAgent: Vec<Vec<String>>,
}

fn main() {
    let root = current_dir().unwrap();
    let circuit_file = root.join("examples/connect4/connect4_nova.r1cs");
    let r1cs = load_r1cs(&FileLocation::PathBuf(circuit_file));
    let witness_generator_file = root.join("examples/connect4/connect4_nova_js/connect4_nova.wasm");
    let game: Game = serde_json::from_str(include_str!("connect4/aggregate.json")).unwrap();
    let start_public_input = vec![F1::from_str_vartime(&game.step_in[0]).unwrap()];
    let mut private_inputs = Vec::new();

    for i in 0..4 {
        let mut private_input = HashMap::new();
        // we start at one since the first was included in the public input
        private_input.insert("board".to_string(), json!(game.board[i]));
        private_input.insert("dense_weights".to_string(), json!(game.dense_weights[i]));
        private_input.insert("dense_bias".to_string(), json!(game.dense_bias[i]));
        private_input.insert(
            "dense_1_weights".to_string(),
            json!(game.dense_1_weights[i]),
        );
        private_input.insert("dense_1_bias".to_string(), json!(game.dense_1_bias[i]));
        private_input.insert(
            "dense_2_weights".to_string(),
            json!(game.dense_2_weights[i]),
        );
        private_input.insert("dense_2_bias".to_string(), json!(game.dense_2_bias[i]));
        private_input.insert(
            "dense_3_weights".to_string(),
            json!(game.dense_3_weights[i]),
        );
        private_input.insert("dense_3_bias".to_string(), json!(game.dense_3_bias[i]));
        private_input.insert(
            "dense_4_weights".to_string(),
            json!(game.dense_4_weights[i]),
        );
        private_input.insert("dense_4_bias".to_string(), json!(game.dense_4_bias[i]));
        private_input.insert("turn".to_string(), json!(game.turn[i]));
        private_input.insert(
            "pathElementsCurrentLeafPlayer".to_string(),
            json!(game.pathElementsCurrentLeafPlayer[i]),
        );
        private_input.insert(
            "pathIndicesCurrentLeafPlayer".to_string(),
            json!(game.pathIndicesCurrentLeafPlayer[i]),
        );
        private_input.insert(
            "belowLeafPlayer".to_string(),
            json!(game.belowLeafPlayer[i]),
        );
        private_input.insert(
            "pathElementsBelowLeafPlayer".to_string(),
            json!(game.pathElementsBelowLeafPlayer[i]),
        );
        private_input.insert(
            "pathIndicesBelowLeafPlayer".to_string(),
            json!(game.pathIndicesBelowLeafPlayer[i]),
        );
        private_input.insert(
            "updatedRootFromPlayerPlay".to_string(),
            json!(game.updatedRootFromPlayerPlay[i]),
        );
        private_input.insert(
            "pathElementsUpdatedRootFromPlayer".to_string(),
            json!(game.pathElementsUpdatedRootFromPlayer[i]),
        );
        private_input.insert(
            "agentMoveRowHelper".to_string(),
            json!(game.agentMoveRowHelper[i]),
        );
        private_input.insert(
            "playerPlayedIndex".to_string(),
            json!(game.playerPlayedIndex[i]),
        );
        private_input.insert(
            "pathElementsCurrentLeafAgent".to_string(),
            json!(game.pathElementsCurrentLeafAgent[i]),
        );
        private_input.insert(
            "pathIndicesCurrentLeafAgent".to_string(),
            json!(game.pathIndicesCurrentLeafAgent[i]),
        );
        private_input.insert("belowLeafAgent".to_string(), json!(game.belowLeafAgent[i]));
        private_input.insert(
            "pathElementsBelowLeafAgent".to_string(),
            json!(game.pathElementsBelowLeafAgent[i]),
        );
        private_input.insert(
            "pathIndicesBelowLeafAgent".to_string(),
            json!(game.pathIndicesBelowLeafAgent[i]),
        );
        private_input.insert(
            "updatedRootFromAgentPlay".to_string(),
            json!(game.updatedRootFromAgentPlay[i]),
        );
        private_input.insert(
            "pathElementsUpdatedRootFromAgent".to_string(),
            json!(game.pathElementsUpdatedRootFromAgent[i]),
        );

        private_inputs.push(private_input);
    }

    let pp = create_public_params(r1cs.clone());

    println!(
        "Number of constraints per step (primary circuit): {}",
        pp.num_constraints().0
    );
    println!(
        "Number of constraints per step (secondary circuit): {}",
        pp.num_constraints().1
    );
    println!(
        "Number of variables per step (primary circuit): {}",
        pp.num_variables().0
    );
    println!(
        "Number of variables per step (secondary circuit): {}",
        pp.num_variables().1
    );
    
    let start = Instant::now();
    let recursive_snark = create_recursive_circuit(
        FileLocation::PathBuf(witness_generator_file),
        r1cs,
        private_inputs,
        start_public_input.clone(),
        &pp,
    )
    .unwrap();
    println!("RecursiveSNARK creation took {:?}", start.elapsed());
    let z0_secondary = vec![<G2 as Group>::Scalar::zero()];

    // verify the recursive SNARK
    println!("Verifying a RecursiveSNARK...");
    let start = Instant::now();
    let res = recursive_snark.verify(
        &pp,
        4,
        start_public_input.clone(),
        z0_secondary.clone(),
    );
    println!(
        "RecursiveSNARK::verify: {:?}, took {:?}",
        res,
        start.elapsed()
    );
    assert!(res.is_ok());
}
