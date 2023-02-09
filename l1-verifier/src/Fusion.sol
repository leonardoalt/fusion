pragma solidity ^0.8.0;

import {Verifier} from "src/Verifier.sol";

contract Fusion is Verifier {
    uint256 public root;
    mapping(uint256 => uint256) public deposits;

    error InvalidPreRoot();
    error InvalidSNARK();
    error InvalidTransactionType();
    error InvalidL1Address();
    error DepositAmountTooLow();
    error DepositAmountNotAvailable();

    struct FusionPoint {
        uint256 x;
        uint256 y;
    }

    struct FusionSignature {
        FusionPoint r;
        uint256 s;
    }

    struct Account {
        uint256 id;
        uint256 balance;
        uint256 nonce;
    }

    struct TxInfo {
        uint256 preRoot;
        uint256 postRoot;
        uint256 kind;
        FusionPoint senderPoint;
        FusionPoint toPoint;
        uint256 nonce;
        uint256 value;
        FusionSignature sig;
        Account senderAcc;
        Account toAcc;
    }

    struct BatchProof {
        Proof proof;
        TxInfo[] txs;
    }

    function submitBlock(BatchProof memory l2Block) external {
        // The proof's pre-state's root must be the current root in this contract.
        if (root != l2Block.txs[0].preRoot) {
            revert InvalidPreRoot();
        }

        verifyBatch(l2Block);

        for (uint256 i = 0; i < l2Block.txs.length; ++i) {
            verifyTx(l2Block.txs[i]);
        }

        // Update the canonical root with the proof's post-state's root.
        root = l2Block.txs[l2Block.txs.length - 1].postRoot;
    }

    function deposit(uint256 l2Recipient) external payable {
        if (msg.value == 0) revert DepositAmountTooLow();
        deposits[l2Recipient] += msg.value;
    }

    function verifyBatch(BatchProof memory l2Block) internal pure {
        // A valid SNARK returns 0.
        if (!verify(l2Block.proof)) {
            revert InvalidSNARK();
        }
    }

    function verifyTx(TxInfo memory l2Tx) internal {
        if (l2Tx.kind == 0) {
            // L2 transfer, do nothing
        } else if (l2Tx.kind == 1) {
            uint256 l2Recipient = l2Tx.senderAcc.id;

            if (deposits[l2Recipient] < l2Tx.value) revert DepositAmountNotAvailable();

            unchecked {
                deposits[l2Recipient] -= l2Tx.value;
            }
        } else if (l2Tx.kind == 2) {
            uint256 l1Recipient = l2Tx.toAcc.id;

            address l1Address = address(uint160(l1Recipient));
            if (l1Recipient != uint160(l1Address)) revert InvalidL1Address();

            payable(l1Address).transfer(l2Tx.value);
        } else {
            revert InvalidTransactionType();
        }
    }
}
