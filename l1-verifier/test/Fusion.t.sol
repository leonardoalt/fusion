// SPDX-License-Identifier: GPL3
pragma solidity ^0.8.17;

import "forge-std/Test.sol";

import "src/Fusion.sol";

contract FusionTest is Test {
    Fusion fusion;

    function setUp() public {
        fusion = new Fusion();
    }

    function testProof1() public {
        Fusion.Proof memory proof;
        proof.a.X = 0x1e1a6fd6fa4907c1f9a14c0e4f7f7b084cb665be6b4f1bdd846a8213228291ca;
        proof.a.Y = 0x058e3962e5fca5ceed3271a1f4c44e7304abc60d2e7162a662b8e7f3f5083fbe;
        proof.b.X = [
            0x209d00968e77938ddaaa56eeb738261652544aa688d8502846090413f1e3152d,
            0x128319844ec9652e69f529672b71005ef527c262e4f18834b07e260a24e58623
        ];
        proof.b.Y = [
            0x2123d822a1d4ac866c4fb397010e1d9228f143c9836e11c56f92003b71676148,
            0x212d6191dcfb56e897f8e6c94b008a5a363b12efca0150bc3e7deabcf3288523
        ];
        proof.c.X = 0x206cddb28a1fd1359f266bc731951fcd2acba276cf11aa9221d3cd3dfe60d995;
        proof.c.Y = 0x11eb4225015668817fd7a724d6f4cdf593cf9eaebb5137e4f3cf402443735c98;

        uint256[8] memory input = [
            uint256(0x0000000000000000000000000000000000000000000000000000000000000000),
            0x25b50f972e6f311960bfbc78176157aacee4ed5a45938c2e1b4c0585b4405386,
            0x2df8c0ab66dc3db17505f16c0b53554ee08f3b301a6299fad988b8a8a487e08a,
            0x0000000000000000000000000000000000000000000000000000000000000001,
            0x0000000000000000000000000000000000000000000000000000000000000000,
            0x25b50f972e6f311960bfbc78176157aacee4ed5a45938c2e1b4c0585b4405386,
            0x0000000000000000000000000000000000000000000000000000000000000000,
            0x0000000000000000000000000000000000000000000000000000000000000000
        ];

        assertEq(fusion.root(), 0);
        fusion.submitBlock([Fusion.TxProof(proof, staticToDynArray(input))]);
        assertEq(fusion.root(), 17055482318342999599154275252646722421924952339552330539432664958755143701382);
    }

    function staticToDynArray(uint256[8] memory input) internal pure returns (uint256[] memory output) {
        output = new uint[](input.length);
        for (uint256 i = 0; i < input.length; ++i) {
            output[i] = input[i];
        }
    }
}
