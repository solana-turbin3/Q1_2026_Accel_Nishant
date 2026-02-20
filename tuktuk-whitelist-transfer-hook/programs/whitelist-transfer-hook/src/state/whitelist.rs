use anchor_lang::prelude::*;

#[account]
pub struct WhitelistedUser {
    pub user: Pubkey,
    pub is_active: bool,
    pub expiry_timestamp: i64,
    pub bump: u8,
}

impl WhitelistedUser {
    pub const LEN: usize = 8 + 32 + 1 + 8 + 1;
}
