use anchor_lang::prelude::*;

mod errors;
mod instructions;
mod state;

use errors::*;
use instructions::*;

declare_id!("4hyzssTc37NxyaaB5Zu4QvxTnRUyi7mmAHhWVBZF7eS2");

#[program]
pub mod vault {
    use super::*;

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount, &ctx.bumps)?;

        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)?;

        Ok(())
    }
}
