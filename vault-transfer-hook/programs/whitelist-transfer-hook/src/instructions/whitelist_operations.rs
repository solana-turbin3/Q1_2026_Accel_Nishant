use anchor_lang::{prelude::*, system_program};

use crate::state::{
    whitelist::{Whitelist, WhitelistEntry},
    TransferHookError,
};

#[derive(Accounts)]
pub struct WhitelistOperations<'info> {
    #[account(
        mut,
        //address = 
    )]
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [b"whitelist"],
        bump,
    )]
    pub whitelist: Account<'info, Whitelist>,
    pub system_program: Program<'info, System>,
}

impl<'info> WhitelistOperations<'info> {
    pub fn add_to_whitelist(&mut self, user: Pubkey) -> Result<()> {
        if !self
            .whitelist
            .address
            .iter()
            .any(|entry| entry.user == user)
        {
            self.realloc_whitelist(true)?;
            self.whitelist.address.push(WhitelistEntry { user, amount: 0 });
        }
        Ok(())
    }

    pub fn remove_from_whitelist(&mut self, user: Pubkey) -> Result<()> {
        if let Some(pos) = self
            .whitelist
            .address
            .iter()
            .position(|entry| entry.user == user)
        {
            self.whitelist.address.remove(pos);
            self.realloc_whitelist(false)?;
        }
        Ok(())
    }

    pub fn realloc_whitelist(&self, is_adding: bool) -> Result<()> {
        let account_info = self.whitelist.to_account_info();

        if is_adding {
            let new_account_size = account_info.data_len() + WhitelistEntry::SIZE;
            let lamports_required = (Rent::get()?).minimum_balance(new_account_size);
            let rent_diff = lamports_required.saturating_sub(account_info.lamports());

            if rent_diff > 0 {
                let cpi_program = self.system_program.to_account_info();
                let cpi_accounts = system_program::Transfer {
                    from: self.admin.to_account_info(),
                    to: account_info.clone(),
                };
                let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
                system_program::transfer(cpi_context, rent_diff)?;
            }

            account_info.resize(new_account_size)?;
            msg!("Account Size Updated: {}", account_info.data_len());
        } else {
            let new_account_size = account_info
                .data_len()
                .checked_sub(WhitelistEntry::SIZE)
                .ok_or(error!(TransferHookError::MathOverflow))?;
            let lamports_required = (Rent::get()?).minimum_balance(new_account_size);
            let rent_diff = account_info.lamports().saturating_sub(lamports_required);

            account_info.resize(new_account_size)?;
            msg!("Account Size Downgraded: {}", account_info.data_len());

            if rent_diff > 0 {
                **self.admin.to_account_info().try_borrow_mut_lamports()? += rent_diff;
                **self.whitelist.to_account_info().try_borrow_mut_lamports()? -= rent_diff;
            }
        }

        Ok(())
    }
}
