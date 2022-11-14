use std::{path::Path, net::SocketAddr, sync::{Arc, Mutex}, time::Duration};

use ethers::{providers::{Provider, Http}, abi::Address};
use hyper::Method;
use jsonrpsee::{server::{ServerBuilder, AllowHosts, ServerHandle}, RpcModule};
use serde::{Serialize, Deserialize};
use tokio::{task, time::interval};
use tower_http::cors::{CorsLayer, Any};

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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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

fn init_db(path: &Path) -> Db {
    Arc::new(Mutex::new(vec![]))
}

fn init_l1(db: Db) -> Provider<Http> {
    Provider::<Http>::try_from(
        "https://mainnet.infura.io/v3/YOUR_API_KEY"
    ).unwrap()
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
