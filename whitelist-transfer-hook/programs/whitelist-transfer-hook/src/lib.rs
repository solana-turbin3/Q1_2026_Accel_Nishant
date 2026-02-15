#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;

mod constant;
mod error;
mod instructions;
mod state;
use instructions::*;

use spl_discriminator::SplDiscriminate;
use spl_tlv_account_resolution::state::ExtraAccountMetaList;
use spl_transfer_hook_interface::instruction::{
    ExecuteInstruction, InitializeExtraAccountMetaListInstruction,
};

declare_id!("EfvcbUrqid3P54BhoFLrJhAdJxe2vxKhGG9sDRvCsWHh");

#[program]
pub mod whitelist_transfer_hook {
    use super::*;

    pub fn init_config(ctx: Context<InitConfig>) -> Result<()> {
        ctx.accounts.init_config(ctx.bumps)
    }
    pub fn add_to_whitelist(ctx: Context<AddToWhitelist>, user: Pubkey) -> Result<()> {
        ctx.accounts.add_to_whitelist(ctx.bumps, user)
    }

    pub fn remove_from_whitelist(ctx: Context<RemoveFromWhitelist>, user: Pubkey) -> Result<()> {
        ctx.accounts.remove_from_whitelist(user)
    }

    pub fn init_mint(ctx: Context<TokenFactory>, decimals: u8) -> Result<()> {
        ctx.accounts.init_mint(ctx.bumps, decimals)
    }

    #[instruction(discriminator = ExecuteInstruction::SPL_DISCRIMINATOR_SLICE)]
    pub fn transfer_token(ctx: Context<TransferHook>, amount: u64) -> Result<()> {
        ctx.accounts.transfer_hook(amount)
    }

    pub fn initialize_transfer_hook(ctx: Context<InitializeExtraAccountMetaList>) -> Result<()> {
        ctx.accounts.initialize_extra_account_meta_list(&ctx.bumps)
    }
    /*

    pub fn initialize_transfer_hook(ctx: Context<InitializeExtraAccountMetaList>) -> Result<()> {
        msg!("Initializing Transfer Hook...");

        // // Get the extra account metas for the transfer hook
        // let extra_account_metas = InitializeExtraAccountMetaList::extra_account_metas()?;

        // msg!("Extra Account Metas: {:?}", extra_account_metas);
        // msg!("Extra Account Metas Length: {}", extra_account_metas.len());

        // // initialize ExtraAccountMetaList account with extra accounts
        // ExtraAccountMetaList::init::<ExecuteInstruction>(
        //     &mut ctx.accounts.extra_account_meta_list.try_borrow_mut_data()?,
        //     &extra_account_metas,
        // )
        // .unwrap();

        Ok(())
    }

    #[instruction(discriminator = ExecuteInstruction::SPL_DISCRIMINATOR_SLICE)]
    pub fn transfer_hook(ctx: Context<TransferHook>, amount: u64) -> Result<()> {
        msg!("=== TRANSFER HOOK START ===");
        // Call the transfer hook logic
        ctx.accounts.transfer_hook(amount)
    }
     */
}
