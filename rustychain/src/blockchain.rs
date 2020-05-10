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

impl BlockChain {
    pub fn new(miner_address: String, difficulty: u32) -> Self {
        let mut chain = BlockChain {
            chain: Vec::new(),
            curr_txns: Vec::new(),
            difficulty,
            miner_address,
            reward: 100f32,
        };
        chain.generate_new_block();

        chain
    }

    pub fn generate_new_block(&mut self) -> bool {
        let header = BlockHeader {
            timestamp: time::now().to_timespec().sec,
            nonce: 0,
            prev_hash: self.last_hash(),
            merkle_hash: String::new(),
            difficulty: self.difficulty
        };

        let reward_trans = Transaction {
            sender: String::from("Root"),
            receiver: self.miner_address.clone(),
            amount: self.reward
        };

        let mut block = Block {
            header,
            count: 0,
            txns: vec![]
        };

        true
    }
}