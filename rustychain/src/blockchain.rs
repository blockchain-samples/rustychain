extern crate time;
extern crate serde;
extern crate serde_json;
extern crate sha2;

use self::sha2::{Sha256, Digest};
use self::serde::Serialize;
use std::fmt::Write;

#[derive(Debug, Clone, Serialize)]
struct Transaction {
    sender: String,
    receiver: String,  
    amount: f32,
}