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
            uint256(0x232a84edf6a603fb4fc60b7f471283767672ebee180889b6f7117ed9c67dbdaf),
            uint256(0x2a2d1ac4f16c00fa17c51a1c9820ff0899166872688ead34eaaf2832e467e2d0)
        );
        vk.beta = Pairing.G2Point(
            [
                uint256(0x0008708cae2f67dfaa4478461f19d4366d762a4d3f29e9300065271d1da2013b),
                uint256(0x1525b51187f0135bf78e0c719be3fe5ede5acf3466dceda229165c252071ad13)
            ],
            [
                uint256(0x1ead075f0e5181bed280a1ef7be0bcdeb5615254a695d038586553be452c37ac),
                uint256(0x12f2256046de29a2ac8e7dd2cb2264ebb8884b01f96bc68c965ee3e0348f9885)
            ]
        );
        vk.gamma = Pairing.G2Point(
            [
                uint256(0x22920718e072ba79d44a4eade05325a324de6aedb9c65b60dd7c8c63811166cf),
                uint256(0x111373e107c35e3a883a01ffff05edf835e4b205007e7e031b8c8cff81112c8c)
            ],
            [
                uint256(0x0b839651be1e3f6a68e79aaf08e0dfe711bde8e52da71e7c6054d6414a0aa639),
                uint256(0x2d30cfef9c498fe8cb0fb85715cc2c8a6024a2880681369d514de8b6198f2236)
            ]
        );
        vk.delta = Pairing.G2Point(
            [
                uint256(0x19b59575df8a99d0984a5fc8f8204bb71d74c0589d2362742991b574e89ae17d),
                uint256(0x14d4572e86d3569fc9df56d268051d97b3771a0ffb4a9b8b88134ef7fb9a0507)
            ],
            [
                uint256(0x00f343c577bbcf1061315902643e5690436426482b8dfe7c2be06e2254b949d6),
                uint256(0x11fe6d8e9a8fa071ab34797c740a0897e25163eb046f25895f02d602ba9134a1)
            ]
        );
        vk.gamma_abc = new Pairing.G1Point[](9);
        vk.gamma_abc[0] = Pairing.G1Point(
            uint256(0x24872374c2aec7df5c60cd22cd74f888add93333b95cdc9980a0566cfd9fff99),
            uint256(0x17784c6a4fbcabaecf6e1b224b78fce650ea1231b53893888dfc949a44ce49aa)
        );
        vk.gamma_abc[1] = Pairing.G1Point(
            uint256(0x1ebf8fa348163362fa82fdbc0f7f6c03a6136e258e6660b76b39dffe9b29da91),
            uint256(0x2e3c7150ca06a8dcba0aa3ced0455693777d26d88e4419ccfe6bfd840116bbab)
        );
        vk.gamma_abc[2] = Pairing.G1Point(
            uint256(0x034356f9e9af2852d839306e9479167dd526113596d12270939e930bfb37d669),
            uint256(0x0e2a8aa6a11b2223b39ac42fea0063aa5e7724845aefe90a2544469ac252aa88)
        );
        vk.gamma_abc[3] = Pairing.G1Point(
            uint256(0x178773fde75c925005b3887e81cc744252478437b74d3477afda73cf49ddb8d5),
            uint256(0x04189698e91f330da256411bec65de50e2a330cb0aae802e4c4610a7999c6bf0)
        );
        vk.gamma_abc[4] = Pairing.G1Point(
            uint256(0x2681e2e1b5d96446836164eb5641ffb384a64c6937784cceafcdfaa8fbed6740),
            uint256(0x2ba7ac4254cdee1d1d002847cdbb8325417421f4aea54a4499972a7e08a4e0d6)
        );
        vk.gamma_abc[5] = Pairing.G1Point(
            uint256(0x29133575ea73ef4a07de5b7f02f6c5f372f0aecdefd4d7a5e417b212428e6ef3),
            uint256(0x14ac9b0745f6b10579cc2692ba01d1aef20cb5e18559549dfa67189f70f1d825)
        );
        vk.gamma_abc[6] = Pairing.G1Point(
            uint256(0x02c83c137982ac91c5128838bec6ee087fc7c15a66a20780120c60c0b05392af),
            uint256(0x0f3423dae3fa60fcb3b765939108961c5573db3f82d796f8e444b8fea68b7707)
        );
        vk.gamma_abc[7] = Pairing.G1Point(
            uint256(0x03d6469f77ae98d638fb9886002c53145d5d822281a3ff36d8f6e360d1096b1f),
            uint256(0x296c3b491e197ba3faeec400d42d0cb2be2592994db5d574de4ec12aa6699423)
        );
        vk.gamma_abc[8] = Pairing.G1Point(
            uint256(0x0f20dbf9f8c02814d372e9fa923960d452194c267a8c035480bccd9835572cf5),
            uint256(0x11ec733ba24954bcd632b0f4990affdfaba602940b60cc12567e15644f95a3b3)
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
