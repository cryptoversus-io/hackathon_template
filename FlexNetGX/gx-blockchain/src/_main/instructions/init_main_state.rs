use crate::{
    _main::main_state::{MainState, MainStateInput},
    constants::SEED_MAIN_STATE,
};
use anchor_lang::{prelude::*, Discriminator};
use anchor_spl::token::Mint;

/// This is the function which used to create PDA account for profile nft configuration
pub fn init_main_state(ctx: Context<AInitMainState>, input: MainStateInput) -> Result<()> {
    let main_state = &mut ctx.accounts.main_state;
    let owner = ctx.accounts.owner.to_account_info();
    input.set_value(main_state);
    main_state.owner = owner.key();
    main_state._bump = ctx.bumps.main_state;

    Ok(())
}

/// Struct used to create context while preparing instruction for mainstate
#[derive(Accounts)]
pub struct AInitMainState<'info> {
    /// owner publickey and mandatory signer while pushing the instruction to solana
    #[account(mut)]
    pub owner: Signer<'info>,

    /// Main state PDA account which manage profile nft configuration
    #[account(
        init,
        payer = owner,
        seeds = [SEED_MAIN_STATE],
        bump,
        space = 8 + MainState::MAX_SIZE, 
    )]
    pub main_state: Account<'info, MainState>,

    /// system program account for this context
    pub system_program: Program<'info, System>,
}

