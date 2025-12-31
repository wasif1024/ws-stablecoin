use crate::constants::{
    CONFIGURATION_SEED, DECIMALS, LIQUIDATION_BONUS, LIQUIDATION_THRESHOLD, MINT_SEED,
    MIN_HEALTH_FACTOR,
};
use crate::states::Configuration;
use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token_interface::{Mint, Token2022, TokenInterface};
#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(init, payer = authority, space = 8 + Configuration::INIT_SPACE,
        seeds = [CONFIGURATION_SEED], bump)]
    pub config_account: Account<'info, Configuration>,
    #[account(init,payer=authority,seeds=[MINT_SEED],bump,mint::decimals=DECIMALS,
        mint::authority=mint_account,mint::freeze_authority=mint_account,mint::token_program=token_program)]
    pub mint_account: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
}
pub fn process_initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
    *ctx.accounts.config_account = Configuration {
        authority: ctx.accounts.authority.key(),
        mint_account: ctx.accounts.mint_account.key(),
        liquidation_threshold: LIQUIDATION_THRESHOLD,
        liquidation_bonus: LIQUIDATION_BONUS,
        min_health_factor: MIN_HEALTH_FACTOR,
        bump_config_account: ctx.bumps.config_account,
        bump_mint_account: ctx.bumps.mint_account,
    };
    Ok(())
}
