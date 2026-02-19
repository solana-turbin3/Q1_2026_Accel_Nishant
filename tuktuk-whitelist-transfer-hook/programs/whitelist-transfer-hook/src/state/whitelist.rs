use anchor_lang::prelude::*;

#[account]
pub struct WhitelistedUser {
    pub user: Pubkey,
    pub bump: u8,
}

impl WhitelistedUser {
    pub const LEN: usize = 8 + 32 + 1;
}
