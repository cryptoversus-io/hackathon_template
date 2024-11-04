use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct ProgramAccount {
    pub owner: Pubkey,
    pub is_initialized: bool,
    pub transaction_count: u64,
    pub total_value_locked: u64,
}

impl Default for ProgramAccount {
    fn default() -> Self {
        Self {
            owner: Pubkey::default(),
            is_initialized: false,
            transaction_count: 0,
            total_value_locked: 0,
        }
    }
}

impl ProgramAccount {
    pub fn new(owner: Pubkey) -> Self {
        Self {
            owner,
            is_initialized: true,
            transaction_count: 0,
            total_value_locked: 0,
        }
    }

    pub fn increment_transaction_count(&mut self) {
        self.transaction_count += 1;
    }

    pub fn update_total_value_locked(&mut self, value: u64) {
        self.total_value_locked = value;
    }
}