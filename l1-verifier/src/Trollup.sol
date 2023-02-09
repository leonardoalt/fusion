pragma solidity ^0.8.0;

import {Verifier} from "src/Verifier.sol";

uint8 constant BLOCK_SIZE = 1;

contract Trollup is Verifier {
    uint256 public root;

    error InvalidPreRoot();
    error InvalidInputLength();
    error InvalidSNARK();
    error InvalidTransactionType();
    error InvalidDeposit();
    error DepositAmountTooLow();
    error DepositAmountNotAvailable();

    // Proof: zk proof (see Verifier.sol)
    // Inputs:
    //  0: preRoot
    //  1: postRoot
    //  2: tx.sender.x
    //  3: tx.sender.y
    //  4: tx.to.x
    //  5: tx.to.y
    //  6: tx.nonce
    //  7: tx.value
    //  8: tx.sig.r.x
    //  9: tx.sig.r.y
    // 10: tx.sig.s
    // FIXME: maybe it is better to just infer the tx type based on the sender/receiver, e.g. 0x000 -> 0xabc
    // 11: tx.kind
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

    struct Deposit {
        uint256 available; // amount of tokens in ETH available to be minted
        uint256 minted; // amount of tokens in ETH minted on L2
    }

    // map of L2 recipient address to L1 deposits
    mapping(uint256 => Deposit) public deposits;

    event NewDeposit(
        address indexed L1sender,
        uint256 indexed L2recipient,
        uint256 amount
    );

    function deposit(uint256 l2Recipient) external payable {
        if (msg.value == 0) revert DepositAmountTooLow();
        deposits[l2Recipient].available += msg.value;
        emit NewDeposit(msg.sender, l2Recipient, msg.value);
    }

    function submitBlock(TxProof[BLOCK_SIZE] memory l2Block) external {
        // The proof's pre-state's root must be the current root in this contract.
        if (root != l2Block[0].input[0]) {
            revert InvalidPreRoot();
        }

        for (uint256 i = 0; i < BLOCK_SIZE; ++i) {
            verifyTx(l2Block[i]);
        }

        // If we have a valid block, process all transactions
        for (uint256 i = 0; i < BLOCK_SIZE; ++i) {
            processTx(l2Block[i]);
        }

        // Update the canonical root with the proof's post-state's root.
        root = l2Block[l2Block.length - 1].input[1];
    }

    function verifyTx(TxProof memory l2Tx) internal view {
        if (l2Tx.input.length != 18) {
            revert InvalidInputLength();
        }

        // A valid SNARK returns 0.
        if (verify(l2Tx.input, l2Tx.proof) != 0) {
            revert InvalidSNARK();
        }
    }

    function processTx(TxProof memory l2Tx) internal {
        uint256 txKind = l2Tx.input[11];
        if (txKind == 0) {
            // L2 transfer
            /* nothing to do on L1 */
        } else if (txKind == 1) {
            // L1 deposit
            uint256 l2TxValue = l2Tx.input[7];
            uint256 l2Recipient = l2Tx.input[15];
            if (deposits[l2Recipient].available == 0 || l2TxValue == 0) revert InvalidDeposit();
            if (l2TxValue > deposits[l2Recipient].available) revert DepositAmountNotAvailable();
            deposits[l2Recipient].available -= l2TxValue;
            deposits[l2Recipient].minted += l2TxValue;
        } else if (txKind == 2) {
            // L1 withdraw (TODO)
            // check if there is sufficient minted tokens to be withdraw from L2 sender
            // decrement value from minted amount
            // transfer value to L1 recipient
        } else revert InvalidTransactionType();
    }
}
