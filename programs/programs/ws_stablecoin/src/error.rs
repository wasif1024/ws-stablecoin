use anchor_lang::prelude::*;

#[error_code]
pub enum CustomErrors {
    #[msg("Price is not available")]
    PriceNotAvailable,
    #[msg("Price is not valid")]
    PriceNotValid,
}