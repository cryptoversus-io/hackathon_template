use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct PlatformState {
    pub admin: Pubkey,
    pub total_users: u64,
    pub total_transactions: u64,
    pub is_initialized: bool,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct UserAccount {
    pub owner: Pubkey,
    pub profile_data: Vec<u8>,
    pub transaction_count: u64,
    pub is_active: bool,
}