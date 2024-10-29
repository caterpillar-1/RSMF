// SPDX-License-Identifier: GPL-3.0-or-later

pragma solidity ^0.8.11;

contract RegionData {
    address deployer;
    address public administration;
    address public propertyManager;

    uint public numberOfOwner;
    mapping(address => bool) public isOwner;

    constructor() {
        deployer = msg.sender;
    }

    function init(address propertyManager_) public {
        require(msg.sender == deployer);
        administration = msg.sender;
        propertyManager = propertyManager_;
    }

    modifier onlyAdministration() {
        require(msg.sender == administration);
        _;
    }

    function insert(
        address newOwner
    ) public onlyAdministration returns (bool rc) {
        rc = !isOwner[newOwner];
        if (rc) {
            numberOfOwner += 1;
            isOwner[newOwner] = true;
        }
    }

    function remove(address owner) public onlyAdministration returns (bool rc) {
        rc = isOwner[owner];
        if (rc) {
            numberOfOwner -= 1;
            isOwner[owner] = false;
        }
    }
}
