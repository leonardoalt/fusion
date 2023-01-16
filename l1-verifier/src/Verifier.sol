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
            uint256(0x06f82ad73e997af1d4fab2330db0444b4f3b097d3ce38aef869d73854b93bc9b),
            uint256(0x0696443c36b96abd9006b0a129d47716d248fdda4f31042238f436c52f932633)
        );
        vk.beta = Pairing.G2Point(
            [
                uint256(0x2563f77a28ac81c05100778356648a9c6115ce539b5b7b6641d031233aedc645),
                uint256(0x1e409a750fd59bf34ace3d9d4a45c111f9d5bd1d763b47bc9e4755f3021eb589)
            ],
            [
                uint256(0x2c2882151640376c941e640666cac71201040cb6ac88e0b183ffb46856b28caa),
                uint256(0x0937d68feb1dbd77dfd7172d57a383ba1217f826ea8a719a81c4ae85637adad0)
            ]
        );
        vk.gamma = Pairing.G2Point(
            [
                uint256(0x14644d0a3ed829e96c26792c49bea0fdb023016302b01f2726e3075e6f06d3eb),
                uint256(0x290d248bd1dfb9f548833bf2c8ffc0335dbdf99e1cbf093ac992b7cdd39685a5)
            ],
            [
                uint256(0x03bfdf434b60c72196420ebd186e7e6f46afb6fa7216886d451bc01f1a32071e),
                uint256(0x0d8a6ed17ed2546b5d1bdc1a1c57f5b6463561a5adcb065f6c07dcd41c19545a)
            ]
        );
        vk.delta = Pairing.G2Point(
            [
                uint256(0x1e34bf03d31ef4591660e095a31a5e265fe93d726d6e6dfde43a65ec329c084f),
                uint256(0x0879a41af969f1ca01f8a159e61fb00e617d8572dc25f7ed857525d5d473e4e8)
            ],
            [
                uint256(0x1305bd1b9c3f4a889780184719ca2329f28c86f0419ee9537af41ff936faa923),
                uint256(0x092a340c1253eb28671ee52499e35ca89247523fdff17c8bb20783b4a1f180ca)
            ]
        );
        vk.gamma_abc = new Pairing.G1Point[](19);
        vk.gamma_abc[0] = Pairing.G1Point(
            uint256(0x295150531ddc95dcc2023a7094cb6b84387dc4e1c70d5bb42fd83b59b3a48171),
            uint256(0x016fc82386b520a0c1573c5cd76e24c0dd7eee042a74d3b34beecf8f1abeb7f6)
        );
        vk.gamma_abc[1] = Pairing.G1Point(
            uint256(0x0885a0b2a0e034eda643f382ec33ba6935f267f6941f8432a70d8b71f3bae876),
            uint256(0x2676b49f50b3d3989a8603ec289720e059fc91ae69c163b39f2ade094f9852e9)
        );
        vk.gamma_abc[2] = Pairing.G1Point(
            uint256(0x05d4a4f135791a6bacd2187362b497d3ed07f542b591579017ca9f3879747522),
            uint256(0x2fa25062edff92aa9c78ac78d4eb914bcc2aa28109423b2abc976ed008b24793)
        );
        vk.gamma_abc[3] = Pairing.G1Point(
            uint256(0x2bd11418e90463341c4b6338bc1c1448d0379bcf6f4e1e5e286d201984909325),
            uint256(0x071ad7c796bff8192d519007114235e35a2153d209402dc4761b450e0b92f2e2)
        );
        vk.gamma_abc[4] = Pairing.G1Point(
            uint256(0x108857573af0b98010a7e1409cfc7110d3cded5e90759b7979c62bc29ada84ff),
            uint256(0x24cc447447a671b9fb31786e5c9de495dff67fb9dae40ff6c15633a00153a63e)
        );
        vk.gamma_abc[5] = Pairing.G1Point(
            uint256(0x1c57837b8bbbf9e76346e577ba45c1f21a979a4a8814e12f8b625b7ff7a4ed7a),
            uint256(0x212178ffdd60886a0e10245ab204fd5bdf9708289853f3106a35d4b1e9d35706)
        );
        vk.gamma_abc[6] = Pairing.G1Point(
            uint256(0x03fb716af45e8fd3a5f3880f9f86bdb213a21fd5403d7024d8dc92aeeabae539),
            uint256(0x030354967d78e514ee7e2745e087a3baa2bd1abb926640aa917287bf676b530e)
        );
        vk.gamma_abc[7] = Pairing.G1Point(
            uint256(0x15fb678a6f115253b76aa8f1be7e2f0a2432dc58a43377269d589949c2638c07),
            uint256(0x107b1b60227d4b0578858a1d0921803ef032cd64626adbb42f34842b4217270d)
        );
        vk.gamma_abc[8] = Pairing.G1Point(
            uint256(0x12f8a913be4b4f7cd587e9859fb17beb7250bbb12751ef8350979c20f3e39381),
            uint256(0x08de7bfa2c9934981307878a473c3e7cd7c93be7bdcf666ea0980bec52fd3126)
        );
        vk.gamma_abc[9] = Pairing.G1Point(
            uint256(0x047166cbdf21a9961ae0141dfaa824fad6501d902dba568768e913a9d63a1a99),
            uint256(0x15fd38a9268e9bed4ad4c17455809a2e8db6c80619a57793d061b5deee9bcd4d)
        );
        vk.gamma_abc[10] = Pairing.G1Point(
            uint256(0x0230cf2adc2e7733f3f8ae8d675727c915bf140663db8ba1b41af061a4854b00),
            uint256(0x07a0abe7cf89a440e8c91bb67e5bfc2ed5e3f876d4884d4e1af70dadd5c42a43)
        );
        vk.gamma_abc[11] = Pairing.G1Point(
            uint256(0x2fdcc67c296cac3630d0be3f5751b789583d67d63312481241ebadb9c709d5e8),
            uint256(0x1ce31c966fb94d0f477d53e7d0839dfc007425912509b7603314c19333f3d39f)
        );
        vk.gamma_abc[12] = Pairing.G1Point(
            uint256(0x29d2e76797f79f61de4ebd3f0cb3399e7eecd78a3d9f8b613aabddf1d0728de4),
            uint256(0x261b62c2bd5fc59dd9f7e152e70d30c108a6c0fd5ec0b60b8baa0eb3d3cc1ee9)
        );
        vk.gamma_abc[13] = Pairing.G1Point(
            uint256(0x0938608020db916c0925d98ed1d89ed9d8c5ae01c69386a1d246a943096b48f2),
            uint256(0x0ec30b587b95ddd84adae8b3d70c7b3bd26e99ff7e2625b2f19c749ca7da1c91)
        );
        vk.gamma_abc[14] = Pairing.G1Point(
            uint256(0x2651c2b5a3d66eea5cc41e7dd7af389c307d3860a14de5ecab2ed580c5a25f72),
            uint256(0x09775bedf89e073dfd49704ebffbfe716a72d61fad286542c67252b9966ec238)
        );
        vk.gamma_abc[15] = Pairing.G1Point(
            uint256(0x1a8db14507036846741b072106c6c4a86cf4ce42991913b6f0b24f1e293e786e),
            uint256(0x301bc5946aa089dffbe6b5fb463617e98671a41e86034fdf4b23cb3c99db46c1)
        );
        vk.gamma_abc[16] = Pairing.G1Point(
            uint256(0x1d705d38619a1092313cb66d82f786713be4c6184e067acae46154548835be6b),
            uint256(0x2c6ec47f57f7eb98a2f4bd8ba4182a8fc32c851961d852707ccc04fb96f7f08a)
        );
        vk.gamma_abc[17] = Pairing.G1Point(
            uint256(0x0265823ac23129946d2a0a796f8efd43ec86fe4aa54a28fd1d3f241be6777695),
            uint256(0x0a5e16ad5a3d4275a736c8ae86b987690db165112d3fafe453d29482ac731670)
        );
        vk.gamma_abc[18] = Pairing.G1Point(
            uint256(0x03d682c360c36889c54a679ead17b342e99879c4144b82d6b5530fc32130b6e5),
            uint256(0x008579669c9f35b7ef0cbbdf228e2f93662af754284e9944a7f910e77d1bf2b2)
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
