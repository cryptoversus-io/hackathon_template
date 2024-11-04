use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CustomProgramError {
    #[error("Invalid instruction")]
    InvalidInstruction,
    #[error("Invalid account data")]
    InvalidAccountData,
    #[error("Verification failed")]
    VerificationFailed,
}

impl From<CustomProgramError> for ProgramError {
    fn from(e: CustomProgramError) -> Self {
        ProgramError::Custom(1)
    }
}