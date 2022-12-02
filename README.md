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
