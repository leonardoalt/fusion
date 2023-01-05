use bitmaps::Bitmap;
use ethers::types::U256;

use core::marker::PhantomData;
use std::collections::BTreeMap;

/// The Merkle tree is represented by its active leaves and
/// intermediate nodes.
/// Whenever a leaf is updated, we add/update each node in the path
/// root -> leaf as a branch in `self.branches`.
/// Each node, including intermediate nodes but excluding the root,
/// is represented by a `BranchKey`, containing its height and
/// the binary path from the root until that node, indexed [0, 256).
/// For leaves, `bitmap` should be the reversed binary representation of
/// its leaf index [0, 2^256 - 1).
/// For intermediate nodes, every bit in [0, height) should be
/// `false`.
/// The leaves have height 0.
#[derive(Default, Clone)]
pub struct MerkleTree<H, T> {
    /// Mapping from leaf index to value.
    leaves: BTreeMap<U256, T>,
    /// Mapping from internal node identifier to its hash.
    branches: BTreeMap<BranchKey, BranchNode>,
    phantom: PhantomData<H>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
struct BranchKey {
    height: u8,
    bitmap: Bitmap<256>,
}

impl BranchKey {
    fn new(height: u8, bitmap: Bitmap<256>) -> Self {
        if let Some(index) = bitmap.first_index() {
            assert!(index >= height as usize);
        }
        Self { height, bitmap }
    }

    fn for_leaf(key: &U256) -> Self {
        BranchKey::new(0, key.to_bitmap())
    }

    fn parent(&self) -> Option<Self> {
        match self.height {
            255 => None,
            _ => {
                let mut p_map = self.bitmap;
                p_map.set(self.height as usize, false);
                Some(Self::new(self.height + 1, p_map))
            }
        }
    }

    /// Iterator that returns the sequence of BranchKeys to the root.
    fn path_to_root(&self) -> BranchKeyIterator {
        BranchKeyIterator(Some(self.clone()))
    }

    fn sibling(&self) -> Self {
        let mut p_map = self.bitmap;
        p_map.set(self.height as usize, !p_map.get(self.height as usize));
        Self::new(self.height, p_map)
    }

    fn left_child(&self) -> Option<Self> {
        match self.height {
            0 => None,
            _ => {
                let mut p_map = self.bitmap;
                p_map.set((self.height as usize) - 1, false);
                Some(Self::new(self.height - 1, p_map))
            }
        }
    }

    fn right_child(&self) -> Option<Self> {
        self.left_child().map(|node| node.sibling())
    }

    fn is_left_child(&self) -> bool {
        !self.bitmap.get(self.height as usize)
    }
}

struct BranchKeyIterator(Option<BranchKey>);

impl Iterator for BranchKeyIterator {
    type Item = BranchKey;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.0.clone();
        self.0 = self.0.clone().and_then(|k| k.parent());
        result
    }
}

#[derive(Clone)]
struct BranchNode(U256);

impl<H: Hasher + Default, T: Value + Clone + Default> MerkleTree<H, T> {
    pub fn root_hash(&self) -> U256 {
        let left = BranchKey::new(255, Bitmap::<256>::default());
        let right = left.sibling();
        self.merge_nodes(&left, &right)
    }

    pub fn get(&self, key: &U256) -> Option<&T> {
        self.leaves.get(key)
    }

    pub fn proof(&self, key: &U256) -> Vec<U256> {
        BranchKey::for_leaf(key)
            .path_to_root()
            .map(|item| self.branch_hash(&item.sibling()))
            .collect()
    }

    pub fn update(&mut self, key: &U256, value: T) {
        self.leaves.insert(*key, value.clone());

        let branch_key = BranchKey::for_leaf(key);

        self.branches
            .insert(branch_key.clone(), BranchNode(Self::leaf_hash(key, &value)));

        self.update_parents(&branch_key);
    }

    pub fn delete(&mut self, key: &U256) {
        self.leaves.remove(key);

        let branch_key = BranchKey::for_leaf(key);
        self.branches.remove(&branch_key);
        self.update_parents(&branch_key);
    }

    pub fn verify_proof(root_hash: &U256, key: &U256, value: &T, proof: &[U256]) -> bool {
        if proof.len() != 256 {
            return false;
        }
        let mut hash = Self::leaf_hash(key, value);
        for (proof_item, path_item) in proof.iter().zip(BranchKey::for_leaf(key).path_to_root()) {
            hash = if path_item.is_left_child() {
                Self::merge_hashes(&hash, proof_item)
            } else {
                Self::merge_hashes(proof_item, &hash)
            };
        }
        hash == *root_hash
    }

    fn update_parents(&mut self, branch_key: &BranchKey) {
        for parent in branch_key.path_to_root().skip(1) {
            let left = parent.left_child().unwrap();
            let right = parent.right_child().unwrap();
            self.set_branch(&parent, BranchNode(self.merge_nodes(&left, &right)));
        }
    }

    fn merge_nodes(&self, key1: &BranchKey, key2: &BranchKey) -> U256 {
        let v1 = self.branch_hash(key1);
        let v2 = self.branch_hash(key2);
        Self::merge_hashes(&v1, &v2)
    }

    fn merge_hashes(v1: &U256, v2: &U256) -> U256 {
        if v1.is_zero() && v2.is_zero() {
            0.into()
        } else if v1.is_zero() {
            *v2
        } else if v2.is_zero() {
            *v1
        } else {
            let mut h = H::default();
            h.write_h256(v1);
            h.write_h256(v2);
            h.finish()
        }
    }

    fn branch_hash(&self, key: &BranchKey) -> U256 {
        match self.branches.get(key) {
            Some(value) => value.0,
            _ => 0.into(),
        }
    }

    /// Hashes the key and the value together. Returns zero if the value is zero
    /// (but hashes even if the key is zero).
    fn leaf_hash(key: &U256, value: &T) -> U256 {
        let value = value.to_u256();
        if value.is_zero() {
            0.into()
        } else {
            let mut h = H::default();
            h.write_h256(key);
            h.write_h256(&value);
            h.finish()
        }
    }

    fn set_branch(&mut self, key: &BranchKey, value: BranchNode) {
        if value.0.is_zero() {
            self.branches.remove(key);
        } else {
            self.branches.insert(key.clone(), value);
        }
    }
}

pub trait ToBitmap {
    fn to_bitmap(&self) -> Bitmap<256>;
}

impl ToBitmap for U256 {
    fn to_bitmap(&self) -> Bitmap<256> {
        let x1 = self.low_u128();
        let x2 = (self >> 128).low_u128();
        Bitmap::<256>::from([x1, x2])
    }
}

pub trait Value {
    fn to_u256(&self) -> U256;
    fn zero() -> Self;
}

pub trait Hasher {
    fn write_h256(&mut self, w: &U256);
    fn finish(self) -> U256;
}

#[cfg(test)]
mod test {
    use crate::poseidon_hasher::PoseidonHasher;

    use super::*;

    impl Value for U256 {
        fn to_u256(&self) -> U256 {
            *self
        }

        fn zero() -> Self {
            Default::default()
        }
    }

    #[test]
    fn siblings() {
        let zero = BranchKey::for_leaf(&0.into());
        let one = BranchKey::for_leaf(&1.into());
        let sib = zero.sibling();
        assert_eq!(one, sib);

        let six = BranchKey::for_leaf(&6.into());
        let seven = BranchKey::for_leaf(&7.into());
        let six_sib = six.sibling();
        assert_eq!(seven, six_sib);

        let last = BranchKey::for_leaf(&U256::max_value());
        let second_last = BranchKey::for_leaf(&(U256::max_value() - 1));
        let last_sib = last.sibling();
        assert_eq!(second_last, last_sib);
    }

    #[test]
    fn iterator_length() {
        assert_eq!(BranchKey::for_leaf(&0.into()).path_to_root().count(), 256);
    }

    #[test]
    fn zero_and_nonexisting_is_same() {
        let mut tree = MerkleTree::<PoseidonHasher, U256>::default();
        let empty_root_hash = tree.root_hash();
        tree.update(&1.into(), 0.into());
        assert_eq!(tree.root_hash(), empty_root_hash);
        tree.update(&0.into(), 0.into());
        assert_eq!(tree.root_hash(), empty_root_hash);
        tree.update(&23.into(), 0.into());
        assert_eq!(tree.root_hash(), empty_root_hash);

        tree.update(&0.into(), 1.into());
        let something_at_zero = tree.root_hash();
        assert!(something_at_zero != empty_root_hash);
        tree.update(&1.into(), 7.into());
        let something_at_one_and_zero = tree.root_hash();
        assert!(something_at_one_and_zero != empty_root_hash);
        assert!(something_at_one_and_zero != something_at_zero);

        tree.delete(&0.into());
        let something_at_one = tree.root_hash();
        assert!(something_at_one != something_at_one_and_zero);
        assert!(something_at_one != something_at_zero);
        assert!(something_at_one != empty_root_hash);

        tree.delete(&1.into());
        assert_eq!(tree.root_hash(), empty_root_hash);
    }

    #[test]
    fn empty_proof() {
        let tree = MerkleTree::<PoseidonHasher, U256>::default();
        let proof = tree.proof(&1.into());
        let root_hash = tree.root_hash();
        assert_eq!(proof.len(), 256);
        assert!(MerkleTree::<PoseidonHasher, U256>::verify_proof(
            &root_hash,
            &1.into(),
            &0.into(),
            &proof
        ));
    }

    #[test]
    fn single_item_proof() {
        let mut tree = MerkleTree::<PoseidonHasher, U256>::default();
        tree.update(&1.into(), 7.into());
        let proof = tree.proof(&1.into());
        let root_hash = tree.root_hash();
        assert_eq!(proof.len(), 256);
        assert!(MerkleTree::<PoseidonHasher, U256>::verify_proof(
            &root_hash,
            &1.into(),
            &7.into(),
            &proof
        ));

        // Proof is invalid on a wrong value
        assert!(!MerkleTree::<PoseidonHasher, U256>::verify_proof(
            &root_hash,
            &1.into(),
            &8.into(),
            &proof
        ));

        // Proof is invalid on a wrong key
        assert!(!MerkleTree::<PoseidonHasher, U256>::verify_proof(
            &root_hash,
            &0.into(),
            &7.into(),
            &proof
        ));
    }

    #[test]
    fn multiple_items_proof() {
        let mut tree = MerkleTree::<PoseidonHasher, U256>::default();
        tree.update(&0.into(), 1.into());
        tree.update(&1.into(), 2.into());
        tree.update(&12.into(), 3.into());
        let root_hash = tree.root_hash();
        assert!(MerkleTree::<PoseidonHasher, U256>::verify_proof(
            &root_hash,
            &0.into(),
            &1.into(),
            &tree.proof(&0.into())
        ));
        assert!(MerkleTree::<PoseidonHasher, U256>::verify_proof(
            &root_hash,
            &1.into(),
            &2.into(),
            &tree.proof(&1.into())
        ));
        assert!(MerkleTree::<PoseidonHasher, U256>::verify_proof(
            &root_hash,
            &11.into(),
            &0.into(),
            &tree.proof(&11.into())
        ));
        assert!(MerkleTree::<PoseidonHasher, U256>::verify_proof(
            &root_hash,
            &12.into(),
            &3.into(),
            &tree.proof(&12.into())
        ));
    }
}
