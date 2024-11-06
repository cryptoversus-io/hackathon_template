use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_program,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum FlexNetInstruction {
    InitializePlatform,
    RegisterUser { profile_data: Vec<u8> },
    ProcessTransaction { amount: u64 },
    UpdateUserProfile { new_profile_data: Vec<u8> },
}

impl FlexNetInstruction {
    pub fn initialize_platform(
        program_id: &Pubkey,
        admin: &Pubkey,
        platform_state: &Pubkey,
    ) -> Result<Instruction, ProgramError> {
        let accounts = vec![
            AccountMeta::new(*admin, true),
            AccountMeta::new(*platform_state, false),
            AccountMeta::new_readonly(system_program::id(), false),
        ];

        let data = FlexNetInstruction::InitializePlatform
            .try_to_vec()
            .unwrap();

        Ok(Instruction {
            program_id: *program_id,
            accounts,
            data,
        })
    }

    // Add other instruction creation methods here
}