use anchor_lang::prelude::*;

/// Struct used to store collection PDA
#[account]
pub struct CollectionState {
    /// gensis profile to create collection
    pub genesis_profile: Pubkey,
    /// collection nft public key
    pub collection_id: Pubkey,
}

impl CollectionState {
    pub const MAX_SIZE: usize = std::mem::size_of::<Self>();
}
