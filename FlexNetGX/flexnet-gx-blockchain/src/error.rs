use num_derive::FromPrimitive;
use solana_program::{decode_error::DecodeError, program_error::ProgramError};
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone, FromPrimitive)]
pub enum FlexNetError {
    #[error("Invalid instruction")]
    InvalidInstruction,
    #[error("Not Authorized")]
    NotAuthorized,
    #[error("Insufficient Funds")]
    InsufficientFunds,
    #[error("Account Not Initialized")]
    AccountNotInitialized,
    #[error("Invalid Account Data")]
    InvalidAccountData,
}

impl From<FlexNetError> for ProgramError {
    fn from(e: FlexNetError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for FlexNetError {
    fn type_of() -> &'static str {
        "FlexNetError"
    }
}

#[derive(Debug)]
pub enum TokenError {
    InsufficientFunds,
    InvalidMint,
    InvalidOwner,
    InvalidTokenAccount,
}

impl From<TokenError> for ProgramError {
    fn from(e: TokenError) -> Self {
        ProgramError::Custom(400 + match e {
            TokenError::InsufficientFunds => 0,
            TokenError::InvalidMint => 1,
            TokenError::InvalidOwner => 2,
            TokenError::InvalidTokenAccount => 3,
        })
    }
}