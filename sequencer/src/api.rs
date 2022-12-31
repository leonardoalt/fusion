use ethers::types::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tx {
    pub sender: Address,
    pub to: Address,
    pub nonce: U256,
    pub value: U256,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignedTx {
    pub tx: Tx,
    pub signature: String,
}

pub const RPC_SUBMIT_TX: &str = "submit_transaction";
