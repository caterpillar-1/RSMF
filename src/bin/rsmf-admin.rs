use alloy::primitives::Address;
use anyhow::Error;
use clap::{Parser, Subcommand};
use confy;
use rsmf::Region;
use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Debug, Parser)]
#[command(about, version, long_about = None)]
struct CommandLineInterface {
    /// All operations are based on the name of the region
    name: String,
    /// Path to the custom config file path
    #[arg(short, long)]
    config: Option<String>,
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
    Insert { name: String, address: Address },
    Remove { name: String, address: Address },
}

fn main() -> Result<(), Error> {
    let cli = CommandLineInterface::parse();
    eprintln!("{cli:?}");

    match cli.command {
        Command::Region(c) => match c {
            RegionCommand::Create { owners_file } => {}
            RegionCommand::Modify(c) => match c {
                RegionModifyCommand::Insert { name, address } => {}
                RegionModifyCommand::Remove { name, address } => {}
            },
        },
        Command::Fund(c) => match c {
            FundCommand::Create => {}
            FundCommand::ListProposal => {}
            FundCommand::ShowProposal { address } => {}
            FundCommand::ApproveProposal { address } => {}
        },
    }

    // confy::store("rsmf-admin", None, config)?;
    Ok(())
}
