use crate::merkle_tree::{MerkleTree, Value};
use crate::poseidon_hasher::{poseidon, PoseidonHasher};

use ethers_core::types::U256;
use serde::{Deserialize, Serialize};

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

impl Value for Account {
    fn to_u256(&self) -> U256 {
        if self.balance.is_zero() && self.nonce.is_zero() {
            return 0.into();
        }

        poseidon(&[self.balance, self.nonce])
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
        assert_eq!(
            s.root(),
            U256::from_dec_str(
                "11451511948541742621487323918061834032674352303614210053651560948921266760133"
            )
            .unwrap()
        );
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

        let proof = s.proof(&0.into());
        assert_eq!(proof.len(), 256);

        assert_eq!(
            s.root(),
            U256::from_dec_str(
                "10530429052483355307531639547793752128181067498703038350353566076039804625754"
            )
            .unwrap()
        );
    }
}
