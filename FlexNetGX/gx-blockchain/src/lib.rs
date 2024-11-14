// gx-blockchain/src/lib.rs
use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    program_pack::{Pack, IsInitialized},

};
use borsh::BorshDeserialize;

pub mod instruction;
pub mod processor;
pub mod state;
pub mod error;

use instruction::BountyInstruction;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("DeGIG Bounty Program: {:?}", instruction_data);

    processor::Processor::process(program_id, accounts, instruction_data)
}
    
    match instruction {
        BountyInstruction::CreateBounty { amount, description_hash } => {
            processor::Processor::process_create_bounty(program_id, accounts, amount, description_hash)
        }
        BountyInstruction::AcceptBounty => {
            processor::Processor::process_accept_bounty(accounts)
        }
        BountyInstruction::SubmitWork { submission_hash } => {
            processor::Processor::process_submit_work(accounts, submission_hash)
        }
    }
}