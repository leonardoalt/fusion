use fusion_api::*;

use futures::{future, prelude::*};
use tarpc::{
    context,
    server::{self, incoming::Incoming, Channel},
    tokio_serde::formats::Json,
};

use tokio::sync::mpsc;

use std::net::{IpAddr, SocketAddr};

#[derive(Clone)]
struct FusionServer(SocketAddr, mpsc::Sender<SignedTx>);

/*
#[tarpc::server]
impl FusionRPC for FusionServer {
    async fn submit_transaction(
        self,
        _: context::Context,
        tx: fusion_api::SignedTx,
    ) -> Result<(), String> {
        self.1.send(tx.clone()).await.unwrap();
        Ok(())
    }
}
*/

/*
pub async fn run_server(sx: mpsc::Sender<SignedTx>, addr: String, port: u16) -> anyhow::Result<()> {
    let mut listener = tarpc::serde_transport::tcp::listen(
        &(IpAddr::V4(addr.parse().unwrap()), port),
        Json::default,
    )
    .await?;
    println!("Listening on port {}", listener.local_addr().port());
    listener.config_mut().max_frame_length(usize::MAX);
    listener
        // Ignore accept errors.
        .filter_map(|r| future::ready(r.ok()))
        .map(server::BaseChannel::with_defaults)
        // Limit channels to 1 per IP.
        .max_channels_per_key(1, |t| t.transport().peer_addr().unwrap().ip())
        .map(|channel| {
            let server = FusionServer(channel.transport().peer_addr().unwrap(), sx.clone());
            channel.execute(server.serve())
        })
        // Max 10 channels.
        .buffer_unordered(10)
        .for_each(|_| async {})
        .await;

    Ok(())
}
*/
