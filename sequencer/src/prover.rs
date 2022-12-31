use crate::api;
use crate::conversions::*;
use crate::merkle_tree::ToBitmap;
use crate::state::{Account, State};

use bitmaps::Bitmap;

use ethers::types::H256;

use serde_json::from_reader;
use serde_tuple::*;

use zokrates_abi::{parse_strict, Encode, Inputs};
use zokrates_ast::ir::ProgEnum;
use zokrates_ast::typed::abi::Abi;
use zokrates_field::Bn128Field;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Serialize_tuple, Deserialize_tuple, Debug)]
pub struct CircuitInput {
    pre_root: H256,
    tx: api::Tx,
    pre_account: Account,
    post_root: H256,
    direction_selector: Vec<bool>,
    pre_path: Vec<H256>,
    post_path: Vec<H256>,
}

impl CircuitInput {
    pub fn new(tx: &api::Tx, pre_state: &State, post_state: &State) -> Self {
        let addr = tx.sender.to_h256();
        let mut dir = addr.to_bitmap().to_vec_bool();
        dir.reverse();
        Self {
            pre_root: pre_state.root(),
            tx: tx.clone(),
            pre_account: pre_state.get(&addr),
            post_root: post_state.root(),
            direction_selector: dir,
            pre_path: pre_state.proof(&addr),
            post_path: post_state.proof(&addr),
        }
    }
}

trait ToVecBool {
    fn to_vec_bool(&self) -> Vec<bool>;
}

impl ToVecBool for Bitmap<256> {
    fn to_vec_bool(&self) -> Vec<bool> {
        let mut v: Vec<bool> = vec![];
        (0..256).for_each(|b| {
            v.push(self.get(b));
        });
        v
    }
}

pub struct Prover;

impl Prover {
    pub fn prove(tx: &api::Tx, pre_state: &State, post_state: &State) -> Result<(), String> {
        let path = Path::new("../circuits/out");
        let file = File::open(path)
            .map_err(|why| format!("Could not open {}: {}", path.display(), why))?;

        let mut reader = BufReader::new(file);

        let prog = match ProgEnum::deserialize(&mut reader).unwrap() {
            ProgEnum::Bn128Program(p) => p,
            _ => panic!(),
        };

        let signature = {
            let path = Path::new("../circuits/abi.json");
            let file = File::open(path)
                .map_err(|why| format!("Could not open {}: {}", path.display(), why))?;
            let mut reader = BufReader::new(file);

            let abi: Abi = from_reader(&mut reader).map_err(|why| why.to_string())?;

            abi.signature()
        };

        let inputs = CircuitInput::new(tx, pre_state, post_state);
        println!("\n\n{}\n\n", serde_json::to_string(&inputs).unwrap());
        //return Ok(());

        let arguments = parse_strict::<Bn128Field>(
            serde_json::to_string(&inputs).unwrap().as_str(),
            signature.inputs,
        )
        .map(Inputs::Abi)
        .map_err(|why| why.to_string())
        .map_err(|e| format!("Could not parse argument: {e}"))?;

        let interpreter = zokrates_interpreter::Interpreter::default();

        let _public_inputs = prog.public_inputs();

        let witness = interpreter
            .execute_with_log_stream(prog, &arguments.encode(), &mut std::io::stdout())
            .map_err(|e| format!("Execution failed: {e}"))?;

        use zokrates_abi::Decode;

        let results_json_value: serde_json::Value =
            zokrates_abi::Value::decode(witness.return_values(), *signature.output)
                .into_serde_json();

        println!("\nWitness: \n{results_json_value}\n");

        Ok(())
    }
}
