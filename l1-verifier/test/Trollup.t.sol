// SPDX-License-Identifier: GPL3
pragma solidity ^0.8.17;

import "forge-std/Test.sol";

import "src/Trollup.sol";

contract TrollupTest is Test {
    Trollup trollup;

    function setUp() public {
        trollup = new Trollup();
    }

    function testProof1() public {
        Trollup.Proof memory proof;
        proof.a.X = 0x050da18ebaa17037013098659129dbf264311e90b361ee84e457eaad058f39db;
        proof.a.Y = 0x085e8434583cefd88771570802cce4af6c96a3ac7e5780d166b66d037b6e5604;
        proof.b.X = [
            0x19587e1997085cdb84ed62776e47aa92983328e851b9114ece40eea0efc9c908,
            0x0b2699bd6a7c4279229ce0b7a99064d66b8c04e7407cbcdd1aa87889af972b4d
        ];
        proof.b.Y = [
            0x0017bfed5c2f3b21e0137b1c48a918b9dd0f1729df6bb8ec3fc948fbf65bb2be,
            0x0ac474b48615eebf3926a5c91cb7fa94a360a278672014365e654a2b046d0522
        ];
        proof.c.X = 0x1a715d2f532ef09a9f27a6ab7070ce15b969382c5337b175020441304382fc79;
        proof.c.Y = 0x14ba38880954a4668e42831ad14ad442209beed1763c2fef50e7525efdb09c6e;

        uint256[8] memory input = [
            uint256(0x0000000000000000000000000000000000000000000000000000000000000000),
            0x000000000000000000000000318a2475f1ba1a1ac4562d1541512d3649ee1131,
            0x0000000000000000000000000000000000000000000000000000000000000000,
            0x0000000000000000000000000000000000000000000000000000000000010000,
            0x0000000000000000000000000000000000000000000000000000000000000000,
            0x000000000000000000000000318a2475f1ba1a1ac4562d1541512d3649ee1131,
            0x0000000000000000000000000000000000000000000000000000000000000000,
            0x0000000000000000000000000000000000000000000000000000000000000000
        ];

        assertEq(trollup.root(), 0);
        trollup.submitBlock(proof, staticToDynArray(input));
        assertEq(trollup.root(), 282821226784406079462832848876016094274823000369);
    }

    function staticToDynArray(uint256[8] memory input) internal pure returns (uint256[] memory output) {
        output = new uint[](input.length);
        for (uint256 i = 0; i < input.length; ++i) {
            output[i] = input[i];
        }
    }
}
