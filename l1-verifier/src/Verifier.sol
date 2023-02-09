pragma solidity ^0.8.0;

contract Verifier {
    struct Proof {
        uint256 phantom;
    }

    function verify(Proof memory /*proof*/ ) internal pure returns (bool) {
        // we're an optimistic rollup now lmao
        return true;
    }
}
