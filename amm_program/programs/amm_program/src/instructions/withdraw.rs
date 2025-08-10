use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{ transfer, Mint, Token, TokenAccount, Transfer, Burn, burn },
};

use crate::state::ConfigState;
use crate::error::AmmError;
use constant_product_curve::ConstantProduct;
#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,
    #[account(
        mut,
        seeds=[b"mint_lp",config_accounts.key().as_ref()],
        bump=config_accounts.lp_bump
    )]
    pub mint_lp: Account<'info, Mint>,
    #[account(
        mut,
        has_one=mint_x,
        has_one=mint_y,
        seeds=[b"config",seed.to_le_bytes().as_ref()],
        bump=config_accounts.config_state_bump
        
    )]
    pub config_accounts: Account<'info, ConfigState>,

    #[account(
        mut,
        associated_token::mint=mint_x,
        associated_token::authority=config_accounts
    )]
    pub vault_x: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint=mint_y,
        associated_token::authority=config_accounts
    )]
    pub vault_y: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer=user,
        associated_token::mint = mint_x,
        associated_token::authority = user,
    )]
    pub user_x: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer=user,
        associated_token::mint = mint_y,
        associated_token::authority = user,
    )]
    pub user_y: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_lp,
        associated_token::authority = user
    )]
    pub user_lp: Account<'info, TokenAccount>,

    // Program
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(&self, min_x: u64, min_y: u64, amount: u64) -> Result<()> {
        require!(self.config_accounts.locked == false, AmmError::PoolLocked);
        require!(amount != 0, AmmError::InvalidAmount);
        require!(min_x != 0 || min_y != 0, AmmError::InvalidAmount);

        let amounts = ConstantProduct::xy_withdraw_amounts_from_l(
            self.vault_x.amount,
            self.vault_y.amount,
            self.mint_lp.supply,
            amount,
            6
        ).map_err(AmmError::from)?;

        require!(min_x <= amounts.x && min_y <= amounts.y, AmmError::SlippageExceeded);

        self.withdraw_token(true, amount)?;
        self.withdraw_token(false, amount)?;
        self.burn_token(amount)
    }

    pub fn withdraw_token(&self, is_x: bool, amount: u64) -> Result<()> {
        let (from, to) = match is_x {
            true => (self.vault_x.to_account_info(), self.user_x.to_account_info()),
            false => (self.vault_y.to_account_info(), self.user_y.to_account_info()),
        };

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Transfer {
            from,
            to,
            authority: self.config_accounts.to_account_info(),
        };

        let seeds: &[&[u8]] = &[
            &b"config"[..],
            &self.config_accounts.seed.to_le_bytes(),
            &[self.config_accounts.lp_bump],
        ];

        let signer_seed: &[&[&[u8]]; 1] = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seed);

        transfer(cpi_ctx, amount)
    }
    pub fn burn_token(&self, amount: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Burn {
            from: self.user_lp.to_account_info(),
            mint: self.mint_lp.to_account_info(),
            authority: self.config_accounts.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        burn(cpi_ctx, amount)
    }
}
