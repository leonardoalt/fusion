pragma solidity ^0.8.0;

import {Verifier} from "src/Verifier.sol";

uint8 constant BLOCK_SIZE = 1;

contract Trollup is Verifier {
    uint256 public root;

    struct TxProof {
        Proof proof;
        uint256[] input;
    }

    function submitBlock(TxProof[BLOCK_SIZE] memory l2Block) external {
        // The proof's pre-state's root must be the current root in this contract.
        require(root == l2Block[0].input[0]);

        for (uint256 i = 0; i < BLOCK_SIZE; ++i) {
            verifyTx(l2Block[i]);
        }

        // Update the canonical root with the proof's post-state's root.
        root = l2Block[l2Block.length - 1].input[1];
    }

    function verifyTx(TxProof memory l2Tx) internal view {
        require(l2Tx.input.length == 20);

        // A valid SNARK returns 0.
        require(verify(l2Tx.input, l2Tx.proof) == 0);
    }
}
