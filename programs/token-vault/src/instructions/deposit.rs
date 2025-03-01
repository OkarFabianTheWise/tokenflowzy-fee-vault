use crate::state::VaultState;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, system_instruction};

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault_state"],
        bump = vault_state.vault_state_bump,
    )]
    pub vault_state: Account<'info, VaultState>,
    pub system_program: Program<'info, System>,
}

pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    let transfer_ix = system_instruction::transfer(
        &ctx.accounts.depositor.key(),
        &ctx.accounts.vault_state.to_account_info().key(),
        amount,
    );

    invoke(
        &transfer_ix,
        &[
            ctx.accounts.depositor.to_account_info(),
            ctx.accounts.vault_state.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    ctx.accounts.vault_state.revenue += amount;
    ctx.accounts.vault_state.tokens_deployed += 1;

    emit!(DepositEvent {
        depositor: *ctx.accounts.depositor.key,
        amount,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}

#[event]
pub struct DepositEvent {
    pub depositor: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}
