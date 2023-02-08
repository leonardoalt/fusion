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
            uint256(0x297cdac9e35d8077895472d2c2d2a71676c0e001d0126a9d12319ee7914d1fe8),
            uint256(0x0a2bb82bf13e43d23f3145840815cf57396d55a41f4438e143cca1dc74dc9985)
        );
        vk.beta = Pairing.G2Point(
            [
                uint256(0x1f1ab9a233e2acf747b2c9e5bd5fb13b31253a475da017b4f811b2759e5995a4),
                uint256(0x0058b360a694dc2e72711e66c2edeed5652dca26c4c639e8836d97902022b0d8)
            ],
            [
                uint256(0x28ca7483534e1069a9426e0d7f9fbc45cacb34620aaae58f3d4ca581ad811d8f),
                uint256(0x21059cd087d8134f9a42da67ff7de052c769eff87b143a94b9bc92fa675dbf83)
            ]
        );
        vk.gamma = Pairing.G2Point(
            [
                uint256(0x09207fcac5afdef4bca08d2beffca91dd634a685eb15488afc799e1133f101ae),
                uint256(0x1c5a50ef0dce16b4a4eecb01ff51d3fcc25f399104388c39637f997bb337d4c6)
            ],
            [
                uint256(0x23f91a6decb7094c49edb7f698c1cae9c79c25c98a50f5560026e477fc107cd1),
                uint256(0x1f02038245039abd176629079cc0492412f667a93285f37fcff26e3bdea337be)
            ]
        );
        vk.delta = Pairing.G2Point(
            [
                uint256(0x2741788bafc49742152727741cf364aa0e8da40f449bd2ac0a95ea198b5b5e51),
                uint256(0x025860ea884c46cf5b910aff15a07696236642fdce0a59063a368f3caeed7256)
            ],
            [
                uint256(0x25392edc68241931a4fdd042140bdc670b391aeb404ed9ec2552e286c4b9083d),
                uint256(0x17c26bce7e7348f9527a6e7bf872a7a26d19d755b4b350063c20ecd77687f2c8)
            ]
        );
        vk.gamma_abc = new Pairing.G1Point[](18);
        vk.gamma_abc[0] = Pairing.G1Point(
            uint256(0x04b263b6923f302c6840bdab7496dc6247f9b2c2700222b5a1ff61aeb9c6d1c4),
            uint256(0x0c23723df6bec79a3c35267bdb555dc1735fff9ab374b08a25871bcfa840d686)
        );
        vk.gamma_abc[1] = Pairing.G1Point(
            uint256(0x0ffe3f0dc36c328b0b20d891658af5930b5905e1aa10fae755fffd5bcf0e95ef),
            uint256(0x27e7316183ae5180512eeffdb9b4ffa79c29c57ef8265d32228691463c0706df)
        );
        vk.gamma_abc[2] = Pairing.G1Point(
            uint256(0x2a168eaef0e4cf47689bd94fffe11452fa15e1eef58ca62b27a09d51ef8fc7a0),
            uint256(0x11ae5b682c67d5b5ed790d46314dba711aa727ec45135bcfc9da05d0b51c6b6b)
        );
        vk.gamma_abc[3] = Pairing.G1Point(
            uint256(0x0707d93413ed09d749a7fffbfa7c22a18c96eec4762c2270b8065f7eac5ef234),
            uint256(0x1e2831087765b510277554f952d2b9012c2d13d1544a79cf9e67dbce7fc1395a)
        );
        vk.gamma_abc[4] = Pairing.G1Point(
            uint256(0x187eaec08942e5d08d81be1e49126a0e398b0bf80c973abfd0e937291ea9a157),
            uint256(0x3017d14ed48be6cdfe9b308b2a1883a2fb1fc4fa811f598eb97ec90d73c1a65b)
        );
        vk.gamma_abc[5] = Pairing.G1Point(
            uint256(0x275b91198404a8e1dc0243c74da1a38b4cbbf429e688a5d1805c58b2970469d3),
            uint256(0x2461a1403c14cef48af28b3a0bd0172f06b2106a88298adb39d9102ad0e5e3ee)
        );
        vk.gamma_abc[6] = Pairing.G1Point(
            uint256(0x0c186dd5df32e5ab28318e94caac5dfa697c6be7d5198c0b41429c4e6bce683e),
            uint256(0x10a598c549b5df5474f2b3278afaa50b32e71a4b1870c4b6ab2e0a51308cc66b)
        );
        vk.gamma_abc[7] = Pairing.G1Point(
            uint256(0x20d7b6d6aea2508a9a0016e408e5cd123ccf31ee34f33611fc2dc62e49330623),
            uint256(0x066d0df216ad76d329bfdef5a0ff416c4d1bc751c4f684a72e560af7c0b0283c)
        );
        vk.gamma_abc[8] = Pairing.G1Point(
            uint256(0x1b4f18325c40f140097eac77eb71e10f9c007ef405a2d9918ab98f0477e313eb),
            uint256(0x1347398cb9d3e1f3a13a7920fc00b051f5d941ca52088fdff8f51f704b116870)
        );
        vk.gamma_abc[9] = Pairing.G1Point(
            uint256(0x055017edbdd67f08d91c98bcf683ca74054f11f9764453a8df8925cfa077fa69),
            uint256(0x127f018e8532275be50812b163d2ca93037586919094366cf7f550f7a463eae4)
        );
        vk.gamma_abc[10] = Pairing.G1Point(
            uint256(0x1f81a9a159bb031f48d3f308223ea0d278d457650eae16513f64e929fc7b3152),
            uint256(0x281ff653d8a0290ba935ebeb036b056ac44bc336decf3466ff33dc7404969d77)
        );
        vk.gamma_abc[11] = Pairing.G1Point(
            uint256(0x05b1fa83bbc356719e34a0ffbc0b4295a815e7da10874be81ebdfb33f21a504d),
            uint256(0x22631a1ba19074cb4c95d9f4fa14c8187d62f1d748d736ffed3dccf31b5c5372)
        );
        vk.gamma_abc[12] = Pairing.G1Point(
            uint256(0x065d807a33f57b717ef604906cb7d2901e7bf26b5b55aeb735b0fe35090631d3),
            uint256(0x1adfb2c5be32a220db9dae812752cb4a4402193ec76fa71f525d5ba1fda38a6a)
        );
        vk.gamma_abc[13] = Pairing.G1Point(
            uint256(0x067f4e49864eb1ff5f9e87ba75dccad2c6bd1b8b7eb385306641c6ffb514df71),
            uint256(0x2cb2ba297d60ac2567d67cb06eb51e1024b65130de11a8a6b4112526834fab89)
        );
        vk.gamma_abc[14] = Pairing.G1Point(
            uint256(0x2bf4858ff70fce0a8eb0f53ef51507595bf7701f5afea66f80b732532452aabb),
            uint256(0x085a7f67908a25f8e2601701317a878d962ab8127d9cac85e83586e9c7c79499)
        );
        vk.gamma_abc[15] = Pairing.G1Point(
            uint256(0x0a021e370a4a8ae92d4785190ea82b26bff638f8765a99d2e47e428efb12caec),
            uint256(0x10dfaaa8cd0c07b247b017cc0ea569d0232f982560bd272fe5d7d01033a6e9e1)
        );
        vk.gamma_abc[16] = Pairing.G1Point(
            uint256(0x2cca0b50b117b438dcd4010bef5c78fc986ecfe2c3eb980f143bac34674cc6da),
            uint256(0x1dde7191c707eefd3de59bad089ccb3ab974f5f596ecf4e9c331cb5108cf1026)
        );
        vk.gamma_abc[17] = Pairing.G1Point(
            uint256(0x153459f819be372a364af8167623e1dcb9d61e91b100cc9129ba496ef5f75d6f),
            uint256(0x2bfc60f71d0e309d7ef610085da9ce53d9c99a61b4d969143bace06b58a6998c)
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
