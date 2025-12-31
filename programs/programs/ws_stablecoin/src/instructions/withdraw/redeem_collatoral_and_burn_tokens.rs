use crate::constants::{COLLATERAL_SEED, CONFIGURATION_SEED};
use crate::instructions::util::calculate_health_factor;
use crate::instructions::withdraw::utils::{burn_tokens, withdraw_sol};
use crate::states::{Collatoral, Configuration};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;
use anchor_spl::token_interface::{Token2022, TokenAccount};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;
#[derive(Accounts)]
pub struct RedeemCollateralAndBurnTokens<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,
    #[account(seeds=[CONFIGURATION_SEED],bump=config_account.bump_config_account,has_one=mint_account)]
    pub config_account: Account<'info, Configuration>,
    #[account(mut,seeds=[COLLATERAL_SEED,depositor.key().as_ref()],bump=collateral_account.bump_collateral_account,has_one=sol_account,has_one=token_account)]
    pub collateral_account: Account<'info, Collatoral>,
    #[account(mut)]
    pub sol_account: SystemAccount<'info>,
    #[account(mut)]
    pub mint_account: InterfaceAccount<'info, Mint>,
    pub price_update: Account<'info, PriceUpdateV2>,
    #[account(mut)]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
}

pub fn process_redeem_collatoral_and_burn_tokens(
    ctx: Context<RedeemCollateralAndBurnTokens>,
    amount_collateral: u64,
    amount_to_burn: u64,
) -> Result<()> {
    let collateral_account = &mut ctx.accounts.collateral_account;
    collateral_account.lamport_balance = ctx.accounts.sol_account.lamports() - amount_collateral;
    calculate_health_factor(
        &ctx.accounts.collateral_account,
        &ctx.accounts.config_account,
        &ctx.accounts.price_update,
    )?;
    burn_tokens(
        &ctx.accounts.mint_account,
        &ctx.accounts.token_program,
        amount_to_burn,
        &ctx.accounts.token_account,
        &ctx.accounts.depositor,
    )?;
    withdraw_sol(
        ctx.accounts.collateral_account.bump_sol_account,
        ctx.accounts.depositor.key(),
        &ctx.accounts.system_program,
        amount_collateral,
        &ctx.accounts.sol_account,
        &ctx.accounts.depositor.to_account_info(),
    )?;
    Ok(())
}
