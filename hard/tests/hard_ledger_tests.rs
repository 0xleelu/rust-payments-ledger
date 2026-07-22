use hard_ledger::{Instruction, Ledger, LedgerError};

fn create_account(id: &str, owner_name: &str, initial_balance: u64) -> Instruction {
    Instruction::CreateAccount {
        account_id: id.to_string(),
        owner_name: owner_name.to_string(),
        initial_balance,
    }
}

fn deposit(id: &str, amount: u64) -> Instruction {
    Instruction::Deposit {
        account_id: id.to_string(),
        amount,
    }
}

fn withdraw(id: &str, amount: u64) -> Instruction {
    Instruction::Withdraw {
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
fn creates_account_with_initial_balance_and_transaction_record() {
    let mut ledger = Ledger::new();
    let instruction = create_account("alice", "Alice", 50);

    let result = ledger.process_instruction(instruction.clone());

    assert_eq!(result, Ok(1));
    let account = ledger.get_account("alice").expect("account should exist");
    assert_eq!(account.owner_name, "Alice");
    assert_eq!(account.balance, 50);
    assert!(!account.is_frozen);
    assert_eq!(ledger.transaction_history().len(), 1);
    assert_eq!(ledger.transaction_history()[0].id, 1);
    assert_eq!(ledger.transaction_history()[0].instruction, instruction);
}

#[test]
fn successful_single_instructions_receive_sequential_ids() {
    let mut ledger = Ledger::new();

    assert_eq!(
        ledger.process_instruction(create_account("alice", "Alice", 10)),
        Ok(1)
    );
    assert_eq!(ledger.process_instruction(deposit("alice", 5)), Ok(2));
    assert_eq!(ledger.process_instruction(withdraw("alice", 3)), Ok(3));

    assert_eq!(ledger.get_account("alice").unwrap().balance, 12);
    assert_eq!(ledger.transaction_history().len(), 3);
}

#[test]
fn failed_single_instruction_does_not_change_history() {
    let mut ledger = Ledger::new();

    ledger
        .process_instruction(create_account("alice", "Alice", 10))
        .expect("create should succeed");
    let result = ledger.process_instruction(withdraw("alice", 20));

    assert_eq!(result, Err(LedgerError::InsufficientFunds));
    assert_eq!(ledger.get_account("alice").unwrap().balance, 10);
    assert_eq!(ledger.transaction_history().len(), 1);
}

#[test]
fn rejects_zero_value_money_movements() {
    let mut ledger = Ledger::new();

    ledger
        .process_instruction(create_account("alice", "Alice", 10))
        .expect("create alice should succeed");
    ledger
        .process_instruction(create_account("bob", "Bob", 10))
        .expect("create bob should succeed");

    assert_eq!(
        ledger.process_instruction(deposit("alice", 0)),
        Err(LedgerError::InvalidAmount)
    );
    assert_eq!(
        ledger.process_instruction(withdraw("alice", 0)),
        Err(LedgerError::InvalidAmount)
    );
    assert_eq!(
        ledger.process_instruction(transfer("alice", "bob", 0)),
        Err(LedgerError::InvalidAmount)
    );
}

#[test]
fn frozen_accounts_cannot_move_money() {
    let mut ledger = Ledger::new();

    ledger
        .process_instruction(create_account("alice", "Alice", 100))
        .expect("create alice should succeed");
    ledger
        .process_instruction(create_account("bob", "Bob", 0))
        .expect("create bob should succeed");
    ledger
        .process_instruction(freeze("alice"))
        .expect("freeze should succeed");

    assert_eq!(
        ledger.process_instruction(deposit("alice", 1)),
        Err(LedgerError::AccountFrozen)
    );
    assert_eq!(
        ledger.process_instruction(withdraw("alice", 1)),
        Err(LedgerError::AccountFrozen)
    );
    assert_eq!(
        ledger.process_instruction(transfer("alice", "bob", 1)),
        Err(LedgerError::AccountFrozen)
    );
    assert_eq!(ledger.get_account("alice").unwrap().balance, 100);
    assert_eq!(ledger.get_account("bob").unwrap().balance, 0);
}

#[test]
fn unfreezing_account_allows_money_movement_again() {
    let mut ledger = Ledger::new();

    ledger
        .process_instruction(create_account("alice", "Alice", 100))
        .expect("create should succeed");
    ledger
        .process_instruction(freeze("alice"))
        .expect("freeze should succeed");
    ledger
        .process_instruction(unfreeze("alice"))
        .expect("unfreeze should succeed");

    assert_eq!(ledger.process_instruction(withdraw("alice", 25)), Ok(4));
    assert_eq!(ledger.get_account("alice").unwrap().balance, 75);
    assert!(!ledger.get_account("alice").unwrap().is_frozen);
}

#[test]
fn batch_applies_all_instructions_and_records_all_transactions() {
    let mut ledger = Ledger::new();

    let result = ledger.process_batch(vec![
        create_account("alice", "Alice", 100),
        create_account("bob", "Bob", 0),
        transfer("alice", "bob", 35),
        withdraw("bob", 5),
    ]);

    assert_eq!(result, Ok(vec![1, 2, 3, 4]));
    assert_eq!(ledger.get_account("alice").unwrap().balance, 65);
    assert_eq!(ledger.get_account("bob").unwrap().balance, 30);
    assert_eq!(ledger.transaction_history().len(), 4);
}

#[test]
fn failed_batch_rolls_back_accounts_history_and_ids() {
    let mut ledger = Ledger::new();

    ledger
        .process_instruction(create_account("alice", "Alice", 100))
        .expect("create should succeed");

    let result = ledger.process_batch(vec![
        create_account("bob", "Bob", 0),
        transfer("alice", "bob", 25),
        withdraw("bob", 100),
    ]);

    assert_eq!(result, Err(LedgerError::InsufficientFunds));
    assert!(ledger.get_account("bob").is_none());
    assert_eq!(ledger.get_account("alice").unwrap().balance, 100);
    assert_eq!(ledger.transaction_history().len(), 1);

    let next_result = ledger.process_instruction(deposit("alice", 1));
    assert_eq!(next_result, Ok(2));
}

#[test]
fn rejects_empty_batch() {
    let mut ledger = Ledger::new();

    let result = ledger.process_batch(Vec::new());

    assert_eq!(result, Err(LedgerError::EmptyBatch));
    assert_eq!(ledger.transaction_history().len(), 0);
}
