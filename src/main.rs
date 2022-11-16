use std::{
    net::SocketAddr,
    path::Path,
    sync::{Arc, Mutex},
    time::Duration,
};

use clap::{Parser, Subcommand};
use ethers::{
    abi::Address,
    core::k256::SecretKey,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer},
    types,
    utils::keccak256,
};
use hyper::Method;
use jsonrpsee::{
    server::{AllowHosts, ServerBuilder, ServerHandle},
    RpcModule,
};
use serde::{Deserialize, Serialize};
use tokio::{task, time::interval};
use tower_http::cors::{Any, CorsLayer};

#[derive(Debug, Serialize, Deserialize)]
struct Tx {
    from: Address,
    to: Address,
    amount: u32,
    // signature: todo :)
}
type Db = Arc<Mutex<Vec<Tx>>>;

const DB_PATH: &str = "./db";
const SERVER_ADDRESS: &str = "127.0.0.1:0";

#[derive(Debug, Parser)]
#[clap(name = "trollup sequencer", version = env!("CARGO_PKG_VERSION"))]
struct Opts {
    #[clap(subcommand)]
    pub sub: Option<Subcommands>,
}

#[derive(Debug, Subcommand)]
pub enum Subcommands {
    #[clap(about = "Sign a trollup transaction.")]
    Sign(TrollupTx),
}

#[derive(Debug, Clone, Parser, Default)]
pub struct TrollupTx {
    #[clap(
        long,
        short = 'p',
        value_name = "PRIVATE_KEY",
        help = "The private key that signs the message",
        default_value = "0x0000000000000000000000000000000000000000000000000000000000000000"
    )]
    pub private_key: ethers::types::H256,
    #[clap(
        long,
        short = 'f',
        value_name = "FROM_ADDRESS",
        help = "The address of the from address.",
        default_value = "0x0000000000000000000000000000000000000000"
    )]
    pub from: ethers::types::Address,
    #[clap(
        long,
        short = 't',
        value_name = "DEST_ADDRESS",
        help = "The address of the destination address.",
        default_value = "0x0000000000000000000000000000000000000000"
    )]
    pub to: ethers::types::Address,
    #[clap(
        long,
        short = 'n',
        value_name = "NONCE",
        help = "The nonce of the transaction.",
        default_value = "0"
    )]
    pub nonce: ethers::types::U256,
    #[clap(
        long,
        short = 'v',
        value_name = "VALUE",
        help = "The value of the transaction.",
        default_value = "0"
    )]
    pub value: ethers::types::U256,
}

async fn run_node() -> anyhow::Result<()> {
    let db_path = Path::new(DB_PATH);
    let db = init_db(db_path);
    let rpc = init_rpc(db.clone()).await.unwrap();
    //let l1 = init_l1(db.clone());

    task::spawn(async move {
        let mut interval = interval(Duration::from_millis(1000 * 5));

        loop {
            interval.tick().await;
            let mut db = db.lock().unwrap();
            println!("submit transactions {:#?}", db);
            db.drain(..);
        }
    });

    tokio::spawn(rpc.stopped());

    println!("Run the following snippet in the developer console in any Website.");
    println!(
        r#"
        fetch("http://{}", {{
            method: 'POST',
            mode: 'cors',
            headers: {{ 'Content-Type': 'application/json' }},
            body: JSON.stringify({{
                jsonrpc: '2.0',
                method: 'submit_transaction',
                params: {{
                    from: '0x0000000000000000000000000000000000000000',
                    to: '0x0000000000000000000000000000000000000000',
                    amount: 42
                }},
                id: 1
            }})
        }}).then(res => {{
            console.log("Response:", res);
            return res.text()
        }}).then(body => {{
            console.log("Response Body:", body)
        }});
    "#,
        SERVER_ADDRESS
    );

    futures::future::pending().await
}

fn hash_tx(sig_args: &TrollupTx) -> ethers::types::TxHash {
    let mut value_bytes = vec![0; 32];
    sig_args.value.to_big_endian(&mut value_bytes);

    let mut nonce_bytes = vec![0; 32];
    sig_args.nonce.to_big_endian(&mut nonce_bytes);

    let msg = [
        sig_args.from.as_fixed_bytes().to_vec(),
        sig_args.to.as_fixed_bytes().to_vec(),
        value_bytes,
        nonce_bytes,
    ]
    .concat();

    types::TxHash::from(keccak256(msg))
}

async fn sign(sig_args: TrollupTx) -> anyhow::Result<()> {
    let wallet: LocalWallet = SecretKey::from_be_bytes(&sig_args.private_key.as_bytes())
        .expect("invalid private key")
        .into();

    let hash = hash_tx(&sig_args).as_fixed_bytes().to_vec();
    let signature = wallet.sign_message(hash.clone()).await?;
    println!("{}", signature);

    Ok(())
}

fn verify_tx_signature(tx: TrollupTx, signature: types::Signature) -> anyhow::Result<()> {
    let hash = hash_tx(&tx).as_fixed_bytes().to_vec();
    signature.verify(hash, tx.from)?;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.sub {
        Some(Subcommands::Sign(sig_args)) => sign(sig_args).await,
        _ => run_node().await,
    }
}

fn init_db(path: &Path) -> Db {
    Arc::new(Mutex::new(vec![]))
}

fn init_l1(db: Db) -> Provider<Http> {
    Provider::<Http>::try_from("https://mainnet.infura.io/v3/YOUR_API_KEY").unwrap()
}

async fn init_rpc(db: Db) -> anyhow::Result<ServerHandle> {
    let cors = CorsLayer::new()
        // Allow `POST` when accessing the resource
        .allow_methods([Method::POST])
        // Allow requests from any origin
        .allow_origin(Any)
        .allow_headers([hyper::header::CONTENT_TYPE]);
    let middleware = tower::ServiceBuilder::new().layer(cors);

    let server = ServerBuilder::default()
        .set_host_filtering(AllowHosts::Any)
        .set_middleware(middleware)
        .build(SERVER_ADDRESS.parse::<SocketAddr>()?)
        .await?;

    println!("{}", server.local_addr().unwrap());

    let mut module = RpcModule::new(());
    module.register_method("submit_transaction", move |params, _| {
        println!("received transaction!");
        let tx: Tx = params.parse()?;
        let mut db = db.lock().unwrap();
        db.push(tx);
        Ok("Transaction received!")
    })?;

    let handle = server.start(module)?;

    Ok(handle)
}
