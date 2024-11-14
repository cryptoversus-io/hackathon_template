use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};
use crate::models::BountyAccount;

entrypoint!(process_instruction);

#[derive(Debug)]
pub enum BountyInstruction {
    CreateBounty {
        amount: u64,
        description_hash: [u8; 32],
    },
    SubmitWork {
        submission_hash: [u8; 32],
    },
    AcceptWork {
        bounty_account: Pubkey,
    },
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = BountyInstruction::unpack(instruction_data)?;

    match instruction {
        BountyInstruction::CreateBounty { amount, description_hash } => {
            msg!("Instruction: Create Bounty");
            process_create_bounty(program_id, accounts, amount, description_hash)
        },
        BountyInstruction::SubmitWork { submission_hash } => {
            msg!("Instruction: Submit Work");
            process_submit_work(program_id, accounts, submission_hash)
        },
        BountyInstruction::AcceptWork { bounty_account } => {
            msg!("Instruction: Accept Work");
            process_accept_work(program_id, accounts, bounty_account)
        },
    }
}