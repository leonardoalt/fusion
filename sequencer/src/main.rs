use std::{
    net::SocketAddr,
    path::Path,
    sync::{Arc, Mutex},
    time::Duration,
};

use ethers::{
    providers::{Http, Provider},
    signers::LocalWallet,
    types,
    utils::keccak256,
};
use hyper::Method;
use jsonrpsee::{
    server::{AllowHosts, ServerBuilder, ServerHandle},
    RpcModule,
};
use tokio::{task, time::interval};
use tower_http::cors::{Any, CorsLayer};

use trollup_l1::trollup;

use sequencer::api::*;
use sequencer::conversions::*;
use sequencer::node::*;
use sequencer::prover::*;
use sequencer::state::{Account, State};

type Db = Arc<Mutex<Vec<SignedTx>>>;

const DB_PATH: &str = "./db";
const SOCKET_ADDRESS: &str = "127.0.0.1:38171";

async fn run_node() -> anyhow::Result<()> {
    let db_path = Path::new(DB_PATH);
    let db = init_db(db_path);
    let rpc = init_rpc(db.clone()).await.unwrap();

    let private_key = std::env::var("ETH_PRIVATE_KEY")?;
    let http_endpoint = std::env::var("ETH_RPC_URL")?;

    task::spawn(async move {
        let mut state = State::default();

        let l1_contract = init_l1(private_key, http_endpoint).await.unwrap();
        let mut interval = interval(Duration::from_millis(1000 * 5));

        loop {
            interval.tick().await;

            let current_root = l1_contract.root().call().await.unwrap();
            println!("Current root is {current_root}");

            let txs: Vec<_> = db
                .lock()
                .unwrap()
                .drain(..)
                .filter(|tx| validate_tx(&state, tx).is_ok())
                .collect();

            assert!(txs.len() <= 1);
            if !txs.is_empty() {
                let pre_state = state.clone();
                state = txs.iter().fold(state, apply_tx);
                println!("Computed L2 state root is {:?}", state.root());

                match Prover::prove(&txs[0].tx, &pre_state, &state) {
                    Err(e) => println!("Could not generate proof: {e}"),
                    Ok((proof, input)) => {
                        l1_contract
                            .submit_block(proof, input.to_vec())
                            .gas(1000000)
                            .send()
                            .await
                            .unwrap();
                    }
                };
            }
        }
    });

    tokio::spawn(rpc.stopped());

    println!("Run the following snippet in the developer console in any Website.");
    println!(
        r#"
        fetch("http://{SOCKET_ADDRESS}", {{
            method: 'POST',
            mode: 'cors',
            headers: {{ 'Content-Type': 'application/json' }},
            body: JSON.stringify({{
                jsonrpc: '2.0',
                method: 'submit_transaction',
                params: {{
                    sender: '0x0000000000000000000000000000000000000000',
                    to: '0x0000000000000000000000000000000000000000',
                    nonce: 3,
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
    "#
    );

    futures::future::pending().await
}

fn validate_tx(state: &State, tx: &SignedTx) -> anyhow::Result<()> {
    let account = state.get(&tx.tx.sender.to_u256());
    if account.balance < tx.tx.value {
        Err(anyhow::anyhow!("Insufficient balance"))
    } else if account.nonce >= tx.tx.nonce {
        Err(anyhow::anyhow!("Nonce too low"))
    } else {
        Ok(())
    }
}

fn apply_tx(mut state: State, tx: &SignedTx) -> State {
    let key_sender = tx.tx.sender.to_u256();
    let key_to = tx.tx.to.to_u256();

    let account_sender = state.get(&key_sender);
    let account_to = state.get(&key_to);

    let new_account_sender = Account::new(
        key_sender,
        account_sender.balance - tx.tx.value,
        tx.tx.nonce,
    );
    let new_account_to = Account::new(key_to, account_to.balance + tx.tx.value, account_to.nonce);

    state.update(&key_sender, new_account_sender);
    state.update(&key_to, new_account_to);

    state
}

fn hash_tx(sig_args: &Tx) -> ethers::types::TxHash {
    let mut value_bytes = vec![0; 32];
    sig_args.value.to_big_endian(&mut value_bytes);

    let mut nonce_bytes = vec![0; 32];
    sig_args.nonce.to_big_endian(&mut nonce_bytes);

    let msg = [
        sig_args.sender.as_fixed_bytes().to_vec(),
        sig_args.to.as_fixed_bytes().to_vec(),
        value_bytes,
        nonce_bytes,
    ]
    .concat();

    types::TxHash::from(keccak256(msg))
}

fn verify_tx_signature(signed_tx: &SignedTx) -> anyhow::Result<()> {
    let hash = hash_tx(&signed_tx.tx).as_fixed_bytes().to_vec();
    let decoded = signed_tx.signature.parse::<types::Signature>()?;
    decoded.verify(hash, signed_tx.tx.sender)?;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    run_node().await
}

fn init_db(_path: &Path) -> Db {
    Arc::new(Mutex::new(vec![]))
}

async fn init_l1(
    private_key: String,
    http_endpoint: String,
) -> anyhow::Result<
    trollup::Trollup<ethers::middleware::SignerMiddleware<Provider<Http>, LocalWallet>>,
> {
    let node = Arc::new(Node::new_with_private_key(private_key, http_endpoint).await?);

    let l1_address: types::Address = std::env::var("TROLLUP_L1_CONTRACT")?.parse()?;
    let l1_contract = trollup::Trollup::new(l1_address, node.http_client.clone());

    Ok(l1_contract)
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
        .build(SOCKET_ADDRESS.parse::<SocketAddr>()?)
        .await?;

    println!("{}", server.local_addr().unwrap());

    let mut module = RpcModule::new(());
    module.register_method(RPC_SUBMIT_TX, move |params, _| {
        println!("received transaction! {params:?}");
        let tx: SignedTx = params.parse()?;

        if let Err(e) = verify_tx_signature(&tx) {
            println!("Received tx with invalid signature {tx:?}. Error: {e}");
        };

        let mut db = db.lock().unwrap();
        db.push(tx);
        Ok(())
    })?;

    let handle = server.start(module)?;

    Ok(handle)
}
