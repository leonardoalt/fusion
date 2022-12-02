# Trollup

Trollup is an experimental Ethereum L2, created with the goal of being the
simplest L2 that can be used in production.

## Tool Suite

This repository contains the entire Trollup tool suite:

- `sequencer`: the Trollup node. Receives L2 transactions via RPC, builds
  blocks, and sends them for verification on L1.
- `l1-verifier`: the Trollup contracts deployed on L1. These contracts provide
  the enter/exit L2 functionalities and block verification for L2 nodes.
- `sign`: a simple CLI interface to sign Trollup transactions and/or send them
  to a node.

## Run it

Requirements: [foundry](https://github.com/foundry-rs/foundry).

1. Start an `anvil` node, and set the env variable `ETH_RPC_URL` to where `anvil` is listening - usually `http://localhost:8545`.
2. Set `ETH_PRIVATE_KEY` to the private key that will deploy the L1 contract. In `l1-verifier`, run `forge script script/Verifier.s.sol --rpc-url $ETH_RPC_URL --private-key $ETH_PRIVATE_KEY --broadcast`.
3. Take the deployed contract address (likely 0x5fbdb2315678afecb367f032d93f642f64180aa3 with `anvil`'s default configuration) and set in `TROLLUP_L1_CONTRACT`.
4. In `sequencer`, run `cargo run`.
5. In `sign`, you can use `cargo run -- send` to sign and send transactions to the Trollup node. Run `cargo run -- send -h` to see the parameters.
6. You can also send Ethereum L1 transactions to the Trollup contract to enter/exit the L2.

## What

Currently, Trollup is basically a dumb payment channel. Users can enter and
exit the L2 via the L1 contract. Users can also send and receive TrollETH via
L2 transactions.

## State

The state currently consists of two addresses,
0x318A2475f1ba1A1AC4562D1541512d3649eE1131 (A1) and
0x419978a8729ed2c3b1048b5Bba49f8599eD8F7C1 (A2). The state root is
`keccak256(A1, A2)`.

## Execution

There is no VM on the L2 at the moment.

## Roadmap

These are roughly the steps we want to achieve.

### Milestone 1 (done)

The goal is to have a working dumb L2 that only knows two addresses.  This
includes a node, ECDSA, state verification on an L1 contract.

### Milestone 2

The end goal is to have the L1 contract only store the state roots and
receive a SNARK for verification.
