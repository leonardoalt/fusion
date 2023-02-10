// This file is MIT Licensed.
//
// Copyright 2017 Christian Reitwiessner
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
pragma solidity ^0.8.0;

library Pairing {
    struct G1Point {
        uint256 X;
        uint256 Y;
    }
    // Encoding of field elements is: X[0] * z + X[1]

    struct G2Point {
        uint256[2] X;
        uint256[2] Y;
    }
    /// @return the generator of G1

    function P1() internal pure returns (G1Point memory) {
        return G1Point(1, 2);
    }
    /// @return the generator of G2

    function P2() internal pure returns (G2Point memory) {
        return G2Point(
            [
                10857046999023057135944570762232829481370756359578518086990519993285655852781,
                11559732032986387107991004021392285783925812861821192530917403151452391805634
            ],
            [
                8495653923123431417604973247489272438418190587263600148770280649306958101930,
                4082367875863433681332203403145435568316851327593401208105741076214120093531
            ]
        );
    }
    /// @return the negation of p, i.e. p.addition(p.negate()) should be zero.

    function negate(G1Point memory p) internal pure returns (G1Point memory) {
        // The prime q in the base field F_q for G1
        uint256 q = 21888242871839275222246405745257275088696311157297823662689037894645226208583;
        if (p.X == 0 && p.Y == 0) {
            return G1Point(0, 0);
        }
        return G1Point(p.X, q - (p.Y % q));
    }
    /// @return r the sum of two points of G1

    function addition(G1Point memory p1, G1Point memory p2) internal view returns (G1Point memory r) {
        uint256[4] memory input;
        input[0] = p1.X;
        input[1] = p1.Y;
        input[2] = p2.X;
        input[3] = p2.Y;
        bool success;
        assembly {
            success := staticcall(sub(gas(), 2000), 6, input, 0xc0, r, 0x60)
            // Use "invalid" to make gas estimation work
            switch success
            case 0 { invalid() }
        }
        require(success);
    }

    /// @return r the product of a point on G1 and a scalar, i.e.
    /// p == p.scalar_mul(1) and p.addition(p) == p.scalar_mul(2) for all points p.
    function scalar_mul(G1Point memory p, uint256 s) internal view returns (G1Point memory r) {
        uint256[3] memory input;
        input[0] = p.X;
        input[1] = p.Y;
        input[2] = s;
        bool success;
        assembly {
            success := staticcall(sub(gas(), 2000), 7, input, 0x80, r, 0x60)
            // Use "invalid" to make gas estimation work
            switch success
            case 0 { invalid() }
        }
        require(success);
    }
    /// @return the result of computing the pairing check
    /// e(p1[0], p2[0]) *  .... * e(p1[n], p2[n]) == 1
    /// For example pairing([P1(), P1().negate()], [P2(), P2()]) should
    /// return true.

    function pairing(G1Point[] memory p1, G2Point[] memory p2) internal view returns (bool) {
        require(p1.length == p2.length);
        uint256 elements = p1.length;
        uint256 inputSize = elements * 6;
        uint256[] memory input = new uint[](inputSize);
        for (uint256 i = 0; i < elements; i++) {
            input[i * 6 + 0] = p1[i].X;
            input[i * 6 + 1] = p1[i].Y;
            input[i * 6 + 2] = p2[i].X[1];
            input[i * 6 + 3] = p2[i].X[0];
            input[i * 6 + 4] = p2[i].Y[1];
            input[i * 6 + 5] = p2[i].Y[0];
        }
        uint256[1] memory out;
        bool success;
        assembly {
            success := staticcall(sub(gas(), 2000), 8, add(input, 0x20), mul(inputSize, 0x20), out, 0x20)
            // Use "invalid" to make gas estimation work
            switch success
            case 0 { invalid() }
        }
        require(success);
        return out[0] != 0;
    }
    /// Convenience method for a pairing check for two pairs.

    function pairingProd2(G1Point memory a1, G2Point memory a2, G1Point memory b1, G2Point memory b2)
        internal
        view
        returns (bool)
    {
        G1Point[] memory p1 = new G1Point[](2);
        G2Point[] memory p2 = new G2Point[](2);
        p1[0] = a1;
        p1[1] = b1;
        p2[0] = a2;
        p2[1] = b2;
        return pairing(p1, p2);
    }
    /// Convenience method for a pairing check for three pairs.

    function pairingProd3(
        G1Point memory a1,
        G2Point memory a2,
        G1Point memory b1,
        G2Point memory b2,
        G1Point memory c1,
        G2Point memory c2
    ) internal view returns (bool) {
        G1Point[] memory p1 = new G1Point[](3);
        G2Point[] memory p2 = new G2Point[](3);
        p1[0] = a1;
        p1[1] = b1;
        p1[2] = c1;
        p2[0] = a2;
        p2[1] = b2;
        p2[2] = c2;
        return pairing(p1, p2);
    }
    /// Convenience method for a pairing check for four pairs.

    function pairingProd4(
        G1Point memory a1,
        G2Point memory a2,
        G1Point memory b1,
        G2Point memory b2,
        G1Point memory c1,
        G2Point memory c2,
        G1Point memory d1,
        G2Point memory d2
    ) internal view returns (bool) {
        G1Point[] memory p1 = new G1Point[](4);
        G2Point[] memory p2 = new G2Point[](4);
        p1[0] = a1;
        p1[1] = b1;
        p1[2] = c1;
        p1[3] = d1;
        p2[0] = a2;
        p2[1] = b2;
        p2[2] = c2;
        p2[3] = d2;
        return pairing(p1, p2);
    }
}

contract Verifier {
    using Pairing for *;

    struct VerifyingKey {
        Pairing.G1Point alpha;
        Pairing.G2Point beta;
        Pairing.G2Point gamma;
        Pairing.G2Point delta;
        Pairing.G1Point[] gamma_abc;
    }

    struct Proof {
        Pairing.G1Point a;
        Pairing.G2Point b;
        Pairing.G1Point c;
    }

    function verifyingKey() internal pure returns (VerifyingKey memory vk) {
        vk.alpha = Pairing.G1Point(
            uint256(0x156f4298431641d9c52f53242c20b411123e08e6b8678f092e901c2ee2747d52),
            uint256(0x0403de71aef0e283a4a2128fa6519a656f7fd604d2153f116d65715a2ec1b42f)
        );
        vk.beta = Pairing.G2Point(
            [
                uint256(0x228690f5dbdc1f9a1389e38a02c3054d0976d01e87f95e1cda3ddd4b53a76398),
                uint256(0x1162dcaaedd7b1fa6a7822dce599cc62e5cc4cd6e43aa7988081ba49794df20e)
            ],
            [
                uint256(0x154f08b947550c80539915fe7cf42bfbde1cbf056ac7e27314fc2ae89e378cd5),
                uint256(0x28edd304a2ee9688a4429dee675256cfa725f1915f39dcf1e1043314f1a89b4c)
            ]
        );
        vk.gamma = Pairing.G2Point(
            [
                uint256(0x16d7fbb83e6cbc1bdce7f60e046c58ffd678e783a56c4dfeef4014791020e06c),
                uint256(0x0b495e18f6d04bb15b9845798ad1cb578733e4d8cb892f1d8461a1e65f44c763)
            ],
            [
                uint256(0x2033883ae4dc5f7a600d4a3641f3c756720dcfe36d04383a9517f41602e696e7),
                uint256(0x1d4cc5c6ba0ec0fa503da4660d64b780c0445d96cf4ffb0ffd2f98f1592094a9)
            ]
        );
        vk.delta = Pairing.G2Point(
            [
                uint256(0x0b16fc9838be6ecac15f599d66f2b10fe3ec1a80365e721f5177342195912099),
                uint256(0x1f870ede5d50e264a09b41c8c40df0548732c62e023c0838a101b064183e7be1)
            ],
            [
                uint256(0x02d64f5084c92ac60c25079c09d8f17687ec562d83565c407e44621f593058ad),
                uint256(0x1c6296917688c7602626e243afe23a91843cfb65e3d3c220a9c3ffaf2f6d74e8)
            ]
        );
        vk.gamma_abc = new Pairing.G1Point[](19);
        vk.gamma_abc[0] = Pairing.G1Point(
            uint256(0x0a6b8b07cdb84a2fd822cfa9ae18d7c2a166658e61a917870f52f319ea34a92d),
            uint256(0x0b686a8fe0587d599e80963ffe20c78117e62cc19d1a7f9ffb9dcd829fc6f3a1)
        );
        vk.gamma_abc[1] = Pairing.G1Point(
            uint256(0x11835b5d167aea2c5ffc1278e11b9e0b298d65c502db7e3a42416792e6eb6baf),
            uint256(0x2c125b5b400d13a1ae962c62dfeab32ee4da5f4508a47c77943ee7a7180a72b9)
        );
        vk.gamma_abc[2] = Pairing.G1Point(
            uint256(0x2d964b9b791d6912cc6c854a681e13ccbc07d115d40844ff2554032aabea8bea),
            uint256(0x0babc469cb59cb3baf39b74ebcc3b5cc7cbad7b81ecf7dd985a0b4d0b09dbef2)
        );
        vk.gamma_abc[3] = Pairing.G1Point(
            uint256(0x0174991f1df5a0dfbcd23ba615968dd0d059c569d005e03a5bc83d59c47d455e),
            uint256(0x169c91eacfaf94ee47a3a3e248d74483680731dba738e29b84dec3421978a1c2)
        );
        vk.gamma_abc[4] = Pairing.G1Point(
            uint256(0x1be49a20d32e3ba1020738d42ffa86b583b16c59c6556f935039e77207737966),
            uint256(0x19a28033d8aae78567a00392f3a0122365766582e0f8e166922bcd7998ca7d98)
        );
        vk.gamma_abc[5] = Pairing.G1Point(
            uint256(0x0fe6fc03fd19cc1b241cfcb742debb9b3fb8bfba302085aba414d5881ad23480),
            uint256(0x10eedf52938864957c333b81497e0977ae36c9cbaa0c988b2060675ced1655dc)
        );
        vk.gamma_abc[6] = Pairing.G1Point(
            uint256(0x00c4c78c0edcdd4f7bdbf5370e5cf0ab59a093af259b338bba9c6edb131d432c),
            uint256(0x2d665f139808ee420ef2434d67067bcc88a4ff235edf9afdf405c809ff28735f)
        );
        vk.gamma_abc[7] = Pairing.G1Point(
            uint256(0x2d16301ea2fc1b2468ba07b57e3a8fa74de7d02a35c0ce8b72328af1db7dfe45),
            uint256(0x1cb877d36020f03e00779f28ba089682703143101e71094f5827ffbdec2eadf0)
        );
        vk.gamma_abc[8] = Pairing.G1Point(
            uint256(0x26c88202d23245ec9cb387a4e28291fec0c3f786e0db9365c13f9daab20643f6),
            uint256(0x305b5a532f40afc0ca09b34f5c10b7d4c00193a68a48409a49ed1673ffba0ae6)
        );
        vk.gamma_abc[9] = Pairing.G1Point(
            uint256(0x2db36a3b796555945beb07be239e6386b38001f71f5f9d26fe4398937886e741),
            uint256(0x0a63a7c2e66acaeb97d5be7a6faa4873451ba6e8a35a97755b7e7d96e9b41bd2)
        );
        vk.gamma_abc[10] = Pairing.G1Point(
            uint256(0x0922e322e208e48c635ae0a6e638a69d05add5fcfea3f6c2129757c47f4ebf15),
            uint256(0x0c3970515345b10b468f1fe3b6733da7b6cb726f497ac4c7a881051ea3d31090)
        );
        vk.gamma_abc[11] = Pairing.G1Point(
            uint256(0x1a5e374bdaa40089a7ff80396850be23d280296472adaf53a0e6402ee4b97598),
            uint256(0x24016c75e6f6edf206f8fff8bcbbf0de517d93cd3ab1c0c13ce6fef089ed2b29)
        );
        vk.gamma_abc[12] = Pairing.G1Point(
            uint256(0x2e66295003ef5d6d962753a45c6b7bef3eae89e4eb7fba8024127d0cfb7b88ba),
            uint256(0x130813dac324f864b4db90cf64520ff26ebbf95224a7446888c0d122abcb4a87)
        );
        vk.gamma_abc[13] = Pairing.G1Point(
            uint256(0x22dd64d9aa3fee8c9bca3d1fae9bae94e972d4d917ce881aed4f1b1501f59203),
            uint256(0x1d61d5a654e0f77aec9a81742e06e2ccec99fcd7ed05ba42c7ab2f7bccc7d120)
        );
        vk.gamma_abc[14] = Pairing.G1Point(
            uint256(0x1fc614b9f0f4e81c5f59698a5fcd76c01a4f68b332f4abcf9e02aed1f3bc8462),
            uint256(0x0d57b19e9b3ab0ffb165d0feee04b7fbc342ceb82993660a211d976cc88bbf5e)
        );
        vk.gamma_abc[15] = Pairing.G1Point(
            uint256(0x25baa1c8840bc2235b217a53d166639800f6a44150c7a9be27a2ca7ca8bfe141),
            uint256(0x2c7fe519bff0a1be8c702889e1029e80da177dfb88420cc2ae7e128009c3425b)
        );
        vk.gamma_abc[16] = Pairing.G1Point(
            uint256(0x1701ae96d1e30a5586a39ab124ed37012a208f55b807cfd1c3e29725fb213fb7),
            uint256(0x214c349ef6e4961f0eea62c20706c32295ff8a921334be8b99422f63011c75c4)
        );
        vk.gamma_abc[17] = Pairing.G1Point(
            uint256(0x2272b2c144ce5d3dda8fedcf172eb9a347592efc5b26c638b7e52db3db854b90),
            uint256(0x0934ebc85ae366e003d4e331cb2f4553d739ab8e55014f982d650a37327a6a77)
        );
        vk.gamma_abc[18] = Pairing.G1Point(
            uint256(0x26bbd937eef4975b5f873ddfc0b11e857a5a3e6dffb5b4550dc9b98f398c4a44),
            uint256(0x0c0fa5ec7892a758cc8f34dc57e2a5e0ba3c98f5168c801feeda09e60eed5457)
        );
    }

    function verify(uint256[] memory input, Proof memory proof) internal view returns (uint256) {
        uint256 snark_scalar_field = 21888242871839275222246405745257275088548364400416034343698204186575808495617;
        VerifyingKey memory vk = verifyingKey();
        require(input.length + 1 == vk.gamma_abc.length);
        // Compute the linear combination vk_x
        Pairing.G1Point memory vk_x = Pairing.G1Point(0, 0);
        for (uint256 i = 0; i < input.length; i++) {
            require(input[i] < snark_scalar_field);
            vk_x = Pairing.addition(vk_x, Pairing.scalar_mul(vk.gamma_abc[i + 1], input[i]));
        }
        vk_x = Pairing.addition(vk_x, vk.gamma_abc[0]);
        if (
            !Pairing.pairingProd4(
                proof.a,
                proof.b,
                Pairing.negate(vk_x),
                vk.gamma,
                Pairing.negate(proof.c),
                vk.delta,
                Pairing.negate(vk.alpha),
                vk.beta
            )
        ) return 1;
        return 0;
    }
}
