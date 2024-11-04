use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
};

mod state;
mod processor;
mod error;
mod security;
mod avs;
mod utils;

use crate::processor::Processor;

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("FlexNet GX Blockchain program entrypoint");
    Processor::process(program_id, accounts, instruction_data)
}
