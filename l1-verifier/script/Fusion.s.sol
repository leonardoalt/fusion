// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "forge-std/Script.sol";

import "src/Fusion.sol";

contract VerifierScript is Script {
    function setUp() public {}

    function run() public returns (Fusion) {
        vm.startBroadcast();

        Fusion t = new Fusion();

        vm.stopBroadcast();

        return t;
    }
}
