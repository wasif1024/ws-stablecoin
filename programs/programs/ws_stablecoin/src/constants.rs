use anchor_lang::prelude::*;

pub const CONFIGURATION_SEED: &[u8] = b"config";
pub const COLLATERAL_SEED: &[u8] = b"collateral";
pub const MINT_SEED: &[u8] = b"mint";
pub const SOL_SEED: &[u8] = b"sol";
pub const TOKEN_ACCOUNT_SEED: &[u8] = b"token";
#[constant]
pub const FEED_ID: &str = "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d";
pub const DECIMALS: u8 = 9;
pub const PRICE_FEED_DECIMALS_ADJUSTMENT: u128 = 10;
pub const LIQUIDATION_THRESHOLD: u64 = 50; //200% overcollateralized
pub const LIQUIDATION_BONUS: u64 = 10; //10% bonus for liquidators
pub const MIN_HEALTH_FACTOR: u64 = 1; //
pub const MAXIMUM_AGE: u64 = 100; //10 minutes
