pub mod merkle_tree;
mod poseidon_hasher;
pub mod state;

use crate::merkle_tree::ToBitmap;
use crate::state::{Account, State};

use fusion_api::*;
use fusion_config::*;
use fusion_l1::fusion;
use fusion_types::{FromBabyJubjubPoint, Point, PublicKey, ToBabyJubjubSignature, ToU256};

use bitmaps::Bitmap;

use ethers_core::types::U256;

use serde::{Deserialize, Serialize};
use serde_json::from_reader;
use serde_tuple::*;

use zokrates_abi::parse_value;
use zokrates_abi::Encode;
use zokrates_ast::ir::ProgEnum;
use zokrates_ast::typed::abi::Abi;
use zokrates_bellperson::nova;
use zokrates_field::PallasField;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Serialize_tuple, Deserialize_tuple, Debug)]
pub struct CircuitInputNova {
    state: NovaState,
    w: NovaWitness,
}

impl CircuitInputNova {
    pub fn new(tx: &SignedTx, pre_state: &State, post_state: &State) -> Self {
        Self {
            state: NovaState::from_state(pre_state),
            w: NovaWitness::new(tx, pre_state, post_state),
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
struct NovaState {
    root: U256,
}

impl NovaState {
    fn from_state(s: &State) -> Self {
        Self { root: s.root() }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct NovaWitness {
    post_root: U256,
    tx: CircuitTx,
    pre_accounts: Vec<Account>,
    direction_selector: Vec<Vec<bool>>,
    pre_path: Vec<Vec<U256>>,
    post_path: Vec<Vec<U256>>,
}

impl NovaWitness {
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
            // TODO fix this when we have Nova with BN
            sig: CircuitTxSignature {
                r: Point {
                    x: 0.into(),
                    y: 0.into(),
                },
                s: 0.into(),
            },
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

pub struct Prover;

impl Prover {
    pub fn prove(
        config: &Config,
        tx: &fusion_api::SignedTx,
        pre_state: &State,
        post_state: &State,
    ) -> Result<(fusion::Proof, fusion::TxInfo), String> {
        let path = Path::new(&config.circuit_path);
        let file = File::open(path)
            .map_err(|why| format!("Could not open {}: {}", path.display(), why))?;

        let mut reader = BufReader::new(file);

        let prog = match ProgEnum::deserialize(&mut reader).unwrap() {
            ProgEnum::PallasProgram(p) => p,
            _ => panic!(),
        };
        let prog = prog.collect();

        let path = Path::new("../circuits/abi.json");
        let file = File::open(path)
            .map_err(|why| format!("Could not open {}: {}", path.display(), why))?;
        let mut reader = BufReader::new(file);

        let abi: Abi = from_reader(&mut reader).map_err(|why| why.to_string())?;

        let signature = abi.signature();

        let init_type = signature.inputs[0].clone();
        let step_type = signature.inputs[1].clone();

        println!("Encoding initial state...");

        let init = parse_value::<PallasField>(
            serde_json::from_str(
                &serde_json::to_string(&NovaState::from_state(pre_state)).unwrap(),
            )
            .unwrap(),
            init_type,
        )
        .unwrap()
        .encode();

        println!("Encoding witness list...");

        let witness = NovaWitness::new(tx, pre_state, post_state);
        let steps: Vec<_> = vec![parse_value::<PallasField>(
            serde_json::from_str(&serde_json::to_string(&witness).unwrap()).unwrap(),
            step_type,
        )
        .unwrap()
        .encode()];

        println!("Reading parameters...");

        let params_path = Path::new("../circuits/nova.params");
        let params_file = File::open(params_path)
            .map_err(|why| format!("Could not open {}: {}", params_path.display(), why))?;

        let params_reader = BufReader::new(params_file);
        let params = serde_cbor::from_reader(params_reader)
            .map_err(|why| format!("Could not deserialize {}: {}", params_path.display(), why))?;

        println!("Proving...");
        let proof = nova::prove(&params, &prog, init, None, steps)
            .map_err(|e| format!("Error `{e:#?}` during proving"))?
            .unwrap();

        let l1_tx = l2_to_l1_tx(pre_state, &witness);
        Ok((proof.to_fusion_l1_proof(), l1_tx))
    }
}

trait ToFusionL1Proof {
    fn to_fusion_l1_proof(&self) -> fusion::Proof;
}

impl<'a> ToFusionL1Proof for nova::RecursiveSNARKWithStepCount<'a, PallasField> {
    fn to_fusion_l1_proof(&self) -> fusion::Proof {
        fusion::Proof { phantom: 0.into() }
    }
}

fn l2_to_l1_tx(pre_state: &State, witness: &NovaWitness) -> fusion::TxInfo {
    fusion::TxInfo {
        pre_root: pre_state.root(),
        post_root: witness.post_root,
        kind: witness.tx.kind,
        sender_point: fusion::FusionPoint {
            x: witness.tx.sender.x,
            y: witness.tx.sender.y,
        },
        to_point: fusion::FusionPoint {
            x: witness.tx.to.x,
            y: witness.tx.to.y,
        },
        nonce: witness.tx.nonce,
        value: witness.tx.value,
        sig: fusion::FusionSignature {
            r: fusion::FusionPoint {
                x: witness.tx.sig.r.x,
                y: witness.tx.sig.r.y,
            },
            s: witness.tx.sig.s,
        },
        sender_acc: fusion::Account {
            id: witness.pre_accounts[0].id,
            balance: witness.pre_accounts[0].balance,
            nonce: witness.pre_accounts[0].nonce,
        },
        to_acc: fusion::Account {
            id: witness.pre_accounts[1].id,
            balance: witness.pre_accounts[1].balance,
            nonce: witness.pre_accounts[1].nonce,
        },
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn nova() {
        let fusion_config = Config::default();

        let (_sk_1, pk_1) = fusion_wallet::new_key_pair();
        let (_sk_2, pk_2) = fusion_wallet::new_key_pair();

        let tx = fusion_api::Tx {
            kind: TxKind::Transfer,
            sender: pk_1.clone().to_u256(),
            to: pk_2.clone().to_u256(),
            nonce: 1.into(),
            value: 0.into(),
        };
        // TODO uncomment when we have Nova + BN
        //let sig = fusion_wallet::sign(&tx, sk_1.to_string()).unwrap();
        let signed_tx = fusion_api::SignedTx {
            tx,
            signature: "".to_string(), //sig.to_string(),
        };

        let state_0 = State::default();
        let state_1 = apply_tx(state_0.clone(), &signed_tx.tx);

        let proof = Prover::prove(&fusion_config, &signed_tx, &state_0, &state_1);
        assert!(proof.is_ok());
    }

    fn apply_tx(mut state: State, tx: &Tx) -> State {
        let sender_pk: PublicKey = tx.sender.into();
        let sender_addr = sender_pk.address();

        let to_pk: PublicKey = tx.to.into();
        let to_addr = to_pk.address();

        let account_sender = state.get(&sender_addr);
        let account_to = state.get(&to_addr);

        let new_account_sender = match tx.kind {
            TxKind::Deposit => {
                Account::new(sender_addr, account_sender.balance + tx.value, tx.nonce)
            }
            TxKind::Transfer | TxKind::Withdraw => {
                Account::new(sender_addr, account_sender.balance - tx.value, tx.nonce)
            }
        };
        let new_account_to = match tx.kind {
            TxKind::Transfer => {
                Account::new(to_addr, account_to.balance + tx.value, account_to.nonce)
            }
            TxKind::Withdraw | TxKind::Deposit => account_to,
        };

        state.update(&sender_addr, new_account_sender);
        state.update(&to_addr, new_account_to);

        state
    }
}
