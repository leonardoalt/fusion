use crate::merkle_tree::Hasher;

use ethers_core::types::U256;
use poseidon::*;
use zokrates_field::Bn128Field;

use trollup_types::ToU256;

#[derive(Default, Clone)]
pub struct PoseidonHasher(Vec<Bn128Field>);

impl Hasher for PoseidonHasher {
    fn write_h256(&mut self, w: &U256) {
        self.0.push(w.to_bn128_field())
    }

    fn finish(self) -> U256 {
        poseidon::hash_BN_128(self.0).to_u256()
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
