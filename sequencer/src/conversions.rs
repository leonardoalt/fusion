use ethers::types::{Address, U256};

pub trait ToEthU256 {
    fn to_u256(&self) -> U256;
}

impl ToEthU256 for Address {
    fn to_u256(&self) -> U256 {
        let bytes20: [u8; 20] = self.to_fixed_bytes();
        //bytes20.reverse();
        let bytes12: [u8; 12] = [0; 12];
        let bytes32: [u8; 32] = [bytes12.as_slice(), bytes20.as_slice()]
            .concat()
            .try_into()
            .unwrap();
        bytes32.into()
    }
}
