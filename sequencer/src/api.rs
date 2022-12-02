use ethers::types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tx {
    pub from: Address,
    pub to: Address,
    pub nonce: U256,
    pub value: U256,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignedTx {
    pub tx: Tx,
    pub signature: String,
}

pub const RPC_SUBMIT_TX: &str = "submit_transaction";
