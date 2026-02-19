use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    pub admin: Pubkey,
    pub mint: Pubkey,
    pub token_account: Pubkey,
    pub bump: u8,
}

impl Vault{
  pub const INIT_SPACE:usize = 32 + 32 + 32 + 1;
}
