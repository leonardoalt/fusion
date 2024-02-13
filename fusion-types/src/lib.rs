#![no_std]

use serde::{Deserialize, Serialize};
extern crate alloc;
use alloc::string::String;

use ruint::aliases::U256;

/// The Fusion private key.
#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct PrivateKey(pub U256);

impl From<String> for PrivateKey {
    fn from(key: String) -> Self {
        U256::from_str_radix(&key, 10).unwrap().into()
    }
}

impl From<&str> for PrivateKey {
    fn from(key: &str) -> Self {
        U256::from_str_radix(&key, 10).unwrap().into()
    }
}

impl From<U256> for PrivateKey {
    fn from(key: U256) -> Self {
        Self(key)
    }
}

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct PublicKey(pub U256);

impl From<String> for PublicKey {
    fn from(key: String) -> Self {
        U256::from_str_radix(&key, 10).unwrap().into()
    }
}

impl From<&str> for PublicKey {
    fn from(key: &str) -> Self {
        U256::from_str_radix(&key, 10).unwrap().into()
    }
}

impl From<U256> for PublicKey {
    fn from(key: U256) -> Self {
        Self(key)
    }
}

impl PublicKey {
    pub fn address(&self) -> U256 {
        self.0
    }
}
