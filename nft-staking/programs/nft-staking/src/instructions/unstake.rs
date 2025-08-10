use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        mpl_token_metadata::instructions::{
            ThawDelegatedAccountCpi,
            ThawDelegatedAccountCpiAccounts,
        },
        MasterEditionAccount,
        Metadata,
    },
    token::{ revoke, Revoke, Mint, Token, TokenAccount },
};

use crate::state::{ StakeAccount, StakeConfigAccount, UserAccount };
use crate::error::StakeError;

#[derive(Accounts)]
pub struct UnStake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint: Account<'info, Mint>,

    // PDA = ["metadata", metadata_program_id, mint_address, "edition"]
    #[account(
        seeds = [b"metadata", metadata_program.key().as_ref(), mint.key().as_ref(), b"edition"],
        seeds::program = metadata_program.key(),
        bump
    )]
    pub edition: Account<'info, MasterEditionAccount>,

    #[account(
        mut,
        associated_token::mint=mint,
        associated_token::authority=user
    )]
    pub mint_ata: Account<'info, TokenAccount>,

    // account ->stakeacc,config,useracc
    #[account(seeds = [b"config"], bump = stake_config_account.bump)]
    pub stake_config_account: Account<'info, StakeConfigAccount>,

    #[account(
        mut,
        seeds=[b"user",user.key().as_ref()],
        bump=stake_config_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        mut,
        close=user,
        seeds = [b"stake".as_ref(), mint.key().as_ref(), stake_config_account.key().as_ref()],
        bump
    )]
    pub stake_account: Account<'info, StakeAccount>,

    // program
    pub metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> UnStake<'info> {
    pub fn unstake(&mut self) -> Result<()> {
        // check stakeat_passes the freezing period then calculate poiint and then undelegate to stakeaccount then it call thaw

        let time_elapsed = ((Clock::get()?.unix_timestamp - self.stake_account.staked_at) /
            86400) as u32;
        require!(
            time_elapsed >= self.stake_config_account.freeze_period,
            StakeError::FreezePeriodNotPassed
        );

        //reward point
        self.user_account.points = time_elapsed.saturating_mul(
            self.stake_config_account.points_per_stake as u32
        );

        self.unfreeze_mint()?;

        self.undelegate_mint()?;

        self.user_account.total_amount_staked -= 1;

        Ok(())
    }

    pub fn unfreeze_mint(&self) -> Result<()> {
        let cpi_program = &self.metadata_program.to_account_info();

        let cpi_accounts = ThawDelegatedAccountCpiAccounts {
            delegate: &self.stake_account.to_account_info(),
            token_account: &self.mint_ata.to_account_info(),
            edition: &self.edition.to_account_info(),
            mint: &self.mint.to_account_info(),
            token_program: &self.token_program.to_account_info(),
        };

        let seeds = &[
            b"stake",
            self.mint.to_account_info().key.as_ref(),
            self.stake_config_account.to_account_info().key.as_ref(),
        ];
        let signers_seed = &[&seeds[..]];

        ThawDelegatedAccountCpi::new(cpi_program, cpi_accounts).invoke_signed(signers_seed)?;

        Ok(())
    }

    pub fn undelegate_mint(&self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Revoke {
            authority: self.user.to_account_info(),
            source: self.mint_ata.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        revoke(cpi_ctx)?;
        Ok(())
    }
}
