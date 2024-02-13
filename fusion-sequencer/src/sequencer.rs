use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use ethers::{
    providers::{Http, Provider},
    signers::LocalWallet,
};

use tokio::sync::mpsc;

use fusion_api::*;
use fusion_config::Config;
//use fusion_l1::fusion;
use fusion_state::state::{Account, State};
use fusion_state::*;
use fusion_types::PublicKey;

use crate::node::*;

type MemPool = Arc<Mutex<Vec<SignedTx>>>;

async fn request_proof(
    config: Config,
    tx: SignedTx,
    pre_state: State,
    post_state: State,
//) -> anyhow::Result<fusion::TxProof, String> {
) -> anyhow::Result<(), String> {
    //Prover::prove(&config, &tx, &pre_state, &post_state)
    Prover::prove()
}

/*
pub async fn run_sequencer(
    config: &Config,
    mut rx: mpsc::Receiver<SignedTx>,
) -> anyhow::Result<()> {
    let db_path = Path::new(&config.database_path);
    let mempool = init_mempool(db_path);

    let mut state = State::default();
    let l1_contract = init_l1(config).await.unwrap();

    while let Some(tx) = rx.recv().await {
        let current_root = l1_contract.root().call().await.unwrap();
        println!("Current root is {current_root}");

        {
            let mut unlocked_mempool = mempool.lock().unwrap();
            unlocked_mempool.push(tx);
            if unlocked_mempool.len() < config.min_tx_block {
                continue;
            }
        }

        let txs: Vec<_> = mempool
            .lock()
            .unwrap()
            .drain(..)
            .filter(|tx| validate_tx(&state, tx).is_ok())
            .collect();

        let mut states = vec![state.clone()];
        for tx in &txs {
            states.push(apply_tx(states.last().unwrap().clone(), &tx.tx));
        }

        state = states.last().unwrap().clone();
        println!("Computed L2 state root is {:?}", state.root());

        println!("Requesting {} proof(s)...", states.len() - 1);
        let mut tasks = vec![];
        states.windows(2).zip(txs.iter()).for_each(|(states, tx)| {
            tasks.push(tokio::spawn(request_proof(
                config.clone(),
                tx.clone(),
                states[0].clone(),
                states[1].clone(),
            )))
        });

        let mut proofs = vec![];
        for task in tasks {
            proofs.push(task.await.unwrap());
        }

        println!("Received all proofs.");
        for proof in proofs {
            match proof {
                Err(e) => println!("Could not generate proof: {e}"),
                Ok(proof) => {
                    println!("Submiting block");
                    l1_contract
                        .submit_block([proof])
                        .gas(1000000)
                        .send()
                        .await
                        .unwrap();
                    println!("Block sent!");
                }
            };
        }
    }

    Ok(())
}
*/

fn validate_tx(state: &State, tx: &SignedTx) -> anyhow::Result<()> {
    verify_tx_signature(tx)?;

    let sender_addr = tx.tx.sender.address();

    let account = state.get(&sender_addr);
    if matches!(tx.tx.kind, TxKind::Transfer) && tx.tx.sender == tx.tx.to {
        Err(anyhow::anyhow!("Tx to self"))
    } else if matches!(tx.tx.kind, TxKind::Transfer | TxKind::Withdraw)
        && account.balance < tx.tx.value
    {
        Err(anyhow::anyhow!("Insufficient balance"))
    } else if account.nonce >= tx.tx.nonce {
        Err(anyhow::anyhow!("Nonce too low"))
    } else {
        Ok(())
    }
}

fn apply_tx(mut state: State, tx: &Tx) -> State {
    let sender_addr = tx.sender.address();

    let to_addr = tx.to.address();

    let account_sender = state.get(&sender_addr);
    let account_to = state.get(&to_addr);

    let new_account_sender = match tx.kind {
        TxKind::Deposit => Account::new(sender_addr, account_sender.balance + tx.value, tx.nonce),
        TxKind::Transfer | TxKind::Withdraw => {
            Account::new(sender_addr, account_sender.balance - tx.value, tx.nonce)
        }
    };
    let new_account_to = match tx.kind {
        TxKind::Transfer => Account::new(to_addr, account_to.balance + tx.value, account_to.nonce),
        TxKind::Withdraw | TxKind::Deposit => account_to,
    };

    state.update(&sender_addr, new_account_sender);
    state.update(&to_addr, new_account_to);

    state
}

fn verify_tx_signature(signed_tx: &SignedTx) -> anyhow::Result<()> {
    Ok(())
    //fusion_wallet::verify_tx_signature(signed_tx)
}

fn init_mempool(_path: &Path) -> MemPool {
    Arc::new(Mutex::new(vec![]))
}

/*
async fn init_l1(
    config: &Config,
) -> anyhow::Result<fusion::Fusion<ethers::middleware::SignerMiddleware<Provider<Http>, LocalWallet>>>
{
    let node = Arc::new(
        Node::new_with_private_key(config.eth_private_key.clone(), config.eth_rpc_url.clone())
            .await?,
    );

    let l1_contract = fusion::Fusion::new(config.fusion_l1_contract, node.http_client.clone());

    Ok(l1_contract)
}
*/

#[cfg(test)]
mod test {
    /*
    use super::*;

    use anvil::{spawn, NodeConfig};
    use ethers::abi::AbiDecode;
    use ethers::prelude::*;
    use ethers::providers::Middleware;
    use ethers::types;

    use tokio::sync::mpsc;

    use fusion_types::ToU256;
    use fusion_wallet;

    #[test]
    fn state_update_test() {
        let state = State::default();

        let (_sk_1, pk_1) = fusion_wallet::new_key_pair();
        let (_sk_2, pk_2) = fusion_wallet::new_key_pair();

        let tx_1 = fusion_api::Tx {
            kind: TxKind::Deposit,
            sender: pk_1.clone().to_u256(),
            to: 0.into(),
            nonce: 1.into(),
            value: 1000.into(),
        };

        let tx_2 = fusion_api::Tx {
            kind: TxKind::Transfer,
            sender: pk_1.clone().to_u256(),
            to: pk_2.clone().to_u256(),
            nonce: 2.into(),
            value: 500.into(),
        };

        let tx_3 = fusion_api::Tx {
            kind: TxKind::Withdraw,
            sender: pk_2.clone().to_u256(),
            to: 0.into(),
            nonce: 3.into(),
            value: 200.into(),
        };

        let state = apply_tx(state, &tx_1);
        let state = apply_tx(state, &tx_2);
        let state = apply_tx(state, &tx_3);

        let acc_1 = state.get(&pk_1.address());
        let acc_2 = state.get(&pk_2.address());

        assert_eq!(acc_1.balance, 500.into());
        assert_eq!(acc_2.balance, 300.into());

        assert_eq!(acc_1.nonce, 2.into());
        assert_eq!(acc_2.nonce, 3.into());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn end_to_end_simple() {
        let anvil_config = NodeConfig::test();

        let (_api, handle) = spawn(anvil_config.clone()).await;
        let provider = handle.http_provider();

        let contract = fusion::Fusion::deploy(provider.clone().into(), ())
            .unwrap()
            .gas(10000000)
            .send()
            .await
            .unwrap();

        let mut fusion_config = Config::default();
        fusion_config.eth_rpc_url = handle.http_endpoint();
        fusion_config.fusion_l1_contract = contract.address();
        let wallet = &anvil_config.genesis_accounts[0];
        fusion_config.eth_private_key = hex::encode(wallet.signer().to_bytes());
        fusion_config.min_tx_block = 1;

        assert_eq!(contract.root().call().await.unwrap(), 0.into());

        let (sx, rx): (mpsc::Sender<SignedTx>, mpsc::Receiver<SignedTx>) = mpsc::channel(1024);

        tokio::spawn(async move {
            run_sequencer(&fusion_config, rx).await.unwrap();
        });

        let n_tx = 3;

        tokio::spawn(async move {
            let (sk_1, pk_1) = fusion_wallet::new_key_pair();
            let (_sk_2, pk_2) = fusion_wallet::new_key_pair();

            for i in 0..n_tx {
                let tx = fusion_api::Tx {
                    kind: TxKind::Transfer,
                    sender: pk_1.clone().to_u256(),
                    to: pk_2.clone().to_u256(),
                    nonce: (i + 1).into(),
                    value: 0.into(),
                };
                let sig = fusion_wallet::sign(&tx, sk_1.to_string()).unwrap();
                let signed_tx = fusion_api::SignedTx {
                    tx,
                    signature: sig.to_string(),
                };
                // TODO: fix this hack somehow
                // Wait until rx starts listening.
                while sx.send(signed_tx.clone()).await.is_err() {}
            }
        });

        let mut state = State::default();
        for _ in 0..n_tx {
            let txs = next_fusion_txs(contract.address(), &provider).await;
            state = txs.iter().fold(state, apply_tx);
        }

        assert_eq!(contract.root().call().await.unwrap(), state.root());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn deposit() {
        let anvil_config = NodeConfig::test();

        let (_api, handle) = spawn(anvil_config.clone()).await;
        let provider = handle.http_provider();

        let contract = fusion::Fusion::deploy(provider.clone().into(), ())
            .unwrap()
            .gas(10000000)
            .send()
            .await
            .unwrap();

        let mut fusion_config = Config::default();
        fusion_config.eth_rpc_url = handle.http_endpoint();
        fusion_config.fusion_l1_contract = contract.address();
        let wallet = &anvil_config.genesis_accounts[0];
        fusion_config.eth_private_key = hex::encode(wallet.signer().to_bytes());
        fusion_config.min_tx_block = 1;

        assert_eq!(contract.root().call().await.unwrap(), 0.into());

        let (sx, rx): (mpsc::Sender<SignedTx>, mpsc::Receiver<SignedTx>) = mpsc::channel(1024);

        tokio::spawn(async move {
            run_sequencer(&fusion_config, rx).await.unwrap();
        });

        let contract_clone = contract.clone();
        let (sk_1, pk_1) = fusion_wallet::new_key_pair();
        let (_sk_2, pk_2) = fusion_wallet::new_key_pair();
        let pk_1_address = pk_1.address();
        let _pk_2_address = pk_2.address();

        tokio::spawn(async move {
            let deposit_amt = 1000;

            contract_clone
                .deposit(pk_1_address)
                .value(deposit_amt)
                .gas(1000000)
                .send()
                .await
                .unwrap();

            assert_eq!(
                contract_clone.deposits(pk_1_address).call().await.unwrap(),
                deposit_amt.into()
            );

            let tx = fusion_api::Tx {
                kind: TxKind::Deposit,
                sender: pk_1.clone().to_u256(),
                to: 0.into(),
                nonce: 1.into(),
                value: deposit_amt.into(),
            };
            let sig = fusion_wallet::sign(&tx, sk_1.to_string()).unwrap();
            let signed_tx = fusion_api::SignedTx {
                tx,
                signature: sig.to_string(),
            };
            // TODO: fix this hack somehow
            // Wait until rx starts listening.
            while sx.send(signed_tx.clone()).await.is_err() {}
        });

        let mut state = State::default();
        for _ in 0..1 {
            let txs = next_fusion_txs(contract.address(), &provider).await;
            state = txs.iter().fold(state, apply_tx);
        }

        let post_pk_1 = state.get(&pk_1_address);
        assert_eq!(post_pk_1.balance, 1000.into());
        assert_eq!(post_pk_1.nonce, 1.into());

        assert_eq!(
            contract.deposits(pk_1_address).call().await.unwrap(),
            0.into()
        );

        assert_eq!(contract.root().call().await.unwrap(), state.root());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn deposit_partial_claim() {
        let anvil_config = NodeConfig::test();

        let (_api, handle) = spawn(anvil_config.clone()).await;
        let provider = handle.http_provider();

        let contract = fusion::Fusion::deploy(provider.clone().into(), ())
            .unwrap()
            .gas(10000000)
            .send()
            .await
            .unwrap();

        let mut fusion_config = Config::default();
        fusion_config.eth_rpc_url = handle.http_endpoint();
        fusion_config.fusion_l1_contract = contract.address();
        let wallet = &anvil_config.genesis_accounts[0];
        fusion_config.eth_private_key = hex::encode(wallet.signer().to_bytes());
        fusion_config.min_tx_block = 1;

        assert_eq!(contract.root().call().await.unwrap(), 0.into());

        let (sx, rx): (mpsc::Sender<SignedTx>, mpsc::Receiver<SignedTx>) = mpsc::channel(1024);

        tokio::spawn(async move {
            run_sequencer(&fusion_config, rx).await.unwrap();
        });

        let contract_clone = contract.clone();
        let (sk_1, pk_1) = fusion_wallet::new_key_pair();
        let (_sk_2, pk_2) = fusion_wallet::new_key_pair();
        let pk_1_address = pk_1.address();
        let _pk_2_address = pk_2.address();

        tokio::spawn(async move {
            let deposit_amt = 1000;

            contract_clone
                .deposit(pk_1_address)
                .value(deposit_amt)
                .gas(1000000)
                .send()
                .await
                .unwrap();

            assert_eq!(
                contract_clone.deposits(pk_1_address).call().await.unwrap(),
                deposit_amt.into()
            );

            for i in 1..=2 {
                let tx = fusion_api::Tx {
                    kind: TxKind::Deposit,
                    sender: pk_1.clone().to_u256(),
                    to: 0.into(),
                    nonce: i.into(),
                    value: 300.into(),
                };
                let sig = fusion_wallet::sign(&tx, sk_1.to_string()).unwrap();
                let signed_tx = fusion_api::SignedTx {
                    tx,
                    signature: sig.to_string(),
                };
                // TODO: fix this hack somehow
                // Wait until rx starts listening.
                while sx.send(signed_tx.clone()).await.is_err() {}
            }
        });

        let mut state = State::default();
        for _ in 0..2 {
            let txs = next_fusion_txs(contract.address(), &provider).await;
            state = txs.iter().fold(state, apply_tx);
        }

        let post_pk_1 = state.get(&pk_1_address);
        assert_eq!(post_pk_1.balance, 600.into());
        assert_eq!(post_pk_1.nonce, 2.into());

        assert_eq!(
            contract.deposits(pk_1_address).call().await.unwrap(),
            400.into()
        );

        assert_eq!(contract.root().call().await.unwrap(), state.root());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn deposit_transfer_withdraw_flow() {
        let anvil_config = NodeConfig::test();

        let (_api, handle) = spawn(anvil_config.clone()).await;
        let provider = handle.http_provider();

        let contract = fusion::Fusion::deploy(provider.clone().into(), ())
            .unwrap()
            .gas(10000000)
            .send()
            .await
            .unwrap();

        let mut fusion_config = Config::default();
        fusion_config.eth_rpc_url = handle.http_endpoint();
        fusion_config.fusion_l1_contract = contract.address();
        let wallet = &anvil_config.genesis_accounts[0];
        fusion_config.eth_private_key = hex::encode(wallet.signer().to_bytes());
        fusion_config.min_tx_block = 1;

        assert_eq!(contract.root().call().await.unwrap(), 0.into());

        let (sx, rx): (mpsc::Sender<SignedTx>, mpsc::Receiver<SignedTx>) = mpsc::channel(1024);

        tokio::spawn(async move {
            run_sequencer(&fusion_config, rx).await.unwrap();
        });

        let contract_clone = contract.clone();
        let (sk_1, pk_1) = fusion_wallet::new_key_pair();
        let (sk_2, pk_2) = fusion_wallet::new_key_pair();
        let pk_1_address = pk_1.address();
        let pk_2_address = pk_2.address();
        let l1_recipient_addr = ethers::types::Address::from_low_u64_be(42);

        // L1 recipient has 0 ETH
        assert_eq!(
            provider.get_balance(l1_recipient_addr, None).await.unwrap(),
            0.into()
        );

        // Contract has 0 ETH
        assert_eq!(
            provider
                .get_balance(contract.address(), None)
                .await
                .unwrap(),
            0.into()
        );

        tokio::spawn(async move {
            let deposit_amt = 1000;

            contract_clone
                .deposit(pk_1_address)
                .value(deposit_amt)
                .gas(1000000)
                .send()
                .await
                .unwrap();

            assert_eq!(
                contract_clone.deposits(pk_1_address).call().await.unwrap(),
                deposit_amt.into()
            );

            let tx = fusion_api::Tx {
                kind: TxKind::Deposit,
                sender: pk_1.clone().to_u256(),
                to: 0.into(),
                nonce: 1.into(),
                value: 1000.into(),
            };
            let sig = fusion_wallet::sign(&tx, sk_1.to_string()).unwrap();
            let signed_tx = fusion_api::SignedTx {
                tx,
                signature: sig.to_string(),
            };
            // TODO: fix this hack somehow
            // Wait until rx starts listening.
            while sx.send(signed_tx.clone()).await.is_err() {}

            let tx = fusion_api::Tx {
                kind: TxKind::Transfer,
                sender: pk_1.clone().to_u256(),
                to: pk_2.clone().to_u256(),
                nonce: 2.into(),
                value: 600.into(),
            };
            let sig = fusion_wallet::sign(&tx, sk_1.to_string()).unwrap();
            let signed_tx = fusion_api::SignedTx {
                tx,
                signature: sig.to_string(),
            };
            // TODO: fix this hack somehow
            // Wait until rx starts listening.
            while sx.send(signed_tx.clone()).await.is_err() {}

            let tx = fusion_api::Tx {
                kind: TxKind::Withdraw,
                sender: pk_2.clone().to_u256(),
                to: l1_recipient_addr.to_low_u64_be().into(),
                nonce: 3.into(),
                value: 100.into(),
            };
            let sig = fusion_wallet::sign(&tx, sk_2.to_string()).unwrap();
            let signed_tx = fusion_api::SignedTx {
                tx,
                signature: sig.to_string(),
            };
            // TODO: fix this hack somehow
            // Wait until rx starts listening.
            while sx.send(signed_tx.clone()).await.is_err() {}
        });

        let mut state = State::default();
        for _ in 0..3 {
            let txs = next_fusion_txs(contract.address(), &provider).await;
            state = txs.iter().fold(state, apply_tx);
        }

        let post_pk_1 = state.get(&pk_1_address);
        assert_eq!(post_pk_1.balance, 400.into());
        assert_eq!(post_pk_1.nonce, 2.into());

        // All deposits were claimed.
        assert_eq!(
            contract.deposits(pk_1_address).call().await.unwrap(),
            0.into()
        );

        let post_pk_2 = state.get(&pk_2_address);
        assert_eq!(post_pk_2.balance, 500.into());
        assert_eq!(post_pk_2.nonce, 3.into());

        // L1 recipient has the 100 that were withdrawn.
        assert_eq!(
            provider.get_balance(l1_recipient_addr, None).await.unwrap(),
            100.into()
        );

        assert_eq!(contract.root().call().await.unwrap(), state.root());
    }

    async fn next_fusion_txs(contract: types::Address, provider: &Provider<Http>) -> Vec<Tx> {
        let mut stream = provider.watch_blocks().await.unwrap();
        loop {
            let blockhash = stream.next().await.unwrap();
            let block = provider
                .get_block_with_txs(blockhash)
                .await
                .unwrap()
                .unwrap();
            let fusion_txs: Vec<_> = block
                .transactions
                .iter()
                .filter(|tx| tx.to == Some(contract))
                .filter_map(|tx| decode_l2_proof(tx.input.clone()))
                .map(|tx_proof| tx_proof_to_fusion_tx(&tx_proof))
                .collect();

            if !fusion_txs.is_empty() {
                return fusion_txs;
            }
        }
    }

    fn decode_l2_proof(input: types::Bytes) -> Option<fusion::TxProof> {
        match fusion::FusionCalls::decode(input) {
            Ok(fusion::FusionCalls::SubmitBlock(fusion::SubmitBlockCall { l_2_block })) => {
                Some(l_2_block[0].clone())
            }
            _ => None,
        }
    }

    fn tx_proof_to_fusion_tx(tx_proof: &fusion::TxProof) -> Tx {
        Tx {
            kind: tx_proof.input[2].into(),
            sender: PublicKey::from_point(tx_proof.input[3], tx_proof.input[4]).to_u256(),
            to: PublicKey::from_point(tx_proof.input[5], tx_proof.input[6]).to_u256(),
            nonce: tx_proof.input[7],
            value: tx_proof.input[8],
        }
    }
    */
}
