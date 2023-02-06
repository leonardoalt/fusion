pragma solidity ^0.8.0;

import {Verifier} from "src/Verifier.sol";

uint8 constant BLOCK_SIZE = 1;

contract Trollup is Verifier {
    uint256 public root;

    error InvalidPreRoot();
    error InvalidInputLength();
    error InvalidSNARK();

    struct TxProof {
        Proof proof;
        uint256[] input;
    }

    function submitBlock(TxProof[BLOCK_SIZE] memory l2Block) external {
        // The proof's pre-state's root must be the current root in this contract.
        if (root != l2Block[0].input[0]) {
            revert InvalidPreRoot();
        }

        for (uint256 i = 0; i < BLOCK_SIZE; ++i) {
            verifyTx(l2Block[i]);
        }

        // Update the canonical root with the proof's post-state's root.
        root = l2Block[l2Block.length - 1].input[1];
    }

    function verifyTx(TxProof memory l2Tx) internal view {
        if (l2Tx.input.length != 20) {
            revert InvalidInputLength();
        }

        // A valid SNARK returns 0.
        if (verify(l2Tx.input, l2Tx.proof) != 0) {
            revert InvalidSNARK();
        }
    }
}
