use std::collections::HashMap;

pub type AccountId = String;

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
    },
    Deposit {
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
pub enum LedgerError {
    AccountAlreadyExists,
    AccountNotFound,
    AccountFrozen,
    InsufficientFunds,
    InvalidAmount,
}

pub struct Ledger {
    accounts: HashMap<AccountId, Account>,
}

impl Ledger {
    pub fn new() -> Self {
        Ledger {
            accounts: HashMap::new(),
        }
    }

    pub fn process_instruction(&mut self, instruction: Instruction) -> Result<(), LedgerError> {
        match instruction {
            Instruction::CreateAccount { account_id, owner_name } => {
                if self.accounts.contains_key(&account_id) {
                    return Err(LedgerError::AccountAlreadyExists);
                }
                let account = Account {
                    owner_name,
                    balance: 0,
                    is_frozen: false,
                };
                self.accounts.insert(account_id, account);
                Ok(())
            }

            Instruction::Deposit { account_id, amount } => {
                if amount == 0 {
                    return Err(LedgerError::InvalidAmount);
                }
                let account = self.accounts.get_mut(&account_id);
                match account {
                    Some(acc) => {
                        if acc.is_frozen {
                            return Err(LedgerError::AccountFrozen);
                        }
                        acc.balance += amount;
                        Ok(())
                    }
                    None => Err(LedgerError::AccountNotFound),
                }
            }

            Instruction::Transfer { from, to, amount } => {
                if amount == 0 {
                    return Err(LedgerError::InvalidAmount);
                }

                if from == to {
                    let account = self.accounts.get(&from);
                    if let Some(acc) = account {
                        if acc.is_frozen {
                            return Err(LedgerError::AccountFrozen);
                        }
                        return Ok(());
                    } else {
                        return Err(LedgerError::AccountNotFound);
                    }
                }


                let (from_exists, from_balance, from_frozen) = {
                    let from_acc = self.accounts.get(&from);
                    match from_acc {
                        Some(acc) => (true, acc.balance, acc.is_frozen),
                        None => (false, 0, false),
                    }
                };

                if !from_exists {
                    return Err(LedgerError::AccountNotFound);
                }
                if from_frozen {
                    return Err(LedgerError::AccountFrozen);
                }
                if from_balance < amount {
                    return Err(LedgerError::InsufficientFunds);
                }

                let to_exists = {
                    let to_acc = self.accounts.get(&to);
                    match to_acc {
                        Some(acc) => {
                            if acc.is_frozen {
                                return Err(LedgerError::AccountFrozen);
                            }
                            true
                        }
                        None => false,
                    }
                };
                if !to_exists {
                    return Err(LedgerError::AccountNotFound);
                }

                
                let from_acc = self.accounts.get_mut(&from).unwrap(); 
                from_acc.balance -= amount;
                let to_acc = self.accounts.get_mut(&to).unwrap();
                to_acc.balance += amount;

                Ok(())
            }

            Instruction::Freeze { account_id } => {
                let account = self.accounts.get_mut(&account_id);
                match account {
                    Some(acc) => {
                        acc.is_frozen = true;
                        Ok(())
                    }
                    None => Err(LedgerError::AccountNotFound),
                }
            }

            Instruction::Unfreeze { account_id } => {
                let account = self.accounts.get_mut(&account_id);
                match account {
                    Some(acc) => {
                        acc.is_frozen = false;
                        Ok(())
                    }
                    None => Err(LedgerError::AccountNotFound),
                }
            }
        }
    }

    pub fn get_account(&self, account_id: &str) -> Option<&Account> {
        self.accounts.get(account_id)
    }
}