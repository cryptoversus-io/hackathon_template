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
    constants::{SEED_ACTIVATION_TOKEN_STATE, SEED_MAIN_STATE, SEED_PROFILE_STATE, TOTAL_SELLER_BASIS_POINTS},
    error::MyError,
    other_states::LineageInfo,
    profile::profile_state::ProfileState,
    utils::{_verify_collection,init_ata_if_needed, transfer_tokens},
};

/// This is the function which used to create instruction for minting actviation token
pub fn mint_activation_token(ctx: Context<AMintActivationToken>, amount: u64) -> Result<()> {
    let minter = ctx.accounts.minter.to_account_info();
    let mint = ctx.accounts.activation_token.to_account_info();
    let activation_token_state = &mut ctx.accounts.activation_token_state;
    let main_state = &mut ctx.accounts.main_state;
    let token_program = ctx.accounts.token_program.to_account_info();
    let profile_state = &mut ctx.accounts.profile_state;
    profile_state.total_minted_sft += amount;

    let cpi_accounts = MintTo {
        mint,
        to: ctx.accounts.receiver_ata.to_account_info(),
        authority: main_state.to_account_info(),
    };

    token::mint_to(
        CpiContext::new_with_signer(
            token_program,
            cpi_accounts,
            &[&[SEED_MAIN_STATE, &[main_state._bump]]],
        ),
        amount,
    )?;

    // NOTE: minting cost distribution
    let token_program = ctx.accounts.token_program.to_account_info();
    let sender_ata = ctx.accounts.user_opos_ata.to_account_info();
    let authority = ctx.accounts.minter.to_account_info();
    let main_state = &mut ctx.accounts.main_state;
    let cost = main_state.invitation_minting_cost * amount;

    // Genesis
    transfer_tokens(
        sender_ata.to_account_info(),
        ctx.accounts
            .genesis_profile_holder_opos_ata
            .to_account_info(),
        authority.to_account_info(),
        token_program.to_account_info(),
        cost as u64,
    )?;
    
    Ok(())
}

/// Struct used to create context while preparing instruction for minting new activation token
#[derive(Accounts)]
pub struct AMintActivationToken<'info> {
    /// minter publickey and mandatory signer while pushing the instruction to solana
    #[account(
        mut,
        address = activation_token_state.creator
    )]
    pub minter: Signer<'info>,

    /// load associated token account for minter
    #[account(
        mut,
        token::mint = profile,
        token::authority = minter,
        constraint = minter_profile_ata.amount == 1 @ MyError::OnlyProfileHolderAllow,
    )]
    pub minter_profile_ata: Box<Account<'info, TokenAccount>>,

    /// activation token associated token account for receiver
    ///CHECK:
    #[account(
        mut,
        token::mint = activation_token
    )]
    pub receiver_ata: Box<Account<'info, TokenAccount>>,

    /// load mainstate account with seed for update purpose
    #[account(
        mut,
        seeds = [SEED_MAIN_STATE],
        bump,
    )]
    pub main_state: Box<Account<'info, MainState>>,

    /// activation token public key
    #[account(
        mut,
        address = profile_state.activation_token.unwrap() @ MyError::ActivationTokenNotFound
    )]
    pub activation_token: Box<Account<'info, Mint>>,

    /// load activiation state account public key 
    #[account(
        mut,
        seeds = [SEED_ACTIVATION_TOKEN_STATE,activation_token.key().as_ref()],
        bump,
    )]
    pub activation_token_state: Box<Account<'info, ActivationTokenState>>,

    /// profile nft public key
    #[account()]
    pub profile: Box<Account<'info, Mint>>,

    /// load profile state account public key 
    #[account(
        mut,
        seeds = [SEED_PROFILE_STATE,profile.key().as_ref()],
        bump,
    )]
    pub profile_state: Box<Account<'info, ProfileState>>,
    /// metaplex token program public key
    pub token_program: Program<'info, Token>,
    /// metaplex associated token program public key
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// system program public key
    pub system_program: Program<'info, System>,

    /// parent profile nft public key
    //NOTE: profile minting cost distribution account
    // #[account(address = activation_token_state.parent_profile @ MyError::ProfileIdMissMatch)]
    pub parent_profile: Box<Account<'info, Mint>>,
    /// grand parent profile nft public key
    pub grand_parent_profile: Box<Account<'info, Mint>>,
    /// great grand parent profile nft public key
    pub great_grand_parent_profile: Box<Account<'info, Mint>>,
    /// great great grand parent profile nft public key
    pub ggreate_grand_parent_profile: Box<Account<'info, Mint>>,
    /// gensis profile nft public key
    pub genesis_profile: Box<Account<'info, Mint>>,

    /// load parent profile state public key
    #[account(
        mut,
        seeds = [SEED_PROFILE_STATE, parent_profile.key().as_ref()],
        bump,
    )]
    pub parent_profile_state: Box<Account<'info, ProfileState>>,

    /// token public key which used to pay for activiation token mint
    ///CHECK:
    #[account(address = main_state.opos_token)]
    pub opos_token: AccountInfo<'info>,

    /// parent profile associated token account public key
    #[account(
        token::mint = parent_profile_state.mint,
        constraint = current_parent_profile_holder_ata.amount == 1
    )]
    pub current_parent_profile_holder_ata: Box<Account<'info, TokenAccount>>,
    /// grand parent profile associated token account public key
    #[account(
        token::mint = parent_profile_state.lineage.parent,
        constraint = current_grand_parent_profile_holder_ata.amount == 1
    )]
    pub current_grand_parent_profile_holder_ata: Box<Account<'info, TokenAccount>>,
    /// great grand parent profile associated token account public key
    #[account(
        token::mint = parent_profile_state.lineage.grand_parent,
        constraint = current_great_grand_parent_profile_holder_ata.amount == 1
    )]
    pub current_great_grand_parent_profile_holder_ata: Box<Account<'info, TokenAccount>>,
    /// great great grand parent profile associated token account public key
    #[account(
        token::mint = parent_profile_state.lineage.great_grand_parent,
        constraint = current_ggreat_grand_parent_profile_holder_ata.amount == 1
    )]
    pub current_ggreat_grand_parent_profile_holder_ata: Box<Account<'info, TokenAccount>>,
    /// gensis profile associated token account public key
    #[account(
        token::mint = main_state.genesis_profile,
        constraint = current_genesis_profile_holder_ata.amount == 1
    )]
    pub current_genesis_profile_holder_ata: Box<Account<'info, TokenAccount>>,

    /// parent profile nft holder public key
    ///CHECK:
    #[account(address = current_parent_profile_holder_ata.owner)]
    pub current_parent_profile_holder: AccountInfo<'info>,
    /// grand parent profile nft holder public key
    ///CHECK:
    #[account(address = current_grand_parent_profile_holder_ata.owner)]
    pub current_grand_parent_profile_holder: AccountInfo<'info>,
    /// great grand parent profile nft holder public key
    ///CHECK:
    #[account(address = current_great_grand_parent_profile_holder_ata.owner)]
    pub current_great_grand_parent_profile_holder: AccountInfo<'info>,
    /// great great grand parent profile nft holder public key
    ///CHECK:
    #[account(address = current_ggreat_grand_parent_profile_holder_ata.owner)]
    pub current_ggreat_grand_parent_profile_holder: AccountInfo<'info>,
    /// gensis profile nft holder public key
    ///CHECK:
    #[account(address = current_genesis_profile_holder_ata.owner)]
    pub current_genesis_profile_holder: AccountInfo<'info>,

    /// signer token assoicated account public key
    #[account(
        mut,
        token::mint = opos_token,
        token::authority = minter,
        constraint= user_opos_ata.amount >= main_state.invitation_minting_cost @ MyError::NotEnoughTokenToMint
    )]
    pub user_opos_ata: Box<Account<'info, TokenAccount>>,

    /// parent profile holder token assoicated account public key
    ///CHECK:
    #[account(
        mut,
        constraint = init_ata_if_needed(
            opos_token.to_account_info(),
            parent_profile_holder_opos_ata.to_account_info(),
            current_parent_profile_holder.to_account_info(),
            minter.to_account_info(),
            token_program.to_account_info(),
            system_program.to_account_info(),
            associated_token_program.to_account_info(),
        ) == Ok(())
        // token::mint = opos_token,
        // token::authority = current_parent_profile_holder,
    )]
    pub parent_profile_holder_opos_ata: AccountInfo<'info>,
    /// grand parent profile holder token assoicated account public key
    ///CHECK:
    #[account(
        mut,
        constraint = init_ata_if_needed(
            opos_token.to_account_info(),
            grand_parent_profile_holder_opos_ata.to_account_info(),
            current_grand_parent_profile_holder.to_account_info(),
            minter.to_account_info(),
            token_program.to_account_info(),
            system_program.to_account_info(),
            associated_token_program.to_account_info(),
        ) == Ok(())
        // token::mint = opos_token,
        // token::authority = current_grand_parent_profile_holder,
    )]
    pub grand_parent_profile_holder_opos_ata: AccountInfo<'info>,
    /// great grand parent profile holder token assoicated account public key
    ///CHECK:
    #[account(
        mut,
        constraint = init_ata_if_needed(
            opos_token.to_account_info(),
            great_grand_parent_profile_holder_opos_ata.to_account_info(),
            current_great_grand_parent_profile_holder.to_account_info(),
            minter.to_account_info(),
            token_program.to_account_info(),
            system_program.to_account_info(),
            associated_token_program.to_account_info(),
        ) == Ok(())
        // token::mint = opos_token,
        // token::authority = current_great_grand_parent_profile_holder,
    )]
    pub great_grand_parent_profile_holder_opos_ata: AccountInfo<'info>,
    /// great great grand parent profile holder token assoicated account public key
    ///CHECK:
    #[account(
        mut,
        constraint = init_ata_if_needed(
            opos_token.to_account_info(),
            ggreat_grand_parent_profile_holder_opos_ata.to_account_info(),
            current_ggreat_grand_parent_profile_holder.to_account_info(),
            minter.to_account_info(),
            token_program.to_account_info(),
            system_program.to_account_info(),
            associated_token_program.to_account_info(),
        ) == Ok(())
        // token::mint = opos_token,
        // token::authority = current_ggreat_grand_parent_profile_holder,
    )]
    pub ggreat_grand_parent_profile_holder_opos_ata: AccountInfo<'info>,
    /// gensis profile holder token assoicated account public key
    ///CHECK:
    #[account(
        mut,
        constraint = init_ata_if_needed(
            opos_token.to_account_info(),
            genesis_profile_holder_opos_ata.to_account_info(),
            current_genesis_profile_holder.to_account_info(),
            minter.to_account_info(),
            token_program.to_account_info(),
            system_program.to_account_info(),
            associated_token_program.to_account_info(),
        ) == Ok(())
        // token::mint = opos_token,
        // token::authority = current_genesis_profile_holder,
    )]
    pub genesis_profile_holder_opos_ata: AccountInfo<'info>,
}
