pragma solidity ^0.8.0;

import {Verifier} from "src/Verifier.sol";

contract Trollup is Verifier {
    uint256 public root;

    function submitBlock(Proof memory proof, uint256[] memory input) external {
        require(input.length == 20);

        // The proof's pre-state's root must be the current root in this contract.
        require(root == input[0]);

        // A valid SNARK returns 0.
        require(verify(input, proof) == 0);

        // Update the canonical root with the proof's post-state's root.
        root = input[1];
    }
}
