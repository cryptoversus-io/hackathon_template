// gx-blockchain/src/instruction.rs
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum BountyInstruction {
    /// Initialize a new bounty
    /// Accounts expected:
    /// 0. `[signer, writable]` The bounty creator
    /// 1. `[writable]` The bounty account to initialize
    /// 2. `[]` System program
    CreateBounty {
        amount: u64,
        description_hash: [u8; 32],
    },

    /// Start working on a bounty
    /// Accounts expected: [signer, bounty_account]
    AcceptBounty,
    /// Submit work for a bounty
    /// Accounts expected:
    /// 0. `[signer]` The hunter submitting work
    /// 1. `[writable]` The bounty account
    SubmitWork {
        submission_hash: [u8; 32],
    },

    /// Accept submitted work
    /// Accounts expected:
    /// 0. `[signer]` The bounty creator
    /// 1. `[writable]` The bounty account
    /// 2. `[writable]` The hunter's account to receive payment
    AcceptWork {},
}