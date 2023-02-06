use tarpc::{client, context, tokio_serde::formats::Json};

use clap::{Parser, Subcommand};
use ethers_core::types::U512;
use num_bigint::BigInt;
use std::net::IpAddr;

use trollup_api::*;
use trollup_types::ToU256;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.sub {
        Subcommands::New => {
            let k = trollup_wallet::new_private_key();
            println!("{}", k.to_string());
            Ok(())
        }
        Subcommands::Public(public_args) => {
            let k = trollup_wallet::new_public_key(&public_args.private_key.into()).to_u256();
            println!("{k}");
            Ok(())
        }
        Subcommands::Sign(sig_args) => {
            let signature = sign(sig_args).unwrap();
            println!("{signature}");
            Ok(())
        }
        Subcommands::Send(send_args) => send(send_args).await,
        Subcommands::Verify(args) => {
            verify(args);
            Ok(())
        }
    }
}

fn sign(sig_args: CLITx) -> anyhow::Result<U512> {
    trollup_wallet::sign(&sig_args.clone().into(), sig_args.private_key.unwrap())
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

    trollup_wallet::verify_tx_signature(&signed)?;

    let server_addr = (IpAddr::V4(SOCKET_ADDRESS.parse().unwrap()), SOCKET_PORT);
    let transport = tarpc::serde_transport::tcp::connect(server_addr, Json::default);
    let client = TrollupRPCClient::new(client::Config::default(), transport.await?).spawn();
    let tx_receipt = client
        .submit_transaction(context::current(), signed)
        .await
        .unwrap();

    println!("{tx_receipt}");

    Ok(())
}

fn verify(args: CLITx) {
    trollup_wallet::verify_tx_signature(&args.into()).unwrap();
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

const SOCKET_ADDRESS: &str = "127.0.0.1";
const SOCKET_PORT: u16 = 38171;

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
    #[clap(about = "Verify transaction signature.")]
    Verify(CLITx),
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
