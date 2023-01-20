# Trollup - the simplest zkRollup

Trollup is an experimental Ethereum ZKRollup, created with the goal of being
the simplest L2 that can be used in production. Users can make L2 transfers, as
well as enter/exit the L2 via the L1 contract (not yet implemented).

Trollup consists of an L2 sequencer, a ZK prover, and an L1 verifier smart
contract. The sequencer keeps track of the canonical L2 state, and receives
transactions to be included in L2 blocks. The prover takes these signed
transactions, the old state and the new state, and builds a Zero Knowledge
Proof (ZKP) that the state was changed accordingly, and that the transaction
signature matches the sender. The verifier smart contract is deployed on
Ethereum and provides the canonical L2 state root. It takes ZKPs of L2
transactions, verifies them, and updates the L2 state root accordingly.

## Tool Suite

This repository contains the entire Trollup tool suite:

- `circuits`: the SNARK state and signature verification ZoKrates code.
- `trollup-prover`: the prover that takes a signed transaction and builds a ZKP of
  state changes and signature.
- `trollup-sequencer`: the Trollup node. Receives L2 transactions via RPC, builds
  blocks, and sends them for verification on L1.
- `l1-verifier`: the Trollup contracts deployed on L1. These contracts provide
  block verification and canonical state root updates for L2 nodes.
- `trollup-wallet`: a simple CLI interface to sign/send Trollup transactions.

## Cloning the repository

```
git clone --recurse-submodules https://github.com/trollup/trollup.git
```

If you already cloned the repository without submodules, you can run the command below to initialize it:
```
git submodule update --init --recursive
```

## Building and Run

Trollup requires the following installed:
- [Rust](https://www.rust-lang.org/learn/get-started)
- [foundry](https://github.com/foundry-rs/foundry)
- [Zokrates](https://zokrates.github.io)

### Building the circuits
```
cd circuits && make
```

**Note:** the current circuits require at least 32GB of RAM to compile.

### Building sequencer
```
cargo build --release --bin trollup-sequencer
```

### Running

1. Set `ETH_PRIVATE_KEY` and `ETH_FROM` to the private and public keys that will deploy and use the L1 contract.
2. Set `ETH_RPC_URL` to an Ethereum RPC endpoint. Since we are using `anvil` here, this is usually `http://localhost:8545`.
3. Run `source ./scripts/run_anvil_and_deploy_contract` which starts `anvil` and deploys the contract.
4. Run `./scripts/run_node` to start the node.
5. You can run `./scripts/listen_to_node` to check the ongoing output from the node.
6. Now you can also run `./scripts/send_random_tx` to send transactions.
7. To stop everything, run `./scripts/kill_node` and `./scripts/kill_anvil`.

## State

The state is a balanced Sparse Merkle Tree similar to [this one](https://github.com/nervosnetwork/sparse-merkle-tree).
The tree has 256 levels besides the root. This implies that it has 2^256
leaves. Each leaf has an index, from 0 (left most) to 2^256 - 1 (right most).
The path from the root to a leaf can be represented by the binary
representation of the leaf's index: when walking down the tree, 0 means the
path goes left, 1 means it goes right.

The tree also contains some optimizations, such as:

- For the sparse branches of tree, we keep all nodes as 0, instead of hash(hash(...(0))) (such as in the beacon chain [deposit contract](https://github.com/axic/eth2-deposit-contract/)).
- When merging two nodes, we apply:
    - merge(0, 0) = 0
    - merge(L, 0) = L, if L != 0
    - merge(0, R) = R, if R != 0
    - merge(L, R) = hash(L, R), if L != 0 and R != 0
- The rules above may lead to collisions if the contents of two leaves are the same. To avoid that, we hash the leaf's contents together with its unique index.

The used hash is Poseidon in order to be SNARK friendly.

## Signature and Addresses

Since we need to verify signatures inside zkSNARKs, we use EdDSA with the [Baby Jubjub Elliptic Curve](https://eips.ethereum.org/EIPS/eip-2494).
A public key (PK) consists of a curve point where `x` and `y` are elements of
the field used by Baby Jubjub. The PK can be compressed into 256 bits, a
Trollup address.  This means that Trollup accounts are not compatible with
Ethereum accounts.

## Execution

There are no VM and smart contracts at the moment.

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

### TODO

- [ ] L2 enter via L1 deposit
- [ ] L2 exit via L1 withdraw
- [ ] Compress transaction proofs into a batch proof
- [ ] Separate mempool from sequencer
- [ ] Reconstruct L2 state from scratch by re-playing block submissions to L1 verifier
- [ ] Multi-key Merkle proofs
