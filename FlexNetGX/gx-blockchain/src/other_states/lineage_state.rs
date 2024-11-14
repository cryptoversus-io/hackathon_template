use anchor_lang::prelude::*;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};

/// Struct used to store lineage state in PDA
#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone, Copy)]
pub struct LineageInfo {
    /// creator public key
    pub creator: Pubkey,
    /// parent public key
    pub parent: Pubkey,
    /// grand parent public key
    pub grand_parent: Pubkey,
    /// great parent public key
    pub great_grand_parent: Pubkey,
    /// great great parent public key
    pub ggreat_grand_parent: Pubkey,
    /// generation number
    pub generation: u64,
    /// total number of child under the lineage
    pub total_child: u64,
}
