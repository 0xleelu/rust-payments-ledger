# Rust Payments Ledger

An in-memory payments ledger written in Rust, inspired by Solana's program/instruction model. 
This project is split into two progressive assignments: **Easy** and **Hard**.

## Features
- **Account Management**: Create accounts with an owner name and balance.
- **Transactions**: Deposit, Withdraw, and Transfer credits.
- **Security**: Freeze/Unfreeze accounts to prevent operations.
- **Audit Trail**: Unique Transaction IDs and full history (`Hard` version).
- **Atomicity**: Batch processing that applies all instructions or rolls back completely (`Hard` version).
- **No panics**: Uses `Result<T, LedgerError>` for all fallible operations.
- **No external crates**: Pure standard Rust.

## Project Structure
- `/easy`: Basic CRUD operations, transfers, and freeze logic.
- `/hard`: Adds withdrawals, transaction IDs, and atomic batch processing.

## Running the Tests
```bash
# Test the easy version
cd easy && cargo test

# Test the hard version
cd hard && cargo test
