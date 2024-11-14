use crate::other_states::LineageInfo;
use anchor_lang::prelude::*;

/// Struct used to store profile state in PDA
#[account]
pub struct ProfileState {
    /// lineage info
    pub lineage: LineageInfo,
    /// profile nft public key
    pub mint: Pubkey,
    /// profile nft associated activation token
    pub activation_token: Option<Pubkey>,
    /// total minted nft 
    pub total_minted_sft: u64,
    /// total minted offers 
    pub total_minted_offers: u64,
    /// LUT public key
    pub lut: Pubkey,
}

impl ProfileState {
    pub const MAX_SIZE: usize = std::mem::size_of::<Self>();
}