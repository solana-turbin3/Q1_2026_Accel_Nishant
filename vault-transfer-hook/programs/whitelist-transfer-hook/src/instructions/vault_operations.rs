use anchor_lang::{prelude::*, system_program::Transfer};
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

use crate::state::{TransferHookError, Vault, VaultError, Whitelist};

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(mut,token::mint=mint,token::authority=user,token::token_program=token_program)]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
      seeds = [b"vault_authority"],
      bump,
    )]
    pub vault_authority: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [b"vault"],
        bump = vault.bump,
        constraint = vault.mint == mint.key() @ VaultError::InvalidVaultMint
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        mut,
        address = vault.token_account,
        token::mint = mint,
        token::authority = vault_authority,
        token::token_program = token_program
    )]
    pub vault_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"whitelist"],
        bump = whitelist.bump
    )]
    pub whitelist: Account<'info, Whitelist>,

    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> Deposit<'info> {
    pub fn deposit(&mut self, amount: u64, bumps: &DepositBumps) -> Result<()> {
        require!(amount > 0, VaultError::InsufficientBalance);

        let entry_index = self
            .whitelist
            .address
            .iter()
            .position(|entry| entry.user == self.user.key())
            .ok_or(error!(TransferHookError::UserNotWhitelisted))?;

        let cpi_accounts = TransferChecked {
            from: self.user_token_account.to_account_info(),
            to: self.vault_token_account.to_account_info(),
            mint: self.mint.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);

        transfer_checked(cpi_ctx, amount, self.mint.decimals)?;

         let new_amount = self.whitelist.address[entry_index]
            .amount
            .checked_add(amount)
            .ok_or(error!(TransferHookError::MathOverflow))?;
        self.whitelist.address[entry_index].amount = new_amount;

        Ok(())

    }
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = user,
        token::token_program = token_program
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds = [b"vault_authority"],
        bump
    )]
    pub vault_authority: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"vault"],
        bump = vault.bump,
        constraint = vault.mint == mint.key() @ VaultError::InvalidVaultMint
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        mut,
        address = vault.token_account,
        token::mint = mint,
        token::authority = vault_authority,
        token::token_program = token_program
    )]
    pub vault_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"whitelist"],
        bump = whitelist.bump
    )]
    pub whitelist: Account<'info, Whitelist>,

    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self, amount: u64, bumps: &WithdrawBumps) -> Result<()> {
        require!(amount > 0, VaultError::InsufficientBalance);

        let entry_index = self
            .whitelist
            .address
            .iter()
            .position(|entry| entry.user == self.user.key())
            .ok_or(error!(TransferHookError::UserNotWhitelisted))?;

        let current_amount = self.whitelist.address[entry_index].amount;
        require!(
            current_amount >= amount,
            TransferHookError::InsufficientWhitelistedBalance
        );

        let signer_seeds: &[&[u8]] = &[b"vault_authority", &[bumps.vault_authority]];
        let signer: &[&[&[u8]]] = &[signer_seeds];

        let cpi_accounts = TransferChecked {
            from: self.vault_token_account.to_account_info(),
            to: self.user_token_account.to_account_info(),
            mint: self.mint.to_account_info(),
            authority: self.vault_authority.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);

        transfer_checked(cpi_ctx, amount, self.mint.decimals)?;

        self.whitelist.address[entry_index].amount = current_amount
            .checked_sub(amount)
            .ok_or(error!(TransferHookError::MathOverflow))?;

        Ok(())
    }
}
