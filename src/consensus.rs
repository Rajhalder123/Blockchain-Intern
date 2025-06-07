use rand::Rng;

pub fn simulate_pow() {
    let miner_power = rand::thread_rng().gen_range(50..200);
    println!("\n[PoW] Miner selected based on highest power: {} GH/s", miner_power);
}

pub fn simulate_pos() {
    let staker_stake = rand::thread_rng().gen_range(1000..10000);
    println!("[PoS] Validator selected based on highest stake: {} coins", staker_stake);
}

pub fn simulate_dpos() {
    let voters = vec!["Alice", "Bob", "Carol"];
    let delegate = voters[rand::thread_rng().gen_range(0..voters.len())];
    println!("[DPoS] Delegate '{}' selected via voting.", delegate);
}
