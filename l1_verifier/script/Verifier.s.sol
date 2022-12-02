// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "forge-std/Script.sol";

import "src/Verifier.sol";

contract VerifierScript is Script {
    function setUp() public {}

    function run() public returns (Verifier) {
        vm.startBroadcast();

        Verifier verifier = new Verifier();

        vm.stopBroadcast();

        return verifier;
    }
}
