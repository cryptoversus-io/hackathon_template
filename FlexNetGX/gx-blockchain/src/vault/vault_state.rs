use anchor_lang::prelude::*;

#[account]
pub struct VaultState {
    pub mint: Pubkey,
    pub lock_date: u64,
    pub authority: Pubkey,
    pub owner: Pubkey,
    pub _bump: u8,
}

impl VaultState {
    pub const MAX_SIZE: usize = std::mem::size_of::<Self>();
}