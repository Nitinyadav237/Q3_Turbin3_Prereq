#![allow(unexpected_cfgs, deprecated)]
pub mod state;
pub mod error;
pub mod instructions;
pub use instructions::*;

use anchor_lang::prelude::*;

declare_id!("9E29mNG4s2KJ6KUTPQo2UW5tYABcHniP4sHqCVoZXDsf");

#[program]
pub mod amm_program {
    use super::*;
    pub fn initialize(
        ctx: Context<Initialize>,
        seed: u64,
        fee: u16,
        authority: Option<Pubkey>
    ) -> Result<()> {
        ctx.accounts.initializer(seed, fee, authority, &ctx.bumps)?;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, max_x: u64, max_y: u64, amount: u64) -> Result<()> {
        ctx.accounts.deposit(max_x, max_y, amount)?;
        Ok(())
    }

    pub fn swap(ctx: Context<Swap>, is_x: bool, min: u64, amount: u64) -> Result<()> {
        ctx.accounts.swap(is_x, amount, min)?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, min_x: u64, min_y: u64, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(min_x, min_y, amount)?;
        Ok(())
    }
}
