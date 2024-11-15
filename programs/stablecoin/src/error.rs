use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Invalid price")]
    InvaildPrice,
    #[msg("You are below the min health factor")]
    BelowMinHealthFactor,
    #[msg("cannot liquidate, Health factor is above liquidation threshold")]
    AboveLiquidationThreshold,
}
