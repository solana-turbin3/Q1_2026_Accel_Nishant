use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::state::Vault;

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
      init,
      payer=admin,
      space= 8 + Vault::INIT_SPACE,
      seeds = [b"vault"],
      bump,
    )]
    pub vault: Account<'info, Vault>,
    #[account(
      seeds = [b"vault_authority"],
      bump,
    )]
    pub vault_authority: UncheckedAccount<'info>,
    #[account(
        init,
        payer = admin,
        seeds = [b"vault-token"],
        bump,
        token::mint = mint,
        token::authority = vault_authority,
        token::token_program = token_program
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> InitializeVault<'info> {
    pub fn initialize_vault(&mut self, bumps: &InitializeVaultBumps) -> Result<()> {
        self.vault.set_inner(Vault {
            admin: self.admin.key(),
            mint: self.mint.key(),
            token_account: self.token_account.key(),
            bump: bumps.vault,
        });

        Ok(())
    }
}
