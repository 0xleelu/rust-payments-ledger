use std::collections::HashMap;

pub type AccountId = String;
pub type TransactionId = u64;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Account {
    pub owner_name: String,
    pub balance: u64,
    pub is_frozen: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    CreateAccount {
        account_id: AccountId,
        owner_name: String,
        initial_balance: u64,
    },
    Deposit {
        account_id: AccountId,
        amount: u64,
    },
    Withdraw {
        account_id: AccountId,
        amount: u64,
    },
    Transfer {
        from: AccountId,
        to: AccountId,
        amount: u64,
    },
    Freeze {
        account_id: AccountId,
    },
    Unfreeze {
        account_id: AccountId,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    pub id: TransactionId,
    pub instruction: Instruction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LedgerError {
    AccountAlreadyExists,
    AccountNotFound,
    AccountFrozen,
    InsufficientFunds,
    InvalidAmount,
    EmptyBatch,
}

#[derive(Debug, Clone)]
pub struct Ledger {
    accounts: HashMap<AccountId, Account>,
    transactions: Vec<Transaction>,
    next_transaction_id: TransactionId,
}

impl Ledger {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            transactions: Vec::new(),
            next_transaction_id: 1,
        }
    }

    pub fn process_instruction(
        &mut self,
        instruction: Instruction,
    ) -> Result<TransactionId, LedgerError> {
        match instruction.clone() {
            Instruction::CreateAccount {
                account_id,
                owner_name,
                initial_balance,
            } => self.create_account(account_id, owner_name, initial_balance)?,
            Instruction::Deposit { account_id, amount } => self.deposit(&account_id, amount)?,
            Instruction::Withdraw { account_id, amount } => self.withdraw(&account_id, amount)?,
            Instruction::Transfer { from, to, amount } => self.transfer(&from, &to, amount)?,
            Instruction::Freeze { account_id } => self.freeze(&account_id)?,
            Instruction::Unfreeze { account_id } => self.unfreeze(&account_id)?,
        }

        Ok(self.record_transaction(instruction))
    }

    pub fn process_batch(
        &mut self,
        instructions: Vec<Instruction>,
    ) -> Result<Vec<TransactionId>, LedgerError> {
        if instructions.is_empty() {
            return Err(LedgerError::EmptyBatch);
        }

        // Backup the entire current state.
        let backup_accounts = self.accounts.clone();
        let backup_transactions = self.transactions.clone();
        let backup_next_id = self.next_transaction_id;

        // Process each instruction without recording.
        for instr in &instructions {
            let result = match instr {
                Instruction::CreateAccount {
                    account_id,
                    owner_name,
                    initial_balance,
                } => self.create_account(account_id.clone(), owner_name.clone(), *initial_balance),
                Instruction::Deposit { account_id, amount } => {
                    self.deposit(account_id, *amount)
                }
                Instruction::Withdraw { account_id, amount } => {
                    self.withdraw(account_id, *amount)
                }
                Instruction::Transfer { from, to, amount } => {
                    self.transfer(from, to, *amount)
                }
                Instruction::Freeze { account_id } => self.freeze(account_id),
                Instruction::Unfreeze { account_id } => self.unfreeze(account_id),
            };
            if let Err(e) = result {
                // Rollback everything.
                self.accounts = backup_accounts;
                self.transactions = backup_transactions;
                self.next_transaction_id = backup_next_id;
                return Err(e);
            }
        }

        // All succeeded – record every transaction.
        let mut ids = Vec::with_capacity(instructions.len());
        for instr in instructions {
            ids.push(self.record_transaction(instr));
        }
        Ok(ids)
    }

    pub fn get_account(&self, account_id: &str) -> Option<&Account> {
        self.accounts.get(account_id)
    }

    pub fn transaction_history(&self) -> &[Transaction] {
        &self.transactions
    }

    fn create_account(
        &mut self,
        account_id: AccountId,
        owner_name: String,
        initial_balance: u64,
    ) -> Result<(), LedgerError> {
        if self.accounts.contains_key(&account_id) {
            return Err(LedgerError::AccountAlreadyExists);
        }

        let account = Account {
            owner_name,
            balance: initial_balance,
            is_frozen: false,
        };
        self.accounts.insert(account_id, account);
        Ok(())
    }

    fn deposit(&mut self, account_id: &str, amount: u64) -> Result<(), LedgerError> {
        if amount == 0 {
            return Err(LedgerError::InvalidAmount);
        }

        let account = self
            .accounts
            .get_mut(account_id)
            .ok_or(LedgerError::AccountNotFound)?;
        if account.is_frozen {
            return Err(LedgerError::AccountFrozen);
        }

        account.balance += amount;
        Ok(())
    }

    fn withdraw(&mut self, account_id: &str, amount: u64) -> Result<(), LedgerError> {
        if amount == 0 {
            return Err(LedgerError::InvalidAmount);
        }
        let account = self
            .accounts
            .get_mut(account_id)
            .ok_or(LedgerError::AccountNotFound)?;
        if account.is_frozen {
            return Err(LedgerError::AccountFrozen);
        }
        if account.balance < amount {
            return Err(LedgerError::InsufficientFunds);
        }
        account.balance -= amount;
        Ok(())
    }

    fn transfer(&mut self, from: &str, to: &str, amount: u64) -> Result<(), LedgerError> {
        if amount == 0 {
            return Err(LedgerError::InvalidAmount);
        }

        let from_account = self
            .accounts
            .get(from)
            .ok_or(LedgerError::AccountNotFound)?;
        let to_account = self.accounts.get(to).ok_or(LedgerError::AccountNotFound)?;

        if from_account.is_frozen || to_account.is_frozen {
            return Err(LedgerError::AccountFrozen);
        }
        if from_account.balance < amount {
            return Err(LedgerError::InsufficientFunds);
        }
        if from == to {
            return Ok(());
        }

        self.accounts.get_mut(from).unwrap().balance -= amount;
        self.accounts.get_mut(to).unwrap().balance += amount;
        Ok(())
    }

    fn freeze(&mut self, account_id: &str) -> Result<(), LedgerError> {
        let account = self
            .accounts
            .get_mut(account_id)
            .ok_or(LedgerError::AccountNotFound)?;
        account.is_frozen = true;
        Ok(())
    }

    fn unfreeze(&mut self, account_id: &str) -> Result<(), LedgerError> {
        let account = self
            .accounts
            .get_mut(account_id)
            .ok_or(LedgerError::AccountNotFound)?;
        account.is_frozen = false;
        Ok(())
    }

    fn record_transaction(&mut self, instruction: Instruction) -> TransactionId {
        let id = self.next_transaction_id;
        self.transactions.push(Transaction { id, instruction });
        self.next_transaction_id += 1;
        id
    }
}