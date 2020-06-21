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

type Transactions = Vec<Transaction>;

#[derive(Debug, Serialize)]
pub struct Block {
    header: BlockHeader,
    count: u32,
    txns: Transactions,
}

pub struct BlockChain {
    chain: Vec<Block>,
    curr_txns: Transactions,
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

    pub fn hash<T: Serialize>(item: &T) -> String {
        let input = serde_json::to_string(&item).unwrap();

        let mut hasher = Sha256::default();
        hasher.input(input.as_bytes());
        let vec_res = hasher.result().to_vec();

        BlockChain::hex_to_string(vec_res.as_slice())
    }

    pub fn hex_to_string(vec_res: &[u8]) -> String {
        let mut s = String::new();
        for b in vec_res {
            write!(&mut s, "{:x}", b).expect("Unable to write bytes!");
        }
        s
    }

    pub fn proof_of_work(header: &mut BlockHeader) {
        loop {
            let hash = BlockChain::hash(header);
            let slice = &hash[..header.difficulty as usize];
            match slice.parse::<u32>() {
                Ok(val) => {
                    if val != 0 {
                        header.nonce += 1;
                    } else {
                        println!("Block hash: {}", hash);
                        break;
                    }
                }
                Err(_) => {
                    header.nonce += 1;
                    continue;
                }
            };
        }
    }

    pub fn set_difficulty(&mut self, difficulty: u32) -> bool {
        self.difficulty = difficulty;
        true
    }

    pub fn set_reward(&mut self, reward: f32) -> bool {
        self.reward = reward;
        true
    }

    pub fn generate_new_block(&mut self) -> bool {
        let header = BlockHeader {
            timestamp: time::now().to_timespec().sec,
            nonce: 0,
            prev_hash: self.last_hash(),
            merkle_hash: String::new(),
            difficulty: self.difficulty,
        };

        let reward_txn = Transaction {
            sender: String::from("Root"),
            receiver: self.miner_address.clone(),
            amount: self.reward,
        };

        let mut block = Block {
            header,
            count: 0,
            txns: vec![],
        };

        block.txns.push(reward_txn);
        block.txns.append(&mut self.curr_txns);
        block.count = block.txns.len() as u32;
        block.header.merkle_hash = BlockChain::generate_merkle_hash(block.txns.clone());
        BlockChain::proof_of_work(&mut block.header);

        println!("{:#?}", &block);
        self.chain.push(block);
        true
    }

    pub fn last_hash(&self) -> String {
        let block = match self.chain.last() {
            Some(block) => block,
            None => return String::from_utf8(vec![48; 64]).unwrap(),
        };
        BlockChain::hash(&block.header)
    }

    fn generate_merkle_hash(curr_trans: Vec<Transaction>) -> String {
        let mut merkle = Vec::new();

        for t in &curr_trans {
            let hash = BlockChain::hash(t);
            merkle.push(hash);
        }

        if merkle.len() % 2 == 1 {
            let last = merkle.last().cloned().unwrap();
            merkle.push(last);
        }

        while merkle.len() > 1 {
            let mut h1 = merkle.remove(0);
            let mut h2 = merkle.remove(0);
            h1.push_str(&mut h2);
            let nh = BlockChain::hash(&h1);
            merkle.push(nh);
        }
        merkle.pop().unwrap()
    }

    pub fn new_transaction(&mut self, sender: String, receiver: String, amount: f32) -> bool {
        self.curr_txns.push(Transaction {
            sender,
            receiver,
            amount,
        });

        true
    }
}
