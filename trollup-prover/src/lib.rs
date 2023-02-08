pub mod merkle_tree;
mod poseidon_hasher;
pub mod state;

use crate::merkle_tree::ToBitmap;
use crate::state::{Account, State};

use trollup_config::*;
use trollup_l1::trollup;
use trollup_types::{FromBabyJubjubPoint, Point, PublicKey, ToBabyJubjubSignature, ToU256};

use bitmaps::Bitmap;

use ethers_core::types::U256;

use serde::{Deserialize, Serialize};
use serde_json::from_reader;
use serde_tuple::*;

use zokrates_abi::{parse_strict, Encode, Inputs};
use zokrates_ark::Ark;
use zokrates_ast::ir::{self, ProgEnum, Witness};
use zokrates_ast::typed::abi::Abi;
use zokrates_field::Bn128Field;
use zokrates_proof_systems::*;

use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

#[derive(Serialize_tuple, Deserialize_tuple, Debug)]
pub struct CircuitInput {
    pre_root: U256,
    post_root: U256,
    tx: CircuitTx,
    pre_accounts: Vec<Account>,
    direction_selector: Vec<Vec<bool>>,
    pre_path: Vec<Vec<U256>>,
    post_path: Vec<Vec<U256>>,
}

impl CircuitInput {
    pub fn new(tx: CircuitTx, pre_state: &State, post_state: &State) -> Self {
        let sender_addr = PublicKey(tx.sender.clone()).address();
        let to_addr = PublicKey(tx.to.clone()).address();
        Self {
            pre_root: pre_state.root(),
            tx,
            pre_accounts: vec![pre_state.get(&sender_addr), pre_state.get(&to_addr)],
            post_root: post_state.root(),
            direction_selector: vec![
                sender_addr.to_bitmap().to_vec_bool(),
                to_addr.to_bitmap().to_vec_bool(),
            ],
            pre_path: vec![pre_state.proof(&sender_addr), pre_state.proof(&to_addr)],
            post_path: vec![post_state.proof(&sender_addr), post_state.proof(&to_addr)],
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CircuitTx {
    sender: Point,
    to: Point,
    nonce: U256,
    value: U256,
    sig: CircuitTxSignature,
}

trait ToCircuitTx {
    fn to_circuit_tx(&self) -> CircuitTx;
}

impl ToCircuitTx for trollup_api::SignedTx {
    fn to_circuit_tx(&self) -> CircuitTx {
        let sender_pk: PublicKey = self.tx.sender.into();
        let to_pk: PublicKey = self.tx.to.into();
        CircuitTx {
            sender: sender_pk.0,
            to: to_pk.0,
            nonce: self.tx.nonce,
            value: self.tx.value,
            sig: self.clone().into(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct CircuitTxSignature {
    r: Point,
    s: U256,
}

impl From<trollup_api::SignedTx> for CircuitTxSignature {
    fn from(tx: trollup_api::SignedTx) -> CircuitTxSignature {
        let sig = tx.signature.to_babyjubjub_signature();
        CircuitTxSignature {
            r: Point::from_babyjubjub_point(&sig.r_b8),
            s: sig.s.to_u256(),
        }
    }
}

pub struct Prover;

impl Prover {
    pub fn prove(
        config: &Config,
        tx: &trollup_api::SignedTx,
        pre_state: &State,
        post_state: &State,
    ) -> Result<trollup::TxProof, String> {
        let path = Path::new(&config.circuit_path);
        let file = File::open(path)
            .map_err(|why| format!("Could not open {}: {}", path.display(), why))?;

        let mut reader = BufReader::new(file);

        let prog = match ProgEnum::deserialize(&mut reader).unwrap() {
            ProgEnum::Bn128Program(p) => p,
            _ => panic!(),
        };
        let prog = prog.collect();

        let witness = Self::compute_witness(config, prog.clone(), tx, pre_state, post_state)?;

        let pk_path = Path::new(&config.proving_key_path);
        let pk_file = File::open(pk_path)
            .map_err(|why| format!("Could not open {}: {}", pk_path.display(), why))?;

        let mut pk: Vec<u8> = Vec::new();
        let mut pk_reader = BufReader::new(pk_file);
        pk_reader
            .read_to_end(&mut pk)
            .map_err(|why| format!("Could not read {}: {}", pk_path.display(), why))?;

        let proof: Proof<Bn128Field, G16> = Ark::generate_proof(prog, witness, pk);
        let ret = proof.to_trollup_l1_tx();

        /*
        let proof = serde_json::to_string_pretty(&TaggedProof::<Bn128Field, G16>::new(
            proof.proof,
            proof.inputs,
        ))
        .unwrap();
        println!("Proof:\n{proof}");
        */

        Ok(ret)
    }

    fn compute_witness<I: IntoIterator<Item = ir::Statement<Bn128Field>>>(
        config: &Config,
        prog: ir::ProgIterator<Bn128Field, I>,
        tx: &trollup_api::SignedTx,
        pre_state: &State,
        post_state: &State,
    ) -> Result<Witness<Bn128Field>, String> {
        let signature = {
            let path = Path::new(&config.circuit_abi_path);
            let file = File::open(path)
                .map_err(|why| format!("Could not open {}: {}", path.display(), why))?;
            let mut reader = BufReader::new(file);

            let abi: Abi = from_reader(&mut reader).map_err(|why| why.to_string())?;

            abi.signature()
        };

        let inputs = CircuitInput::new(tx.to_circuit_tx(), pre_state, post_state);
        //println!("\n\n{}\n\n", serde_json::to_string(&inputs).unwrap());

        let arguments = parse_strict(
            serde_json::to_string(&inputs).unwrap().as_str(),
            signature.inputs,
        )
        .map(Inputs::Abi)
        .map_err(|why| why.to_string())
        .map_err(|e| format!("Could not parse argument: {e}"))?;

        let interpreter = zokrates_interpreter::Interpreter::default();

        let _public_inputs = prog.public_inputs();

        let encoded = arguments.encode();
        let witness = interpreter
            .execute_with_log_stream(prog, &encoded, &mut std::io::stdout())
            .map_err(|e| format!("Execution failed: {e}"))?;

        // Uncomment to see the witness verification result values
        /*
        use zokrates_abi::Decode;

        let results_json_value: serde_json::Value =
            zokrates_abi::Value::decode(witness.return_values(), *signature.output)
                .into_serde_json();

        println!("\nWitness: \n{results_json_value}\n");
        */

        Ok(witness)
    }
}

trait ToTrollupL1 {
    fn to_trollup_l1_tx(&self) -> trollup::TxProof;
    fn to_trollup_l1_proof(&self) -> trollup::Proof;
    fn to_trollup_l1_input(&self) -> [U256; 17usize];
}

impl ToTrollupL1 for Proof<Bn128Field, G16> {
    fn to_trollup_l1_tx(&self) -> trollup::TxProof {
        trollup::TxProof {
            proof: self.to_trollup_l1_proof(),
            input: self.to_trollup_l1_input().to_vec(),
        }
    }

    fn to_trollup_l1_proof(&self) -> trollup::Proof {
        trollup::Proof {
            a: trollup::G1Point {
                x: U256::from_str_radix(&self.proof.a.0[2..], 16).unwrap(),
                y: U256::from_str_radix(&self.proof.a.1[2..], 16).unwrap(),
            },
            b: match &self.proof.b {
                G2Affine::Fq2(f) => trollup::G2Point {
                    x: [
                        U256::from_str_radix(&f.0 .0[2..], 16).unwrap(),
                        U256::from_str_radix(&f.0 .1[2..], 16).unwrap(),
                    ],
                    y: [
                        U256::from_str_radix(&f.1 .0[2..], 16).unwrap(),
                        U256::from_str_radix(&f.1 .1[2..], 16).unwrap(),
                    ],
                },
                _ => panic!(),
            },
            c: trollup::G1Point {
                x: U256::from_str_radix(&self.proof.c.0[2..], 16).unwrap(),
                y: U256::from_str_radix(&self.proof.c.1[2..], 16).unwrap(),
            },
        }
    }

    fn to_trollup_l1_input(&self) -> [U256; 17usize] {
        assert_eq!(self.inputs.len(), 17);
        self.inputs
            .iter()
            .map(|x| U256::from_str_radix(&x[2..], 16).unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }
}
