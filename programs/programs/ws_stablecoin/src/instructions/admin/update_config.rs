use crate::constants::CONFIGURATION_SEED;
use crate::states::Configuration;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(mut,seeds=[CONFIGURATION_SEED],bump=config_account.bump_config_account)]
    pub config_account: Account<'info, Configuration>,
}
pub fn process_update_config(ctx: Context<UpdateConfig>, min_health_factor: u64) -> Result<()> {
    ctx.accounts.config_account.min_health_factor = min_health_factor;
    Ok(())
}
