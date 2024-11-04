use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};
use crate::state::instruction::ProgramInstruction;

pub mod initialize;
pub mod transaction;

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = ProgramInstruction::unpack(instruction_data)?;
        
        match instruction {
            ProgramInstruction::Initialize => {
                initialize::process(program_id, accounts)
            },
            ProgramInstruction::ProcessTransaction { data } => {
                transaction::process(program_id, accounts, data)
            },
            ProgramInstruction::VerifyTransaction { transaction_id } => {
                transaction::verify(program_id, accounts, transaction_id)
            },
        }
    }
}