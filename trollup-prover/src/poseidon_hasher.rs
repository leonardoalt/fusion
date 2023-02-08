use crate::merkle_tree::Hasher;

use ethers_core::types::U256;
use poseidon_rs::*;

use trollup_types::{ToFr, ToU256};

pub fn poseidon(args: &[U256]) -> U256 {
    Poseidon::new()
        .hash(args.iter().map(|a| a.to_fr()).collect())
        .unwrap()
        .to_u256()
}

#[derive(Default, Clone)]
pub struct PoseidonHasher(Vec<Fr>);

impl Hasher for PoseidonHasher {
    fn write_h256(&mut self, w: &U256) {
        self.0.push(w.to_fr())
    }

    fn finish(self) -> U256 {
        Poseidon::new().hash(self.0).unwrap().to_u256()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hasher() {
        let mut hasher = PoseidonHasher::default();
        hasher.write_h256(&0.into());
        hasher.write_h256(&1.into());
        assert_eq!(
            hasher.finish(),
            U256::from_dec_str(
                "12583541437132735734108669866114103169564651237895298778035846191048104863326"
            )
            .unwrap()
        );
    }
}
