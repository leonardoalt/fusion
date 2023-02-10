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
            uint256(0x22004253f00cd1a64cd6f1303d3ebb3cd12c7c0941c4c09ecf5d3e1008204ab2),
            uint256(0x2b9031138d9403cfaed8bc0282eb1ec39f13f3a5b2f55a2e7e4f9ea1cbc4f512)
        );
        vk.beta = Pairing.G2Point(
            [
                uint256(0x0f30544ddb798f22c9e557137337d58441ae9ad7530faf64647605807b86d03f),
                uint256(0x1b3419c58f9fbe1e58eba7fb69fc115018909e1a45b9592368d758d0c09bddef)
            ],
            [
                uint256(0x13dcf5a823d85482f929e0a9e9e6a63d066437995334f6c290aec0728b25ed96),
                uint256(0x25fd9e0c09cf68ea4cf9acbe625797c17f1f6ca976bf88de349c0602c6cbe747)
            ]
        );
        vk.gamma = Pairing.G2Point(
            [
                uint256(0x067696285961bfa5ab748f3359f08bcf592a5b0cae7508950bf0c4db973b8f67),
                uint256(0x058e3d28f22f3710a62cbcf0e0609d54349984db61f82a6b6bf7d164e999dbf9)
            ],
            [
                uint256(0x1bdc33eae4a7cea4e94b634a2a50548e5acc466033b9045d0892927bd5afc2df),
                uint256(0x159f43c482b48ab07356b2a8214ecd6c2ffdcc56bb46a32ee3dc9a9a832792eb)
            ]
        );
        vk.delta = Pairing.G2Point(
            [
                uint256(0x0772102e0f3278f04cff738427f7df080987dc31369809973d388cc9830d8356),
                uint256(0x0d8fa6e12f53bb176b9688ec8036baf460e855705ae0ad50b0599ad8088fc80b)
            ],
            [
                uint256(0x2411774933b5b1ac93501e7ba912597d4db01daed7b9dd9642593537c798feeb),
                uint256(0x2027602d325876d6c7b1a3a659cee0e1e8474c4e9ec77a5d56bbc552b5309769)
            ]
        );
        vk.gamma_abc = new Pairing.G1Point[](19);
        vk.gamma_abc[0] = Pairing.G1Point(
            uint256(0x2c88936d73d99c9d64914bb2f93dbd32d574792c66967cdd9e0de58fa8f8c7e9),
            uint256(0x0069fbadbcf9a9baf393e2e7cfbcc2812af395ab923945954b3692e7f9e3b5cc)
        );
        vk.gamma_abc[1] = Pairing.G1Point(
            uint256(0x0b8cd1da0a614550d33202a5dae797038ad3a0fbc89cdc0162d10c07186a77d1),
            uint256(0x1b00d8bcd9ad98464c7043bfdd07ab00e53a11bd382a41f192bd5c9dea7deda6)
        );
        vk.gamma_abc[2] = Pairing.G1Point(
            uint256(0x1e4e46dbf59952f6cda0c16523b17bea95ad3d1002a12bb34bffad406ccf13d3),
            uint256(0x0d91697fdc114778e7101a96be2233c1b1b17732adbe95783e63aa35591be008)
        );
        vk.gamma_abc[3] = Pairing.G1Point(
            uint256(0x2c2e462dae75f075d8bbd5859811ef0304bf4337cc61c8604c4202a1dd199844),
            uint256(0x12224c6fa30fa2755edaf4ef31de1b8d3a67f81711002351d9c4e5a0a566026d)
        );
        vk.gamma_abc[4] = Pairing.G1Point(
            uint256(0x0b34dae17aef7c6469d275d750c5c505f2dd7c3055aab8eb316989d658e11964),
            uint256(0x2e803697ac18a886809fede2001fb4ae5f8c4053d979acf7441f7591b5599c8e)
        );
        vk.gamma_abc[5] = Pairing.G1Point(
            uint256(0x11e4c0d14c50417c36e08142a60269573a5881eda0b067f5e3bb121f258b5cf4),
            uint256(0x291abc843c5bb2eca7bca2ef084e84c386ed43a978bb169226759423302b2898)
        );
        vk.gamma_abc[6] = Pairing.G1Point(
            uint256(0x2bb2c4e14f5d055e7496e44d0d3b8bc426b2079e67ac6039c8914fb68dc17107),
            uint256(0x21413ca0ce6ae8dc235fa3b2a69606b754e0e56ac9ee10eb606f7576ac9bc52c)
        );
        vk.gamma_abc[7] = Pairing.G1Point(
            uint256(0x0d0aac7ca66d17b7ff012908ab02d9a0f137832ad949a2292f51afa8fa344bbc),
            uint256(0x1180af50bf11532f56308e5f63e3626d89882af62fc588d8e7b4b873b6bd1389)
        );
        vk.gamma_abc[8] = Pairing.G1Point(
            uint256(0x158ce71db852340569ffe52087236dcafc30e5cb5605cb8572fce6c0c7ce385c),
            uint256(0x0a648d89b3d35a3067705ab4c297030c0006c0c6af8f1684a604a6b541013c4e)
        );
        vk.gamma_abc[9] = Pairing.G1Point(
            uint256(0x305e9daeb32ea7134b54614b7c89514bf7f4a4ccaa0818fd05b47e16bf8aa933),
            uint256(0x2d7a44921597fa4b5f311fc555704dd3a395e47672fbc30fa322e84e7d574ea9)
        );
        vk.gamma_abc[10] = Pairing.G1Point(
            uint256(0x146b4e9bf5494ecfe968978c466e71ed8f12410891562d964ad6b1d1362d08aa),
            uint256(0x205e30e7e33b72c11cc0df6f913239234de6b8a77bee7e95f2f88679adb50035)
        );
        vk.gamma_abc[11] = Pairing.G1Point(
            uint256(0x1a39ceee6a31a4fc196718617afe6bcde246b7a11b537a9553e360750255cf10),
            uint256(0x176fa44ff29799357cd5f351b4de825bac18bca4e5b2038e33ff7b7ba8be45e5)
        );
        vk.gamma_abc[12] = Pairing.G1Point(
            uint256(0x0efa01abf163385f4e09925f8ac959a81f78684bd2b80d1655f151996030b4ca),
            uint256(0x128b228af95e4bd98c18bd663fc9fa79e953b6c848d17ef87cfff9a05a4bc1a0)
        );
        vk.gamma_abc[13] = Pairing.G1Point(
            uint256(0x2419d0749e24e80ccc0c9acab5f50016a3ae5606bce213174cf5bdaf2c855d9b),
            uint256(0x2862b77b695ca85ee701978bc45538dc5142f02368dcb5084c32839e477b72f7)
        );
        vk.gamma_abc[14] = Pairing.G1Point(
            uint256(0x02379d696e3f6d3e589b2c00bab9d0e693e07a3cff1ffe42b3031ce473c1da19),
            uint256(0x10da37a17079d6a9d61dd5ec05c1c4c118b31052987eb7fd95ed9567a96c58a9)
        );
        vk.gamma_abc[15] = Pairing.G1Point(
            uint256(0x1f03cf33a8759b8ac81b4b4aba5bc971aac5d2612272828ff458b11294eb5d07),
            uint256(0x29a41c7ba2c28a1170676f4e159c05cf0b70e24e706bf83ea49cdced8ef86406)
        );
        vk.gamma_abc[16] = Pairing.G1Point(
            uint256(0x133ae86f06c0ddb1cfc0e282849e06571d2e3b8a5692e5e1e60b4b87e81f5b46),
            uint256(0x29c2611ca28b649f7a273a1cc20d48991aef5a5828678030122d96aba25426e5)
        );
        vk.gamma_abc[17] = Pairing.G1Point(
            uint256(0x1a4eead82ed7c5319979266bdfec366392aa367cf2888a2057704364b3b2d907),
            uint256(0x0fecfb80df7095b471dddd87a3a53946fa8bebba80947db44cd5fdfb8056ca2a)
        );
        vk.gamma_abc[18] = Pairing.G1Point(
            uint256(0x1a11379ec03d661bb0a82cb882e8ae5d7596413a29b249820b1fd95bbe68dbd8),
            uint256(0x2acc5a3fd18a1b63d6308eb90e54fbcce75b2194ae50ad2ab786fab092da205e)
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
