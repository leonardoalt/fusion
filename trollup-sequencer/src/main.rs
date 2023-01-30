use tokio::sync::mpsc;

use trollup_api::*;
use trollup_config::Config;

use trollup_sequencer::sequencer::*;
use trollup_sequencer::server::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_file("../trollup.toml".to_string());
    let (sx, rx): (mpsc::Sender<SignedTx>, mpsc::Receiver<SignedTx>) = mpsc::channel(1024);

    let socket_address = config.socket_address.to_string();
    tokio::spawn(async move {
        run_server(sx, socket_address, config.socket_port)
            .await
            .unwrap();
    });

    run_sequencer(&config, rx).await
}
