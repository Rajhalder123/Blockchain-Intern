use sha2::{Sha256, Digest};
use chrono::Utc;

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u32,
    pub timestamp: String,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(index: u32, data: String, previous_hash: String) -> Self {
        let timestamp = Utc::now().to_rfc3339();
        let mut block = Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, self.data, self.previous_hash, self.nonce
        ));
        format!("{:x}", hasher.finalize())
    }

    pub fn is_valid(&self) -> bool {
        self.hash == self.calculate_hash()
    }
}
