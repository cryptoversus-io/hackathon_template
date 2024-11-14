use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{self, Mint, Token, TokenAccount, Transfer}};

use crate::{constants::SEED_VAULT, error::MyError, vault::VaultState};

pub fn unstake_vault(
    ctx: Context<UnstakeVault>,
    value: u64
) -> Result<()> {
    {
        ctx.accounts.init_unstake_vault(value)?;
    }
    Ok(())
}

#[derive(Accounts)]

pub struct UnstakeVault<'info> {
    #[account(
        mut,
    )]
    pub receiver: Signer<'info>,

    #[account(
        mut,
        token::mint = mint,
        constraint = receiver_ata.owner == vault.authority
    )]
    pub receiver_ata: Box<Account<'info, TokenAccount>>,

    #[account()]
    pub mint: Box<Account<'info, Mint>>,

    ///CHECK:
    pub stake_key: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [SEED_VAULT, stake_key.key().as_ref(),mint.key().as_ref()],
        bump
    )]
    pub vault: Box<Account<'info, VaultState>>,

    #[account(
        mut,
        associated_token::mint = mint, // mint of the token
        associated_token::authority = vault, //authority that should be a PDA account
        constraint = vault.authority.key() == receiver.key() @ MyError::OnlyOwnerCanCall,
    )]
    token_account: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,
}

impl<'info> UnstakeVault<'info> {
    pub fn init_unstake_vault(&self, value: u64) -> Result<()> {
        if self.vault.lock_date as i64 > self.clock.unix_timestamp {
            return Err(error!(MyError::TimeLockNotExpired));
        }
        let cpi_accounts = Transfer {
            from: self
            .token_account
            .to_account_info(),
            to: self.receiver_ata.to_account_info(),
            authority: self
            .vault
            .to_account_info()
        };
        token::transfer(CpiContext::new(self.token_program.to_account_info(), cpi_accounts).with_signer(&[&[SEED_VAULT, self.stake_key.key().as_ref() ,self.mint.key().as_ref(), &[self.vault._bump]]]), value)?;
        Ok(())
    }
}