use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken, 
    token::{ Mint, Token, TokenAccount }
};

use crate::state::ConfigState;
#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct Initialize<'info>{
    #[account(mut)]
    pub initializer:Signer<'info>,
    pub mint_x:Account<'info,Mint>,
    pub mint_y:Account<'info,Mint>,
    #[account(
        init,
        payer=initializer,
        seeds=[b"mint_lp",config_account.key().as_ref()],
        bump,
        mint::decimals=6,
        mint::authority=config_account

    )]
    pub mint_lp_token:Account<'info,Mint>,
    #[account(
        init,
        payer=initializer,
        space=8+ ConfigState::INIT_SPACE,
        seeds=[b"config",seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub config_account:Account<'info,ConfigState>,

    #[account(
        init,
        payer=initializer,
        associated_token::mint=mint_x,
        associated_token::authority=config_account,
    )]
    pub vault_x:Account<'info,TokenAccount>,

     #[account(
        init,
        payer=initializer,
        associated_token::mint=mint_y,
        associated_token::authority=config_account,
    )]
    pub vault_y:Account<'info,TokenAccount>,

    // program
    pub token_program:Program<'info,Token>,
    pub associated_token_program:Program<'info,AssociatedToken>,
    pub system_program:Program<'info,System>
}

impl<'info>Initialize<'info>{
    pub fn initializer(&mut self,seed:u64,fee:u16,authority:Option<Pubkey>,bumps:&InitializeBumps)->Result<()>{
        self.config_account.set_inner(ConfigState { 
            seed, 
            mint_x: self.mint_x.key(), 
            mint_y: self.mint_y.key(), 
            fee,
            locked: false, 
            authority, 
            config_state_bump: bumps.config_account, 
            lp_bump: bumps.mint_lp_token
        });
        Ok(())
    }
}