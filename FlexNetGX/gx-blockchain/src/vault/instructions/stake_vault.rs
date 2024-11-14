use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};

use crate::{constants::SEED_VAULT, utils::transfer_tokens, vault::VaultState};

pub fn stake_vault(
    ctx: Context<StakeVault>,
    value: u64
) -> Result<()> {
    {
        ctx.accounts.init_stake_vault(value)?;
    }
    Ok(())
}

#[derive(Accounts)]
pub struct StakeVault<'info> {
    #[account(
        mut,
    )]
    pub owner: Signer<'info>,

    #[account(
        mut,
        token::mint = mint,
    )]
    pub owner_ata: Box<Account<'info, TokenAccount>>,

    #[account()]
    pub mint: Box<Account<'info, Mint>>,

    ///CHECK:
    pub authority: AccountInfo<'info>,

    ///CHECK:
    pub stake_key: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [SEED_VAULT, stake_key.key().as_ref(), mint.key().as_ref()],
        bump,
    )]
    pub vault: Box<Account<'info, VaultState>>,
    #[account(
        mut,
        associated_token::mint = mint, // mint of the token
        associated_token::authority = vault //authority that should be a PDA account
    )]
    token_account: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,
}

impl<'info> StakeVault<'info> {
    pub fn init_stake_vault(&self, value: u64) -> Result<()> {
        // transfer_tokens(
        //     self.owner_ata.to_account_info(),
        //     self.token_account.to_account_info(),
        //     self.owner.to_account_info(),
        //     self.token_program.to_account_info(),
        //     value
        // )?;
        Ok(())
    }
}