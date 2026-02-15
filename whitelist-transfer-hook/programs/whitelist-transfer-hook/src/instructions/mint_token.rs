

use anchor_lang::{
    prelude::*,
    solana_program::program::invoke,
    system_program::{create_account, CreateAccount},
};
use anchor_spl::token_2022::{
    spl_token_2022::{
        extension::{transfer_hook::instruction::initialize as init_transfer_hook, ExtensionType},
        instruction::initialize_mint2,
        state::Mint as Token2022Mint,
    },
    Token2022,
};

use crate::{constant::{INIT_CONFIG_SEED, MINT_TOKEN_SEED}, state::Config};

#[derive(Accounts)]
pub struct TokenFactory<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        seeds = [INIT_CONFIG_SEED ,config.admin.key().as_ref()],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,

    /// CHECK: We will create and initialize this mint account manually
    #[account(
        mut, 
        seeds=[MINT_TOKEN_SEED, config.key().as_ref()],
        bump
    )]
    pub mint: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
}

impl<'info> TokenFactory<'info> {
    pub fn init_mint(&mut self,bump:TokenFactoryBumps, decimals: u8) -> Result<()> {
        // Calculate the space needed for mint with TransferHook extension
        let extension_types = vec![ExtensionType::TransferHook];
        let space = ExtensionType::try_calculate_account_len::<Token2022Mint>(&extension_types)
            .map_err(|_| error!(crate::error::ErrorCode::ExtensionInitializationFailed))?;

        msg!("Mint account space needed: {} bytes", space);

        // Calculate rent
        let lamports = Rent::get()?.minimum_balance(space);


        let config_binding = self.config.clone().key();
        let signer_seeds: &[&[u8]] = &[
            MINT_TOKEN_SEED,
            config_binding.as_ref(),
            &[bump.mint],
        ];

        create_account(
            CpiContext::new_with_signer(
                self.system_program.to_account_info(),
                CreateAccount {
                    from: self.payer.to_account_info(),
                    to: self.mint.to_account_info(),
                },
                &[signer_seeds],
            ),
            lamports,
            space as u64,
            &self.token_program.key(),
        )?;

        msg!("Mint account created");
        

        // Initialize the TransferHook extension via CPI
        let init_hook_ix =
        init_transfer_hook(
                &self.token_program.key(),
                &self.mint.key(),
                Some(self.payer.key()),
                Some(crate::ID),
            )?;

        invoke(
            &init_hook_ix,
            &[self.mint.to_account_info()],
        )?;

        msg!("Transfer hook extension initialized");

        // Initialize the base mint via CPI
        let init_mint_ix = initialize_mint2(
            &self.token_program.key(),
            &self.mint.key(),
            &self.payer.key(),
            Some(&self.payer.key()),
            decimals,
        )?;

        invoke(
            &init_mint_ix,
            &[self.mint.to_account_info()],
        )?;

        msg!("Mint initialized successfully");
        msg!("Mint address: {}", self.mint.key());
        msg!("Transfer hook program: {}", crate::ID);
        msg!("Transfer hook authority: {}", self.payer.key());

        Ok(())
    }
}



/*

/// Use when, We need raw solana type .
 pub mint: AccountInfo<'info>, 

/// Anchor has wrapeer on AccountInfo, but at runtime are both same
 pub owner: UncheckedAccount<'info>


**UncheckedAccount**
This tells Anchor:
 This is an account in instruction
✔ Validate PDA using seeds
✔ Do NOT deserialize
✔ Do NOT expect discriminator
*/