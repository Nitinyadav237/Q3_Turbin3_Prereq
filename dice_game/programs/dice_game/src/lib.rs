#![allow(unexpected_cfgs, deprecated)]
use anchor_lang::prelude::*;

pub mod state;
pub mod error;
pub mod instructions;
pub use instructions::*;

declare_id!("EP6BsCcMdxxNvB3we8DnmGG5bc3ogMhPekiCZzwk9X7X");

#[program]
pub mod dice_game {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, amount: u64) -> Result<()> {
        ctx.accounts.initialize(amount)?;
        Ok(())
    }
    pub fn place_bet(ctx: Context<PlaceBet>, bet_amount: u64, seed: u128, roll: u8) -> Result<()> {
        ctx.accounts.create_bet(bet_amount, seed, roll, &ctx.bumps)?;
        ctx.accounts.deposit(bet_amount)?;
        Ok(())
    }
    pub fn resolve_bet(ctx: Context<ResolveBet>, sig: Vec<u8>) -> Result<()> {
        ctx.accounts.verify_ed25519_signature(&sig)?;
        ctx.accounts.resolve_bet(&sig, &ctx.bumps)?;
        Ok(())
    }
    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.refund(&ctx.bumps)?;
        Ok(())
    }
}
