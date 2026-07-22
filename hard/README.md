# Rust Assignment: Audited Payments Ledger (Hard)

Build a more complete in-memory payments ledger in Rust. This version is still
plain Rust and does not require Solana, Anchor, wallets, validators, RPC, HTTP,
or a database.

Compared with the easy assignment, this version adds:

- Withdrawals.
- Transaction IDs.
- Transaction history.
- Atomic batch processing.

The completed solution is roughly 100-120 lines of Rust, depending on style.
The easy/shared ledger behavior is already included in this starter so you can
focus on the hard-only parts.

## Solana-Inspired Mapping

- Ledger accounts are similar to Solana accounts: keyed pieces of state.
- `Instruction` is similar to a Solana instruction: a request to perform one
  action.
- `process_instruction` is similar to a program processor: it validates and
  applies one state transition.
- `process_batch` is similar to applying multiple instructions atomically: all
  instructions succeed, or none of them change ledger state.

## Goal

Implement the TODOs in `src/lib.rs` so all tests pass.

The ledger should support:

- Creating accounts with an initial balance.
- Depositing credits.
- Withdrawing credits.
- Transferring credits between accounts.
- Freezing and unfreezing accounts.
- Recording successful instructions in transaction history.
- Processing batches atomically.

Already provided:

- Creating accounts.
- Depositing credits.
- Transferring credits between accounts.
- Freezing and unfreezing accounts.
- Looking up accounts.

You need to implement:

- Withdrawing credits.
- Recording successful instructions with transaction IDs.
- Processing batches atomically.

## Rules

- Do not add external crates.
- Do not panic for expected errors.
- Use `Result` for operations that can fail.
- Reject zero-value deposits, withdrawals, and transfers.
- Frozen accounts cannot deposit, withdraw, or transfer.
- Failed single instructions must not create transaction records.
- Failed batches must not change accounts, transaction history, or the next
  transaction ID.
- Keep the assignment logic in `src/lib.rs`; `src/main.rs` is only a runner hint.

## Commands

Check that the hard assignment compiles:

```sh
cargo check
```

Run the hard assignment tests:

```sh
cargo test
```

The starter project compiles, but the tests fail until you replace the `todo!()`
sections in `src/lib.rs`.
