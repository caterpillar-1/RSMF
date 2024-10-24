pub mod bcos;

use lazy_static::lazy_static;
use confy;
use serde::{Serialize, Deserialize};
use alloy::primitives::*;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    address: Address,
    regions: Vec<Region>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Owner {
    name: String,
    address: Address,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Region {
    address: Address,
}

// impl Region {
//     fn new(address: Address) -> Self {
// 
//     }
// 
//     fn address(&self) -> Address {
//         self.address
//     }
// 
//     fn create() -> Self {
//         
//     }
// 
//     fn destory(self) {
// 
//     }
// }

lazy_static! {
    pub static ref CONFIG: Config = confy::load("rsmf-admin", None).unwrap();
}
