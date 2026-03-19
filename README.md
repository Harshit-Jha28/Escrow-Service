# 🔐 Decentralized Escrow Service on Stellar (Soroban)

## 📌 Project Description

This project implements a decentralized escrow service using Soroban smart contracts on the Stellar blockchain. It enables trustless transactions between a buyer and a seller, ensuring funds are only released when both parties agree to the terms.

The smart contract removes the need for intermediaries, reducing risk and increasing transparency.

---

## 🚀 What It Does

- Allows a buyer to create an escrow agreement with a seller
- Locks transaction details securely on-chain
- Enables the buyer to release funds to the seller
- Allows the seller to trigger a refund (basic dispute fallback)
- Tracks escrow agreements with unique IDs

---

## ✨ Features

- 🔐 **Trustless Transactions**  
  Eliminates third-party escrow agents

- 👥 **Dual-Party Interaction**  
  Buyer and seller roles clearly defined

- 🧾 **On-chain Record**  
  All escrow agreements are stored on-chain

- ⚖️ **Release & Refund Logic**  
  Simple mechanism for completing or canceling deals

- 🔑 **Authentication**  
  Uses Soroban’s `require_auth` for secure approvals

- 📊 **Escrow Tracking**  
  Each escrow is uniquely identifiable

---

## 🛠️ Tech Stack

- Soroban SDK (Rust)
- Stellar Blockchain
- Smart Contract Storage

---

## 📦 Contract Functions

| Function | Description |
|----------|------------|
| `create_escrow` | Creates a new escrow agreement |
| `release` | Buyer releases funds to seller |
| `refund` | Seller refunds buyer |
| `get_escrow` | Retrieves escrow details |

---

## 🌐 Deployed Smart Contract Link

- Testnet: https://stellar.expert/explorer/testnet/tx/be4f57da672108cd6460fde30c23a3b5fb07a9349d9596a6d58ddd919bb11b27
- Mainnet: https://stellar.expert/explorer/public/contract/XXX  


---

## 🧪 How to Run Locally

1. Install Soroban CLI  
   ```bash
   cargo install --locked soroban-cli<img width="1920" height="1080" alt="Screenshot (20)" src="https://github.com/user-attachments/assets/c9000ef6-ddb6-4c25-92dd-6ff1ae26215c" />

<img width="1920" height="1080" alt="Screenshot (89)" src="https://github.com/user-attachments/assets/ffb7e976-876c-4799-ada2-6abbf7f49882" />
