use anchor_lang::prelude::*;

use crate::other_states::MintingCostDistribution;

#[account]
pub struct LaunchPassState {
    pub owner: Pubkey,
    pub cost: u64,
    pub distribution: MintingCostDistribution,
    pub redeem_date: u64,
    pub redeem_amount: u64,
    pub _bump: u8
}

impl LaunchPassState {
    pub const MAX_SIZE: usize = std::mem::size_of::<Self>();
}
