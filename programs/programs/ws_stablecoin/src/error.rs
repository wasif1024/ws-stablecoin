use anchor_lang::prelude::*;

#[error_code]
pub enum CustomErrors {
    #[msg("Price is not available")]
    PriceNotAvailable,
    #[msg("Price is not valid")]
    PriceNotValid,
    #[msg("Health factor is too low")]
    HealthFactorTooLow,
    #[msg("Health factor is above minimum health factor.Cannot Liquidate")]
    AboveMinimumHealthFactor,
}
