use anchor_lang::prelude::*;

#[account]
pub struct Agent {
    pub context: Pubkey,
    pub bump: u8,
}

impl Agent {
    pub const LEN: usize = 8 + 32 + 1;
}
