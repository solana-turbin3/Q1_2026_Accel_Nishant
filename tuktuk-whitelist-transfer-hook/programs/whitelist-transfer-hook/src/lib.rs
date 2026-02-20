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
    
      pub fn expire_user(ctx: Context<ExpireUser>, user: Pubkey) -> Result<()> {
        ctx.accounts.expire_user(user)
    }

    pub fn schedule_expiry(ctx: Context<ScheduleExpiry>, user: Pubkey, expiry_timestamp: i64) -> Result<()> {//it will call by tuktup cpi
        ctx.accounts.schedule_expiry(user, expiry_timestamp)
    }

}
