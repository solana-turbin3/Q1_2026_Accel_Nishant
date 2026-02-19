use anchor_lang::prelude::*;

use crate::state::WhitelistedUser;

#[derive(Accounts)]
pub struct RemoveFromWhitelist<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [b"whitelist", admin.key().as_ref()],
        bump=whitelist.bump,
        close = admin
    )]
    pub whitelist: Account<'info, WhitelistedUser>,
    pub system_program: Program<'info, System>,
}

// impl<'info> RemoveFromWhitelist<'info> {
//     pub fn remove_from_whitelist(&mut self, _address: Pubkey) -> Result<()> {
//         **self.admin.lamports.borrow_mut() += self.whitelist.to_account_info().lamports();
//         **self.whitelist.to_account_info().lamports.borrow_mut() = 0;

//         Ok(())
//     }
// }
