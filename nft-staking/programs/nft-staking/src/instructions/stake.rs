use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        mpl_token_metadata::instructions::{
            FreezeDelegatedAccountCpi,
            FreezeDelegatedAccountCpiAccounts,
        },
        MasterEditionAccount,
        Metadata,
        MetadataAccount,
    },
    token::{ approve, Approve, Mint, Token, TokenAccount },
};

use crate::state::{ StakeAccount, StakeConfigAccount, UserAccount };
use crate::error::StakeError;

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint: Account<'info, Mint>,
    pub collection_mint: Account<'info, Mint>,

    #[account(
        seeds = [b"metadata", metadata_program.key().as_ref(), mint.key().as_ref()],
        seeds::program = metadata_program.key(),
        bump,
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() ==
        collection_mint.key().as_ref(),
        constraint = metadata.collection.as_ref().unwrap().verified == true
    )]
    pub metadata: Account<'info, MetadataAccount>,
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
        init,
        payer = user,
        space = 8 + StakeAccount::INIT_SPACE,
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

impl<'info> Stake<'info> {
    pub fn stake(&mut self, bump: &StakeBumps) -> Result<()> {
        // check max_Stake then initalize stakeaccount and then approve to delegate to stakeaccount then it call freeze

        require!(
            self.user_account.total_amount_staked < self.stake_config_account.max_stake,
            StakeError::MaxStakeReached
        );

        self.stake_account.set_inner(StakeAccount {
            mint: self.mint.key(),
            owner: self.user.key(),
            staked_at: Clock::get()?.unix_timestamp,
            bump: bump.stake_account,
        });

        self.delegate_mint()?;

        self.freeze_mint()?;

        self.user_account.total_amount_staked += 1;

        Ok(())
    }

    pub fn delegate_mint(&self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Approve {
            authority: self.user.to_account_info(),
            delegate: self.stake_account.to_account_info(),
            to: self.mint_ata.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        approve(cpi_ctx, 1)?;
        Ok(())
    }
    pub fn freeze_mint(&mut self) -> Result<()> {
        let cpi_program = &self.metadata_program.to_account_info();

        let cpi_accounts = FreezeDelegatedAccountCpiAccounts {
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

        FreezeDelegatedAccountCpi::new(cpi_program, cpi_accounts).invoke_signed(signers_seed)?;


        Ok(())
    }
}
