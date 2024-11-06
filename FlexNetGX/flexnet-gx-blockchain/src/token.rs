use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program::invoke,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
    program_pack::Pack,
};

use spl_token::{
    instruction as token_instruction,
    state::{Account as TokenAccount, Mint},
};

pub struct TokenManager<'a> {
    pub program_id: &'a Pubkey,
    pub token_program: &'a AccountInfo<'a>,
}

impl<'a> TokenManager<'a> {
    pub fn new(program_id: &'a Pubkey, token_program: &'a AccountInfo<'a>) -> Self {
        Self {
            program_id,
            token_program,
        }
    }

    /// Creates a new token mint
    pub fn create_mint(
        &self,
        payer: &AccountInfo<'a>,
        mint: &AccountInfo<'a>,
        mint_authority: &Pubkey,
        freeze_authority: Option<&Pubkey>,
        decimals: u8,
        system_program: &AccountInfo<'a>,
        rent_sysvar: &AccountInfo<'a>,
    ) -> ProgramResult {
        // Create mint account
        let rent = &Rent::from_account_info(rent_sysvar)?;
        let mint_rent = rent.minimum_balance(Mint::LEN);

        invoke(
            &system_instruction::create_account(
                payer.key,
                mint.key,
                mint_rent,
                Mint::LEN as u64,
                &spl_token::id(),
            ),
            &[payer.clone(), mint.clone(), system_program.clone()],
        )?;

        // Initialize mint
        invoke(
            &token_instruction::initialize_mint(
                &spl_token::id(),
                mint.key,
                mint_authority,
                freeze_authority,
                decimals,
            )?,
            &[mint.clone(), rent_sysvar.clone()],
        )?;

        Ok(())
    }

    /// Creates a new token account
    pub fn create_token_account(
        &self,
        payer: &AccountInfo<'a>,
        token_account: &AccountInfo<'a>,
        mint: &AccountInfo<'a>,
        owner: &AccountInfo<'a>,
        system_program: &AccountInfo<'a>,
        rent_sysvar: &AccountInfo<'a>,
    ) -> ProgramResult {
        let rent = &Rent::from_account_info(rent_sysvar)?;
        let account_rent = rent.minimum_balance(TokenAccount::LEN);

        invoke(
            &system_instruction::create_account(
                payer.key,
                token_account.key,
                account_rent,
                TokenAccount::LEN as u64,
                &spl_token::id(),
            ),
            &[payer.clone(), token_account.clone(), system_program.clone()],
        )?;

        invoke(
            &token_instruction::initialize_account(
                &spl_token::id(),
                token_account.key,
                mint.key,
                owner.key,
            )?,
            &[
                token_account.clone(),
                mint.clone(),
                owner.clone(),
                rent_sysvar.clone(),
            ],
        )?;

        Ok(())
    }

    pub fn mint_to(
        &self,
        mint: &AccountInfo<'a>,
        destination: &AccountInfo<'a>,
        authority: &AccountInfo<'a>,
        amount: u64,
    ) -> ProgramResult {
        invoke(
            &token_instruction::mint_to(
                &spl_token::id(),
                mint.key,
                destination.key,
                authority.key,
                &[],
                amount,
            )?,
            &[mint.clone(), destination.clone(), authority.clone()],
        )
    }

    pub fn transfer(
        &self,
        source: &AccountInfo<'a>,
        destination: &AccountInfo<'a>,
        authority: &AccountInfo<'a>,
        amount: u64,
    ) -> ProgramResult {
        invoke(
            &token_instruction::transfer(
                &spl_token::id(),
                source.key,
                destination.key,
                authority.key,
                &[],
                amount,
            )?,
            &[source.clone(), destination.clone(), authority.clone()],
        )
    }
}

