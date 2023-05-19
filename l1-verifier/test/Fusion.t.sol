// SPDX-License-Identifier: GPL3
pragma solidity ^0.8.17;

import "forge-std/Test.sol";

import "src/Fusion.sol";

contract FusionTest is Test {
    Fusion fusion;

    function setUp() public {
        fusion = new Fusion();
    }
}
