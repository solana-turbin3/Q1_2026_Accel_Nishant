use anchor_lang::prelude::*;

#[error_code]
pub enum AppError {
    #[msg("Too early to claim tokens")]
    TooEarlyToClaim,
}
