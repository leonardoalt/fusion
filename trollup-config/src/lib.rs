use ethers::types;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Config {
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
