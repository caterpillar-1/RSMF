// SPDX-License-Identifier: GPL-3.0-or-later

pragma solidity ^0.8.11;

contract Value {
    address deployer;
    uint public integer = 0;
    string public str = "";
    mapping(uint => string) public map;

    constructor() {
        deployer = msg.sender;
    }

    function setInteger(uint integer_) public {
        require(msg.sender == deployer);
        integer = integer_;        
    }

    function setString(string memory str_) public {
        str = str_;
    }

    function setMap(uint idx, string memory str_) public {
        map[idx] = str_;
    }
}
