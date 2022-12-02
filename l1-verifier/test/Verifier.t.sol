// SPDX-License-Identifier: GPL3
pragma solidity ^0.8.17;

import "forge-std/Test.sol";

import "src/Verifier.sol";

contract VerifierTest is Test {
    Verifier verifier;

    address constant addr0 = 0x318A2475f1ba1A1AC4562D1541512d3649eE1131;
    address constant addr1 = 0x419978a8729ed2c3b1048b5Bba49f8599eD8F7C1;

    function setUp() public {
        verifier = new Verifier();
    }

    ////////////////
    // Main tests //
    ////////////////

    function testDeposit() public {
        uint[2] memory preState = verifier.currentState();
        fundL2(addr0, 100);
        uint[2] memory postState = verifier.currentState();
        assertEq(postState[0], preState[0] + 100);
    }

    function testWithdraw() public {
        uint[2] memory preState = verifier.currentState();
        fundL2(addr0, 100);
        exitL2(addr0, 100);
        uint[2] memory postState = verifier.currentState();
        assertEq(postState[0], preState[0]);
    }

    function testL2Tx() public {
        fundL2(addr0, 1_000_000);

        uint[2] memory state0 = verifier.currentState();
        uint[2] memory expectedState = [ state0[0] - 0x4242, state0[1] ];

        bytes32 expectedRoot = root(expectedState);
        verifier.submitBlock(oneTx(simpleTx()), expectedRoot);
    }

    //////////////////////
    // Helper functions //
    //////////////////////

    function root(uint[2] memory state) internal pure returns (bytes32) {
        return keccak256(abi.encodePacked(state[0], state[1]));
    }

    function fundL2(address who, uint amt) internal {
        vm.startPrank(who);
        vm.deal(who, amt);
        verifier.deposit{value: amt}();
        vm.stopPrank();

        submitEmptyBlock();
    }

    function exitL2(address who, uint amt) internal {
        vm.startPrank(who);
        verifier.withdraw(amt);
        vm.stopPrank();

        submitEmptyBlock();
    }

    function oneTx(Tx memory _tx) internal pure returns (Tx[] memory) {
        Tx[] memory oTx = new Tx[](1);
        oTx[0] = _tx;
        return oTx;
    }

    function submitEmptyBlock() internal {
        Tx[] memory txBlock;
        verifier.submitBlock(txBlock, verifier.root());
    }

    function simpleTx() internal pure returns (Tx memory) {
        return Tx({from: addr0, to: address(0), amt: 0x4242, nonce: 0, signature: hex"da1dea8a04e8e0e2567c2d6217bb0d8d521619b2e9d275905e7b9db031a4dab67b489fbd42b33125da937382f985eb8086e6600b48932ff02d6f29e1482e0cc21c"});
    }
}
