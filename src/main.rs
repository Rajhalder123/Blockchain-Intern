mod block;
mod mining;
mod consensus;

use block::Block;
use mining::mine_block;
use consensus::*;

fn main() {
    println!("🚀 Blockchain Simulation");

    // === BLOCKCHAIN with 3 linked blocks ===
    let mut chain = vec![];

    // Genesis
    let mut block1 = Block::new(0, "Genesis Block".to_string(), "0".to_string());
    block1 = mine_block(block1, 3);
    chain.push(block1.clone());

    let mut block2 = Block::new(1, "Second Block".to_string(), block1.hash.clone());
    block2 = mine_block(block2, 3);
    chain.push(block2.clone());

    let mut block3 = Block::new(2, "Third Block".to_string(), block2.hash.clone());
    block3 = mine_block(block3, 3);
    chain.push(block3.clone());

    println!("\n🧱 Full Blockchain:");
    for block in &chain {
        println!("{:#?}", block);
    }

    // === DATA TAMPERING DEMO ===
    println!("\n🔧 Tampering Block 1 data...");
    let mut tampered = chain[1].clone();
    tampered.data = "Hacked Data!".to_string();
    tampered.hash = tampered.calculate_hash();

    println!("\n🧪 Validity Check After Tampering:");
    println!(
        "Block 2 valid? {}\nBlock 3 valid? {}",
        tampered.is_valid(),
        chain[2].is_valid()
    );

    // === CONSENSUS MECHANISMS ===
    println!("\n📊 Consensus Mechanism Simulation:");
    simulate_pow();
    simulate_pos();
    simulate_dpos();
}
