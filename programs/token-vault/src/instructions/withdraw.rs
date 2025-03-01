use crate::{error::ErrorCode, state::VaultState};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    pub owner: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault_state"],
        bump = vault_state.vault_state_bump,
        constraint = owner.key() == vault_state.owner @ ErrorCode::UnauthorizedWithdrawal
    )]
    pub vault_state: Account<'info, VaultState>,
    #[account(mut)]
    /// CHECK: Recipient for withdrawal
    pub recipient: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    require!(
        ctx.accounts.vault_state.to_account_info().lamports() >= amount,
        ErrorCode::InsufficientFunds
    );

    **ctx
        .accounts
        .vault_state
        .to_account_info()
        .try_borrow_mut_lamports()? -= amount;
    **ctx.accounts.recipient.try_borrow_mut_lamports()? += amount;

    emit!(WithdrawEvent {
        recipient: *ctx.accounts.recipient.key,
        amount,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}

#[event]
pub struct WithdrawEvent {
    pub recipient: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}
