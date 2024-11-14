use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Burn, Mint, MintTo, Token, TokenAccount, Transfer},
};

use mpl_token_metadata::{
    instructions::{
        ApproveCollectionAuthority, Create, CreateBuilder, Verify, VerifyInstructionArgs
    },
    types::{CreateArgs, Creator},
    ID as MPL_ID,
};

use crate::{constants::{SEED_LAUNCH_PASS, SEED_VAULT}, error::MyError, LaunchPassState, VaultState};

pub fn redeem_launch_pass(ctx: Context<RedeemLaunchPass>) -> Result<()> {
    {
        ctx.accounts.burn_launch_pass();
    }
    {
        ctx.accounts.redeem_launch_pass();
    }
    Ok(())
}

#[derive(Accounts)]
pub struct RedeemLaunchPass<'info> {
    #[account(
        mut,
    )]
    pub user: Signer<'info>,

    #[account(mut)]
    pub launch_token: Box<Account<'info, Mint>>,

    ///CHECK:
    pub owner: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [SEED_LAUNCH_PASS, owner.key().as_ref(),launch_token.key().as_ref()],
        bump,
    )]
    pub launc_pass_state: Box<Account<'info, LaunchPassState>>,

    ///CHECK:
    pub stake_key: AccountInfo<'info>,
    ///CHECK:
    #[account(
        mut,
        seeds = [SEED_VAULT, stake_key.key().as_ref(),mint.key().as_ref()],
        bump
    )]
    pub vault: Box<Account<'info, VaultState>>,

    ///CHECK:
    #[account(
        mut,
        token::mint = launch_token
    )]
    pub user_launch_token_ata: Box<Account<'info, TokenAccount>>,

    ///CHECK:
    #[account()]
    pub sysvar_instructions: AccountInfo<'info>,

    #[account(
        mut,
        token::mint = mint,
    )]
    pub receiver_ata: Box<Account<'info, TokenAccount>>,

    #[account()]
    pub mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        associated_token::mint = mint, // mint of the token
        associated_token::authority = vault, //authority that should be a PDA account
    )]
    token_account: Account<'info, TokenAccount>,


    ///CHECK:
    #[account(address = MPL_ID)]
    pub mpl_program: AccountInfo<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,
    
}

impl<'info> RedeemLaunchPass<'info> {
    pub fn redeem_launch_pass(&mut self) -> Result<()> {
        if self.clock.unix_timestamp > self.launc_pass_state.redeem_date as i64 {
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
        token::transfer(CpiContext::new(self.token_program.to_account_info(), cpi_accounts).with_signer(&[&[SEED_VAULT, self.stake_key.key().as_ref() ,self.mint.key().as_ref(), &[self.vault._bump]]]), self.launc_pass_state.redeem_amount)?;
        Ok(())
    }
    pub fn burn_launch_pass(&mut self) -> Result<()> {
        let mint = self.launch_token.to_account_info();
        let user = self.user.to_account_info();
        let user_launch_token_ata = self.user_launch_token_ata.to_account_info();
        let system_program = self.system_program.to_account_info();
        let token_program = self.token_program.to_account_info();
        let mpl_program = self.mpl_program.to_account_info();
        let sysvar_instructions = self.sysvar_instructions.to_account_info();

        let cpi_accounts = Burn {
            mint,
            from: user_launch_token_ata,
            authority: user,
        };

        token::burn(CpiContext::new(token_program, cpi_accounts), 1)?;
        Ok(())
    }
}