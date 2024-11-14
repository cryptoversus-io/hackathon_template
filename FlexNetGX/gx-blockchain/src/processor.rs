// gx-blockchain/src/processor.rs
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
    msg,
};

use borsh::BorshDeserialize;
use crate::instruction::BountyInstruction;
use crate::state::{BountyAccount, BountyStatus};

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = BountyInstruction::try_from_slice(instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        match instruction {
            BountyInstruction::CreateBounty { amount, description_hash } => {
                msg!("Instruction: Create Bounty");
                Self::process_create_bounty(program_id, accounts, amount, description_hash)
            },
            BountyInstruction::AcceptBounty => {
                msg!("Instruction: Accept Bounty");
                Self::process_accept_bounty(accounts)
            },
            BountyInstruction::SubmitWork { submission_hash } => {
                msg!("Instruction: Submit Work");
                Self::process_submit_work(accounts, submission_hash)
            },
        }
    }

    pub fn process_create_bounty(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
        description_hash: [u8; 32],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let creator_info = next_account_info(account_info_iter)?;
        let bounty_account_info = next_account_info(account_info_iter)?;
        let system_program = next_account_info(account_info_iter)?;

        msg!("Creator: {}", creator_info.key);
        msg!("Bounty Account: {}", bounty_account_info.key);

        if !creator_info.is_signer {
            msg!("Creator must be a signer");
            return Err(ProgramError::MissingRequiredSignature);
        }

        // Verify we have sufficient lamports
        let rent = Rent::get()?;
        let required_lamports = rent.minimum_balance(std::mem::size_of::<BountyAccount>());
        
        if bounty_account_info.lamports() < required_lamports {
            msg!("Insufficient lamports for account rent exemption");
            return Err(ProgramError::InsufficientFunds);
        }

        // Create the bounty account
        let bounty = BountyAccount {
            creator: *creator_info.key,
            amount,
            description_hash,
            status: BountyStatus::Open,
            hunter: None,
        };

        bounty.serialize(&mut *bounty_account_info.data.borrow_mut())?;
        msg!("Bounty created successfully");
        Ok(())
    }

    pub fn process_accept_bounty(accounts: &[AccountInfo]) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let hunter_info = next_account_info(account_info_iter)?;
        let bounty_account_info = next_account_info(account_info_iter)?;

        msg!("Hunter: {}", hunter_info.key);
        msg!("Bounty Account: {}", bounty_account_info.key);

        if !hunter_info.is_signer {
            msg!("Hunter must be a signer");
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mut bounty = BountyAccount::try_from_slice(&bounty_account_info.data.borrow())?;
        
        if bounty.status != BountyStatus::Open {
            msg!("Bounty is not open for acceptance");
            return Err(ProgramError::InvalidAccountData);
        }

        bounty.status = BountyStatus::InProgress;
        bounty.hunter = Some(*hunter_info.key);
        bounty.serialize(&mut *bounty_account_info.data.borrow_mut())?;
        
        msg!("Bounty accepted successfully");
        Ok(())
    }

    pub fn process_submit_work(
        accounts: &[AccountInfo],
        submission_hash: [u8; 32],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let hunter_info = next_account_info(account_info_iter)?;
        let bounty_account_info = next_account_info(account_info_iter)?;
        let creator_info = next_account_info(account_info_iter)?;

        msg!("Hunter: {}", hunter_info.key);
        msg!("Bounty Account: {}", bounty_account_info.key);

        if !hunter_info.is_signer {
            msg!("Hunter must be a signer");
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mut bounty = BountyAccount::try_from_slice(&bounty_account_info.data.borrow())?;
        
        if bounty.status != BountyStatus::InProgress {
            msg!("Bounty is not in progress");
            return Err(ProgramError::InvalidAccountData);
        }

        if bounty.hunter != Some(*hunter_info.key) {
            msg!("Only assigned hunter can submit work");
            return Err(ProgramError::InvalidAccountData);
        }

        // Verify the creator account
        if bounty.creator != *creator_info.key {
            msg!("Invalid creator account");
            return Err(ProgramError::InvalidAccountData);
        }

        bounty.status = BountyStatus::Completed;
        bounty.serialize(&mut *bounty_account_info.data.borrow_mut())?;
        
        msg!("Work submitted successfully");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::clock::Epoch;
    use solana_program::program_pack::Pack;
    use solana_program_test::*;
    use solana_sdk::{account::Account, signature::Signer};

    #[tokio::test]
    async fn test_create_bounty() {
        let program_id = Pubkey::new_unique();
        let creator_key = Pubkey::new_unique();
        let bounty_key = Pubkey::new_unique();

        let mut creator_account = Account::new(
            1000000000, // lamports
            0,
            &system_program::id(),
        );
        creator_account.is_signer = true;

        let mut bounty_account = Account::new(
            rent::MINIMUM_BALANCE,
            std::mem::size_of::<BountyAccount>(),
            &program_id,
        );

        let mut program_test = ProgramTest::new(
            "gx_blockchain",
            program_id,
            processor!(Processor::process),
        );

        program_test.add_account(creator_key, creator_account);
        program_test.add_account(bounty_key, bounty_account);

        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Test create bounty instruction
        let instruction = BountyInstruction::CreateBounty {
            amount: 100,
            description_hash: [0; 32],
        };

        let mut transaction = Transaction::new_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer], recent_blockhash);

        banks_client.process_transaction(transaction).await.unwrap();
    }
}