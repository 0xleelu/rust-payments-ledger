# Rust Assignment: In-Memory Payments Ledger (Easy)

This repository contains two versions of the ledger assignment:

- Easy: this folder.
- Hard: see `../hard/`.

Build a small in-memory payments ledger in Rust. This assignment is not a
Solana program and does not require any wallet, validator, RPC endpoint, or web3
tooling. It uses a few Solana-inspired ideas in a plain web2-style Rust crate:

- Ledger accounts are similar to Solana accounts: keyed pieces of state.
- `Instruction` is similar to a Solana instruction: a request to perform one action.
- `process_instruction` is similar to a Solana program processor: one entry point
  that applies instructions to state.

## Goal

Implement the ledger behavior in `src/lib.rs` so all tests pass.

The ledger should support:

- Creating accounts.
- Depositing credits.
- Transferring credits between accounts.
- Freezing and unfreezing accounts.
- Returning clear errors for expected failure cases.

## Rules

- Do not add external crates.
- Do not panic for expected errors.
- Use `Result<(), LedgerError>` for operations that can fail.
- Reject zero-value deposits and transfers.
- Failed transfers must not partially mutate state.
- Keep the assignment logic in `src/lib.rs`; `src/main.rs` is only a runner hint.

## Commands

Check that the crate compiles:

```sh
cargo check
```

Run the assignment tests:

```sh
cargo test
```

The starter project compiles, but the tests fail until you replace the `todo!()`
sections in `src/lib.rs`.
