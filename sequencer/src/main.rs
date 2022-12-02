use std::{
    collections::HashMap,
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

mod api;
use api::*;

mod node;
use node::Node;

use l2_bindings::l2;

type Db = Arc<Mutex<Vec<SignedTx>>>;

const DB_PATH: &str = "./db";
const SOCKET_ADDRESS: &str = "127.0.0.1:38171";

impl From<SignedTx> for l2::Tx {
    fn from(tx: SignedTx) -> Self {
        Self {
            from: tx.tx.from,
            to: tx.tx.to,
            amt: tx.tx.value,
            nonce: tx.tx.nonce,
            signature: tx.signature.parse().unwrap(),
        }
    }
}

async fn run_node() -> anyhow::Result<()> {
    let db_path = Path::new(DB_PATH);
    let db = init_db(db_path);
    let rpc = init_rpc(db.clone()).await.unwrap();

    let private_key = std::env::var("ETH_PRIVATE_KEY")?;
    let http_endpoint = std::env::var("ETH_RPC_URL")?;

    task::spawn(async move {
        let l1_contract = init_l1(private_key, http_endpoint).await.unwrap();
        let mut interval = interval(Duration::from_millis(1000 * 5));

        let addr0: types::Address = "0x318A2475f1ba1A1AC4562D1541512d3649eE1131"
            .parse()
            .unwrap();
        let addr1: types::Address = "0x419978a8729ed2c3b1048b5Bba49f8599eD8F7C1"
            .parse()
            .unwrap();

        loop {
            interval.tick().await;

            let current_root = l1_contract.root().call().await.unwrap();
            println!("Current root is {}", types::H256::from(current_root));

            let state = l1_contract.current_state().call().await.unwrap();
            let state = HashMap::<types::Address, types::U256>::from([
                (addr0, state[0]),
                (addr1, state[1]),
            ]);
            println!("Current L1 state is {:?}", state);

            let txs: Vec<_> = db
                .lock()
                .unwrap()
                .drain(..)
                .filter(|tx| validate_tx(&state, tx).is_ok())
                .collect();

            let state = txs.iter().fold(state, apply_tx);
            println!("Computed L2 state is {:?}", state);
            l1_contract
                .submit_block(
                    txs.into_iter().map(|tx| tx.into()).collect(),
                    compute_root(&state).into(),
                )
                .send()
                .await
                .unwrap();
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
        SOCKET_ADDRESS
    );

    futures::future::pending().await
}

fn validate_tx(state: &HashMap<types::Address, types::U256>, tx: &SignedTx) -> anyhow::Result<()> {
    match state.get(&tx.tx.from) {
        Some(entry) if *entry >= tx.tx.value => Ok(()),
        _ => Err(anyhow::anyhow!("Insufficient balance")),
    }
}

fn apply_tx(
    mut state: HashMap<types::Address, types::U256>,
    tx: &SignedTx,
) -> HashMap<types::Address, types::U256> {
    match state.get_mut(&tx.tx.from) {
        Some(entry) if *entry >= tx.tx.value => {
            *entry -= tx.tx.value;
        }
        _ => panic!(),
    };
    *state.entry(tx.tx.to).or_insert_with(|| 0.into()) += tx.tx.value;
    state
}

fn compute_root(state: &HashMap<types::Address, types::U256>) -> types::H256 {
    let addr0: types::Address = "0x318A2475f1ba1A1AC4562D1541512d3649eE1131"
        .parse()
        .unwrap();
    let addr1: types::Address = "0x419978a8729ed2c3b1048b5Bba49f8599eD8F7C1"
        .parse()
        .unwrap();

    let mut addr0_bytes = vec![0; 32];
    state[&addr0].to_big_endian(&mut addr0_bytes);

    let mut addr1_bytes = vec![0; 32];
    state[&addr1].to_big_endian(&mut addr1_bytes);

    keccak256([addr0_bytes, addr1_bytes].concat()).into()
}

fn hash_tx(sig_args: &Tx) -> ethers::types::TxHash {
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

fn verify_tx_signature(signed_tx: &SignedTx) -> anyhow::Result<()> {
    let hash = hash_tx(&signed_tx.tx).as_fixed_bytes().to_vec();
    let decoded = signed_tx.signature.parse::<types::Signature>()?;
    decoded.verify(hash, signed_tx.tx.from)?;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    run_node().await
}

fn init_db(path: &Path) -> Db {
    Arc::new(Mutex::new(vec![]))
}

async fn init_l1(
    private_key: String,
    http_endpoint: String,
) -> anyhow::Result<l2::L2<ethers::middleware::SignerMiddleware<Provider<Http>, LocalWallet>>> {
    let node = Arc::new(Node::new_with_private_key(private_key, http_endpoint).await?);

    let l2_address: types::Address = std::env::var("TROLLUP_L1_CONTRACT")?.parse()?;
    let l2_contract = l2::L2::new(l2_address, node.http_client.clone());

    Ok(l2_contract)
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
        println!("received transaction! {:?}", params);
        let tx: SignedTx = params.parse()?;

        verify_tx_signature(&tx)?;

        let mut db = db.lock().unwrap();
        db.push(tx);
        Ok(())
    })?;

    let handle = server.start(module)?;

    Ok(handle)
}
