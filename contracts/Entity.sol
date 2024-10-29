// SPDX-License-Identifier: GPL-3.0-or-later

pragma solidity ^0.8.11;

import "./RegionData.sol";

enum Entity {
    Owner,
    PropertyManager,
    Administration
}

contract inRegion {
    RegionData regionData;
    address deployer;

    constructor() {
        deployer = msg.sender;
    }

    modifier only(Entity e) {
        if (e == Entity.Owner) {
            require(regionData.isOwner(msg.sender));
        } else if (e == Entity.Administration) {
            require(msg.sender == regionData.administration());
        } else if (e == Entity.PropertyManager) {
            require(msg.sender == regionData.propertyManager());
        }
        _;
    }

    modifier onlyDeployer {
        require(msg.sender == deployer);
        _;
    }

    modifier onlyRelevant() {
        require(
            msg.sender == regionData.administration() ||
                msg.sender == regionData.propertyManager() ||
                regionData.isOwner(msg.sender),
            "Only revelant legal person can operate."
        );
        _;
    }
}
