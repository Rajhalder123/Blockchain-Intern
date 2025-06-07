use sha2::{Sha256, Digest};
use chrono::Utc;





#[derive(Debug)]
#[allow(dead_code)] 
struct Block {
    index: u32,
    timestamp: String,
    data: String,
    previous_hash: String,
    hash: String,
    nonce: u64,
}

impl Block {
    fn new(index: u32, data: String, previous_hash: String) -> Self {
        let timestamp = Utc::now().to_rfc3339();
        let mut nonce = 0;
        let mut hash = Block::calculate_hash(index, &timestamp, &data, &previous_hash, nonce);

        // Simple Proof of Work: repeat until hash starts with "00"
        while &hash[..2] != "00" {
            nonce += 1;
            hash = Block::calculate_hash(index, &timestamp, &data, &previous_hash, nonce);
        }

        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
            nonce,
        }
    }

    fn calculate_hash(index: u32, timestamp: &str, data: &str, previous_hash: &str, nonce: u64) -> String {
        let mut hasher = Sha256::new();
        hasher.update(index.to_string());
        hasher.update(timestamp);
        hasher.update(data);
        hasher.update(previous_hash);
        hasher.update(nonce.to_string());
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}

fn main() {
    // Create genesis block
    let genesis_block = Block::new(0, "Genesis Block".to_string(), "0".to_string());

    // Create second block
    let second_block = Block::new(1, "Second Block Data".to_string(), genesis_block.hash.clone());

    // Create third block
    let third_block = Block::new(2, "Third Block Info".to_string(), second_block.hash.clone());

    // Print blocks
    println!("\nBlock 1: {:#?}", genesis_block);
    println!("\nBlock 2: {:#?}", second_block);
    println!("\nBlock 3: {:#?}", third_block);
}
