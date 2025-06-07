use std::time::Instant;
use crate::block::Block;

pub fn mine_block(mut block: Block, difficulty: usize) -> Block {
    let target = "0".repeat(difficulty);
    let start_time = Instant::now();

    while &block.calculate_hash()[..difficulty] != target {
        block.nonce += 1;
    }

    block.hash = block.calculate_hash();
    let elapsed = start_time.elapsed();
    println!(
        "⛏️  Mined block {} in {} attempts, time: {:?}",
        block.index, block.nonce, elapsed
    );
    block
}
