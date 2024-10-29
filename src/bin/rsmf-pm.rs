use alloy::primitives::Address;
use anyhow::{bail, Result};
use clap::Parser;
use lazy_static::lazy_static;
use rsmf::bcos::*;
use rsmf::contract::*;
use secp256k1::Message;
use serde::{Deserialize, Serialize};
// use url::Url;
use reqwest::Url;
use rpassword;
use secp256k1::{
    self, generate_keypair,
    hashes::{sha256, Hash},
    PublicKey, SecretKey,
};
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

lazy_static! {
    static ref APP_NAME: &'static str = "rsmf-pm";
}

#[derive(Debug, Parser)]
#[command(about = "Regional Special Maintaince Fund - Property Manager CLI", version, long_about = None)]
enum Command {
    /// Create a new proposal named [NAME] with local file [FILE], then get its address.
    Create {
        name: String,
        amount: usize,
        filename: PathBuf,
    },
    Withdraw {
        address: Address,
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct Config {
    /// name of the owner
    name: Option<String>,
    /// Internet address of the owner committee server
    server: Option<String>,
    /// token to access the community server
    token: Option<String>,
    /// keypair to sign proposal documents
    keypair: Option<(SecretKey, PublicKey)>,
    /// address of the Fund contract in the region
    fund: Option<Address>,
    /// proposals created by the current owner
    proposals: Vec<Proposal>,
}

fn check_config(config: &mut Config) -> Result<()> {
    type Checker<T> = fn(&T) -> Result<()>;
    fn fill_param<T>(param: &mut Option<T>, prompt: &str) -> Result<()>
    where
        T: FromStr,
    {
        if param.is_some() {
            return Ok(());
        }
        print!("{}: ", prompt);
        let stdin = std::io::stdin();
        std::io::stdout().flush()?;
        let mut buf = String::new();
        stdin.read_line(&mut buf)?;
        match T::from_str(buf.trim()) {
            Ok(value) => {
                *param = Some(value);
                Ok(())
            }
            Err(_) => bail!("Failed to parse input"),
        }
    }

    let params = [
        ("Your name", &mut config.name),
        ("Server url", &mut config.server),
    ];
    for (prompt, param) in params {
        fill_param(param, prompt)?;
    }

    let params = [("Fund address", &mut config.fund)];
    for (prompt, param) in params {
        fill_param(param, prompt)?;
    }

    if config.keypair.is_none() {
        config.keypair = Some(generate_keypair(&mut rand::thread_rng()))
    }

    if config.token.is_none() {
        config.token = Some(rpassword::prompt_password(
            "Token (to login community server): ",
        )?)
    }

    Ok(())
}

fn check_url(s: &str) -> Result<Url> {
    match Url::parse(s) {
        Ok(u) => Ok(u),
        Err(e) => bail!(e),
    }
}

fn main() -> Result<()> {
    let cli = Command::parse();
    let mut config: Config = confy::load(&APP_NAME, "cli")?;

    check_config(&mut config)?;
    let server = check_url(&config.server.as_ref().unwrap())?;

    // connect to blockchain
    let mut session = Session::new(&APP_NAME);
    let mut fund = Fund::from(config.fund.unwrap());

    // connect to region server
    let client = reqwest::blocking::Client::new();

    match cli {
        Command::Create {
            name,
            amount,
            filename,
        } => {
            assert!(filename.is_file());
            let mut file = File::open(&filename)?;
            let mut buf = Vec::new();
            file.read_to_end(&mut buf)?;
            let digest = secp256k1::hashes::sha256::Hash::hash(&buf);
            let message = Message::from_digest(digest.to_byte_array());
            let signature = config.keypair.unwrap().0.sign_ecdsa(message);
            println!(
                "File signature signed by your private key: '{}'.",
                signature
            );
            let base = server.join(&session.address().to_string())?;
            let file_url = base.join(filename.file_name().unwrap().to_str().unwrap())?;
            let key_url = base.join(".bin")?;
            client
                .put(file_url.clone())
                .bearer_auth(config.token.as_ref().unwrap())
                .body(buf)
                .send()?;
            println!(
                "File '{}' has been uploaded to the community server successfully at '{}'.",
                filename.display(),
                file_url
            );

            client
                .put(key_url.clone())
                .bearer_auth(config.token.as_ref().unwrap())
                .body(config.keypair.unwrap().1.to_string())
                .send()?;
            println!(
                "Your public key has been uploaded to the community server successfully at {}.",
                key_url
            );
            let address = fund.propose(&mut session, &name, &signature.to_string(), amount);
            println!(
                "New proposal has been created at {} on blockchain.",
                address
            );
        }
        Command::Withdraw { address } => {
            let proposal = Proposal::from(address);
            proposal.withdraw(&mut session);
        }
    }

    confy::store(&APP_NAME, "cli", config)?;

    Ok(())
}
