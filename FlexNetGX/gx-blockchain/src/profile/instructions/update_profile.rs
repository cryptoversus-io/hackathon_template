use std::collections::HashMap;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Burn, Mint, MintTo, Token, TokenAccount},
};
use mpl_token_metadata::{
    instructions::UpdateV1Builder,
    types::{CreateArgs, Creator, Data, DataV2},
    ID as MPL_ID,
};
use solana_address_lookup_table_program::{
    instruction::{create_lookup_table, extend_lookup_table, freeze_lookup_table},
    ID as ADDRESS_LOOKUP_TABLE_PROGRAM,
};
use solana_program::program::{invoke, invoke_signed};

use crate::{
    _main::MainState,
    activation_token::ActivationTokenState,
    constants::{
        SEED_ACTIVATION_TOKEN_STATE, SEED_MAIN_STATE, SEED_PROFILE_STATE, SEED_VAULT,
        TOTAL_SELLER_BASIS_POINTS,
    },
    error::MyError,
    other_states::LineageInfo,
    profile_state::ProfileState,
    utils::{
        get_vault_pda, init_ata_if_needed, transfer_tokens,
        verify_collection_item_by_main,
    },
};

/// This is the function used to update profile nft 
pub fn update_profile(
    ctx: Context<AUpdateMint>,
    name: Box<String>,
    symbol: Box<String>,
    uri_hash: Box<String>,
) -> Result<()> {
    let name = *name;
    let symbol = *symbol;
    let uri_hash = *uri_hash;
    {
        //NOTE: updating profile
        ctx.accounts.update(name, symbol, uri_hash)?;
    }
    Ok(())
}



/// Struct used to create context while preparing instruction for update profile
#[derive(Accounts)]
#[instruction(
    name: Box<String>,
    symbol: Box<String>,
    uri: Box<String>,
)]
pub struct AUpdateMint<'info> {
    /// user publickey and mandatory signer while pushing the instruction to solana
    #[account(mut)]
    pub user: Signer<'info>,

    /// mpl program public key
    ///CHECK:
    #[account(address = MPL_ID)]
    pub mpl_program: AccountInfo<'info>,
    /// token program public key
    pub token_program: Program<'info, Token>,
    /// associated token program public key
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// sytem program public key
    pub system_program: Program<'info, System>,

    /// profile nft public key
    #[account(
        mut,
    )]
    pub mint: Box<Account<'info, Mint>>,

    /// load mainstate account by public key
    #[account(
        mut,
        seeds = [SEED_MAIN_STATE],
        bump,
    )]
    pub main_state: Box<Account<'info, MainState>>,

    /// profile metadata public key
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
    pub metadata: AccountInfo<'info>,

    /// system var instruction public key
    ///CHECK:
    #[account()]
    pub sysvar_instructions: AccountInfo<'info>,


}

impl<'info> AUpdateMint<'info> {
    pub fn update(&mut self, name: String, symbol: String, uri_hash: String) -> Result<()> {
        let mint = self.mint.to_account_info();
        let user = self.user.to_account_info();
        let system_program = self.system_program.to_account_info();
        let token_program = self.token_program.to_account_info();
        let mpl_program = self.mpl_program.to_account_info();
        let metadata = self.metadata.to_account_info();
        let mpl_program = self.mpl_program.to_account_info();
        let sysvar_instructions = self.sysvar_instructions.to_account_info();
        let main_state = &mut self.main_state;

        let creators = vec![
            //NOTE: currently not royalty info for creator
            Creator {
                address: user.key(),
                verified: false,
                share: 100,
            }
        ];

        let asset_data = Data {
            name,
            symbol,
            uri: uri_hash,
            seller_fee_basis_points: 0, //EX: 20% (80% goes to seller)
            creators: Some(creators),
        };

        let ix = UpdateV1Builder::new()
        .authority(main_state.key())
        .metadata(metadata.key())
        .payer(user.key())
        .mint(mint.key())
        .sysvar_instructions(sysvar_instructions.key())
        .data(asset_data)
        .instruction();


        invoke_signed(
            &ix,
            &[
                mint,
                user,
                main_state.to_account_info(),
                metadata,
                mpl_program,
                token_program,
                system_program,
                sysvar_instructions,
            ],
            &[
                &[SEED_MAIN_STATE, &[self.main_state._bump]],
            ],
        )?;

        Ok(())
    }

}
