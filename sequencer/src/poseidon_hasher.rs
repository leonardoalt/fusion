use crate::merkle_tree::Hasher;

use ethers::types::{H256, U256};
use poseidon::*;
use zokrates_field::Bn128Field;

#[derive(Default, Clone)]
pub struct PoseidonHasher(Vec<Bn128Field>);

impl Hasher for PoseidonHasher {
    fn write_h256(&mut self, h: &U256) {
        self.0.push(h.to_bn128_field())
    }

    fn finish(self) -> U256 {
        poseidon::hash_BN_128(self.0).to_u256()
    }
}

pub trait ToH256 {
    fn to_h256(&self) -> H256;
}

impl ToH256 for Bn128Field {
    fn to_h256(&self) -> H256 {
        // `to_byte_vector` returns as little endian
        // we reverse to keep our H256 as big endian
        let mut bytes: [u8; 32] = self.to_byte_vector().try_into().unwrap();
        bytes.reverse();
        bytes.into()
    }
}

pub trait ToU256 {
    fn to_u256(&self) -> U256;
}

impl ToU256 for Bn128Field {
    fn to_u256(&self) -> U256 {
        let mut bytes: [u8; 32] = self.to_byte_vector().try_into().unwrap();
        bytes.reverse();
        bytes.into()
    }
}

pub trait ToBn128Field {
    fn to_bn128_field(&self) -> Bn128Field;
}

/*
impl ToBn128Field for Address {
    fn to_bn128_field(&self) -> Bn128Field {
        let mut bytes = [self.to_fixed_bytes().to_vec(), [0; 12].to_vec()].concat();
        //bytes.reverse();
        println!("{:?}", bytes);
        Bn128Field::from_byte_vector(bytes.into())
    }
}
*/

impl ToBn128Field for H256 {
    fn to_bn128_field(&self) -> Bn128Field {
        // `from_byte_vector` takes little endian
        // we keep our H256 in big endian so need to reverse here
        let mut bytes = self.to_fixed_bytes();
        bytes.reverse();
        Bn128Field::from_byte_vector(bytes.into())
    }
}

impl ToBn128Field for U256 {
    fn to_bn128_field(&self) -> Bn128Field {
        let mut n_bytes = vec![0; 32];
        self.to_little_endian(&mut n_bytes);
        Bn128Field::from_byte_vector(n_bytes)
    }
}
