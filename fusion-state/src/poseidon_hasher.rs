extern crate alloc;
use alloc::vec::Vec;

pub trait Hasher {
    fn write_h256(&mut self, w: &U256);
    fn finish(&mut self) -> U256;
}

#[cfg(not(feature = "powdr"))]
use crate::poseidon_gl;
use ruint::aliases::U256;

const GOLDILOCKS: u64 = 0xffffffff00000001;

#[derive(Default, Clone)]
// Each u64 here must be < Goldilocks.
pub struct PoseidonHasher(Vec<u64>);

impl Hasher for PoseidonHasher {
    fn write_h256(&mut self, w: &U256) {
        for limb in w.as_limbs() {
            if limb < &GOLDILOCKS {
                self.0.push(*limb);
            } else {
                let lower = *limb as u32;
                let upper = (limb >> 32) as u32;
                self.0.push(lower as u64);
                self.0.push(upper as u64);
            }
        }
    }

    fn finish(&mut self) -> U256 {
        let (first_chunk, rest) = self.0.split_at(4);

        let initial_acc = U256::from_limbs(first_chunk.try_into().unwrap());

        let acc = rest.chunks(4).fold(initial_acc, |acc, chunk| {
            let mut i = [0; 12];
            i[..4].copy_from_slice(acc.as_limbs());
            i[4..4 + chunk.len()].copy_from_slice(chunk);

            U256::from_limbs(hash(i))
        });

        acc
    }
}

#[cfg(feature = "powdr")]
fn hash(i: [u64; 12]) -> [u64; 4] {
    powdr_riscv_runtime::coprocessors::poseidon_gl(i)
}

#[cfg(not(feature = "powdr"))]
fn hash(i: [u64; 12]) -> [u64; 4] {
    poseidon_gl::poseidon_gl_u64(i)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hasher() {
        let mut hasher = PoseidonHasher::default();
        hasher.write_h256(&U256::ZERO);
        hasher.write_h256(&U256::from_limbs([1, 0, 0, 0]));
        assert_eq!(
            hasher.finish(),
            U256::from_str_radix(
                "43345161635195967832771964327305036471043959247426079103102845517912376067933",
                10
            )
            .unwrap()
        );
    }
}
