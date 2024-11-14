use crate::{
    _main::main_state::{MainState, MainStateInput},
    constants::SEED_MAIN_STATE,
    error::MyError,
};
use anchor_lang::prelude::*;

/// This is the function which used to update PDA account for all saved value
pub fn update_main_state(ctx: Context<AUpdateMainState>, input: MainStateInput) -> Result<()> {
    let main_state = &mut ctx.accounts.main_state;
    input.set_value(main_state);
    Ok(())
}

/// Struct used to create context while preparing instruction for update mainstate
#[derive(Accounts)]
pub struct AUpdateMainState<'info> {
    /// owner publickey and mandatory signer while pushing the instruction to solana
    #[account(
        mut,
        address = main_state.owner @ MyError::OnlyOwnerCanCall,
    )]
    pub owner: Signer<'info>,
    /// load mainstate account with seed for update purpose
    #[account(
        mut,
        seeds = [SEED_MAIN_STATE],
        bump,
    )]
    pub main_state: Account<'info, MainState>,
}
