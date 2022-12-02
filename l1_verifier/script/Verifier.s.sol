// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "forge-std/Script.sol";

import "src/L2.sol";

contract L2Script is Script {
    function setUp() public {}

    function run() public returns (L2) {
        vm.startBroadcast();

        L2 l2 = new L2();

        vm.stopBroadcast();

        return l2;
    }
}
