mod macros;
mod cintf;
mod rintf;

pub use rintf::*;

use serde::{Deserialize, Serialize};
use std::fmt::Debug;


pub trait Context {}

pub struct Response {
    pub error: i32,
    pub desc: String,
    pub data: String,
    pub size: usize,
    pub context: Box<dyn Context>,
}
