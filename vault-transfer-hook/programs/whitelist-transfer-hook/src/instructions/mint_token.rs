use anchor_lang::prelude::*;
use anchor_spl::token_interface::{mint_to, Mint, MintTo, TokenAccount, TokenInterface};

use crate::state::{MintError, Vault, VaultError, Whitelist};

#[derive(Accounts)]
pub struct MintToken<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        token::mint = mint,
        token::authority = admin,
        token::token_program = token_program
    )]
    pub destination_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        seeds = [b"vault_authority"],
        bump
    )]
    pub vault_authority: UncheckedAccount<'info>,
    #[account(
        seeds = [b"vault"],
        bump = vault.bump,
        constraint = vault.mint == mint.key() @ VaultError::InvalidVaultMint
    )]
    pub vault: Account<'info, Vault>,

    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> MintToken<'info> {
    pub fn mint(&mut self, amount: u64, bumps: &MintTokenBumps) -> Result<()> {
        require!(amount > 0, MintError::InvalidAmount);
        require_keys_eq!(
            self.vault.admin,
            self.admin.key(),
            MintError::UnauthorizedAdmin
        );

        let cpi_accounts = MintTo {
            mint: self.mint.to_account_info(),
            to: self.destination_token_account.to_account_info(),
            authority: self.vault_authority.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);

        mint_to(cpi_ctx, amount)?;
        Ok(())
    }
}
