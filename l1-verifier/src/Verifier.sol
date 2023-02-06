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
            uint256(0x290f8bb5d79d6d0dbe1c03ca1011fa9b0b7c6ecda13f37cfba658a4907183e82),
            uint256(0x01bdd72ab2502e44350a2c689656a181d6b4296b8d5166e8985cf791214cb5dd)
        );
        vk.beta = Pairing.G2Point(
            [
                uint256(0x2097d33038a83c026f6368e2a92256591f4c2e1e4f434bc9bfb78a4f63e8d6da),
                uint256(0x288c98376de73cd6fc6244cd2f922dd5d67389b6d98ecc5a8678cec63fa33699)
            ],
            [
                uint256(0x0cadbda639bd8c91b3da3d95a0ad0d16c5ed6bbefe4239e10df63a44fe9dc7fa),
                uint256(0x0a20619d0ebf2d01922725e4636dc59f6399f23bd92f641ccac6495f4707c59a)
            ]
        );
        vk.gamma = Pairing.G2Point(
            [
                uint256(0x1699b936b058af6b03c0875a819c79f7efe5cc124e27e9b1795e0ae99f0eb17b),
                uint256(0x2d8e4ab909054c94d10faf0a9838d101307578438eff38336640ce8292297be1)
            ],
            [
                uint256(0x0ceadc89b0d21f6881e47d23c19a89d4ddf3bfd00d0a1f70cc97d35779b37c86),
                uint256(0x0335b618b04cfeffae49902bfd90b27473f5185b0875facaa446ec96c4c427f6)
            ]
        );
        vk.delta = Pairing.G2Point(
            [
                uint256(0x20c7bd11eb26046af5aaba82e57671bc5a96381e89577d120ff7e83cdeffa552),
                uint256(0x2b2a2bf9ee96069bb2d09f01ddab62728b65c37e932d5a6f5fd6b488c99785b7)
            ],
            [
                uint256(0x15d299ea897bbe8777e50a221570e48bedc74c76f0378f6a4ec5856cc9f36ec5),
                uint256(0x0368a9236c577309e63b0581be044d231a934e37c9ef3ed339e7cd398e0ea776)
            ]
        );
        vk.gamma_abc = new Pairing.G1Point[](21);
        vk.gamma_abc[0] = Pairing.G1Point(
            uint256(0x26428f1cc3497817783af1b2bbf9307742be492cbc3f39ef8c4639844b62eb20),
            uint256(0x2b7ceba4863591b34957f4a286ecb47d4917d4991789835431f0dcb16bac0930)
        );
        vk.gamma_abc[1] = Pairing.G1Point(
            uint256(0x0b876906484105f0ba685cf120171e4e6f1b8087b6ab097de676f441b073de86),
            uint256(0x2b8ab2b876a2ca288e6cc97e5c396d69504ffd4e716d39e140dad48acfe3bf9b)
        );
        vk.gamma_abc[2] = Pairing.G1Point(
            uint256(0x0ecb31ec1f0a6f5b085babcc0921e5c4683af0b9dc5e16f1d7761de291d48d68),
            uint256(0x28d1202dff883363d90ef693bf1f5672e43de0dcb364a0ca0d6c7c2d5e4e735a)
        );
        vk.gamma_abc[3] = Pairing.G1Point(
            uint256(0x0c162682fa1477599be6334337ddc979c28d8a3a1dd18edbb5731699ce962372),
            uint256(0x2c922f58408b2b1d04306262ecd018810272a9abda833e3e9e64e71185bca297)
        );
        vk.gamma_abc[4] = Pairing.G1Point(
            uint256(0x1b97c31415496279a71c1ee0c1c4a5f5a30b9818e95447b60c9b452a1dec5eb7),
            uint256(0x21ff09cece580082abb3699979c6f40dc05ed3437218e026f43962cc88e8f43a)
        );
        vk.gamma_abc[5] = Pairing.G1Point(
            uint256(0x1fd2d912092d81e9b234417ff59392701a1296764bcdee079f8d144a120ffbd6),
            uint256(0x13c6f6a981ec443978ca844e396bbed8065aac15a75c2fd02a2f96e47fbf24bb)
        );
        vk.gamma_abc[6] = Pairing.G1Point(
            uint256(0x0043e53e8692422e2e44f142ff4aaf54e53db43e55804e397e022ac3c5cecf03),
            uint256(0x00e79de0cc7688636e59a8fbd0e67b138fbb84df19513e74675ddec279b18fb4)
        );
        vk.gamma_abc[7] = Pairing.G1Point(
            uint256(0x1ac610f7bfa3fd301f668401c39e4ed65f0cfb7c7726cb1fd95aadf7e84b4348),
            uint256(0x2115d4a0c0aa5ebfc44c4a42b549b5262374037f8c9f76333429639f52577a31)
        );
        vk.gamma_abc[8] = Pairing.G1Point(
            uint256(0x2f6aa182bc65cac0c5e5f21f03f474ce44cbb5fad370c4b584063b3fc517c5fe),
            uint256(0x1b0c97e9c968beab6a66c9b81188328a286b3858b345186af759a17cde8e7c3f)
        );
        vk.gamma_abc[9] = Pairing.G1Point(
            uint256(0x0740f4076838085a6183a7cdebd811f24932c6dc041eaf7481fbe23d81667687),
            uint256(0x25e52414c179e1c77b62b5681f402e301a1c120faf66bfbc5d97e10a963cdc11)
        );
        vk.gamma_abc[10] = Pairing.G1Point(
            uint256(0x0844d8c2856eb58c5058fac08689f57716655e6b36346d6ea137e27b1aafc0c4),
            uint256(0x2d05eaa47be44f41c3b645882e640ca4453b1b2e58ff03910498086af6448852)
        );
        vk.gamma_abc[11] = Pairing.G1Point(
            uint256(0x28cd6042b707cc8f32f2a1fb2e4bdbc7ff17cb5468a12ebadb71af1c27a5fbfe),
            uint256(0x28c4395b18c54f664dad47398b8913580192ae06fdd01ec0e63fd146ce26b269)
        );
        vk.gamma_abc[12] = Pairing.G1Point(
            uint256(0x186d5236f22221b2656360e6ff5fb420f54ffa539b4df86161bc11919924f96d),
            uint256(0x2dacbcdabd151b9076c34b5d63dddce80c36a34dc013f6bd17ec2f2998778283)
        );
        vk.gamma_abc[13] = Pairing.G1Point(
            uint256(0x236f5fa8d339fd32b71b44b88b596c1013e4f527700b566429c81c625392ed74),
            uint256(0x0413e46bab46a6e337c8b91e4a516705dd48bba14eb6cdc91573684e57b55cdc)
        );
        vk.gamma_abc[14] = Pairing.G1Point(
            uint256(0x162e6a98a3aef486b657224a2734a9dce051ca497571b306baa916752a074961),
            uint256(0x1f184e0c901decb679cf320bcdc27be65c3897c0d0f34c81828a28a81007c44a)
        );
        vk.gamma_abc[15] = Pairing.G1Point(
            uint256(0x267e86e0864ab82d3d4f1b942ba1b4463dca76095f63b33f88d18ceb2a2f752e),
            uint256(0x21e125ba61a642342157a41c4aa077a9ce60a64a584bddf25bce720aac72e301)
        );
        vk.gamma_abc[16] = Pairing.G1Point(
            uint256(0x2145f45f569d56390aa4579420c41727a29cbc8bdefd9f0fc9baf07472f6f9de),
            uint256(0x254546e2d86527ce3a10f8df73f2e44b61085b7831bd776f68ef234c2649300a)
        );
        vk.gamma_abc[17] = Pairing.G1Point(
            uint256(0x2671263de6756f7c6d714a75480b9d060b4efd3fccc5d2229830396410f384a1),
            uint256(0x24f452bc36bbadc749643279a25c9dde72a5d95a1efc2e677a1738549ca28498)
        );
        vk.gamma_abc[18] = Pairing.G1Point(
            uint256(0x2dfc8e1ed39f5d65e80298d10a65d154d5f0621368eddd81a0cdeeb419d084fc),
            uint256(0x25117c0d88687d8057e88d89f2c53aa2185795daf5025e6737dd6d81e687ede7)
        );
        vk.gamma_abc[19] = Pairing.G1Point(
            uint256(0x181593ed30e38be05aac78d84dad1bb15ac222c234b505fa524bf605fcfa43ae),
            uint256(0x0ce9a2e123a9101f22b063678459d44e58574889348caaf8815fa1d146c531e6)
        );
        vk.gamma_abc[20] = Pairing.G1Point(
            uint256(0x11f37bf7bb920da5c220da0c865052d021e4bfc35ae5c46ebf26aea6952fbe43),
            uint256(0x2df9ac6050451c357616fb3964fedbc796dab1dc40fb565f8de5976dea5fa93d)
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
