# Fusion zkRollup

Fusion is an experimental progressive Ethereum zkRollup written in Rust and
focuses on performance, modularity, and applying cutting-edge Verifiable
Computation proof systems.

Fusion is conceptually based on [the original zkRollup](https://github.com/barryWhiteHat/roll_up/).
We use [Zokrates](https://zokrates.github.io) for all circuits which provides
high level abstractions for all algorithms and multiple backends.  Currently
the prover builds a Groth16 SNARK for each transaction in parallel, and there
is no batching.  Together with ZoKrates we are experimenting with
[Nova](https://github.com/microsoft/Nova) recursive proofs to enable incredibly
fast transaction batching and compression.

## Tool Suite

This repository contains the entire Fusion tool suite:

- `circuits`: the SNARK state and signature verification ZoKrates code.
- `fusion-prover`: the prover that takes a signed transaction and builds a SNARK of
  state changes and signature.
- `fusion-sequencer`: the Fusion node. Receives L2 transactions via RPC, builds
  blocks, and sends them for verification on L1.
- `l1-verifier`: the Fusion contracts deployed on L1. These contracts provide
  block verification and canonical state root updates for L2 nodes.
- `fusion-wallet`: a simple CLI interface to sign/send Fusion transactions.

## Cloning the repository

```
git clone --recurse-submodules https://github.com/leonardoalt/fusion.git
```

If you already cloned the repository without submodules, you can run the command below to initialize it:
```
git submodule update --init --recursive
```

## Dependencies

Fusion requires the following installed:
- [Rust](https://www.rust-lang.org/learn/get-started)
- [foundry](https://github.com/foundry-rs/foundry)
- [Zokrates](https://zokrates.github.io)

### Building the circuits
```
cd circuits && make
```

**Note:** the current circuits require at least 32GB of RAM to compile.

### Building the sequencer (has prover as dependency)
```
cargo build --release --bin fusion-sequencer
```

### Running

The easiest way to see everything running is via Rust tests with

```bash
cargo test --release -- --nocapture
```

For more details see the tests in `fusion-sequencer`.
If you want to run it in production style, you may want to follow this list:

1. Set `eth_private_key` in `fusion.toml` to the private key that will deploy the contract and submit L2 blocks.
2. Set `eth_rpc_url` in `fusion.toml` to an Ethereum RPC endpoint. Since we are using `anvil` here, this is usually `http://localhost:8545`.
3. Run `source ./scripts/run_anvil_and_deploy_contract` which starts `anvil` and deploys the contract.
4. Set `fusion_l1_contract` in `fusion.toml` with the address of the deployed contract.
5. Run `./scripts/run_node` to start the node.
6. You can run `./scripts/listen_to_node` to check the ongoing output from the node.
7. Now you can also run `./scripts/send_random_tx` to send transactions.
8. To stop everything, run `./scripts/kill_node` and `./scripts/kill_anvil`.

## State

The state is a balanced Sparse Merkle Tree similar to [this one](https://github.com/nervosnetwork/sparse-merkle-tree).
The tree has 256 levels besides the root. This implies that it has 2^256
leaves. Each leaf has an index, from 0 (left most) to 2^256 - 1 (right most).
The path from the root to a leaf can be represented by the binary
representation of the leaf's index: when walking down the tree, 0 means the
path goes left, 1 means it goes right.

The tree also contains some optimizations, such as:

- For the sparse branches of tree, we keep all nodes as 0, instead of `hash(hash(...(0)))` (such as in the Ethereum Beacon Chain [Deposit Contract](https://github.com/axic/eth2-deposit-contract/)).
- When merging two nodes, we apply:
    - `merge(0, 0) = 0`
    - `merge(L, 0) = L, if L != 0`
    - `merge(0, R) = R, if R != 0`
    - `merge(L, R) = hash(L, R), if L != 0 and R != 0`
- The rules above may lead to collisions if the contents of two leaves are the same. To avoid that, we hash the leaf's contents together with its unique index.

The used hash is Poseidon in order to be SNARK friendly.

## Signature and Addresses

Since we need to verify signatures inside zkSNARKs, we use EdDSA with the [Baby Jubjub Elliptic Curve](https://eips.ethereum.org/EIPS/eip-2494).
A public key (PK) consists of a curve point where `x` and `y` are elements of
the field used by Baby Jubjub. The PK can be compressed into 256 bits, which
does not necessarily fit in the field.
Given a public key `(x, y)`, its Fusion address consists of `poseidon(x, y)`.
This means that Fusion accounts are not compatible with Ethereum accounts.

## Execution

There are no VM and smart contracts at the moment.

## Modularity

Fusion is designed to be highly modular and extensible. For example, here are a
few things that can be modified quite easily:

- the sequencer's block building strategy
- the RPC node
- the L2 state data structures
- the proving backend (see branch `nova`)
- the signature algorithm

This enables fast experimental iterations on all fronts which can quickly turn
into progress.

## Roadmap (and TODO items)

### Done

- [x] L2 RPC node
- [x] L2 sequencer
- [x] State verification circuit
- [x] State verification prover
- [x] Baby Jubjub signature verification circuit
- [x] Baby Jubjub signature verification prover
- [x] Transaction verification circuit
- [x] Transaction verification prover
- [x] L1 verifier
- [x] L2 block verification using L1 verifier
- [x] Baby Jubjub CLI wallet
- [x] L2 enter via L1 deposit
- [x] L2 exit via L1 withdraw

### TODO

- [ ] Transaction fees
- [ ] Compress transaction proofs into a batch proof (in progress using Nova)
- [ ] Separate mempool from sequencer
- [ ] Reconstruct L2 state from scratch by re-playing block submissions to L1 verifier
- [ ] Multi-key Merkle proofs
- [ ] Forced transactions
- [ ] Sequencer auction
