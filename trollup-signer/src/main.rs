use std::{sync::Arc, time::Duration};

use clap::{Parser, Subcommand};
use ethers::{
    core::k256::SecretKey,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer},
    types,
    utils::keccak256,
};
use trollup_api::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.sub {
        Subcommands::Sign(sig_args) => {
            let signature = sign(sig_args).await?;
            println!("{signature}");
            Ok(())
        }
        Subcommands::Send(send_args) => send(send_args).await,
    }
}

async fn sign(sig_args: CLITx) -> anyhow::Result<types::Signature> {
    let wallet: LocalWallet = SecretKey::from_be_bytes(sig_args.private_key.as_bytes())
        .expect("invalid private key")
        .into();

    let hash = hash_tx(&sig_args.into()).as_fixed_bytes().to_vec();
    let signature = wallet.sign_message(hash.clone()).await?;

    Ok(signature)
}

async fn send(send_args: CLITx) -> anyhow::Result<()> {
    let signed: SignedTx = if send_args.signature.is_some() {
        send_args.clone().into()
    } else {
        SignedTx {
            tx: send_args.clone().into(),
            signature: sign(send_args).await?.to_string(),
        }
    };

    verify_tx_signature(&signed)?;

    let provider =
        Provider::<Http>::try_from(SERVER_ADDRESS)?.interval(Duration::from_millis(10u64));
    let client = Arc::new(provider);
    client.request(RPC_SUBMIT_TX, signed).await?;

    Ok(())
}

impl From<CLITx> for Tx {
    fn from(tx: CLITx) -> Self {
        Self {
            sender: tx.sender,
            to: tx.to,
            nonce: tx.nonce,
            value: tx.value,
        }
    }
}

impl From<CLITx> for SignedTx {
    fn from(tx: CLITx) -> Self {
        Self {
            tx: Tx {
                sender: tx.sender,
                to: tx.to,
                nonce: tx.nonce,
                value: tx.value,
            },
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
    #[clap(about = "Sign a trollup transaction.")]
    Sign(CLITx),
    #[clap(about = "Send trollup transaction, optionally sign it before.")]
    Send(CLITx),
}

#[derive(Debug, Clone, Parser, Default)]
pub struct CLITx {
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
        value_name = "SENDER_ADDRESS",
        help = "The address of the sender address.",
        default_value = "0x0000000000000000000000000000000000000000"
    )]
    pub sender: ethers::types::Address,
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
        short = 'v',
        value_name = "VALUE",
        help = "The value of the transaction.",
        default_value = "0"
    )]
    pub value: ethers::types::U256,
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
        short = 's',
        value_name = "SIGNATURE",
        help = "The signed transaction."
    )]
    pub signature: Option<String>,
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
