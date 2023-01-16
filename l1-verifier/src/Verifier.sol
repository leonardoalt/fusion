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
            uint256(0x0b688fbced4588ea7de7e6ff15b732234c9e2e204af53847382db85bdaad08ec),
            uint256(0x27154172aa51cb4cc926f451ddf7853599cfc9f342a5905e411a698db11cfb75)
        );
        vk.beta = Pairing.G2Point(
            [
                uint256(0x228c50d8e157909e53e76c1910c50a8e38e48b2adc3d3ce25fe609321e6e3649),
                uint256(0x26c24eeaf5b06b1050845a72af68ebf77734a78014605992871264bf4e5e37f0)
            ],
            [
                uint256(0x0f8bf0bf2a8c118a521f10a028c707f5e7aeb6032a76a4bfbe1a39bd075cc9ef),
                uint256(0x1594ac6e3c0bb783d7743011c76569aeb1d0cd8ce5c9410ba32a8cb12b4a69fe)
            ]
        );
        vk.gamma = Pairing.G2Point(
            [
                uint256(0x03a85a5e8a4c99589ce4a176bbd2240e1eb8805130aa5bc7aed74c8cf26ba81a),
                uint256(0x06fe33a499d57ea63af0bd88ad92be93f17ded4b3c95fcdc61749f735c7115d5)
            ],
            [
                uint256(0x305be599679833b7c1e982ddb6043d9aca20571422a69293afbbe692d15dcace),
                uint256(0x280c871c347ee17e5ade2dd82b6d464766220dc8eddcbf29c7ee896413b26020)
            ]
        );
        vk.delta = Pairing.G2Point(
            [
                uint256(0x1accef20b4d9f9353f38ac21c325d0318b0e8bf86a458adb62f5b63e9d40f330),
                uint256(0x027c490716b0df293f151a1678e1b7c3a685d492aff8b1742ce9a41b7e869371)
            ],
            [
                uint256(0x27402f0913250d17ac251d9eb82da15af8557e2accb0bb52527f1604561740b3),
                uint256(0x0f78992ef2054d104d2d66bfe36d12844aaf1fb6b4fbcdc42309cc8b6ebd51ea)
            ]
        );
        vk.gamma_abc = new Pairing.G1Point[](15);
        vk.gamma_abc[0] = Pairing.G1Point(
            uint256(0x04c5a57384d3d14d1a920ba1e8708864492afd20694a63f697e0a7ba99240213),
            uint256(0x27e69f56dc9324e4d56b1ef901b200da296999d183b6598ebcd9c2c4bf3150aa)
        );
        vk.gamma_abc[1] = Pairing.G1Point(
            uint256(0x1215567c947125790e493c35fbf1af7e6d83a12102704a2c625f048a53b6d441),
            uint256(0x1c81d29aebe40a70568cf5a47f56be64c04320638ea342037fdd5be8f7469673)
        );
        vk.gamma_abc[2] = Pairing.G1Point(
            uint256(0x2f1f16f36413ca91ea0c954772e34bdf40dbb5b588fcbffe58727faa5b2509f6),
            uint256(0x2f6bb3e32a03d55203007e7852708eefa98dce1035c6883f0fc509d2b24d6792)
        );
        vk.gamma_abc[3] = Pairing.G1Point(
            uint256(0x18af76e7316af93d23d944b5a47e33c821e93c4bf410ec482e6ea7cc16af0f1f),
            uint256(0x0a1f192bd7298ca99c3e7cb6804b0b75eea9340e973488a15aee6d256a539e10)
        );
        vk.gamma_abc[4] = Pairing.G1Point(
            uint256(0x29739322878170d85f24c5088faa0ce4b9c40337d14b6ba93f4f101d9b7345d9),
            uint256(0x3008cde1c2ffaa623085ef904e1254e8ce18fd9bd8c6cfacf7764a00fc2253c3)
        );
        vk.gamma_abc[5] = Pairing.G1Point(
            uint256(0x0173dc25ed87ddd92d3f55b2cba0a4e86d0e69dcef6de913817301de8f332334),
            uint256(0x07f4948aa1ba3c60910a75439ce9f2173f736dcf881c34df7c227423385dee1b)
        );
        vk.gamma_abc[6] = Pairing.G1Point(
            uint256(0x22f6c075c892fd83c84c81c51da0a461c79a1c9b6073a96b2ed2534f85163a5d),
            uint256(0x1b14f59c961fd82865bc5ca044ffebe3c87a5d7bc54f5e895e9a535ccf80fa35)
        );
        vk.gamma_abc[7] = Pairing.G1Point(
            uint256(0x288d4e115cbf9af19432541ceaacf8c5964b9d536e83ef4c35e2266093fa6ce2),
            uint256(0x1f4eb859ecc2d614ebb986574e70372ec23b096f2dfb51d2a9b251ab2fadb339)
        );
        vk.gamma_abc[8] = Pairing.G1Point(
            uint256(0x1f96475947dc737322e6f605ac81a62912ccc3153843c5d2f9c1d5a520079757),
            uint256(0x0afb9e88bba3b35002a58c2367cdca6ea3522f21e7d868c2a2628fcfe2573eca)
        );
        vk.gamma_abc[9] = Pairing.G1Point(
            uint256(0x2e61a5f6af7e14d530ae18ae74f40ad8c6d3625c1a39f66f06b7515e2d581fcf),
            uint256(0x273c27f18f2f73b9186c314ff09838ee35de8c8d1a8570db711256662d82862c)
        );
        vk.gamma_abc[10] = Pairing.G1Point(
            uint256(0x1af3eda874043baf45bdb10c7e0884bfc57d0839f2216a26bfc5b9c1b6c7e5eb),
            uint256(0x06dd3700e1cbad0353c5089d9719031b2c27ad0d0fc7e90e8ab9764fb5744d2e)
        );
        vk.gamma_abc[11] = Pairing.G1Point(
            uint256(0x00362efdb91431d7710273156ccceb3aaa34b218b3a36cf6ace8ed0a175d47e7),
            uint256(0x057e87c32c1272474e2579a291bf437c3d987f0677ecf4f51681d26e04441b37)
        );
        vk.gamma_abc[12] = Pairing.G1Point(
            uint256(0x29e9e9a4837b9b62a909268f9b9bdbec04bc145d68a458a453737099459bc0c6),
            uint256(0x1a844cdf4f1b7be63f2ea35997a5ddfc005d6e8476b9a599cf7fd4f6522fb869)
        );
        vk.gamma_abc[13] = Pairing.G1Point(
            uint256(0x12317b13c4a731473af67be206510d50e49f544cab2dca4be769b61a976ae702),
            uint256(0x260be7739e7bbe6fcecc9cd9995a676277aa44c4a28bc66d757f8ab847047e76)
        );
        vk.gamma_abc[14] = Pairing.G1Point(
            uint256(0x204e08544e2b1663a9e7c744cc9ae810b1dc6384b9430712d57b5354233586c2),
            uint256(0x13e705c94984cfff42d7f8621daed4b878abac7aadfe7fdd6d14eeb1ccf35732)
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
