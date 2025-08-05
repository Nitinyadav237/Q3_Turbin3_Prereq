#![allow(unexpected_cfgs,deprecated)]
use anchor_lang::prelude::*;
pub mod state;
pub mod instructions;
pub use instructions::*;
declare_id!("6ih8oekeSo9ohzMEk5r4vrCKZZqufNS5KSDRJqZpz7ST");

#[program]
pub mod escrow_program {
    use super::*;

    pub fn maker(ctx:Context<Maker>,seed:u64,deposit_amount:u64,receive:u64) -> Result<()> {
        ctx.accounts.init_escrow_state(seed,receive,&ctx.bumps)?;
        ctx.accounts.deposit(deposit_amount)?;
        Ok(())
    }
    
    pub fn taker(ctx:Context<Taker>) -> Result<()> {
        ctx.accounts.deposit()?;
        ctx.accounts.withdraw_close()?;
        Ok(())
    }
    
    pub fn refund(ctx:Context<Refund>) -> Result<()> {
        ctx.accounts.refund_close_account()?;
        Ok(())
    }
}

