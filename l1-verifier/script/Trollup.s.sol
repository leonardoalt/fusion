// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "forge-std/Script.sol";

import "src/Trollup.sol";

contract VerifierScript is Script {
    function setUp() public {}

    function run() public returns (Trollup) {
        vm.startBroadcast();

        Trollup t = new Trollup();

        vm.stopBroadcast();

        return t;
    }
}
