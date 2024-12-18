use alloy::primitives::Address;
use clap::{Parser, Subcommand};
use lazy_static::lazy_static;
use rsmf::bcos::*;
use rsmf::contract::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Parser)]
#[command(about = "Regional Special Maintaince Fund - Administration CLI", version, long_about = None)]
struct CommandLineInterface {
    /// All operations are based on the name of the region
    name: String,
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    #[command(subcommand)]
    Region(RegionCommand),
    #[command(subcommand)]
    Fund(FundCommand),
}

#[derive(Debug, Subcommand)]
enum FundCommand {
    /// Create a fund for the selected region
    Create,
    /// List all pending proposals
    ListProposal,
    /// Show the proposal with given adderss
    ShowProposal { address: String },
    /// Approve the proposal with given adderss
    ApproveProposal { address: String },
}

#[derive(Debug, Subcommand)]
enum RegionCommand {
    Create {
        /// the address of the property manager in the region on the chain
        property_manager: Address,
        /// path to the .csv file containing owners and their addresses
        #[arg(short, long = "with-owners-file")]
        owners_file: Option<String>,
    },
    #[command(subcommand)]
    Modify(RegionModifyCommand),
}

#[derive(Debug, Subcommand)]
enum RegionModifyCommand {
    // TODO: how to use customized parser for struct Owner?
    Insert { address: Address },
    Remove { address: Address },
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
struct Config {
    regions: HashMap<String, Region>,
}

lazy_static! {
    static ref APP_NAME: &'static str = "rsmf-admin";
}

fn main() {
    let mut config: Config = confy::load(&APP_NAME, "cli").unwrap();
    let cli = CommandLineInterface::parse();
    eprintln!("{cli:?}");

    let mut session = Session::new(&APP_NAME);

    match cli.command {
        Command::Region(c) => match c {
            RegionCommand::Create {
                property_manager,
                owners_file,
            } => {
                // assert!(!config.regions.contains_key(&cli.name));
                let mut region = Region::new(&mut session, property_manager);
                if let Some(path) = owners_file {
                    let mut csv_reader = csv::Reader::from_path(path).unwrap();
                    for result in csv_reader.records() {
                        let record = result.unwrap();
                        assert!(record.len() == 2);
                        let name: String = record[0].parse().unwrap();
                        let address: Address = record[1].parse().unwrap();
                        region.insert(&mut session, address);
                    }
                }
                config.regions.insert(cli.name, region);
            }
            RegionCommand::Modify(c) => {
                let region = config.regions.get_mut(&cli.name).unwrap();
                let mut rc = false;
                match c {
                    RegionModifyCommand::Insert { address } => {
                        rc = region.insert(&mut session, address);
                        print!("Inserted");
                    }
                    RegionModifyCommand::Remove { address } => {
                        rc = region.remove(&mut session, address);
                        print!("Removed");
                    }
                }
                println!(" {} new owners.", if rc { 1 } else { 0 });
            }
        },
        Command::Fund(c) => match c {
            FundCommand::Create => {
                let region = config.regions.get_mut(&cli.name).unwrap();
                region.create_fund(&mut session).unwrap();
            }
            FundCommand::ListProposal => {
                todo!()
            }
            FundCommand::ShowProposal { address } => {
                todo!()
            }
            FundCommand::ApproveProposal { address } => {
                let address: Address = address.parse().unwrap();
                let proposal = Proposal::from(address);
                proposal.verify_and_pay(&mut session);
            }
        },
    }

    confy::store(&APP_NAME, "cli", config).unwrap();
}
