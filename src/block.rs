use std::time::{SystemTime, UNIX_EPOCH};
use std::fmt;

pub struct Block {
  timestamp: u64,
  data: String,
  prev_block_hash: String,
  hash: String
}

impl Block {
  pub fn new(data: String, prev_block_hash: String) -> Self {
    Self {
      data,
      timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64,
      prev_block_hash,
      hash: "x213214".into()
    }
  }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.timestamp, self.data, self.prev_block_hash, self.hash)
    }
}
