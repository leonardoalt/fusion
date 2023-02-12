use ethers_core::types::{U256, U512};
use ff::*;
use num_bigint::{
    BigInt,
    Sign::{NoSign, Plus},
};
use poseidon_rs::*;
use serde::{Deserialize, Serialize};
use std::string::ToString;

/// The Fusion private key.
/// It simply wraps a Baby Jubjub private key.
/// It can be imported/exported to String,
/// and imported from U256.
pub struct PrivateKey(pub babyjubjub_rs::PrivateKey);

impl From<String> for PrivateKey {
    fn from(key: String) -> Self {
        U256::from_dec_str(&key).unwrap().into()
    }
}

impl From<U256> for PrivateKey {
    fn from(key: U256) -> Self {
        let mut bytes = vec![0; 32];
        key.to_big_endian(&mut bytes);
        Self(babyjubjub_rs::PrivateKey::import(bytes).unwrap())
    }
}

impl ToString for PrivateKey {
    fn to_string(&self) -> String {
        U256::from_big_endian(&self.0.key).to_string()
    }
}

/// The Fusion public key.
/// It simply wraps a point. Ideally this would wrap a
/// babyjubjub_rs::Point, but that struct does not have
/// Serialize/Deserialize.
/// Therefore we implement our own Point struct with
/// conversion to/from babyjubjub_rs::Point.
///
/// Both `x` and `y` are field elements but are represented
/// here by an U256 for convenience, which may be changed
/// in the future.
///
/// The U256 representation of a public key is a compressed
/// point which does not fit in a field element.
/// In order to get the address of a public key we need to
/// compute `poseidon(x, y)` which results in a field element.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PublicKey(pub Point);

impl PublicKey {
    pub fn address(&self) -> U256 {
        self.to_fr().to_u256()
    }

    pub fn from_point(x: U256, y: U256) -> Self {
        Self(Point { x, y })
    }
}

impl From<U256> for PublicKey {
    fn from(key: U256) -> Self {
        Self::from_babyjubjub_point(&key.to_babyjubjub_point())
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Point {
    x: U256,
    y: U256,
}

pub trait FromBabyJubjubPoint {
    fn from_babyjubjub_point(point: &babyjubjub_rs::Point) -> Self;
}

impl FromBabyJubjubPoint for PublicKey {
    fn from_babyjubjub_point(point: &babyjubjub_rs::Point) -> Self {
        Self(Point::from_babyjubjub_point(point))
    }
}

impl FromBabyJubjubPoint for Point {
    fn from_babyjubjub_point(point: &babyjubjub_rs::Point) -> Self {
        Self {
            x: U256::from_str_radix(&ff::to_hex(&point.x), 16).unwrap(),
            y: U256::from_str_radix(&ff::to_hex(&point.y), 16).unwrap(),
        }
    }
}

pub trait ToBabyJubjubPoint {
    fn to_babyjubjub_point(&self) -> babyjubjub_rs::Point;
}

impl ToBabyJubjubPoint for PublicKey {
    fn to_babyjubjub_point(&self) -> babyjubjub_rs::Point {
        babyjubjub_rs::Point {
            x: babyjubjub_rs::Fr::from_str(&self.0.x.to_string()).unwrap(),
            y: babyjubjub_rs::Fr::from_str(&self.0.y.to_string()).unwrap(),
        }
    }
}

/// Decompresses a Baby Jubjub point.
impl ToBabyJubjubPoint for U256 {
    fn to_babyjubjub_point(&self) -> babyjubjub_rs::Point {
        let mut bytes = vec![0; 32];
        self.to_big_endian(&mut bytes);
        babyjubjub_rs::decompress_point(bytes.try_into().unwrap()).unwrap()
    }
}

pub trait ToU256 {
    fn to_u256(&self) -> U256;
}

/// Compresses a point into a U256.
impl ToU256 for PublicKey {
    fn to_u256(&self) -> U256 {
        U256::from_big_endian(self.to_babyjubjub_point().compress().as_slice())
    }
}

impl ToU256 for Fr {
    fn to_u256(&self) -> U256 {
        U256::from_str_radix(&ff::to_hex(self), 16).unwrap()
    }
}

impl ToU256 for BigInt {
    fn to_u256(&self) -> U256 {
        let bytes = self.to_bytes_be();
        assert!(bytes.0 == NoSign || bytes.0 == Plus);
        bytes.1.as_slice().try_into().unwrap()
    }
}

pub trait ToFr {
    fn to_fr(&self) -> Fr;
}

/// Computes a single field element which represents the
/// address of a public key.
impl ToFr for PublicKey {
    fn to_fr(&self) -> Fr {
        Poseidon::new()
            .hash(vec![self.0.x.to_fr(), self.0.y.to_fr()])
            .unwrap()
    }
}

/// Converts a U256 into a field element.
/// Panics if it does not fit.
impl ToFr for U256 {
    fn to_fr(&self) -> Fr {
        Fr::from_str(&self.to_string()).unwrap()
    }
}

pub trait ToBigInt {
    fn to_big_int(&self) -> BigInt;
}

impl ToBigInt for U256 {
    fn to_big_int(&self) -> BigInt {
        let mut bytes = vec![0; 32];
        self.to_big_endian(&mut bytes);
        BigInt::from_bytes_be(Plus, bytes.as_slice())
    }
}

pub trait ToBabyJubjubSignature {
    fn to_babyjubjub_signature(&self) -> babyjubjub_rs::Signature;
}

/// Parses a String into a Baby Jubjub signature.
impl ToBabyJubjubSignature for String {
    fn to_babyjubjub_signature(&self) -> babyjubjub_rs::Signature {
        U512::from_dec_str(self).unwrap().to_babyjubjub_signature()
    }
}

/// Decompresses a Baby Jubjub signature from a U512.
impl ToBabyJubjubSignature for U512 {
    fn to_babyjubjub_signature(&self) -> babyjubjub_rs::Signature {
        let mut bytes = vec![0; 64];
        self.to_little_endian(&mut bytes);
        babyjubjub_rs::decompress_signature(bytes.as_slice().try_into().unwrap()).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn private_key_str() {
        let sk_str = "7037699680704680447486894718306905702926540801898916944203447270992684920821";
        let sk: PrivateKey = sk_str.to_string().into();
        assert_eq!(sk_str, sk.to_string());
    }

    #[test]
    fn private_key_u256() {
        let sk_u256 = U256::from_dec_str(
            "7037699680704680447486894718306905702926540801898916944203447270992684920821",
        )
        .unwrap();
        let sk: PrivateKey = sk_u256.into();
        assert_eq!(sk_u256.to_string(), sk.to_string());
    }

    #[test]
    fn public_key_bjj() {
        let pk = PublicKey::from_point(1.into(), 2.into());
        assert_eq!(
            pk.0,
            PublicKey::from_babyjubjub_point(&pk.to_babyjubjub_point()).0
        );
    }

    #[test]
    fn public_key_compress_decompress() {
        let pk = U256::from_dec_str("42").unwrap();
        assert_eq!(
            pk,
            PublicKey::from_babyjubjub_point(&pk.to_babyjubjub_point()).to_u256()
        );
    }

    #[test]
    fn public_key_address() {
        let pk = PublicKey::from_point(1.into(), 2.into());
        assert_eq!(
            pk.address(),
            U256::from_dec_str(
                "7853200120776062878684798364095072458815029376092732009249414926327459813530"
            )
            .unwrap()
        );
    }

    #[test]
    fn u256_conversion() {
        let x = U256::from_dec_str("42").unwrap();
        let y = x.to_big_int();
        assert_eq!(x.to_string(), y.to_string());
    }

    #[test]
    fn u256_fr_conversion() {
        let x = U256::from_dec_str("42").unwrap();
        assert_eq!(x, x.to_fr().to_u256());
    }

    #[test]
    fn signature_compress_decompress() {
        let sig = "122241928682229286598976029249532022025637739860654613779160404391018488754905842538161678835085790288668324736777854019032918292839284526282556835964629";
        let bjj_sig = sig.to_string().to_babyjubjub_signature();
        let u512_sig = U512::from_dec_str(sig).unwrap().to_babyjubjub_signature();
        assert_eq!(bjj_sig.s, u512_sig.s);
    }
}
