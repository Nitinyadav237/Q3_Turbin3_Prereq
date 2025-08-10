#![allow(unexpected_cfgs, deprecated)]
use anchor_lang::prelude::*;
pub mod state;
pub mod error;
pub mod instructions;
pub use instructions::*;

declare_id!("F7qdEa1E6DJdS2ePSxxpyZb6WMXXujWXXR3X5zNCxety");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        points_per_stake: u8,
        max_stake: u8,
        freeze_period: u32
    ) -> Result<()> {
        ctx.accounts.initialize_config(points_per_stake, max_stake, freeze_period, &ctx.bumps)
    }

    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        ctx.accounts.initialize_user(&ctx.bumps)
    }

    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        ctx.accounts.stake(&ctx.bumps)
    }
    pub fn unstake(ctx: Context<UnStake>) -> Result<()> {
        ctx.accounts.unstake()
    }

    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        ctx.accounts.claim()
    }
}


