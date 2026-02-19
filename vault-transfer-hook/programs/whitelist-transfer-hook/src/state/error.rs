use anchor_lang::prelude::*;

#[error_code]
pub enum TransferHookError {
    #[msg("Transfer hook was invoked outside of a transfer")]
    NotTransferring,
    #[msg("Transfer must involve the vault token account")]
    TransferMustTouchVault,
    #[msg("Source owner is not whitelisted for vault deposit")]
    SourceNotWhitelisted,
    #[msg("Destination owner is not whitelisted for vault withdrawal")]
    DestinationNotWhitelisted,
    #[msg("Mint does not match the vault mint")]
    InvalidVaultMint,
    #[msg("User is not whitelisted")]
    UserNotWhitelisted,
    #[msg("Insufficient balance recorded for this whitelisted user")]
    InsufficientWhitelistedBalance,
    #[msg("Amount must be greater than zero")]
    InvalidAmount,
    #[msg("Math overflow")]
    MathOverflow,
}
#[error_code]
pub enum VaultError {
    #[msg("Mint does not match the vault mint")]
    InvalidVaultMint,
    #[msg("Insufficient balance in the vault")]
    InsufficientBalance,
}

#[error_code]
pub enum MintError {
    #[msg("Only vault admin can mint")]
    UnauthorizedAdmin,
    #[msg("Amount must be greater than zero")]
    InvalidAmount,
}
