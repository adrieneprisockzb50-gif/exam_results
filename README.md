# Project Title

Exam Results – An Immutable Exam Score Publishing System on Stellar

## Project Vision

This project demonstrates how to build an **immutable exam score publishing system** using Soroban smart contracts on Stellar. It provides:
- How to write a Soroban smart contract in Rust
- How to manage persistent storage with access control
- How to handle admin-only operations with user authentication
- How to deploy and interact with contracts on Stellar Testnet

The goal is to provide a secure, transparent system where exam results can be published by administrators and read by anyone.

---

## Description

An immutable Soroban smart contract that allows **administrators to publish exam results** on Stellar Testnet. Students' scores are stored permanently on-chain and can be retrieved by anyone. The system ensures only authorized administrators can publish results while maintaining transparency for score verification.

---

## Features

### 1. Admin-Only Publishing
- Only the designated admin can publish exam results
- Access control enforced through Soroban's require_auth
- Immutable storage once published

### 2. Public Read Access
- Anyone can retrieve a student's exam result
- Transparent and verifiable on-chain data
- No authentication required for reading

### 3. Score Data Structure
- Stores student ID (Address), score (u32), and pass status (bool)
- Efficient lookup using Soroban's Map data structure

### 4. Beginner-Friendly
- Clear, readable Rust code for Soroban
- Minimal, working example for learning

---

## Contract

- **Network**: Stellar Testnet
- **Admin**: Set during contract initialization
- **Contract ID**: [CCWY2FFK4JZFMNKYTPPYD6PI62LFQPYITMHLDHKZ5TBU66WQ5FQGHDTV](https://stellar.expert/explorer/testnet/tx/9f425306288f84f93343f4ca3b05a613c08c1ca786116394eb180cb8b59c49b3)

![screenshot](https://i.ibb.co/6cdtLVRC/image.png)

---

## Future Scopes

### 1. Batch Publishing
- Allow admins to publish multiple results in a single transaction

### 2. Result Updates
- Add functionality to update existing results with proper authorization

### 3. Student Self-Registration
- Allow students to register their own addresses for results

### 4. Frontend dApp
- Build a simple web interface for admins to publish results and students to check scores

### 5. Event Emitters
- Emit events when results are published for better off-chain tracking

### 6. Result Statistics
- Add view functions for aggregate statistics (pass rate, average score, etc.)

### 7. Time-stamped Results
- Include timestamps for when results were published

### 8. Course/Subject Support
- Support multiple courses or subjects per student

---

## Profile

- **Name:** adrieneprisockzb50
