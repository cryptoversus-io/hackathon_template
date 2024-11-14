// gx-blockchain/src/state.rs
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
    program_error::ProgramError,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct BountyAccount {
    pub is_initialized: bool,
    pub creator: Pubkey,
    pub amount: u64,
    pub description_hash: [u8; 32],
    pub hunter: Option<Pubkey>,
    pub submission_hash: Option<[u8; 32]>,
    pub status: BountyStatus,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
pub enum BountyStatus {
    Open,
    InProgress,
    Completed,
}