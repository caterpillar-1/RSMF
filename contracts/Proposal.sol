// SPDX-License-Identifier: GPL-3.0-or-later

pragma solidity ^0.8.11;

import "./Entity.sol";
import "./RegionData.sol";

contract Proposal is inRegion {
    enum VoteChoices {
        Abort,
        Approve,
        Disapprove
    }

    enum Status {
        /// an owner / pm proposes
        Draft,
        /// contract can safely assume the result of the vote (2/3)
        Voted,
        /// The administration has verified the proposal and transfered the fund into the proposal contract.
        VerifiedAndPaid,
        /// pm has withdrawn the money and start execution.
        Withdrawn
    }

    string public name;
    address public proposer;
    bytes public signature;
    string public url;
    uint amount;

    mapping(address => VoteChoices) choices;

    uint public numberOfApprove;
    uint public numberOfDisapprove;

    Status public status = Status.Draft;

    // TODO: We need a snapshot of regionData at construction.
    constructor(
        string memory name_,
        address proposer_,
        bytes memory signature_,
        string memory url_,
        uint amount_,
        RegionData regionData_
    ) {
        name = name_;
        proposer = proposer_;
        signature = signature_;
        url = url_;
        amount = amount_;
        regionData = regionData_;
    }

    modifier when(Status s) {
        require(status == s);
        _;
        if (s == Status.Draft) {
            if (numberOfApprove * 3 >= regionData.numberOfOwner() * 2) {
                status = Status.Voted;
            }
        } else if (s == Status.Voted) {
            status = Status.VerifiedAndPaid;
        } else if (s == Status.VerifiedAndPaid) {
            status = Status.Withdrawn;
        }
    }

    function vote(VoteChoices c) public only(Entity.Owner) when(Status.Draft) {
        VoteChoices o = choices[msg.sender];
        if (c == VoteChoices.Abort) {
            if (o == VoteChoices.Approve) {
                numberOfApprove -= 1;
            } else if (o == VoteChoices.Disapprove) {
                numberOfDisapprove -= 1;
            }
        } else if (c == VoteChoices.Approve) {
            if (o == VoteChoices.Abort) {
                numberOfApprove += 1;
            } else if (o == VoteChoices.Disapprove) {
                numberOfApprove += 1;
                numberOfDisapprove -= 1;
            }
        } else if (c == VoteChoices.Disapprove) {
            if (o == VoteChoices.Abort) {
                numberOfDisapprove += 1;
            } else if (o == VoteChoices.Approve) {
                numberOfApprove -= 1;
                numberOfDisapprove += 1;
            }
        } else {
            require(false, "Invalid VoteChoice");
        }
    }

    function verifyAndPay()
        public
        payable
        only(Entity.Administration)
        when(Status.Voted)
    {
        require(msg.value >= amount);
        if (msg.value > amount)
            payable(msg.sender).transfer(msg.value - amount);
    }

    function withdraw()
        public
        only(Entity.PropertyManager)
        when(Status.VerifiedAndPaid)
    {
        payable(msg.sender).transfer(amount);
    }
}
