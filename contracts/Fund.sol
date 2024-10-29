// SPDX-License-Identifier: GPL-3.0-or-later

pragma solidity ^0.8.11;

import "./Entity.sol";
import "./RegionData.sol";
import "./Proposal.sol";

contract Fund is inRegion {
    uint public numberOfProposals;
    Proposal[] public proposals;

    function init(RegionData regionData_) public onlyDeployer {
        regionData = regionData_;
        assert(msg.sender == regionData.administration());
    }

    function propose(
        string memory name,
        bytes memory signature,
        uint amount
    ) public onlyRelevant returns (Proposal p) {
        p = new Proposal(name, msg.sender, signature, amount, regionData);
        proposals[numberOfProposals++] = p;
    }
}
