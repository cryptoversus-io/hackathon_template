use anchor_lang::prelude::*;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};
use mpl_token_metadata::accounts::Metadata;

use crate::error::MyError;
use crate::other_states::{MintingCostDistribution, TradingPriceDistribution};

/// Struct used to store PDA main state account 
#[account]
pub struct MainState {
    /// PDA owner account public key
    pub owner: Pubkey,
    /// token address to receive cost for mining
    pub opos_token: Pubkey,
    /// profile minting cost for nft
    pub profile_minting_cost: u64,
    /// invitation minting cost for nft
    pub invitation_minting_cost: u64,
    /// mint cost distribution details
    pub minting_cost_distribution: MintingCostDistribution,
    /// trade cost distribution details
    pub trading_price_distribution: TradingPriceDistribution,
    /// seller fee basis points for nft
    pub seller_fee_basis_points: u16, //NOTE: may be later change
    /// PDA account bump details for siging purpose
    pub _bump: u8,
    /// totla minted profile count
    pub total_minted_profile: u64,
    /// collection to verify profile nft while mint
    pub profile_collection: Pubkey,
    /// store gensis profile in state account to build lineage
    pub genesis_profile: Pubkey,
    /// Lut account to compress public key size to reduce size on large account
    pub common_lut: Pubkey,
}

impl MainState {
    pub const MAX_SIZE: usize = std::mem::size_of::<Self>();
}

/// Struct used as param to receive details from function call
#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone, Copy)]
pub struct MainStateInput {
    /// profile minting cost for nft
    pub profile_minting_cost: u64,
    /// invitation minting cost for nft
    pub invitation_minting_cost: u64,
    /// token address to receive cost for mining
    pub opos_token: Pubkey,
    /// mint cost distribution details
    pub minting_cost_distribution: MintingCostDistribution,
    /// trade cost distribution details
    pub trading_price_distribution: TradingPriceDistribution,
    // pub activation_token_collection_id: Pubkey,
}

impl MainStateInput {
    pub fn set_value(&self, mut state: &mut MainState) {
        // state.activation_token_collection_id = self.activation_token_collection_id;
        state.opos_token = self.opos_token;
        state.minting_cost_distribution = self.minting_cost_distribution;
        state.trading_price_distribution = self.trading_price_distribution;
        state.profile_minting_cost = self.profile_minting_cost;
        state.invitation_minting_cost = self.invitation_minting_cost;
    }
}
