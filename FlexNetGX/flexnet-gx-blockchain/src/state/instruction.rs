use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum ProgramInstruction {
    Initialize,
    ProcessTransaction {
        data: Vec<u8>,
    },
    VerifyTransaction {
        transaction_id: String,
    },
}

impl ProgramInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let instruction = Self::try_from_slice(input)
            .map_err(|_| ProgramError::InvalidInstructionData)?;
        Ok(instruction)
    }
}