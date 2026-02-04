use anchor_lang::prelude::*;

#[account]
// pub struct Whitelist {
//     pub address: Vec<Pubkey>,
//     pub bump: u8,
// }

pub struct WhitelistedUser {
    pub bump: u8,
}
