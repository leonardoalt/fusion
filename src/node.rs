use ethers::prelude::*;
use std::{convert::TryFrom, sync::Arc, time::Duration};

#[derive(Debug, Clone)]
pub struct Node {
    pub http_client: Arc<Provider<Http>>,
    pub http_endpoint: String,
}

impl Node {
    /// Reads from env variables `ETH_RPC_URL` and `ETH_WS_URL`
    /// - ETH_RPC_URL: json-rpc over http
    /// - ETH_WS_URL: json-rpc over ws
    pub async fn new_local_node_from_env() -> anyhow::Result<Self> {
        let http_endpoint = std::env::var("ETH_RPC_URL")?;

        // Polling duration of 10 second
        let http_client = Provider::<Http>::try_from(http_endpoint.clone())?
            .interval(Duration::from_millis(10000u64));

        let http_client = Arc::new(http_client);

        Ok(Node {
            http_client,
            http_endpoint,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn sanity_check_endpoint() -> anyhow::Result<()> {
        let node: Node = Node::new_local_node_from_env().await?;

        // Anvil's chainid is 31337
        assert_eq!(node.http_client.get_chainid().await?, U256::from(31337));
        Ok(())
    }
}
