use ethers_core::types;
use serde::{Deserialize, Serialize};
use std::default::Default;
use std::path::PathBuf;

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub circuit_path: String,
    pub circuit_abi_path: String,
    pub proving_key_path: String,
    pub database_path: String,
    pub eth_rpc_url: String,
    pub eth_private_key: String,
    pub min_tx_block: usize,
    pub socket_address: String,
    pub socket_port: u16,
    pub trollup_l1_contract: types::Address,
}

impl Config {
    pub fn from_file(path: String) -> Self {
        let config_file_str = std::fs::read_to_string(PathBuf::from(path)).unwrap();
        toml::from_str(&config_file_str).unwrap()
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            circuit_path: "../circuits/out".to_string(),
            circuit_abi_path: "../circuits/abi.json".to_string(),
            proving_key_path: "../circuits/proving.key".to_string(),
            database_path: "./db".to_string(),
            eth_rpc_url: "http://localhost:8545".to_string(),
            eth_private_key: String::default(),
            min_tx_block: 1,
            socket_address: "127.0.0.1".to_string(),
            socket_port: 38171,
            trollup_l1_contract: types::Address::default(),
        }
    }
}
