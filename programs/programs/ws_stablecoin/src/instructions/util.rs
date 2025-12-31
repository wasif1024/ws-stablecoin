use anchor_lang::prelude::*;
use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;
use anchor_spl::token_interface::{Mint, Token2022, TokenAccount, TokenInterface};
use crate::states::{Collatoral, Configuration};
use crate::constants::{FEED_ID, MAXIMUM_AGE, PRICE_FEED_DECIMALS_ADJUSTMENT};
use crate::error::CustomErrors;
use pyth_solana_receiver_sdk::price_update::{PriceUpdateV2,get_feed_id_from_hex,};
pub fn calculate_health_factor(collatoral:&Account<Collatoral>,config:&Account<Configuration>,price_feed:&Account<PriceUpdateV2>)->Result<u64>{
    let collateral_value_in_usd=get_usd_value(&collatoral.lamport_balance,price_feed)?;
    let collatoral_adjusted_for_liquidation_threshold=(collateral_value_in_usd * config.liquidation_threshold) / 100;
    if collatoral.amount_minted==0{
        msg!("Health Factor Max");
        return Ok(u64::MAX);
    }
    let health_factor=collatoral_adjusted_for_liquidation_threshold / collatoral.amount_minted;
    Ok(health_factor)
}
pub fn get_usd_value(lamport_balance:&u64,price_feed:&Account<PriceUpdateV2>)->Result<u64>{
    let feed_id=get_feed_id_from_hex(FEED_ID)?;
    let price=price_feed.get_price_no_older_than(&Clock::get()?,MAXIMUM_AGE,&feed_id)?;
    require!(price.price>0,CustomErrors::PriceNotValid);
    let price_in_usd=price.price as u128 * PRICE_FEED_DECIMALS_ADJUSTMENT;
    let amount_in_usd=(*lamport_balance as u128 * price_in_usd) / (LAMPORTS_PER_SOL as u128);
    Ok(amount_in_usd as u64)
}