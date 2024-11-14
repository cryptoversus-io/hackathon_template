use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};
use mpl_token_metadata::{
    instructions::{Create, CreateBuilder},
    types::{
         CreateArgs, Creator, DataV2,
    },
    ID as MPL_ID,
};
use solana_program::program::invoke_signed;

use crate::{constants::SEED_LAUNCH_PASS, launchpass::LaunchPassState, other_states::MintingCostDistribution};

pub fn init_launch_pass(
    ctx: Context<InitLaunchPass>,
    redeem_amount: u64,
    redeem_date: u64,
    cost: u64,
    distribution: MintingCostDistribution,
    name: String,
    symbol: String,
    uri: String
) -> Result<()> {
    {
        let launch_pass = &mut ctx.accounts.launch_pass;
        launch_pass.owner = ctx.accounts.owner.key();
        launch_pass.redeem_date = redeem_date;
        launch_pass.redeem_amount = redeem_amount;
        launch_pass.cost = cost;
        launch_pass.distribution = distribution;
        launch_pass._bump = ctx.bumps.launch_pass;
    }
    {
        ctx.accounts.init_launch(name, symbol, uri);
    }
    Ok(())
}

#[derive(Accounts)]
pub struct InitLaunchPass<'info> {
    #[account(
        mut,
    )]
    pub owner: Signer<'info>,

    #[account(
        init,
        signer,
        payer = owner,
        mint::decimals = 0,
        mint::authority = launch_pass,
        mint::freeze_authority = launch_pass,
    )]
    pub mint: Box<Account<'info, Mint>>,
    
    #[account(
        init,
        seeds = [SEED_LAUNCH_PASS, owner.key().as_ref(), mint.key().as_ref()],
        bump,
        payer = owner,
        space= 8 + LaunchPassState::MAX_SIZE
    )]
    pub launch_pass: Box<Account<'info, LaunchPassState>>,
    
    ///CHECK:
    #[account(
        mut,
        seeds=[
            "metadata".as_bytes(),
            MPL_ID.as_ref(),
            mint.key().as_ref(),
        ],
        bump,
        seeds::program = MPL_ID
    )]
    pub mint_metadata: AccountInfo<'info>,
    
    ///CHECK:
    #[account()]
    pub sysvar_instructions: AccountInfo<'info>,

    ///CHECK:
    #[account(address = MPL_ID)]
    pub mpl_program: AccountInfo<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,
}

impl<'info> InitLaunchPass<'info> {
    pub fn init_launch(&self, name: String, symbol: String, uri: String) -> Result<()> {
        let mint = self.mint.to_account_info();
        let user = self.owner.to_account_info();
        let system_program = self.system_program.to_account_info();
        let token_program = self.token_program.to_account_info();
        let mpl_program = self.mpl_program.to_account_info();
        let metadata = self.mint_metadata.to_account_info();
        let mpl_program = self.mpl_program.to_account_info();
        let ata_program = self.associated_token_program.to_account_info();
        let sysvar_instructions = self.sysvar_instructions.to_account_info();
        let main_state = self.launch_pass.clone();

        let asset_data = CreateArgs::V1 {
            name,
            symbol,
            uri,
            collection: None,
            uses: None,
            creators: Some(vec![
                Creator {
                    address: main_state.key(),
                    verified: true,
                    share: 0,
                },
                Creator {
                    address: user.key(),
                    verified: false,
                    share: 100,
                },
            ]),
            collection_details: None,
            is_mutable: true, //NOTE: may be for testing
            rule_set: None,
            token_standard: mpl_token_metadata::types::TokenStandard::FungibleAsset,
            primary_sale_happened: false,
            seller_fee_basis_points: 100,
            decimals: Some(0),
            print_supply: None,
        };

        let ix = CreateBuilder::new()
        .metadata(metadata.key())
        .master_edition(None)
        .mint( mint.key(), true)
        .authority(main_state.key())
        .payer(user.key())
        .update_authority(main_state.key(),true)
        .spl_token_program(Some(token_program.key()))
        .sysvar_instructions(sysvar_instructions.key())
        .create_args(asset_data)
        .instruction();

        invoke_signed(
            &ix,
            &[
                mint.to_account_info(),
                user.to_account_info(),
                main_state.to_account_info(),
                metadata,
                mpl_program,
                token_program.to_account_info(),
                system_program.to_account_info(),
                sysvar_instructions,
            ],
            &[&[SEED_LAUNCH_PASS, user.key().as_ref(), self.mint.key().as_ref(), &[main_state._bump]]],
        )?;
        Ok(())
    }
}