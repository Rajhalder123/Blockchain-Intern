# 🧱 Mini Blockchain Simulation in Rust

A simple blockchain simulation project written in **Rust** to understand the fundamentals of blockchain, mining, and consensus mechanisms.

---

## 📁 Folder Structure

blockchain_simulation/
├── Cargo.toml
├── src/
│ ├── main.rs # Runs the blockchain simulation
│ ├── block.rs # Defines the Block struct and mining logic
│ ├── mining.rs # Nonce mining simulation (Proof of Work)
│ └── consensus.rs # PoW, PoS, DPoS consensus simulation
└── README.md

markdown
Copy
Edit

---

## ✅ Practical Tasks Overview

### 1. 🧱 Block Simulation in Code

- Implemented `Block` struct with:
  - `index`, `timestamp`, `data`, `previous_hash`, `hash`, `nonce`
- Used `sha2` crate for SHA-256 hashing.
- Linked 3 blocks via `previous_hash`.
- Demonstrated how tampering block data affects entire chain validity.

### 2. 🔁 Nonce Mining Simulation (Proof of Work)

- Added `mine_block(difficulty: usize)` method.
- Simulated PoW where hash must start with `0000` or similar.
- Output includes:
  - Time taken
  - Number of nonce attempts

### 3. ⚖️ Consensus Mechanism Simulation

- Simulated and compared:
  - **PoW**: Miner with highest random power wins.
  - **PoS**: Validator with highest stake is selected.
  - **DPoS**: Delegate chosen based on most votes from voters.

---

## 📘 Theoretical Part

### 🔗 Blockchain Basics

A **blockchain** is a distributed digital ledger that records transactions in linked blocks secured by cryptographic hashes. Each block includes a timestamp, data, and a reference to the previous block’s hash. This structure prevents tampering because changing one block’s data would require recalculating all subsequent hashes. Blockchain operates without a central authority, using consensus algorithms like PoW or PoS to validate transactions across a decentralized network.

#### 🔍 Use Cases:
- ✅ **Supply Chain**: Track items in real-time from production to delivery.
- ✅ **Digital Identity**: Secure and decentralized identity systems.

---

### 📦 Block Anatomy

┌────────────────────────────┐
│ Block │
│ ┌───────┐ ┌──────────────┐ │
│ │ Data │ │ Timestamp │ │
│ └───────┘ └──────────────┘ │
│ ┌────────────────────────┐ │
│ │ Previous Hash │ │
│ └────────────────────────┘ │
│ ┌─────────────┐ ┌────────┐ │
│ │ Nonce │ │ Merkle │ │
│ │ │ │ Root │ │
│ └─────────────┘ └────────┘ │
└────────────────────────────┘

yaml
Copy
Edit

🧪 **Merkle Root** Explanation:
If we have transactions A, B, C, and D:
- Hash A+B = AB, Hash C+D = CD
- Hash AB+CD = **Merkle Root**
If one transaction changes, the root changes, ensuring data integrity.

---

### 🔐 Consensus Algorithms

#### ✅ Proof of Work (PoW)
PoW involves solving cryptographic puzzles. Miners compete to find a valid hash. It requires energy due to heavy computation. It's secure but energy-intensive.

#### ✅ Proof of Stake (PoS)
In PoS, validators are chosen based on how many coins they stake. It’s energy-efficient and discourages attacks by risking the stake.

#### ✅ Delegated Proof of Stake (DPoS)
Token holders vote for delegates who validate transactions. It’s faster and democratic but can be slightly centralized due to fewer validators.

---

## ▶️ How to Run

### 🚀 Prerequisites
- Rust installed: https://www.rust-lang.org/tools/install

### 📦 Install Dependencies
```bash
cargo add sha2 chrono rand
🏃‍♂️ Run Project
bash
Copy
Edit
cargo run
📚 Credits
Created by Raj Haldar for educational purposes.
Mentored by Blockchain Intern Team

📌 License
MIT License – for learning and educational use.

yaml
Copy
Edit

---

Let me know if you want a downloadable version or GitHub repo setup!