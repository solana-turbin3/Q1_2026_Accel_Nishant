use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct WhitelistEntry {
    pub user: Pubkey,
    pub amount: u64,
}

#[account]
pub struct Whitelist {
    pub address: Vec<WhitelistEntry>,
    pub bump: u8,
}

impl WhitelistEntry {
    pub const SIZE: usize = 8 + 32;
}
