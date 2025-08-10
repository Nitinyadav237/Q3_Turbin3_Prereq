use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{ transfer, Mint, Token, TokenAccount, Transfer },
};
use constant_product_curve::{ConstantProduct, LiquidityPair};

use crate::error::AmmError;
use crate::state::ConfigState;

#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct Swap<'info> {
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
        seeds = [b"config", seed.to_le_bytes().as_ref()],
        bump = config_accounts.config_state_bump
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
        payer = user,
        associated_token::mint = mint_x,
        associated_token::authority = user
    )]
    pub user_x: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_y,
        associated_token::authority = user
    )]
    pub user_y: Account<'info, TokenAccount>,

    // program
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
impl<'info> Swap<'info> {
    pub fn swap(&self, is_x: bool,amount: u64,min: u64) -> Result<()> {

        require!(self.config_accounts.locked==false,AmmError::PoolLocked);
        require!(amount!=0 ,AmmError::InvalidAmount);

        let mut curve=ConstantProduct::init(
              self.vault_x.amount,
             self.vault_y.amount, 
             self.mint_lp.supply, 
            self.config_accounts.fee,
            None
        ).map_err(AmmError::from)?;

        let liquidity_pair=match is_x  {
            true=>LiquidityPair::X,
            false=>LiquidityPair::Y
        };

        let response=curve.swap(liquidity_pair, amount, min).map_err(AmmError::from)?;
        
        require!(response.deposit!=0,AmmError::InvalidAmount);
        require!(response.withdraw!=0,AmmError::InvalidAmount);
        
        self.deposit_token(is_x, response.deposit)?;
        self.withdraw_token(is_x, response.withdraw)?;

        Ok(())
    }
    pub fn deposit_token(&self, is_x: bool, amount: u64) -> Result<()> {
        let (from, to) = match is_x {
            true => (self.user_x.to_account_info(), self.vault_x.to_account_info()),
            false => (self.user_y.to_account_info(), self.vault_y.to_account_info()),
        };

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Transfer {
            from,
            to,
            authority: self.user.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, amount)
    }
    pub fn withdraw_token(&self, is_x: bool, amount: u64) -> Result<()> {
        let (from, to) = match is_x {
            true => (self.vault_y.to_account_info(), self.user_y.to_account_info()),
            false => (self.vault_x.to_account_info(), self.user_x.to_account_info()),
        };

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Transfer {
            from,
            to,
            authority: self.config_accounts.to_account_info(),
        };

      let seeds = &[
            &b"config"[..],
            &self.config_accounts.seed.to_le_bytes(),
            &[self.config_accounts.config_state_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer(cpi_ctx, amount)
    }
}
