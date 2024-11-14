use crate::other_states::LineageInfo;
use anchor_lang::prelude::*;

/// Struct used to store activation token state in PDA
#[account]
pub struct ActivationTokenState {
    // lineage: LineageInfo,
    pub parent_profile: Pubkey,
    pub creator: Pubkey,
}

impl ActivationTokenState {
    pub const MAX_SIZE: usize = std::mem::size_of::<Self>();
}
