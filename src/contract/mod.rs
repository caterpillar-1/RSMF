use alloy::primitives::Address;
use clap::Subcommand;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::bcos::{Contract, Session};
use std::{fs::File, io::Read, ops::Add, path::Path};

// New-type pattern

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Fund {
    address: Address,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Region {
    address: Address,
    fund: Option<Fund>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Proposal {
    address: Address,
}

#[derive(Debug, Clone, Subcommand)]
pub enum VoteChoice {
    Abort,
    Approve,
    Disapprove,
}

pub enum ProposalStatus {
    Draft,
    Voted,
    VerifiedAndPaid,
    Withdrawn,
}

fn read_file(f: &str) -> String {
    let mut buf = String::new();
    File::open(f).unwrap().read_to_string(&mut buf).unwrap();
    buf
}

impl Region {
    pub fn new(session: &mut Session, property_manager: Address) -> Self {
        let address = session
            .deploy(&RegionContract.clone(), &json!([property_manager]))
            .unwrap();
        let region = Region {
            address,
            fund: None,
        };
        session.call(&region, "init", &json!([property_manager]));
        region
    }

    pub fn insert(&mut self, session: &mut Session, new_owner: Address) -> bool {
        let result = session.call(self, "insert", &json!([new_owner]));
        eprintln!("Region::insert: result is {}.", result);
        result.as_array().unwrap()[0].as_bool().unwrap()
    }

    pub fn remove(&mut self, session: &mut Session, owner: Address) -> bool {
        let result = session.call(self, "remove", &json!([owner]));
        eprintln!("Region::insert: result is {}.", result);
        result.as_array().unwrap()[0].as_bool().unwrap()
    }

    pub fn create_fund(&mut self, session: &mut Session) -> Result<(), ()> {
        match &self.fund {
            Some(_) => Err(()),
            None => {
                self.fund = Some(Fund::new(session, &self));
                Ok(())
            }
        }
    }
}

impl From<Address> for Region {
    fn from(value: Address) -> Self {
        Self {
            address: value,
            fund: None,
        }
    }
}

impl Fund {
    pub fn new(session: &mut Session, region: &Region) -> Fund {
        let address = session
            .deploy(&ProposalContract.clone(), &json!([]))
            .unwrap();
        let fund = Self { address };
        session.call(&fund, "init", &json!([region.address()]));
        fund
    }

    pub fn propose(&mut self, session: &mut Session, name: &str, signature: &str, amount: usize) -> Address
    {
        let address = session
            .deploy(&FundContract.clone(), &json!([name, signature, amount]))
            .unwrap();
        address
    }
}

impl From<Address> for Fund {
    fn from(value: Address) -> Self {
        Self { address: value }
    }
}

impl Proposal {
    pub fn name(&self, session: &mut Session) -> String {
        let result = session.call(self, "name", &json!([]));
        result.as_array().unwrap()[0].as_str().unwrap().to_owned()
    }

    pub fn proposer(&self, session: &mut Session) -> Address {
        let result = session.call(self, "proposer", &json!([]));
        result.as_array().unwrap()[0].as_str().unwrap().parse().unwrap()
    }

    pub fn signature(&self, session: &mut Session) -> String {
        let result = session.call(self, "signature", &json!([]));
        result.as_array().unwrap()[0].as_str().unwrap().to_owned()
    }

    pub fn url(&self, session: &mut Session) -> String {
        let result = session.call(self, "url", &json!([]));
        result.as_array().unwrap()[0].as_str().unwrap().to_owned()
    }

    pub fn amount(&self, session: &mut Session) -> usize {
        let result = session.call(self, "amount", &json!([]));
        result.as_array().unwrap()[0].as_str().unwrap().parse().unwrap()
    }

    pub fn status(&self, session: &mut Session) -> ProposalStatus {
        let result = session.call(self, "amount", &json!([]));
        match result.as_array().unwrap()[0].as_u64().unwrap() {
            0 => ProposalStatus::Draft,
            1 => ProposalStatus::Voted,
            2 => ProposalStatus::VerifiedAndPaid,
            3 => ProposalStatus::Withdrawn,
            _ => panic!("error reading proposal status")
        }
    }

    pub fn vote(&self, session: &mut Session, c: VoteChoice) {
        let c = match c {
            VoteChoice::Abort => 0,
            VoteChoice::Approve => 1,
            VoteChoice::Disapprove => 2,
        };
        session.call(self, "vote", &json!([c]));
    }

    pub fn verify_and_pay(&self, session: &mut Session) {
        session.call(self, "verifyAndPay", &json!([]));
    }

    pub fn withdraw(&self, session: &mut Session) {
        session.call(self, "withdraw", &json!([]));
    }
}

impl From<Address> for Proposal {
    fn from(value: Address) -> Self {
        Self { address: value }
    }
}

impl Contract for Fund {
    fn address(&self) -> Address {
        self.address
    }
    fn abi(&self) -> String {
        read_file("./build/Fund.abi")
    }

    fn bin(&self) -> String {
        read_file("./build/Fund.bin")
    }
}

impl Contract for Region {
    fn address(&self) -> Address {
        self.address
    }
    fn abi(&self) -> String {
        read_file("./build/RegionData.abi")
    }

    fn bin(&self) -> String {
        read_file("./build/RegionData.bin")
    }
}

impl Contract for Proposal {
    fn address(&self) -> Address {
        self.address
    }
    fn abi(&self) -> String {
        read_file("./build/Proposal.abi")
    }

    fn bin(&self) -> String {
        read_file("./build/Proposal.bin")
    }
}

lazy_static! {
    static ref RegionContract: Region = Region::default();
    static ref FundContract: Fund = Fund::default();
    static ref ProposalContract: Proposal = Proposal::default();
}
