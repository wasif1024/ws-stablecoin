use anchor_lang::prelude::*;
pub mod constants;
pub mod instructions;
pub mod states;
use instructions::*;
mod error;
declare_id!("8pxt53YeQbcVLbkEqU8gS2MjKdKkZy1FgAuU7CRzxbgR");

#[program]
pub mod ws_stablecoin {
    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
        //msg!("Greetings from: {:?}", ctx.program_id);
        admin::initialize_config::process_initialize_config(ctx)
        //Ok(())
    }
    pub fn update_config(ctx: Context<UpdateConfig>, min_health_factor: u64) -> Result<()> {
        admin::update_config::process_update_config(ctx, min_health_factor)
    }
    pub fn deposit_collateral_and_mint_token(
        ctx: Context<DepositCollateralAndMintToken>,
        amount_collateral: u64,
        amount_to_mint: u64,
    ) -> Result<()> {
        deposit::deposit_collateral_and_mint_token::process_deposit_collateral_and_mint_token(
            ctx,
            amount_collateral,
            amount_to_mint,
        )
    }
    pub fn redeem_collateral_and_burn_tokens(
        ctx: Context<RedeemCollateralAndBurnTokens>,
        amount_collateral: u64,
        amount_to_burn: u64,
    ) -> Result<()> {
        withdraw::redeem_collatoral_and_burn_tokens::process_redeem_collatoral_and_burn_tokens(
            ctx,
            amount_collateral,
            amount_to_burn,
        )
    }
    pub fn liquidate(ctx: Context<Liquidate>, amount_to_burn: u64) -> Result<()> {
        withdraw::liquidate::process_liquidate(ctx, amount_to_burn)
    }
}
