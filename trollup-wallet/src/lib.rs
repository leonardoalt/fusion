use ethers_core::types::U512;

use trollup_api::{hash_tx, SignedTx, Tx};
use trollup_types::{
    FromBabyJubjubPoint, PrivateKey, PublicKey, ToBabyJubjubPoint, ToBabyJubjubSignature, ToBigInt,
};

pub fn sign(tx: &Tx, private_key: String) -> anyhow::Result<U512> {
    let wallet: PrivateKey = private_key.into();
    let msg = hash_tx(tx).to_big_int();

    match wallet.0.sign(msg) {
        Ok(sig) => Ok(U512::from_little_endian(sig.compress().as_slice())),
        Err(e) => Err(anyhow::anyhow!(e)),
    }
}

pub fn verify_tx_signature(tx: &SignedTx) -> anyhow::Result<()> {
    let pk = tx.tx.sender.to_babyjubjub_point();
    let sig = tx.signature.to_babyjubjub_signature();
    let msg = hash_tx(&tx.tx).to_big_int();

    match babyjubjub_rs::verify(pk, sig, msg) {
        true => Ok(()),
        false => Err(anyhow::anyhow!("Invalid signature.")),
    }
}

pub fn new_private_key() -> PrivateKey {
    PrivateKey(babyjubjub_rs::new_key())
}

pub fn new_public_key(sk: &PrivateKey) -> PublicKey {
    PublicKey::from_babyjubjub_point(&sk.0.public())
}

pub fn new_key_pair() -> (PrivateKey, PublicKey) {
    let sk = new_private_key();
    let pk = new_public_key(&sk);
    (sk, pk)
}

#[cfg(test)]
mod test {
    use super::*;
    use trollup_api::TxKind;
    use trollup_types::ToU256;

    #[test]
    fn signatures() {
        let (sk_1, pk_1) = new_key_pair();
        let (_sk_2, pk_2) = new_key_pair();
        let tx = Tx {
            kind: TxKind::Transfer,
            sender: pk_1.to_u256(),
            to: pk_2.to_u256(),
            nonce: 1.into(),
            value: 0.into(),
        };
        let sig = sign(&tx, sk_1.to_string());
        let mut signed_tx = SignedTx {
            tx,
            signature: sig.unwrap().to_string(),
        };
        assert!(verify_tx_signature(&signed_tx).is_ok());
        signed_tx.tx.nonce = 3.into();
        assert!(verify_tx_signature(&signed_tx).is_err());
    }
}
