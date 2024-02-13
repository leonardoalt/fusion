use tokio::sync::mpsc;

use fusion_api::*;
use fusion_types::*;
use fusion_config::Config;

use fusion_sequencer::sequencer::*;
use fusion_sequencer::server::*;

use ruint::aliases::U256;

fn main() {
    let tx = Tx {
        kind: TxKind::Transfer,
        sender: PublicKey::from("0"),
        to: PublicKey::from("0"),
        nonce: U256::ZERO,
        value: U256::from_limbs([1, 0, 0, 0]),
    };
}

/*
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_file("../fusion.toml".to_string());
    let (sx, rx): (mpsc::Sender<SignedTx>, mpsc::Receiver<SignedTx>) = mpsc::channel(1024);

    let socket_address = config.socket_address.to_string();
    tokio::spawn(async move {
        run_server(sx, socket_address, config.socket_port)
            .await
            .unwrap();
    });

    run_sequencer(&config, rx).await
}
*/
