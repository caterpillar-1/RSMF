use alloy::primitives::{Address, U256};
use alloy::transports::http::reqwest::header::VacantEntry;
use lazy_static::lazy_static;
use rand::Rng;
use rsmf::bcos::Contract;
use rsmf::bcos::Session;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use std::fs::File;
use std::io::Read;
use rand;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct ValueContract {
    address: Address
}

impl ValueContract {
    fn read_file(f: &str) -> String {
        let mut buf = String::new();
        File::open(f)
            .unwrap()
            .read_to_string(&mut buf)
            .unwrap();
        buf
    }

    fn new(session: &mut Session) -> Self {
        Self {
            address: session.deploy(&VALUE_CONTRACT.clone(), &json!([])).unwrap()
        }
    }

    fn set_integer(&mut self, session: &mut Session, x: u64) {
        session.call(self, "setInteger", &json!([x]));
    }
    fn set_string(&mut self, session: &mut Session, s: &str) {
        session.call(self, "setString", &json!([s]));
    }

    fn set_map(&mut self, session: &mut Session, idx: u64, s: &str) {
        session.call(self, "setMap", &json!([idx, s]));
    }
    fn get_integer(&self, session: &mut Session) -> u64 {
        let result = session.call(self, "integer", &json!([]));
        eprintln!("{}", result);
        result.as_array().unwrap()[0].as_str().unwrap().parse().unwrap()
    }

    fn get_string(&self, session: &mut Session) -> String {
        let result = session.call(self, "str", &json!([]));
        eprintln!("{}", result);
        result.as_array().unwrap()[0].as_str().unwrap().to_owned()
    }

    fn get_map(&self, session: &mut Session, idx: u64) -> String {
        let result = session.call(self, "map", &json!([idx]));
        eprintln!("{}", result);
        result.as_array().unwrap()[0].as_str().unwrap().to_owned()
    }
}

lazy_static! {
    static ref VALUE_CONTRACT: ValueContract = ValueContract::default();
}

impl Contract for ValueContract {
    fn address(&self) -> alloy::primitives::Address {
        self.address
    }
    fn abi(&self) -> String {
        Self::read_file("./build/Value.abi")
    }

    fn bin(&self) -> String {
        Self::read_file("./build/Value.bin")
    }
}

// fn main() {
//     let mut session = Session::new("rsmf-test");
//     println!("{}", session.version());
//     let mut value = 42;
//     let value_contract = ValueContract::new(&mut session, value);
//     // value = value_contract.get(&mut session);
//     // println!("{}", value);
// 
//     for _ in 0..1000 {
//         value = rand::random();
//         value_contract.set(&mut session, value);
//         let result = value_contract.get(&mut session);
//         assert_eq!(value, result);
//     }
// }


// fn main() {
//     let s: String = "aaabbb".to_owned();
//     println!("{:?}", Address::default().to_string());
//     println!("{:?}", s);
// }

fn main() {
    let mut session = Session::new("rsmf-test");
    println!("{}", session.version());
    let mut value_contract = ValueContract::new(&mut session);
    let mut i: u64 = 0;
    let mut s = String::new();
    let mut idx: u64 = 0;
    for _ in 0..20 {
        value_contract.set_integer(&mut session, i);
        value_contract.set_string(&mut session, &s);
        value_contract.set_map(&mut session, idx, &s);
        let ri = value_contract.get_integer(&mut session);
        let rs = value_contract.get_string(&mut session);
        let rsm = value_contract.get_map(&mut session, idx);
        // println!("{}", ri);
        // println!("{}", rs);
        // println!("{}", rsm);
        assert_eq!(i, ri);
        assert_eq!(s, rs);
        assert_eq!(s, rsm);
        i = rand::random();
        s = rand::thread_rng().sample_iter(&rand::distributions::Alphanumeric).take(16).map(char::from).collect();
        idx = rand::random();
    }
}
