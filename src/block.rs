use chrono::UTC;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::fmt;

const DIFFICULTY_PREFIX: &str = "00";

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub id: u64,
    pub timestamp: i64,
    pub data: String,
    pub prev_block_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(id: u64, data: String, prev_block_hash: String) -> Self {
        let mut new_block = Self {
            id,
            data,
            timestamp: UTC::now().timestamp(),
            prev_block_hash,
            hash: String::default(),
            nonce: 0,
        };
        let (nonce, hash) = new_block.mine_block();
        new_block.hash = hash;
        new_block.nonce = nonce;
        info!("mined! hash: {}", new_block.hash);
        new_block
    }

    pub fn block_hash(&self) -> Vec<u8> {
        let data = json!({
            "id": self.id,
            "prev_block_hash": self.prev_block_hash,
            "data": self.data,
            "timestamp": self.timestamp,
            "nonce": self.nonce
        });

        let mut hasher = Sha256::new();
        hasher.update(data.to_string().as_bytes());
        hasher.finalize().as_slice().to_owned()
    }

    fn mine_block(&mut self) -> (u64, String) {
        info!("mining in progress");

        let mut nonce = self.nonce;

        loop {
            if nonce % 100000 == 0 {
                info!("nonce: {}", nonce);
            }
            self.nonce = nonce;
            let hash = self.block_hash();
            let binary_hash = hash_to_binary_representation(&hash);
            if binary_hash.starts_with(DIFFICULTY_PREFIX) {
                info!(
                    "mined! nonce: {}, hash: {}, binary hash: {}",
                    nonce,
                    hex::encode(&hash),
                    binary_hash
                );
                return (nonce, hex::encode(&hash));
            }
            nonce += 1;
        }
    }
}

fn hash_to_binary_representation(hash: &[u8]) -> String {
    let mut res: String = String::default();
    for c in hash {
        res.push_str(&format!("{:b}", c));
    }
    res
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pretty_json = serde_json::to_string_pretty(&self).expect("failed to parse blocks");
        write!(f, "{}", pretty_json)
    }
}

impl Blockchain {
    pub fn new() -> Self {
        Self { blocks: vec![] }
    }

    pub fn genesis(&mut self) {
        self.blocks
            .push(Block::new(0, "genesis".into(), "genesis".into()));
    }

    pub fn add_block(&mut self, block: Block) {
        let latest_block = self.blocks.last().expect("missing genesis block");
        if self.block_valid(&block, &latest_block) {
            self.blocks.push(block);
        } else {
            error!("Block[id:{}] is not a valid block", block.id);
        }
    }

    pub fn print_blockchain(&self) {
        info!("---Blockchain start---");
        println!("{}", self);
        info!("---Blockchain end---");
    }

    fn block_valid(&self, block: &Block, prev_block: &Block) -> bool {
        if block.prev_block_hash != prev_block.hash {
            warn!("Block[id:{}] has invalid prev hash", block.id);
            return false;
        } else if !hash_to_binary_representation(
            &hex::decode(&block.hash).expect("can't decode from hex"),
        )
        .starts_with(DIFFICULTY_PREFIX)
        {
            warn!("block with id: {} has invalid difficulty", block.id);
            return false;
        } else if block.id != prev_block.id + 1 {
            warn!(
                "Block[id:{}] does not come after Block[id:{}]",
                block.id, prev_block.id
            );
            return false;
        } else if hex::encode(block.block_hash()) != block.hash {
            warn!("Block[id:{}] has invalid hash", block.id);
            return false;
        }
        true
    }
}

impl fmt::Display for Blockchain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pretty_json =
            serde_json::to_string_pretty(&self.blocks).expect("failed to parse blocks");
        write!(f, "{}", pretty_json)
    }
}
