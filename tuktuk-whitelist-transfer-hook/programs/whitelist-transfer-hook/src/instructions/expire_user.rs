use anchor_lang::prelude::*;

use crate::{constant::{INIT_CONFIG_SEED, WHITELISTED_USER_SEED}, error::ErrorCode, state::{Config,WhitelistedUser}};
// #[derive(Accounts)]
// #[instruction(user: Pubkey)]
// pub struct RemoveFromWhitelist<'info> {
//     #[account(mut)]
//     pub admin: Signer<'info>,
//     #[account(
//         seeds = [INIT_CONFIG_SEED ,admin.key().as_ref()],
//         bump = config.bump,
//         constraint = config.admin == admin.key() @ ErrorCode::Unauthorized,
//     )]
//     pub config: Account<'info, Config>,
//     #[account(
//         mut,
//         close = admin,
//         seeds = [WHITELISTED_USER_SEED, user.as_ref()],
//         bump
//     )]
//     pub whitelisted_user: Account<'info, WhitelistedUser>,
//     pub system_program: Program<'info, System>,
// }


#[derive(Accounts)]
#[instruction(user: Pubkey)]
pub struct ExpireUser<'info> {
    #[account(mut)]
    pub admin: AccountInfo<'info>,
    #[account(
        seeds = [INIT_CONFIG_SEED ,admin.key().as_ref()],
        bump = config.bump,
        constraint = config.admin == admin.key() @ ErrorCode::Unauthorized,
    )]
    pub config: Account<'info, Config>,
     #[account(
        mut,
        seeds = [WHITELISTED_USER_SEED, user.as_ref()],
        bump
    )]
    pub whitelisted_user: Account<'info, WhitelistedUser>,
}

impl<'info> ExpireUser<'info> {
    pub fn expire_user(&mut self, user: Pubkey) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;

        require!(
            now >= self.whitelisted_user.expiry_timestamp,
            ErrorCode::NotExpired
        );
        // self.whitelisted_user.set_inner(WhitelistedUser {
        //     user: self.whitelisted_user.user,
        //     is_active: false,
        //     expiry_timestamp: self.whitelisted_user.expiry_timestamp,
        //     bump: self.whitelisted_user.bump,
        // });

        self.whitelisted_user.is_active = false;
        msg!("Expired user. User: {} ", self.whitelisted_user.user);
        Ok(())
    }
}
