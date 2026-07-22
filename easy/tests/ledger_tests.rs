use bootcamp_rust_assignment::{Instruction, Ledger, LedgerError};

fn create_account(id: &str, owner_name: &str) -> Instruction {
    Instruction::CreateAccount {
        account_id: id.to_string(),
        owner_name: owner_name.to_string(),
    }
}

fn deposit(id: &str, amount: u64) -> Instruction {
    Instruction::Deposit {
        account_id: id.to_string(),
        amount,
    }
}

fn transfer(from: &str, to: &str, amount: u64) -> Instruction {
    Instruction::Transfer {
        from: from.to_string(),
        to: to.to_string(),
        amount,
    }
}

fn freeze(id: &str) -> Instruction {
    Instruction::Freeze {
        account_id: id.to_string(),
    }
}

fn unfreeze(id: &str) -> Instruction {
    Instruction::Unfreeze {
        account_id: id.to_string(),
    }
}

#[test]
fn creates_account_with_initial_state() {
    let mut ledger = Ledger::new();

    let result = ledger.process_instruction(create_account("alice", "Alice"));

    assert_eq!(result, Ok(()));
    let account = ledger.get_account("alice").expect("account should exist");
    assert_eq!(account.owner_name, "Alice");
    assert_eq!(account.balance, 0);
    assert!(!account.is_frozen);
}

#[test]
fn rejects_duplicate_account_creation() {
    let mut ledger = Ledger::new();

    ledger
        .process_instruction(create_account("alice", "Alice"))
        .expect("first create should succeed");
    let result = ledger.process_instruction(create_account("alice", "Another Alice"));

    assert_eq!(result, Err(LedgerError::AccountAlreadyExists));
}

#[test]
fn deposits_into_existing_account() {
    let mut ledger = Ledger::new();

    ledger
        .process_instruction(create_account("alice", "Alice"))
        .expect("create should succeed");
    let result = ledger.process_instruction(deposit("alice", 50));

    assert_eq!(result, Ok(()));
    assert_eq!(ledger.get_account("alice").unwrap().balance, 50);
}

#[test]
fn rejects_zero_deposit() {
    let mut ledger = Ledger::new();

    ledger
        .process_instruction(create_account("alice", "Alice"))
        .expect("create should succeed");
    let result = ledger.process_instruction(deposit("alice", 0));

    assert_eq!(result, Err(LedgerError::InvalidAmount));
    assert_eq!(ledger.get_account("alice").unwrap().balance, 0);
}

#[test]
fn transfers_between_accounts() {
    let mut ledger = Ledger::new();

    ledger
        .process_instruction(create_account("alice", "Alice"))
        .expect("create alice should succeed");
    ledger
        .process_instruction(create_account("bob", "Bob"))
        .expect("create bob should succeed");
    ledger
        .process_instruction(deposit("alice", 100))
        .expect("deposit should succeed");

    let result = ledger.process_instruction(transfer("alice", "bob", 40));

    assert_eq!(result, Ok(()));
    assert_eq!(ledger.get_account("alice").unwrap().balance, 60);
    assert_eq!(ledger.get_account("bob").unwrap().balance, 40);
}

#[test]
fn rejects_transfer_with_insufficient_funds_without_changing_balances() {
    let mut ledger = Ledger::new();

    ledger
        .process_instruction(create_account("alice", "Alice"))
        .expect("create alice should succeed");
    ledger
        .process_instruction(create_account("bob", "Bob"))
        .expect("create bob should succeed");
    ledger
        .process_instruction(deposit("alice", 25))
        .expect("deposit should succeed");

    let result = ledger.process_instruction(transfer("alice", "bob", 40));

    assert_eq!(result, Err(LedgerError::InsufficientFunds));
    assert_eq!(ledger.get_account("alice").unwrap().balance, 25);
    assert_eq!(ledger.get_account("bob").unwrap().balance, 0);
}

#[test]
fn rejects_transfer_involving_frozen_account() {
    let mut ledger = Ledger::new();

    ledger
        .process_instruction(create_account("alice", "Alice"))
        .expect("create alice should succeed");
    ledger
        .process_instruction(create_account("bob", "Bob"))
        .expect("create bob should succeed");
    ledger
        .process_instruction(deposit("alice", 100))
        .expect("deposit should succeed");
    ledger
        .process_instruction(freeze("bob"))
        .expect("freeze should succeed");

    let result = ledger.process_instruction(transfer("alice", "bob", 40));

    assert_eq!(result, Err(LedgerError::AccountFrozen));
    assert_eq!(ledger.get_account("alice").unwrap().balance, 100);
    assert_eq!(ledger.get_account("bob").unwrap().balance, 0);
}

#[test]
fn freezes_and_unfreezes_account() {
    let mut ledger = Ledger::new();

    ledger
        .process_instruction(create_account("alice", "Alice"))
        .expect("create should succeed");

    assert_eq!(ledger.process_instruction(freeze("alice")), Ok(()));
    assert!(ledger.get_account("alice").unwrap().is_frozen);

    assert_eq!(ledger.process_instruction(unfreeze("alice")), Ok(()));
    assert!(!ledger.get_account("alice").unwrap().is_frozen);
}
