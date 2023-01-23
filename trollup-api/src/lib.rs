use ethers_core::types::U256;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tx {
    pub sender: U256,
    pub to: U256,
    pub nonce: U256,
    pub value: U256,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignedTx {
    pub tx: Tx,
    pub signature: String,
}

pub const RPC_SUBMIT_TX: &str = "submit_transaction";

#[tarpc::service]
pub trait TrollupRPC {
    async fn submit_transaction(tx: SignedTx) -> String;
}
