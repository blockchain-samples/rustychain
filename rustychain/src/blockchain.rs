extern crate serde;
extern crate serde_json;
extern crate sha2;
extern crate time;

use self::serde::Serialize;
use self::sha2::{Digest, Sha256};
use std::fmt::Write;

#[derive(Clone, Debug, Serialize)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: f32,
}

#[derive(Debug, Serialize)]
pub struct BlockHeader {
    timestamp: i64,
    nonce: u32,
    prev_hash: String,
    merkle_hash: String,
    difficulty: u32,
}

type transactions = Vec<Transaction>;

#[derive(Debug, Serialize)]
pub struct Block {
    header: BlockHeader,
    count: u32,
    txns: transactions,
}

pub struct BlockChain {
    chain: Vec<Block>,
    curr_txns: transactions,
    difficulty: u32,
    miner_address: String,
    reward: f32,
}
