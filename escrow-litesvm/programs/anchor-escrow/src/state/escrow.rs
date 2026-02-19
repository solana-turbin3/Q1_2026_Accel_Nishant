use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct Escrow {
    pub seed: u64,
    pub maker: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub receive: u64,
    pub created_at: i64, //unix timestamp types i64. because of it can be negative if we go back after 1970
    pub bump: u8,
}