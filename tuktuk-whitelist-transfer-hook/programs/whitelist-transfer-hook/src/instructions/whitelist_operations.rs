use crate::{
    constant::{INIT_CONFIG_SEED, WHITELISTED_USER_SEED},
    error::ErrorCode,
    state::{whitelist::WhitelistedUser, Config},
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(user: Pubkey)]
pub struct AddToWhitelist<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        seeds = [INIT_CONFIG_SEED ,admin.key().as_ref()],
        bump = config.bump,
        constraint = config.admin == admin.key() @ ErrorCode::Unauthorized,
    )]
    pub config: Account<'info, Config>,
    #[account(
        init,
        payer = admin,
        space = WhitelistedUser::LEN,
        seeds = [WHITELISTED_USER_SEED, user.as_ref()],
        bump
    )]
    pub whitelisted_user: Account<'info, WhitelistedUser>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(user: Pubkey)]
pub struct RemoveFromWhitelist<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        seeds = [INIT_CONFIG_SEED ,admin.key().as_ref()],
        bump = config.bump,
        constraint = config.admin == admin.key() @ ErrorCode::Unauthorized,
    )]
    pub config: Account<'info, Config>,
    #[account(
        mut,
        close = admin,
        seeds = [WHITELISTED_USER_SEED, user.as_ref()],
        bump
    )]
    pub whitelisted_user: Account<'info, WhitelistedUser>,
    pub system_program: Program<'info, System>,
}

impl<'info> AddToWhitelist<'info> {
    pub fn add_to_whitelist(&mut self, bump: AddToWhitelistBumps, user: Pubkey) -> Result<()> {
        self.whitelisted_user.set_inner(WhitelistedUser {
            user: user,
            bump: bump.whitelisted_user,
            is_active: true,
            expiry_timestamp: Clock::get().unwrap().unix_timestamp + 60, // 1 minute
        });

        msg!("Added to whitelist. User: {}", user.key());
        Ok(())
    }
}

impl<'info> RemoveFromWhitelist<'info> {
    pub fn remove_from_whitelist(&mut self, user: Pubkey) -> Result<()> {
        msg!("Remove from whitelist. User: {}", user.key());
        Ok(())
    }
}

/*
impl<'info> WhitelistOperations<'info> {
    pub fn add_to_whitelist(&mut self, bump: WhitelistOperationsBumps) -> Result<()> {
        // if !self.whitelist.address.contains(&address) {
        //     self.realloc_whitelist(true)?;
        //     self.whitelist.address.push(address);
        // }

        self.whitelist.set_inner(WhitelistedUser {
            bump: bump.whitelist,
        });
        Ok(())
    }

    // pub fn remove_from_whitelist(&mut self, _address: Pubkey) -> Result<()> {
    //     // if let Some(pos) = self.whitelist.address.iter().position(|&x| x == address) {
    //     //     self.whitelist.address.remove(pos);
    //     //     self.realloc_whitelist(false)?;
    //     // }

    //     Ok(())
    // }


    pub fn realloc_whitelist(&self, is_adding: bool) -> Result<()> {
        // Get the account info for the whitelist
        let account_info = self.whitelist.to_account_info();

        if is_adding {
            // Adding to whitelist
            let new_account_size = account_info.data_len() + std::mem::size_of::<Pubkey>();
            // Calculate rent required for the new account size
            let lamports_required = (Rent::get()?).minimum_balance(new_account_size);
            // Determine additional rent required
            let rent_diff = lamports_required - account_info.lamports();

            // Perform transfer of additional rent
            let cpi_program = self.system_program.to_account_info();
            let cpi_accounts = system_program::Transfer {
                from: self.admin.to_account_info(),
                to: account_info.clone(),
            };
            let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
            system_program::transfer(cpi_context, rent_diff)?;

            // Reallocate the account
            account_info.resize(new_account_size)?;
            msg!("Account Size Updated: {}", account_info.data_len());
        } else {
            // Removing from whitelist
            let new_account_size = account_info.data_len() - std::mem::size_of::<Pubkey>();
            // Calculate rent required for the new account size
            let lamports_required = (Rent::get()?).minimum_balance(new_account_size);
            // Determine additional rent to be refunded
            let rent_diff = account_info.lamports() - lamports_required;

            // Reallocate the account
            account_info.resize(new_account_size)?;
            msg!("Account Size Downgraded: {}", account_info.data_len());

            // Perform transfer to refund additional rent
            **self.admin.to_account_info().try_borrow_mut_lamports()? += rent_diff;
            **self.whitelist.to_account_info().try_borrow_mut_lamports()? -= rent_diff;
        }

        Ok(())
    }


}
 */
