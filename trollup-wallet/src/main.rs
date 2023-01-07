use std::{sync::Arc, time::Duration};

use clap::{Parser, Subcommand};
use ethers::{
    providers::{Http, Provider},
    types::{U256, U512},
};
use num_bigint::BigInt;

use trollup_api::*;
use trollup_types::{PrivateKey, ToU256};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.sub {
        Subcommands::New => {
            new_private_key();
            Ok(())
        }
        Subcommands::Public(public_args) => {
            new_public_key(public_args.private_key);
            Ok(())
        }
        Subcommands::Sign(sig_args) => {
            let signature = sign(sig_args).unwrap();
            println!("{signature}");
            Ok(())
        }
        Subcommands::Send(send_args) => send(send_args).await,
    }
}

fn new_private_key() {
    let key = babyjubjub_rs::new_key();
    println!("{}", key.scalar_key());
}

fn new_public_key(sk: String) {
    let key: PrivateKey = sk.into();
    let pk = U256::from_big_endian(key.0.public().compress().as_slice());
    println!("{pk}");
}

fn sign(sig_args: CLITx) -> anyhow::Result<U512> {
    trollup_signature::sign(&sig_args.clone().into(), sig_args.private_key.unwrap())
}

async fn send(send_args: CLITx) -> anyhow::Result<()> {
    let signed: SignedTx = if send_args.signature.is_some() {
        send_args.clone().into()
    } else {
        SignedTx {
            tx: send_args.clone().into(),
            signature: sign(send_args).unwrap().to_string(),
        }
    };

    trollup_signature::verify_tx_signature(&signed)?;

    let provider =
        Provider::<Http>::try_from(SERVER_ADDRESS)?.interval(Duration::from_millis(10u64));
    let client = Arc::new(provider);
    client.request(RPC_SUBMIT_TX, signed).await?;

    Ok(())
}

impl From<CLITx> for Tx {
    fn from(tx: CLITx) -> Self {
        Self {
            sender: tx.sender.to_u256(),
            to: tx.to.to_u256(),
            nonce: tx.nonce.to_u256(),
            value: tx.value.to_u256(),
        }
    }
}

impl From<CLITx> for SignedTx {
    fn from(tx: CLITx) -> Self {
        Self {
            tx: tx.clone().into(),
            signature: tx.signature.unwrap(),
        }
    }
}

const SERVER_ADDRESS: &str = "http://localhost:38171";

#[derive(Debug, Parser)]
#[clap(name = "Trollup transaction signer and sender", version = env!("CARGO_PKG_VERSION"))]
struct Opts {
    #[clap(subcommand)]
    pub sub: Subcommands,
}

#[derive(Debug, Subcommand)]
pub enum Subcommands {
    #[clap(about = "Generate a new random private key.")]
    New,
    #[clap(about = "Generate a public key from a private key.")]
    Public(CLIPublic),
    #[clap(about = "Sign a trollup transaction.")]
    Sign(CLITx),
    #[clap(about = "Send trollup transaction, optionally sign it before.")]
    Send(CLITx),
}

#[derive(Debug, Clone, Parser, Default)]
pub struct CLIPublic {
    #[clap(
        long,
        short = 'p',
        value_name = "PRIVATE_KEY",
        help = "The private key"
    )]
    pub private_key: String,
}

#[derive(Debug, Clone, Parser, Default)]
pub struct CLITx {
    #[clap(
        long,
        short = 'p',
        value_name = "PRIVATE_KEY",
        help = "The private key that signs the message"
    )]
    pub private_key: Option<String>,
    #[clap(
        long,
        short = 'f',
        value_name = "SENDER_ADDRESS",
        help = "The address of the sender address."
    )]
    pub sender: BigInt,
    #[clap(
        long,
        short = 't',
        value_name = "DEST_ADDRESS",
        help = "The address of the destination address."
    )]
    pub to: BigInt,
    #[clap(
        long,
        short = 'v',
        value_name = "VALUE",
        help = "The value of the transaction.",
        default_value = "0"
    )]
    pub value: BigInt,
    #[clap(
        long,
        short = 'n',
        value_name = "NONCE",
        help = "The nonce of the transaction.",
        default_value = "0"
    )]
    pub nonce: BigInt,
    #[clap(
        long,
        short = 's',
        value_name = "SIGNATURE",
        help = "The signed transaction."
    )]
    pub signature: Option<String>,
}
