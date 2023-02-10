use tarpc::{client, context, tokio_serde::formats::Json};

use clap::{Parser, Subcommand};
use num_bigint::BigInt;
use std::net::IpAddr;

use trollup_api::*;
use trollup_config::Config;
use trollup_types::ToU256;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_file("../trollup.toml".to_string());

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
        Subcommands::Sign(cli_tx) => {
            let signature =
                trollup_wallet::sign(&cli_tx.clone().into(), cli_tx.private_key.unwrap()).unwrap();
            println!("{signature}");
            Ok(())
        }
        Subcommands::Send {
            send_sub: SendSubcommands::Transfer(cli_tx),
        } => {
            let tx: TransferTx = cli_tx.clone().into();
            let signed_tx = SignedTx {
                tx: tx.clone().0,
                signature: match cli_tx.signature {
                    Some(sig) => sig,
                    None => trollup_wallet::sign(&tx.0, cli_tx.private_key.unwrap())
                        .unwrap()
                        .to_string(),
                },
            };
            send(signed_tx, &config).await
        }
        Subcommands::Send {
            send_sub: SendSubcommands::Withdraw(_cli_tx),
        } => Ok(()),
        Subcommands::Verify(args) => {
            trollup_wallet::verify_tx_signature(&args.into()).unwrap();
            Ok(())
        }
    }
}

async fn send(tx: SignedTx, config: &Config) -> anyhow::Result<()> {
    trollup_wallet::verify_tx_signature(&tx)?;

    let server_addr = (
        IpAddr::V4(config.socket_address.parse().unwrap()),
        config.socket_port,
    );
    let transport = tarpc::serde_transport::tcp::connect(server_addr, Json::default);
    let client = TrollupRPCClient::new(client::Config::default(), transport.await?).spawn();
    client
        .submit_transaction(context::current(), tx)
        .await
        .unwrap()
        .unwrap();

    Ok(())
}

#[derive(Debug, Clone)]
pub struct TransferTx(pub Tx);

impl From<CLITx> for TransferTx {
    fn from(cli_tx: CLITx) -> Self {
        let tx = Tx {
            sender: cli_tx.sender.to_u256(),
            to: cli_tx.to.to_u256(),
            nonce: cli_tx.nonce.to_u256(),
            value: cli_tx.value.to_u256(),
            kind: TxKind::Transfer,
        };
        Self(tx)
    }
}

impl From<CLITx> for Tx {
    fn from(cli_tx: CLITx) -> Self {
        Self {
            sender: cli_tx.sender.to_u256(),
            to: cli_tx.to.to_u256(),
            nonce: cli_tx.nonce.to_u256(),
            value: cli_tx.value.to_u256(),
            kind: cli_tx.kind.unwrap().into(),
        }
    }
}

impl From<CLITx> for SignedTx {
    fn from(cli_tx: CLITx) -> Self {
        Self {
            tx: cli_tx.clone().into(),
            signature: cli_tx.signature.unwrap(),
        }
    }
}

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
    Send {
        #[clap(subcommand)]
        send_sub: SendSubcommands,
    },
    #[clap(about = "Verify transaction signature.")]
    Verify(CLITx),
}

#[derive(Debug, Subcommand)]
pub enum SendSubcommands {
    #[clap(about = "Send an L2 transfer.")]
    Transfer(CLITx),
    #[clap(about = "Withdraw from the L2 into Ethereum L1.")]
    Withdraw(CLITx),
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
    #[clap(long, short = 'k', value_name = "KIND", help = "The transaction kind.")]
    pub kind: Option<u8>,
    #[clap(
        long,
        short = 's',
        value_name = "SIGNATURE",
        help = "The signed transaction."
    )]
    pub signature: Option<String>,
}
