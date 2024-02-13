#![no_std]

pub mod merkle_tree;
mod poseidon_hasher;
pub mod state;

#[cfg(not(feature = "powdr"))]
mod poseidon_gl;

use crate::merkle_tree::ToBitmap;
use crate::state::{Account, State};

use fusion_api::*;
use fusion_types::PublicKey;

extern crate alloc;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

/*
use fusion_api::*;
use fusion_config::*;
use fusion_l1::fusion;
use fusion_types::{FromBabyJubjubPoint, Point, PublicKey, ToBabyJubjubSignature, ToU256};
*/

use bitmaps::Bitmap;

pub fn apply_tx(mut state: State, tx: &Tx) -> State {
    let sender_pk: PublicKey = tx.sender.clone();
    let sender_addr = sender_pk.address();

    let to_pk: PublicKey = tx.to.clone();
    let to_addr = to_pk.address();

    let account_sender = state.get(&sender_addr);
    let account_to = state.get(&to_addr);

    let new_account_sender = match tx.kind {
        TxKind::Deposit => Account::new(sender_addr, account_sender.balance + tx.value, tx.nonce),
        TxKind::Transfer | TxKind::Withdraw => {
            Account::new(sender_addr, account_sender.balance - tx.value, tx.nonce)
        }
    };
    let new_account_to = match tx.kind {
        TxKind::Transfer => Account::new(to_addr, account_to.balance + tx.value, account_to.nonce),
        TxKind::Withdraw | TxKind::Deposit => account_to,
    };

    state.update(&sender_addr, new_account_sender);
    state.update(&to_addr, new_account_to);

    state
}

/*
use serde::{Deserialize, Serialize};
use serde_json::from_reader;
use serde_tuple::*;
*/

/*
use rand_0_8::rngs::StdRng;
use rand_0_8::SeedableRng;
*/

/*
use std::borrow::Borrow;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
*/

/*
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
    pub fn new(tx: &SignedTx, pre_state: &State, post_state: &State) -> Self {
        let circuit_tx = tx.to_circuit_tx();

        let sender_addr = PublicKey(circuit_tx.sender.clone()).address();
        let to_addr = PublicKey(circuit_tx.to.clone()).address();

        let pre_account_from = pre_state.get(&sender_addr);
        let pre_account_to = match tx.tx.kind {
            TxKind::Withdraw => Account {
                id: tx.tx.to,
                balance: 0.into(),
                nonce: 0.into(),
            },
            _ => pre_state.get(&to_addr),
        };

        Self {
            pre_root: pre_state.root(),
            tx: circuit_tx,
            pre_accounts: vec![pre_account_from, pre_account_to],
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
*/

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

/*
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CircuitTx {
    kind: U256,
    sender: Point,
    to: Point,
    nonce: U256,
    value: U256,
    sig: CircuitTxSignature,
}

trait ToCircuitTx {
    fn to_circuit_tx(&self) -> CircuitTx;
}

impl ToCircuitTx for fusion_api::SignedTx {
    fn to_circuit_tx(&self) -> CircuitTx {
        let sender_pk: PublicKey = self.tx.sender.into();
        let to_pk: PublicKey = self.tx.to.into();
        CircuitTx {
            kind: self.tx.kind.to_u256(),
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

impl From<fusion_api::SignedTx> for CircuitTxSignature {
    fn from(tx: fusion_api::SignedTx) -> CircuitTxSignature {
        let sig = tx.signature.to_babyjubjub_signature();
        CircuitTxSignature {
            r: Point::from_babyjubjub_point(&sig.r_b8),
            s: sig.s.to_u256(),
        }
    }
}
*/

pub struct Prover;

type TxProof = ();

impl Prover {
    pub fn prove() -> Result<TxProof, String> {
        Ok(())
    }

    /*
    pub fn prove(
        config: &Config,
        tx: &fusion_api::SignedTx,
        pre_state: &State,
        post_state: &State,
    ) -> Result<fusion::TxProof, String> {
        let path = Path::new(&config.circuit_path);
        let file = File::open(path)
            .map_err(|why| format!("Could not open {}: {}", path.display(), why))?;

        let mut reader = BufReader::new(file);

        let prog = match ProgEnum::deserialize(&mut reader).unwrap() {
            ProgEnum::Bn128Program(p) => p,
            _ => panic!(),
        };
        let prog = prog.collect();

        let witness = Self::compute_witness(
            config,
            prog.statements.iter(),
            &prog.arguments,
            &prog.solvers,
            tx,
            pre_state,
            post_state,
        )?;

        let pk_path = Path::new(&config.proving_key_path);
        let pk_file = File::open(pk_path)
            .map_err(|why| format!("Could not open {}: {}", pk_path.display(), why))?;

        let pk_reader = BufReader::new(pk_file);

        let mut rng = StdRng::from_entropy();
        let proof: Proof<Bn128Field, G16> = Ark::generate_proof(prog, witness, pk_reader, &mut rng);
        let ret = proof.to_fusion_l1_tx();

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

    fn compute_witness<'a, S: Borrow<ir::Statement<'a, Bn128Field>>>(
        config: &Config,
        statements: impl Iterator<Item = S>,
        arguments: &[Parameter],
        solvers: &[Solver<'a, Bn128Field>],
        tx: &SignedTx,
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

        let inputs = CircuitInput::new(tx, pre_state, post_state);
        //println!("\n\n{}\n\n", serde_json::to_string(&inputs).unwrap());

        let witness = parse_strict(
            serde_json::to_string(&inputs).unwrap().as_str(),
            signature.inputs,
        )
        .map(Inputs::Abi)
        .map_err(|why| why.to_string())
        .map_err(|e| format!("Could not parse argument: {e}"))?;

        let interpreter = zokrates_interpreter::Interpreter::default();

        let encoded = witness.encode();
        let witness = interpreter
            .execute_with_log_stream(
                &encoded,
                statements,
                arguments,
                solvers,
                &mut std::io::stdout(),
            )
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
    */
}
