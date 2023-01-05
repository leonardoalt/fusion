use bitmaps::Bitmap;
use ethers::types::{H256, U256};

use core::marker::PhantomData;
use std::collections::BTreeMap;

/// The Merkle tree is represented by its active leaves and
/// intermediate nodes.
/// Whenever a leaf is updated, we add/update each node in the path
/// root -> leaf as a branch in `self.branches`.
/// Each node, including intermediate nodes but excluding the root,
/// is represented by a `BranchKey`, containing its height and
/// the binary path from the root until that node, indexed [0, 256).
/// For leaves, `bitmap` should be the binary representation of
/// its leaf index [0, 2^256 - 1).
/// For intermediate nodes, every bit in [height + 1, 256) should be
/// `false`.
/// The leaves have height 255.
#[derive(Default, Clone)]
pub struct MerkleTree<H, T> {
    keys: BTreeMap<U256, T>,
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
        Self { height, bitmap }
    }

    fn parent(&self) -> Option<Self> {
        match self.height {
            0 => None,
            _ => {
                let mut p_map = self.bitmap;
                p_map.set(self.height as usize, false);
                Some(Self::new(self.height - 1, p_map))
            }
        }
    }

    fn sibling(&self) -> Self {
        let mut p_map = self.bitmap;
        p_map.set(self.height as usize, !p_map.get(self.height as usize));
        Self::new(self.height, p_map)
    }

    fn left_child(&self) -> Option<Self> {
        match self.height {
            255 => None,
            _ => {
                let mut p_map = self.bitmap;
                p_map.set((self.height as usize) + 1, false);
                Some(Self::new(self.height + 1, p_map))
            }
        }
    }

    fn right_child(&self) -> Option<Self> {
        self.left_child().map(|node| node.sibling())
    }
}

#[derive(Clone)]
struct BranchNode(U256);

impl<H: Hasher + Default, T: Value + Clone + Default> MerkleTree<H, T> {
    pub fn root(&self) -> U256 {
        let left = BranchKey::new(0, Bitmap::<256>::default());
        let right = left.sibling();
        self.merge_nodes(&left, &right)
    }

    pub fn get(&self, key: &U256) -> Option<T> {
        match self.keys.get(key) {
            Some(value) => Some(value.clone()),
            None => None
        }
    }

    pub fn proof(&self, key: &U256) -> Vec<U256> {
        let mut siblings = vec![];

        let mut leaf_key = Some(BranchKey::new(255, key.to_bitmap()));
        // TODO make this safer
        for _ in (0..=255).rev() {
            let inner_key = leaf_key.unwrap();
            siblings.push(self.branch_root(&inner_key.sibling()));
            leaf_key = inner_key.parent();
        }

        siblings
    }

    pub fn update(&mut self, key: &U256, value: T) {
        // TODO remove entry from `self.keys` if `T == 0`.
        self.keys.insert(*key, value.clone());

        let branch_key = BranchKey::new(255, key.to_bitmap());
        self.branches
            .insert(branch_key.clone(), BranchNode(value.to_u256()));

        let mut last_key = branch_key;
        // TODO make this safer
        for _ in (0..=254).rev() {
            let parent = last_key.parent().unwrap();
            let left = parent.left_child().unwrap();
            let right = parent.right_child().unwrap();

            self.branches
                .insert(parent.clone(), BranchNode(self.merge_nodes(&left, &right)));

            last_key = parent;
        }
    }

    fn merge_nodes(&self, key1: &BranchKey, key2: &BranchKey) -> U256 {
        let v1 = self.branch_root(key1);
        let v2 = self.branch_root(key2);

        if v1.is_zero() && v2.is_zero() {
            0.into()
        } else if v1.is_zero() {
            v2
        } else if v2.is_zero() {
            v1
        } else {
            let mut h = H::default();
            h.write_h256(&v1);
            h.write_h256(&v2);
            h.finish()
        }
    }

    fn branch_root(&self, key: &BranchKey) -> U256 {
        match self.branches.get(key) {
            Some(value) => value.0,
            _ => 0.into()
        }
    }
}

pub trait ToBitmap {
    fn to_bitmap(&self) -> Bitmap<256>;
}

impl ToBitmap for H256 {
    // TODO this function needs to ensure that
    // the returned bitmap is the binary representation
    // of the given number.
    fn to_bitmap(&self) -> Bitmap<256> {
        let u = U256::from_little_endian(self.as_fixed_bytes());
        let x1 = u.low_u128();
        let x2 = (u >> 128).low_u128();
        Bitmap::<256>::from([x1, x2])
    }
}

impl ToBitmap for U256 {
    // TODO this function needs to ensure that
    // the returned bitmap is the binary representation
    // of the given number.
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