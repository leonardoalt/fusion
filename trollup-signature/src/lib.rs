use ethers_core::types::{U256, U512};

use trollup_api::{SignedTx, Tx};
use trollup_types::{
    PrivateKey, ToBabyJubjubPoint, ToBabyJubjubSignature, ToBigInt, ToBn128Field, ToU256,
};

pub fn sign(tx: &Tx, private_key: String) -> anyhow::Result<U512> {
    let wallet: PrivateKey = private_key.into();
    let msg = hash_tx(tx).to_big_int();

    match wallet.0.sign(msg) {
        Ok(sig) => Ok(U512::from_little_endian(sig.compress().as_slice())),
        Err(e) => Err(anyhow::anyhow!(e)),
    }
}

pub fn hash_tx(tx: &Tx) -> U256 {
    poseidon::hash_BN_128(
        [
            tx.sender.to_bn128_field(),
            tx.to.to_bn128_field(),
            tx.nonce.to_bn128_field(),
            tx.value.to_bn128_field(),
        ]
        .to_vec(),
    )
    .to_u256()
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
