#![allow(unexpected_cfgs)]
#![allow(deprecated)]
use anchor_lang::prelude::*;
use ephemeral_rollups_sdk::anchor::ephemeral;
use ephemeral_vrf_sdk::anchor::vrf;
use ephemeral_vrf_sdk::instructions::{create_request_randomness_ix, RequestRandomnessParams};
use ephemeral_vrf_sdk::types::SerializableAccountMeta;

mod instructions;
mod state;

use instructions::*;

use crate::state::UserAccount;

declare_id!("EQkMxVqHWsEPHD44yAicQZ55Av8AGbfLdgsuZPJmUBqm");

#[ephemeral]
#[program]
pub mod er_state_account {

    use crate::state::user_account;

    use super::*;

    pub fn initialize(ctx: Context<InitUser>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)?;

        Ok(())
    }

    pub fn update(ctx: Context<UpdateUser>, new_data: u64) -> Result<()> {
        ctx.accounts.update(new_data)?;

        Ok(())
    }

    pub fn update_commit(ctx: Context<UpdateCommit>, new_data: u64) -> Result<()> {
        ctx.accounts.update_commit(new_data)?;

        Ok(())
    }

    pub fn delegate(ctx: Context<Delegate>) -> Result<()> {
        ctx.accounts.delegate()?;

        Ok(())
    }

    pub fn undelegate(ctx: Context<Undelegate>) -> Result<()> {
        ctx.accounts.undelegate()?;

        Ok(())
    }

    pub fn close(ctx: Context<CloseUser>) -> Result<()> {
        ctx.accounts.close()?;

        Ok(())
    }

    /*-----For Task 1 : update state outside ER ( ephemeral rollup ) -----*/
    // Request Randomness
    pub fn randomize_user_account(
        ctx: Context<RandomizeUserAccountCtx>,
        client_seed: u8,
    ) -> Result<()> {
        msg!("Requesting randomness...");
        let ix = create_request_randomness_ix(RequestRandomnessParams {
            payer: ctx.accounts.payer.key(),
            oracle_queue: ctx.accounts.oracle_queue.key(),
            callback_program_id: ID,
            callback_discriminator: instruction::CallbackUserAccount::DISCRIMINATOR.to_vec(),
            caller_seed: [client_seed; 32],
            // Specify any account that is required by the callback
            accounts_metas: Some(vec![SerializableAccountMeta {
                pubkey: ctx.accounts.user_account.key(),
                is_signer: false,
                is_writable: true,
            }]),
            ..Default::default()
        });
        ctx.accounts
            .invoke_signed_vrf(&ctx.accounts.payer.to_account_info(), &ix)?;
        Ok(())
    }

    // Consume Randomness
    pub fn callback_user_account(
        ctx: Context<CallbackUserAccountCtx>,
        randomness: [u8; 32],
    ) -> Result<()> {
        // random_u8_with_range(&randomness, 1, 6)
        let rnd_u64 = ephemeral_vrf_sdk::rnd::random_u64(&randomness);
        msg!("Consuming random number: {:?}", rnd_u64);
        let user_account = &mut ctx.accounts.user_account;
        // player.last_result = rnd_u8; // Update the player's last result

        user_account.data = rnd_u64;
        Ok(())
    }
}

#[vrf]
#[derive(Accounts)]
pub struct RandomizeUserAccountCtx<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(seeds = [b"user", user_account.user.key().as_ref()], bump)]
    pub user_account: Account<'info, UserAccount>,
    /// CHECK: The oracle queue
    #[account(mut, address = ephemeral_vrf_sdk::consts::DEFAULT_QUEUE)]
    pub oracle_queue: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CallbackUserAccountCtx<'info> {
    /// This check ensure that the vrf_program_identity (which is a PDA) is a singer
    /// enforcing the callback is executed by the VRF program trough CPI
    #[account(address = ephemeral_vrf_sdk::consts::VRF_PROGRAM_IDENTITY)]
    pub vrf_program_identity: Signer<'info>,
    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,
}

/*
For task 1 step : update state inside ER ( ephemeral rollup )

1. Init user account
2. call randomize_user_account instruction -> It call update the state
3. We can fetch the state and check the state is updated or not but it will with radom number as expected.




*/
