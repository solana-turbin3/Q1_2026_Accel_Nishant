use anchor_lang::prelude::*;
use tuktuk_program::{
    TransactionSourceV0, compile_transaction, tuktuk::{
        cpi::{accounts::QueueTaskV0, queue_task_v0}, program::Tuktuk, types::TriggerV0
    }, types::QueueTaskArgsV0
};
use anchor_lang::{InstructionData, ToAccountMetas, solana_program::instruction::Instruction};
use crate::state::WhitelistedUser;

#[derive(Accounts)]
pub struct ScheduleExpiry<'info> {
    #[account(mut)]
    pub task_queue: UncheckedAccount<'info>,

    pub task_queue_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub task: AccountInfo<'info>,

    #[account(mut)]
    pub queue_authority: Signer<'info>,

    #[account(mut)]
    pub whitelisted_user: Account<'info, WhitelistedUser>,

    pub system_program: Program<'info, System>,

    pub tuktuk_program: Program<'info, Tuktuk>,
}

impl<'info> ScheduleExpiry<'info> {
   pub fn schedule_expiry(
    ctx: Context<ScheduleExpiry>,
    task_id: u16,
) -> Result<()> {
    let (compiled_tx, _) = compile_transaction(
        vec![Instruction {
            program_id: crate::ID,
            accounts: crate::accounts::ExpireUser {
                whitelisted_user: ctx.accounts.whitelisted_user.to_account_info(),
            }
            .to_account_metas(None)
            .to_vec(),
            data:   crate::instruction::ExpireUser {}.data(),
        }],
        vec![],
    )
    .unwrap();

    queue_task_v0(
        CpiContext::new(
            ctx.accounts.tuktuk_program.to_account_info(),
            QueueTaskV0 {
                payer: ctx.accounts.queue_authority.to_account_info(),
                queue_authority: ctx.accounts.queue_authority.to_account_info(),
                task_queue: ctx.accounts.task_queue.to_account_info(),
                task_queue_authority: ctx.accounts.task_queue_authority.to_account_info(),
                task: ctx.accounts.task.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
            },
        ),
        QueueTaskArgsV0 {
            trigger: TriggerV0::Timestamp(
                ctx.accounts.whitelisted_user.expiry_timestamp,
            ),
            transaction: TransactionSourceV0::CompiledV0(compiled_tx),
            crank_reward: None,
            free_tasks: 1,
            id: task_id,
            description: "expire whitelist user".to_string(),
        },
    )?;

    Ok(())
}
}