use crate::constants::{COLLATERAL_SEED, CONFIGURATION_SEED, SOL_SEED};
use crate::instructions::deposit::util::{deposit_sol, mint_tokens};
use crate::states::{Collatoral, Configuration};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, Token2022, TokenAccount},
};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;
#[derive(Accounts)]
pub struct DepositCollateralAndMintToken<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,
    #[account(seeds=[CONFIGURATION_SEED],bump=config_account.bump_config_account,has_one=mint_account)]
    pub config_account: Box<Account<'info, Configuration>>,
    #[account(mut)]
    pub mint_account: InterfaceAccount<'info, Mint>,
    #[account(init_if_needed,payer=depositor,space=8+Collatoral::INIT_SPACE,seeds=[COLLATERAL_SEED,depositor.key().as_ref()],bump)]
    pub collateral_account: Account<'info, Collatoral>,
    #[account(mut,seeds=[SOL_SEED,depositor.key().as_ref()],bump)]
    pub sol_account: SystemAccount<'info>,
    #[account(init_if_needed,payer=depositor,associated_token::mint=mint_account,
        associated_token::authority=depositor,associated_token::token_program=token_program)]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub price_update: Account<'info, PriceUpdateV2>,
}
pub fn process_deposit_collateral_and_mint_token(
    ctx: Context<DepositCollateralAndMintToken>,
    amount_collateral: u64,
    amount_to_mint: u64,
) -> Result<()> {
    let collator_account = &mut ctx.accounts.collateral_account;
    collator_account.lamport_balance = ctx.accounts.sol_account.lamports() + amount_collateral;
    collator_account.amount_minted += amount_to_mint;
    if collator_account.is_initialized == false {
        collator_account.is_initialized = true;
        collator_account.depositor = ctx.accounts.depositor.key();
        collator_account.sol_account = ctx.accounts.sol_account.key();
        collator_account.token_account = ctx.accounts.token_account.key();
        collator_account.bump_collateral_account = ctx.bumps.collateral_account;
        collator_account.bump_sol_account = ctx.bumps.sol_account;
    }
    deposit_sol(
        &ctx.accounts.system_program,
        &ctx.accounts.depositor,
        &ctx.accounts.sol_account,
        amount_collateral,
    )?;
    mint_tokens(
        &ctx.accounts.token_program,
        ctx.accounts.config_account.bump_mint_account,
        &ctx.accounts.mint_account,
        &ctx.accounts.token_account,
        amount_to_mint,
    )?;
    Ok(())
}
