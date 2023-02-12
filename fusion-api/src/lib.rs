use ethers_core::types::U256;
use fusion_types::{FromBabyJubjubPoint, PublicKey, ToBabyJubjubPoint, ToFr, ToU256};
use poseidon_rs::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tx {
    pub kind: TxKind,
    pub sender: U256,
    pub to: U256,
    pub nonce: U256,
    pub value: U256,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TxKind {
    Transfer,
    Deposit,
    Withdraw,
}

impl ToU256 for TxKind {
    fn to_u256(&self) -> U256 {
        match self {
            TxKind::Transfer => 0.into(),
            TxKind::Deposit => 1.into(),
            TxKind::Withdraw => 2.into(),
        }
    }
}

impl From<u8> for TxKind {
    fn from(k: u8) -> Self {
        match k {
            0 => TxKind::Transfer,
            1 => TxKind::Deposit,
            2 => TxKind::Withdraw,
            _ => panic!(),
        }
    }
}

impl From<U256> for TxKind {
    fn from(k: U256) -> Self {
        match k.as_u32() {
            0 => TxKind::Transfer,
            1 => TxKind::Deposit,
            2 => TxKind::Withdraw,
            _ => panic!(),
        }
    }
}

pub fn hash_tx(tx: &Tx) -> U256 {
    let sender_pk = PublicKey::from_babyjubjub_point(&tx.sender.to_babyjubjub_point());
    let to_pk = PublicKey::from_babyjubjub_point(&tx.to.to_babyjubjub_point());
    Poseidon::new()
        .hash(
            [
                tx.kind.to_u256().to_fr(),
                sender_pk.to_fr(),
                to_pk.to_fr(),
                tx.nonce.to_fr(),
                tx.value.to_fr(),
            ]
            .to_vec(),
        )
        .unwrap()
        .to_u256()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignedTx {
    pub tx: Tx,
    pub signature: String,
}

#[tarpc::service]
pub trait FusionRPC {
    async fn submit_transaction(tx: SignedTx) -> Result<(), String>;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hash() {
        let tx = Tx {
            sender: U256::from_dec_str(
                "11693830015789570214896451416834991706586932551962432904221523856506008194081",
            )
            .unwrap(),
            to: U256::from_dec_str(
                "11693830015789570214896451416834991706586932551962432904221523856506008194081",
            )
            .unwrap(),
            nonce: 0.into(),
            value: 0.into(),
            kind: TxKind::Transfer,
        };
        assert_eq!(
            hash_tx(&tx),
            U256::from_dec_str(
                "5446841522722730699994610570698753366919140210878808046341046395713679433299"
            )
            .unwrap()
        );
    }
}
