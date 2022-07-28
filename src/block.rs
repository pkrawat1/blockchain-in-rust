use chrono::UTC;
use log::info;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Block {
    timestamp: i64,
    data: String,
    prev_block_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(data: String, prev_block_hash: String) -> Self {
        let mut new_block = Self {
            data,
            timestamp: UTC::now().timestamp(),
            prev_block_hash,
            hash: String::default(),
        };
        new_block.hash = Block::mine_block(&new_block);
        info!("mined! hash: {}", new_block.hash);
        new_block
    }

    fn mine_block(&self) -> String {
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
