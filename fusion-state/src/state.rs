use crate::merkle_tree::{MerkleTree, Value};
use crate::poseidon_hasher::{Hasher, PoseidonHasher};

use ruint::aliases::U256;
use serde::{Deserialize, Serialize};

extern crate alloc;
use alloc::vec::Vec;

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

fn poseidon(args: &[U256]) -> U256 {
    let mut hasher = PoseidonHasher::default();
    for a in args {
        hasher.write_h256(a);
    }
    hasher.finish()
}

#[derive(Serialize, Deserialize, Default, Clone)]
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
            None => Account::new(*key, U256::ZERO, U256::ZERO),
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
            return U256::ZERO;
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

    const ONE: U256 = U256::from_limbs([1, 0, 0, 0]);
    const TWO: U256 = U256::from_limbs([2, 0, 0, 0]);
    const THREE: U256 = U256::from_limbs([3, 0, 0, 0]);
    const N42: U256 = U256::from_limbs([42, 0, 0, 0]);
    const N43: U256 = U256::from_limbs([43, 0, 0, 0]);
    const N44: U256 = U256::from_limbs([44, 0, 0, 0]);

    #[test]
    fn empty_tree() {
        let s = State::default();
        assert_eq!(s.root(), U256::ZERO);
    }

    #[test]
    fn add_account() {
        let mut s = State::default();
        s.update(
            &U256::ZERO,
            Account {
                id: U256::ZERO,
                balance: N42,
                nonce: ONE,
            },
        );
        let acc = s.get(&U256::ZERO);
        assert_eq!(acc.id, U256::ZERO);
        assert_eq!(acc.balance, N42);
        assert_eq!(acc.nonce, ONE);
        assert_eq!(
            s.root(),
            U256::from_str_radix(
                "11451511948541742621487323918061834032674352303614210053651560948921266760133",
                10
            )
            .unwrap()
        );
    }

    #[test]
    fn merkle_proof() {
        let mut s = State::default();

        let acc0 = Account {
            id: U256::ZERO,
            balance: N42,
            nonce: ONE,
        };

        let acc1 = Account {
            id: ONE,
            balance: N43,
            nonce: TWO,
        };

        let acc2 = Account {
            id: TWO,
            balance: N44,
            nonce: THREE,
        };

        s.update(&U256::ZERO, acc0.clone());
        s.update(&ONE, acc1.clone());
        s.update(&TWO, acc2.clone());

        let proof = s.proof(&U256::ZERO);
        assert_eq!(proof.len(), 256);

        assert_eq!(
            s.root(),
            U256::from_str_radix(
                "10530429052483355307531639547793752128181067498703038350353566076039804625754",
                10
            )
            .unwrap()
        );
    }
}
