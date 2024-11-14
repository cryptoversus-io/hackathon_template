use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, MintTo, Token, TokenAccount},
};
use mpl_token_metadata::{
    instructions::Create,
    types::Creator,
    ID as MPL_ID,
};
use solana_program::program::{invoke, invoke_signed};

use crate::{
    _main::MainState,
    activation_token::ActivationTokenState,
    constants::{SEED_ACTIVATION_TOKEN_STATE, SEED_LAUNCH_PASS, SEED_MAIN_STATE, SEED_PROFILE_STATE, TOTAL_SELLER_BASIS_POINTS},
    error::MyError,
    other_states::LineageInfo,
    profile::profile_state::ProfileState,
    utils::{_verify_collection,init_ata_if_needed, transfer_tokens}, LaunchPassState,
};

pub fn buy_launch_pass(ctx: Context<BuyLaunchPass>) -> Result<()> {
    let minter = ctx.accounts.receiver.to_account_info();
    let mint = ctx.accounts.mint.to_account_info();
    let main_state = &mut ctx.accounts.launc_pass_state;
    let token_program = ctx.accounts.token_program.to_account_info();

    let cpi_accounts = MintTo {
        mint,
        to: ctx.accounts.receiver_ata.to_account_info(),
        authority: main_state.to_account_info(),
    };

    token::mint_to(
        CpiContext::new_with_signer(
            token_program,
            cpi_accounts,
            &[&[SEED_LAUNCH_PASS, ctx.accounts.owner.key().as_ref(), ctx.accounts.mint.key().as_ref(), &[main_state._bump]]],
        ),
        1,
    )?;

    // NOTE: minting cost distribution
    // let token_program = ctx.accounts.token_program.to_account_info();
    // let sender_ata = ctx.accounts.sender_ata.to_account_info();
    // let authority = ctx.accounts.receiver.to_account_info();
    // let cost = main_state.cost;
    // let minting_cost_distribution = main_state.distribution;

    // let parent_cost = (cost as u128 * minting_cost_distribution.parent as u128
    //     / TOTAL_SELLER_BASIS_POINTS as u128) as u64;

    // let grand_parent_cost = (cost as u128 * minting_cost_distribution.grand_parent as u128
    //         / TOTAL_SELLER_BASIS_POINTS as u128) as u64;

    // parent
    // transfer_tokens(
    //     sender_ata.to_account_info(),
    //     ctx.accounts
    //         .parent_profile_holder_opos_ata
    //         .to_account_info(),
    //     authority.to_account_info(),
    //     token_program.to_account_info(),
    //     parent_cost,
    // )?;

    // // Grand Parent
    // transfer_tokens(
    //     sender_ata.to_account_info(),
    //     ctx.accounts
    //         .grand_parent_profile_holder_opos_ata
    //         .to_account_info(),
    //     authority.to_account_info(),
    //     token_program.to_account_info(),
    //     grand_parent_cost,
    // )?;

    // // Genesis
    // transfer_tokens(
    //     sender_ata.to_account_info(),
    //     ctx.accounts
    //         .owner_ata
    //         .to_account_info(),
    //     authority.to_account_info(),
    //     token_program.to_account_info(),
    //     cost as u64,
    // )?;
    
    Ok(())
}

#[derive(Accounts)]
pub struct BuyLaunchPass<'info> {
    #[account(
        mut,
    )]
    pub receiver: Signer<'info>,

    ///CHECK:
    #[account(
        mut,
        token::mint = mint
    )]
    pub receiver_ata: Box<Account<'info, TokenAccount>>,

    ///CHECK:
    pub owner: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [SEED_LAUNCH_PASS, owner.key().as_ref() ,mint.key().as_ref()],
        bump,
    )]
    pub launc_pass_state: Box<Account<'info, LaunchPassState>>,

    // ///CHECK:
    // #[account(
    //     mut,
    //     token::mint = usdc_mint
    // )]
    // pub owner_ata: Box<Account<'info, TokenAccount>>,

    // ///CHECK:
    // #[account(
    //     mut,
    //     token::mint = usdc_mint
    // )]
    // pub sender_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
    )]
    pub mint: Box<Account<'info, Mint>>,

    // ///CHECK:
    // pub usdc_mint: AccountInfo<'info>,


    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,

    //NOTE: profile minting cost distribution account
    // #[account(address = activation_token_state.parent_profile @ MyError::ProfileIdMissMatch)]
    // pub parent_profile: Box<Account<'info, Mint>>,
    // pub grand_parent_profile: Box<Account<'info, Mint>>,

    // // Current parent profile holded ata
    // #[account(
    //     token::mint = usdc_mint,
    // )]
    // pub current_parent_profile_holder_ata: Box<Account<'info, TokenAccount>>,
    // #[account(
    //     token::mint = usdc_mint,
    // )]
    // pub current_grand_parent_profile_holder_ata: Box<Account<'info, TokenAccount>>,

    // // Current profile holders
    // ///CHECK:
    // #[account(address = current_parent_profile_holder_ata.owner)]
    // pub current_parent_profile_holder: AccountInfo<'info>,
    // ///CHECK:
    // #[account(address = current_grand_parent_profile_holder_ata.owner)]
    // pub current_grand_parent_profile_holder: AccountInfo<'info>,

    // ///CHECK:
    // #[account(
    //     mut,
    //     constraint = init_ata_if_needed(
    //         usdc_mint.to_account_info(),
    //         parent_profile_holder_opos_ata.to_account_info(),
    //         current_parent_profile_holder.to_account_info(),
    //         receiver.to_account_info(),
    //         token_program.to_account_info(),
    //         system_program.to_account_info(),
    //         associated_token_program.to_account_info(),
    //     ) == Ok(())
    //     // token::mint = opos_token,
    //     // token::authority = current_parent_profile_holder,
    // )]
    // // pub parent_profile_holder_opos_ata: Box<Account<'info, TokenAccount>>,
    // pub parent_profile_holder_opos_ata: AccountInfo<'info>,
    // ///CHECK:
    // #[account(
    //     mut,
    //     constraint = init_ata_if_needed(
    //         usdc_mint.to_account_info(),
    //         grand_parent_profile_holder_opos_ata.to_account_info(),
    //         current_grand_parent_profile_holder.to_account_info(),
    //         receiver.to_account_info(),
    //         token_program.to_account_info(),
    //         system_program.to_account_info(),
    //         associated_token_program.to_account_info(),
    //     ) == Ok(())
    //     // token::mint = opos_token,
    //     // token::authority = current_grand_parent_profile_holder,
    // )]
    // pub grand_parent_profile_holder_opos_ata: AccountInfo<'info>,

}