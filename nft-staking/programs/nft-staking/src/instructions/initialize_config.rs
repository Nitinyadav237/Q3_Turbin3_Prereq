use anchor_lang::prelude::*;
use anchor_spl::token::{ Mint, Token };

use crate::state::StakeConfigAccount;

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    // account stakeconfig ,stakeacc and useracc
    #[account(
        init,
        payer = admin,
        space = 8 + StakeConfigAccount::INIT_SPACE,
        seeds = [b"config"],
        bump
    )]
    pub stake_config_account: Account<'info, StakeConfigAccount>,

    #[account(
        init,
        payer = admin,
        seeds = [b"reward", stake_config_account.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = stake_config_account
    )]
    pub rewards_mint: Account<'info, Mint>,

    // program
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeConfig<'info> {
    pub fn initialize_config(
        &mut self,
        points_per_stake: u8,
        max_stake: u8,
        freeze_period: u32,
        bump: &InitializeConfigBumps
    ) -> Result<()> {
        self.stake_config_account.set_inner(StakeConfigAccount {
            points_per_stake,
            max_stake,
            freeze_period,
            rewards_bump: bump.rewards_mint,
            bump: bump.stake_config_account,
        });
        Ok(())
    }
}
