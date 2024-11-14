// gx-blockchain/src/validation.rs
use solana_program::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::Sysvar,
    program_pack::Pack,
};
use crate::state::{BountyAccount, BountyStatus};

pub struct Validation;

impl Validation {
    pub fn verify_signer(account_info: &AccountInfo) -> ProgramResult {
        if !account_info.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        Ok(())
    }

    pub fn verify_bounty_account(
        bounty_info: &AccountInfo,
        expected_status: Option<BountyStatus>,
    ) -> Result<BountyAccount, ProgramError> {
        let bounty = BountyAccount::try_from_slice(&bounty_info.data.borrow())?;
        
        if let Some(status) = expected_status {
            if bounty.status != status {
                return Err(ProgramError::InvalidAccountData);
            }
        }
        
        Ok(bounty)
    }

    pub fn verify_hunter(
        bounty: &BountyAccount,
        hunter_info: &AccountInfo,
    ) -> ProgramResult {
        if bounty.hunter != Some(*hunter_info.key) {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(())
    }
}