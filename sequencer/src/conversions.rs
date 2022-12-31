use ethers::types::{Address, H256};

pub trait ToEthH256 {
    fn to_h256(&self) -> H256;
}

impl ToEthH256 for Address {
    fn to_h256(&self) -> H256 {
        // we keep our H256 as big endian
        // need to reverse
        let mut bytes20: [u8; 20] = self.to_fixed_bytes();
        bytes20.reverse();
        let bytes12: [u8; 12] = [0; 12];
        let bytes32: [u8; 32] = [bytes12.as_slice(), bytes20.as_slice()]
            .concat()
            .try_into()
            .unwrap();
        bytes32.into()
    }
}

