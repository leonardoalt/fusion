// SPDX-License-Identifier: GPL3
pragma solidity ^0.8.17;

struct Tx {
    address from;
    address to;
    uint256 amt;
    uint256 nonce;
    bytes signature;
}

enum TransferKind {
    Deposit,
    Withdraw
}

struct Transfer {
    TransferKind kind;
    address from;
    uint256 amt;
}

contract Verifier {
    mapping(address => uint256) state;
    bytes32 public root;

    address constant addr0 = 0x318A2475f1ba1A1AC4562D1541512d3649eE1131;
    address constant addr1 = 0x419978a8729ed2c3b1048b5Bba49f8599eD8F7C1;

    Transfer[] pendingL1Transfers;

    error InvalidSignature();
    error InvalidStateRoot();

    constructor() {
        // Initialize balances with something for
        // easier L2 e2e tests.
        state[addr0] = 1_000_000;
        state[addr1] = 1_000_000;
        updateRoot();
    }

    //////////////////////////////
    // State mutating functions //
    //////////////////////////////

    function deposit() external payable {
        require(msg.value > 0);
        pendingL1Transfers.push(Transfer({kind: TransferKind.Deposit, from: msg.sender, amt: msg.value}));
    }

    function withdraw(uint256 amt) external {
        pendingL1Transfers.push(Transfer({kind: TransferKind.Withdraw, from: msg.sender, amt: amt}));
    }

    function submitBlock(Tx[] memory _tx, bytes32 _newRoot) external {
        for (uint i = 0; i < _tx.length; ++i) {
            if (!isValidSignature(_tx[i])) {
                revert InvalidSignature();
            }
            updateState(_tx[i]);
        }
        updateRoot();

        if (_newRoot != root) {
            revert InvalidStateRoot();
        }

        applyL1PendingTransfers();
    }

    function updateState(Tx memory _tx) internal {
        state[_tx.from] -= _tx.amt;
        state[_tx.to] += _tx.amt;
    }

    function applyL1PendingTransfers() internal {
        for (uint256 i = 0; i < pendingL1Transfers.length; ++i) {
            applyL1Transfer(pendingL1Transfers[i]);
        }
        delete pendingL1Transfers;
        updateRoot();
    }

    function applyL1Transfer(Transfer storage transfer) internal {
        if (transfer.kind == TransferKind.Deposit) {
            state[transfer.from] += transfer.amt;
        } else {
            state[transfer.from] -= transfer.amt;
            payable(transfer.from).transfer(transfer.amt);
        }
    }

    function updateRoot() internal {
        root = keccak256(abi.encodePacked(state[addr0], state[addr1]));
    }

    /////////////////////////////
    // View and pure functions //
    /////////////////////////////

    function currentState() public view returns (uint256[2] memory) {
        return [state[addr0], state[addr1]];
    }

    function messageHash(Tx memory _tx) internal pure returns (bytes32) {
        return keccak256(abi.encodePacked(_tx.from, _tx.to, _tx.amt, _tx.nonce));
    }

    function isValidSignature(Tx memory _tx) internal pure returns (bool) {
        bytes32 message = prefixed(messageHash(_tx));
        return recoverSigner(message, _tx.signature) == _tx.from;
    }

    function recoverSigner(bytes32 message, bytes memory sig) internal pure returns (address) {
        (uint8 v, bytes32 r, bytes32 s) = splitSignature(sig);

        return ecrecover(message, v, r, s);
    }

    function splitSignature(bytes memory sig) internal pure returns (uint8 v, bytes32 r, bytes32 s) {
        require(sig.length == 65);

        assembly {
            // first 32 bytes, after the length prefix.
            r := mload(add(sig, 32))
            // second 32 bytes.
            s := mload(add(sig, 64))
            // final byte (first byte of the next 32 bytes).
            v := byte(0, mload(add(sig, 96)))
        }

        return (v, r, s);
    }

    function prefixed(bytes32 hash) internal pure returns (bytes32) {
        return keccak256(abi.encodePacked("\x19Ethereum Signed Message:\n32", hash));
    }
}
