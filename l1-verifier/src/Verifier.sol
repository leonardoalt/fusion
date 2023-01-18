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
            uint256(0x1627c4bb4f235ffc1cbad4d290d670d70ac1e9f6584d17473bd73f623d0308eb),
            uint256(0x130d14032b87d4f33cde88477bcc6c03109f576fd0a7873c8652b1a6c022335c)
        );
        vk.beta = Pairing.G2Point(
            [
                uint256(0x190c9e0fde56bde325c7fb3739ebce3d50694e266a759f9ff985fd036743aeb3),
                uint256(0x2e597c5d711300b52e541a5c36c9f9670e0665f54de871a3849e7a59088498fa)
            ],
            [
                uint256(0x22a24109cf185a633c992e5d246bdb4a2f4e519b78a99317e96a3b64ff08a4a5),
                uint256(0x0b5887b82b23c8fa38e6b10589ae85d089cfecf48a9767df7bdb7535a6228d48)
            ]
        );
        vk.gamma = Pairing.G2Point(
            [
                uint256(0x03746b25c614783085e58f02765f730f986bfd9c539a7d54c84da8385123b120),
                uint256(0x0689617067c64d517a83d254bb798ab6c4ef198e2d88a33cd8df2c96e4200e0f)
            ],
            [
                uint256(0x21c0534f145edd4bc1e130245f6bf41963b8cd14b417d0c4db7ba8f9735d42d3),
                uint256(0x10bd3e116d0b35ed5def903ae949a1393eb5e184c408a55081aa7e3b333b926b)
            ]
        );
        vk.delta = Pairing.G2Point(
            [
                uint256(0x27f0ab0caef178696fd6fb09d90e8a00173bfe02c1e3f76d8aef1370e619831e),
                uint256(0x2a074c6bdc0dfc9e367dc30aa9cb31eb3dc0153a1cba22346e5ec5fc878b0a68)
            ],
            [
                uint256(0x0ba5bd81ce69e7b2718d1bbb584040f3d366c62e6a5f289323d771ba0c6d3922),
                uint256(0x20728511d3608a8d3b5fd9bce8096fa2f0fe2b88ee0ab83399450cb17aebc238)
            ]
        );
        vk.gamma_abc = new Pairing.G1Point[](21);
        vk.gamma_abc[0] = Pairing.G1Point(
            uint256(0x06d1f3f066edaa3685eaa713dc84f6f8e290972ddfbdf195324314a319779a5f),
            uint256(0x1a8a67d666ec17570fc29c464c2762b9d67a5a305287bbd5d516a0b45ae4f412)
        );
        vk.gamma_abc[1] = Pairing.G1Point(
            uint256(0x2ee817f58bbaa79be14ded824fe3767b33c4c1ad39dc499a0b7f987b7acee450),
            uint256(0x185c93e7c0409e821fbf06f24d9480e834514541378baf2280e38a8d4c8614d5)
        );
        vk.gamma_abc[2] = Pairing.G1Point(
            uint256(0x02f38b7cd454a76186f6c661ba0d0a6b7dca24c3ee4632f82a8fcc5f622c5c3a),
            uint256(0x2f9164b5bcbd103e9556c7d7d6cf02068f1b9f1a94b625e8b19e0935b9ca1245)
        );
        vk.gamma_abc[3] = Pairing.G1Point(
            uint256(0x1ddff3df123b53f055200353a385509500f621c42658743557704fcd1e6ca74d),
            uint256(0x1005b6fb681eda6ccc9216493972ce4ac447ab3fa59fc48dca509b39b8f4f927)
        );
        vk.gamma_abc[4] = Pairing.G1Point(
            uint256(0x2abe8c68fd779d8b81e5207a7ae5e7abb3d337af69403250b658a68845bdef74),
            uint256(0x079de83604e57310eb6679287e5099d24803baece3516df10cb87bdef134ba69)
        );
        vk.gamma_abc[5] = Pairing.G1Point(
            uint256(0x2e5ce6bd54bd13c792f2ec1334605ed1becbfc5ca906ae7e916b7ad924f915f6),
            uint256(0x061e327cb02f17fc4e4a9338377d0798620515e9a442e02a1447f275318c99f7)
        );
        vk.gamma_abc[6] = Pairing.G1Point(
            uint256(0x02ff278c04ec67356f19555a59662034fb31a498a95a932d4cac448ce1c7c447),
            uint256(0x0d598846dd628da8383f85378ea45acaeb430d0dd4d0fec540baa1417700f68d)
        );
        vk.gamma_abc[7] = Pairing.G1Point(
            uint256(0x2dd6e61b04f40cc370daec918088e2dd3c89d2cb06af04e11b686824bd3e3461),
            uint256(0x1fc7511743f0e8f81fbce5026c7f915465c8b5a81e0b06f2976932d335e14991)
        );
        vk.gamma_abc[8] = Pairing.G1Point(
            uint256(0x1296d259dea3fd00b6511945b63bf6d365e3e66975356243fc1ab9dbb3d1f2e0),
            uint256(0x177089885c2e85515848cccc3da69337c1a06dbe9cc0c6431893021fbc3bf91f)
        );
        vk.gamma_abc[9] = Pairing.G1Point(
            uint256(0x03395a2aab3f0571f87445647ad793d1624d0946827be76be7502883fe596f39),
            uint256(0x0d83145bdd06c4b9f64fbe370e53e2b08b7a2896345ace4086ccd01dcbd41f1d)
        );
        vk.gamma_abc[10] = Pairing.G1Point(
            uint256(0x227c46372b26d66eb0a78bb0d539258cf00a277e6e76d15ec377fc2fb6560e0e),
            uint256(0x2052cd1ca090caa901f1336b71b7ef22d8a622cf452aeb9b3353f771d154ac9f)
        );
        vk.gamma_abc[11] = Pairing.G1Point(
            uint256(0x2b01be3a8862cea4aea8ab991b26c9c1420118c6cd5be50cb8b8686317db69a8),
            uint256(0x2358f0748bddec3b2d1a863aa69b5b3b11f9863d27a3682c040d726962a782a6)
        );
        vk.gamma_abc[12] = Pairing.G1Point(
            uint256(0x16cd4992338faf22192769a49b802e302d55361a705eb1c084743fc8bb119b63),
            uint256(0x0a7e7448e21423eaaa162f77abdb0e2e65ee6a9b2dc650ac523cf3a68b904a76)
        );
        vk.gamma_abc[13] = Pairing.G1Point(
            uint256(0x0b5dd807aca4a471d710ce43e1dfeb39aa741b65fa27da221932e3faa4ebda19),
            uint256(0x19369275e51ac64eb4d877d98ce9452db81202c98a49799efc15673f3fb4e331)
        );
        vk.gamma_abc[14] = Pairing.G1Point(
            uint256(0x0fbe80976a0913a367b8e26f7e7056b31a7d0c6c1571ff0c014a2c0e4ecd3db4),
            uint256(0x0fc790e3b5e0ef960ca88cbd6d713fbd3c53a433006bda549cdf33c75b6e536c)
        );
        vk.gamma_abc[15] = Pairing.G1Point(
            uint256(0x0a994015098da0d1cafba01cfe852ad7243ace402ed5f0e5d3c03cc4f9156216),
            uint256(0x0fcf1dc106bdf98e9f55b7787ceb3ae6a62e34c144b83a40e78e4b570a9d7824)
        );
        vk.gamma_abc[16] = Pairing.G1Point(
            uint256(0x2de67bc8186c7857cc673e4582833eb1e9816320203c4529429bf507fa6b2752),
            uint256(0x18dce1a24839ba705675f95fc2c906503358cc5c7e189c4a218b3456638d1c98)
        );
        vk.gamma_abc[17] = Pairing.G1Point(
            uint256(0x045cbe12bafa50d56858808435cfbb15e726849f78cd1ea2c9312918177d4067),
            uint256(0x161f7c300b75bdc14470f19fd41bd7edc3dcf4ce332c811d3dc6579aac5bab71)
        );
        vk.gamma_abc[18] = Pairing.G1Point(
            uint256(0x15c9a0c69520fe52e4d5bd7e69fac4b79602d20abc125a5c684b7711016efc38),
            uint256(0x2894c1d670f911cd3809b859b93786d97b58f4742b90e8d5a3aea5eb57129eef)
        );
        vk.gamma_abc[19] = Pairing.G1Point(
            uint256(0x0082c698389cd75c105e734392dc2a414481883e6ecb19180156bd34043192a1),
            uint256(0x14dbc3fa9b6b67fbcebd3b19d586aa85f2bcc9a85eec61cee5149af47e7f99ac)
        );
        vk.gamma_abc[20] = Pairing.G1Point(
            uint256(0x2209a57304df0d0084b6a785ae4c0eb99f3aac46baf6181439762edb2e5fb08b),
            uint256(0x0f3829b8d02d20e87e28c37eea5c5d19e4d43c44156fef157e060eb9f5e18443)
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
