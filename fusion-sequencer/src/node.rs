use ethers::core::utils::hex;
use ethers::prelude::*;
use k256::SecretKey;
use log::info;
use std::{convert::TryFrom, sync::Arc, time::Duration};

#[derive(Debug, Clone)]
pub struct Node {
    pub http_client: Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
    pub http_endpoint: String,
}

impl Node {
    pub async fn new_with_private_key(
        priv_key: String,
        http_endpoint: String,
    ) -> anyhow::Result<Self> {
        let priv_key = hex::decode(priv_key)?;
        let provider = Provider::<Http>::try_from(http_endpoint.clone())?
            .interval(Duration::from_millis(10u64));
        let chain_id: u64 = provider.get_chainid().await?.as_u64();

        let wallet: LocalWallet = SecretKey::from_slice(&priv_key)
            .expect("did not get private key")
            .into();
        info!("Wallet with address: {:?}", wallet.address().clone());
        let provider = provider.with_sender(wallet.address());
        let wallet = wallet.with_chain_id(chain_id);
        let http_client = SignerMiddleware::new(provider, wallet);
        let http_client = Arc::new(http_client);

        Ok(Node {
            http_client,
            http_endpoint,
        })
    }
}
