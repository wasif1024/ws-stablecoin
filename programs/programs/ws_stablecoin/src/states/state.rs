use anchor_lang::prelude::*;
#[account]
#[derive(InitSpace, Debug)]
pub struct Collatoral {
    pub depositor: Pubkey,
    pub sol_account: Pubkey, //PDA of collatoral account that allows to deposit the sol
    pub token_account: Pubkey, //Token account for stablecoin
    pub lamport_balance: u64,
    pub amount_minted: u64,
    pub bump_collateral_account: u8, //this is basically the bump of pda for collateral account
    pub bump_sol_account: u8,        //this is basically the bump of pda for token account
    pub is_initialized: bool,
}

#[account]
#[derive(InitSpace, Debug)]
pub struct Configuration {
    pub authority: Pubkey,
    pub mint_account: Pubkey, //This is mint of stable mint coin account
    pub liquidation_threshold: u64,
    pub liquidation_bonus: u64,
    pub min_health_factor: u64,
    pub bump_config_account: u8,
    pub bump_mint_account: u8,
}
