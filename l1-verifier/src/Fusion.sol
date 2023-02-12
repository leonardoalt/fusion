pragma solidity ^0.8.0;

import {Verifier} from "src/Verifier.sol";

uint8 constant BLOCK_SIZE = 1;

contract Fusion is Verifier {
    uint256 public root;
    mapping(uint256 => uint256) public deposits;

    error InvalidPreRoot();
    error InvalidInputLength();
    error InvalidSNARK();
    error InvalidTransactionType();
    error InvalidL1Address();
    error DepositAmountTooLow();
    error DepositAmountNotAvailable();

    // Proof: zk proof (see Verifier.sol)
    // Inputs:
    //  0: preRoot
    //  1: postRoot
    //  2: tx.kind
    //  3: tx.sender.x
    //  4: tx.sender.y
    //  5: tx.to.x
    //  6: tx.to.y
    //  7: tx.nonce
    //  8: tx.value
    //  8: tx.sig.r.x
    // 10: tx.sig.r.y
    // 11: tx.sig.s
    // 12: sender.id
    // 13: sender.balance
    // 14: sender.nonce
    // 15: to.id
    // 16: to.balance
    // 17: to.nonce
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

    function deposit(uint256 l2Recipient) external payable {
        if (msg.value == 0) revert DepositAmountTooLow();
        deposits[l2Recipient] += msg.value;
    }

    function verifyTx(TxProof memory l2Tx) internal {
        if (l2Tx.input.length != 18) {
            revert InvalidInputLength();
        }

        verifyAndProcessDeposit(l2Tx);

        // A valid SNARK returns 0.
        if (verify(l2Tx.input, l2Tx.proof) != 0) {
            revert InvalidSNARK();
        }
    }

    function verifyAndProcessDeposit(TxProof memory l2Tx) internal {
        uint256 txKind = l2Tx.input[2];
        if (txKind == 0) {
            // L2 transfer, do nothing
        } else if (txKind == 1) {
            uint256 l2Value = l2Tx.input[8];
            uint256 l2Recipient = l2Tx.input[12];

            if (deposits[l2Recipient] < l2Value) revert DepositAmountNotAvailable();

            unchecked {
                deposits[l2Recipient] -= l2Value;
            }
        } else if (txKind == 2) {
            uint256 l2Value = l2Tx.input[8];
            uint256 l1Recipient = l2Tx.input[15];

            address l1Address = address(uint160(l1Recipient));
            if (l1Recipient != uint160(l1Address)) revert InvalidL1Address();

            payable(l1Address).transfer(l2Value);
        } else {
            revert InvalidTransactionType();
        }
    }
}
