use anchor_lang::prelude::*;

use crate::{constant::INIT_CONFIG_SEED, state::Config};

#[derive(Accounts)]
pub struct InitConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = Config::LEN,
        seeds = [INIT_CONFIG_SEED,admin.key().as_ref()],
        bump,
    )]
    pub config: Account<'info, Config>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitConfig<'info> {
    pub fn init_config(&mut self, bump: InitConfigBumps) -> Result<()> {
        self.config.set_inner(Config {
            admin: self.admin.key(),
            bump: bump.config,
        });

        msg!("Initialized config. Admin: {} ", self.admin.key());
        Ok(())
    }
}
