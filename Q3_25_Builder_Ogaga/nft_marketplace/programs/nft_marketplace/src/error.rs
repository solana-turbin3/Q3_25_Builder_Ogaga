use anchor_lang::prelude::*;

#[error_code]
pub enum MarketplaceError {
    #[msg("Numerical overflow")]
    NumericalOverflow,
}
