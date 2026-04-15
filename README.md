# 🧠 On-Chain Habit Tracker (Soroban Smart Contract)

## 📌 Description

This project is a decentralized Habit Tracker built using Soroban smart contracts on the Stellar network. It allows users to create, manage, and track daily habits directly on-chain.

Unlike traditional apps, all data is stored on the blockchain, ensuring transparency and immutability.

---

## 🚀 Features

* Create a new habit
* View all habits
* Mark habit as completed
* Track habit streaks
* Reset daily status
* Delete habits

---

## ⚙️ How It Works

Users interact with the smart contract functions via Soroban Studio or CLI.

Each habit is stored on-chain with:

* Unique ID
* Title
* Completion status
* Streak count

When a habit is marked as completed, the streak automatically increases.

---

## 🧾 Smart Contract Information

* Network: Stellar Testnet
* Contract ID: `PASTE_YOUR_CA_HERE`

---

## 🖥️ How to Run

### 1. Build Contract

```
cargo build --target wasm32-unknown-unknown --release
```

### 2. Deploy Contract

```
soroban contract deploy \
--wasm target/wasm32-unknown-unknown/release/your_contract.wasm \
--source alice \
--network testnet
```

### 3. Invoke Functions

Example:

```
create_habit("Exercise")
complete_habit(1)
get_habits()
```

---

## 📸 Screenshots

(Add screenshots from Soroban Studio here)

* Contract deployed
* Creating habit
* Fetching habits

---

## 💡 Use Case

This project demonstrates how blockchain can be used for personal productivity tools such as habit tracking while ensuring data integrity.

---

## 🛠️ Built With

* Rust
* Soroban SDK
* Stellar Testnet

---

## 👨‍💻 Author

Your Name
