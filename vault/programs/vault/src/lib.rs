#![allow(deprecated, unexpected_cfgs)]
use anchor_lang::prelude::*;
pub mod state;
pub mod instructions;
pub use instructions::*;
declare_id!("4PzMXvgLpbwrRrbz5u13od6N7tXxN8v3BnLsXFGzKRMU");

#[program]
pub mod vault {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)?;
        Ok(())
    }

    pub fn deposit(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)?;
        Ok(())
    }

    pub fn close(ctx: Context<Close>) -> Result<()> {
        ctx.accounts.close()?;
        Ok(())
    }
}
