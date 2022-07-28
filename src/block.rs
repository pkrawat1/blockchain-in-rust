use chrono::UTC;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::fmt;

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
}

impl Block {
    pub fn new(id: u64, data: String, prev_block_hash: String) -> Self {
        let mut new_block = Self {
            id,
            data,
            timestamp: UTC::now().timestamp(),
            prev_block_hash,
            hash: String::default(),
        };
        new_block.hash = new_block.block_hash();
        info!("mined! hash: {}", new_block.hash);
        new_block
    }

    fn block_hash(&self) -> String {
        let data = json!({
            "prev_block_hash": self.prev_block_hash,
            "data": self.data,
            "timestamp": self.timestamp,
        });

        let mut hasher = Sha256::new();
        hasher.update(data.to_string().as_bytes());

        format!("{:x}", hasher.finalize())
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}, {}, {}, {})",
            self.timestamp, self.data, self.prev_block_hash, self.hash
        )
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
        let pretty_json =
            serde_json::to_string_pretty(&self.blocks).expect("failed to parse blocks");
        info!("{}", pretty_json);
        info!("---Blockchain end---");
    }

    fn block_valid(&self, block: &Block, prev_block: &Block) -> bool {
        if block.prev_block_hash != prev_block.hash {
            warn!("Block[id:{}] has invalid prev hash", block.id);
            return false;
        } else if block.id != prev_block.id + 1 {
            warn!(
                "Block[id:{}] does not come after Block[id:{}]",
                block.id, prev_block.id
            );
            return false;
        } else if block.hash != Block::block_hash(&block) {
            warn!("Block[id:{}] has invalid hash", block.id);
            return false;
        }
        true
    }
}
