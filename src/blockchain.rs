extern crate serde;
extern crate serde_json;
extern crate sha2;

use colored::Colorize;
use sha2::{Digest, Sha256};
use std::fmt::Write;

#[derive(Debug, Clone, Serialize)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: f32,
}

#[derive(Debug, Serialize)]
pub struct BlockHeader {
    timestamp: i64,
    nonce: u32,        //number of time loop ran until it found the solution
    prev_hash: String, //Previous BlockHash
    merkle: String,
    difficulty: u32,
}

#[derive(Debug, Serialize)]
pub struct Block {
    header: BlockHeader,
    count: u32,                     // number of transactions in the block
    transactions: Vec<Transaction>, // transactions in the block
}

pub struct Chain {
    chain: Vec<Block>,
    curr_trans: Vec<Transaction>,
    difficulty: u32,
    miner_addr: String,
    reward: f32,
}

impl Chain {
    /// new method generates a new chain and generates a new block in the process while return that new chain as a result
    pub fn new(miner_addr: String, difficulty: u32) -> Chain {
        let mut chain = Chain {
            chain: Vec::new(),
            curr_trans: Vec::new(),
            difficulty,
            miner_addr,
            reward: 100.00,
        };
        chain.generate_new_block();
        chain
    }

    /// as suggest this generates a new transaction
    pub fn new_transaction(&mut self, sender: String, receiver: String, amount: f32) -> bool {
        self.curr_trans.push(Transaction {
            sender,
            receiver,
            amount,
        });

        true
    }

    /// this return the hash of last block header in the chain
    pub fn last_block_hash(&self) -> String {
        let block = match self.chain.last() {
            Some(block) => block,
            None => return String::from_utf8(vec![48; 64]).unwrap(), //Genesis Block
        };
        Chain::hash(&block.header)
    }

    /// updates "mining" difficulty
    pub fn update_difficulty(&mut self, difficulty: u32) -> bool {
        self.difficulty = difficulty;
        true
    }

    /// update rewards for miners mining a block this can be used in halving process
    pub fn update_reward(&mut self, reward: f32) -> bool {
        self.reward = reward;
        true
    }
    /// generates a new block
    pub fn generate_new_block(&mut self) -> bool {
        // new header
        let header = BlockHeader {
            timestamp: std::time::Instant::now().elapsed().as_secs() as i64,
            nonce: 0,
            merkle: String::new(),
            prev_hash: self.last_block_hash(),
            difficulty: self.difficulty,
        };
        //  reward transaction for mining the block
        let reward_trans = Transaction {
            sender: String::from("Root"), //if no sender root
            receiver: self.miner_addr.clone(),
            amount: self.reward,
        };

        // new block
        let mut block: Block = Block {
            header,
            count: 0,
            transactions: vec![],
        };
        block.transactions.push(reward_trans); //push the new transaction
        block.transactions.append(&mut self.curr_trans); // append previous transactions as mutable;
        block.count = block.transactions.len() as u32; // find the number of transactions
        block.header.merkle = Chain::get_merkle(block.transactions.clone()); // create a merkle hash for all the transaction in the block
        Chain::proof_of_work(&mut block.header); //do a proof of work i.e miners show they hAVE MINED by finding a value of the nonce while validating transactions
        println!("{:#?}", &block);
        self.chain.push(block);
        true
    }

    /// FUNCTION FOR GENERATING MERKLE HASH FOR TRANSACTIONS
    fn get_merkle(curr_trans: Vec<Transaction>) -> String {
        let mut merkle = Vec::new();
        for t in &curr_trans {
            let hash = Chain::hash(t);
            merkle.push(hash);
        }

        if merkle.len() % 2 == 1 {
            let last = merkle.last().clone().unwrap();
            merkle.push(last.to_string());
        }

        while merkle.len() > 1 {
            let mut h1 = merkle.remove(0); //get the first hash in the merkle a.k.a h1
            let mut h2 = merkle.remove(0); //get the first hash in the merkle a.k.a h2
            h1.push_str(&mut h2);
            let nh = Chain::hash(&h1);
            merkle.push(nh)
        }

        merkle.pop().unwrap()
    }
    ///finds the block header hash depending with the difficulty and returns that block
    pub fn proof_of_work(header: &mut BlockHeader) {
        loop {
            let hash = Chain::hash(header);
            let slice = &hash[..header.difficulty as usize];
            match slice.parse::<u32>() {
                Ok(val) => {
                    if val != 0 {
                        header.nonce += 1;
                    } else {
                        println!("Block hash: {}", hash.bright_yellow());
                        break;
                    }
                }
                Err(_) => {
                    header.nonce += 1;
                    continue;
                }
            }
        }
    }

    ///this the function that hash a given value and return the hash a a String
    pub fn hash<T: serde::Serialize>(item: &T) -> String {
        let input = serde_json::to_string(&item).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let res = hasher.finalize();
        let vec_res = res.to_vec();

        Chain::hex_to_string(vec_res.as_slice())
    }
    pub fn hex_to_string(vec_res: &[u8]) -> String {
        let err_msg = "unable to write!".bright_red();
        let mut s = String::new();
        for b in vec_res {
            write!(&mut s, "{:x}", b).expect(&err_msg.to_string());
        }

        s
    }
}
