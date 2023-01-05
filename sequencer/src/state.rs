use crate::merkle_tree::{Hasher, MerkleTree, Value};

use ethers::types::U256;
use poseidon::*;
use serde::{Deserialize, Serialize};
use zokrates_field::Bn128Field;

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Account {
    pub id: U256,
    pub balance: U256,
    pub nonce: U256,
}

impl Account {
    pub fn new(id: U256, balance: U256, nonce: U256) -> Self {
        Account { id, balance, nonce }
    }
}

#[derive(Default, Clone)]
pub struct State {
    inner: MerkleTree<PoseidonHasher, Account>,
}

impl State {
    pub fn root(&self) -> U256 {
        self.inner.root_hash()
    }

    pub fn get(&self, key: &U256) -> Account {
        match self.inner.get(key) {
            Some(acc) => acc.clone(),
            None => Account::new(*key, 0.into(), 0.into()),
        }
    }

    pub fn proof(&self, key: &U256) -> Vec<U256> {
        self.inner.proof(key)
    }

    pub fn update(&mut self, key: &U256, value: Account) {
        self.inner.update(key, value)
    }
}

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

impl ToBn128Field for U256 {
    fn to_bn128_field(&self) -> Bn128Field {
        let mut n_bytes = vec![0; 32];
        self.to_little_endian(&mut n_bytes);
        Bn128Field::from_byte_vector(n_bytes)
    }
}

impl Value for Account {
    fn to_u256(&self) -> U256 {
        if self.balance.is_zero() && self.nonce.is_zero() {
            return 0.into();
        }

        poseidon::hash_BN_128(vec![
            self.id.to_bn128_field(),
            self.balance.to_bn128_field(),
            self.nonce.to_bn128_field(),
        ])
        .to_u256()
    }

    fn zero() -> Self {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_tree() {
        let s = State::default();
        assert_eq!(s.root(), 0.into());
    }

    #[test]
    fn add_account() {
        let mut s = State::default();
        s.update(
            &0.into(),
            Account {
                id: 0.into(),
                balance: 42.into(),
                nonce: 1.into(),
            },
        );
        let acc = s.get(&0.into());
        assert_eq!(acc.id, 0.into());
        assert_eq!(acc.balance, 42.into());
        assert_eq!(acc.nonce, 1.into());
    }

    #[test]
    fn merkle_proof() {
        let mut s = State::default();

        let acc0 = Account {
            id: 0.into(),
            balance: 42.into(),
            nonce: 1.into(),
        };

        let acc1 = Account {
            id: 1.into(),
            balance: 43.into(),
            nonce: 2.into(),
        };

        let acc2 = Account {
            id: 2.into(),
            balance: 44.into(),
            nonce: 3.into(),
        };

        s.update(&0.into(), acc0.clone());
        s.update(&1.into(), acc1.clone());
        s.update(&2.into(), acc2.clone());

        println!("Leaf 0 = {:?}", acc0.to_u256());
        println!("Leaf 1 = {:?}", acc1.to_u256());
        println!("Leaf 2 = {:?}", acc2.to_u256());

        let proof = s.proof(&0.into());
        assert_eq!(proof.len(), 256);

        println!("Root is {:?}", s.root());
        println!("Proof is {proof:?}");
    }
}
