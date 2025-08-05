use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        Mint,
        TokenAccount,
        TokenInterface,
        transfer_checked,
        TransferChecked,
        close_account,
        CloseAccount,
    },
};

use crate::state::EscrowState;

#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct Taker<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(mut)]
    pub maker: SystemAccount<'info>,
    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_b,
        associated_token::authority = escrow_state,
        associated_token::token_program = token_program
    )]
    pub maker_ata_b: InterfaceAccount<'info, TokenAccount>, //usdc

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_b,
        associated_token::authority = escrow_state,
        associated_token::token_program = token_program
    )]
    pub taker_ata_a: InterfaceAccount<'info, TokenAccount>, // bonk

    #[account(
        mut,
        associated_token::mint=mint_b,
        associated_token::authority=taker,
        associated_token::token_program=token_program
    )]
    pub taker_ata_b: InterfaceAccount<'info, TokenAccount>, // usdc for ata b

    #[account(
        mut,
        close=maker,
        seeds = [b"escrow_state", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump=escrow_state.bump
    )]
    pub escrow_state: Account<'info, EscrowState>,

    #[account(
        mut,
        associated_token::mint=mint_a,
        associated_token::authority=escrow_state,
        associated_token::token_program=token_program

    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    // program
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Taker<'info> {
    pub fn deposit(&self) -> Result<()> {
        // transfer token b to maker ata-b and transfer token a to maker ata and close the account the account
        let cpi_accounts = TransferChecked {
            from: self.taker_ata_b.to_account_info(),
            to: self.maker_ata_b.to_account_info(),
            mint: self.mint_b.to_account_info(),
            authority: self.taker.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);

        transfer_checked(cpi_ctx, self.escrow_state.receive, self.mint_b.decimals)?;

        Ok(())
    }
    pub fn withdraw_close(&self) -> Result<()> {
        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.taker_ata_a.to_account_info(),
            mint: self.mint_a.to_account_info(),
            authority: self.escrow_state.to_account_info(),
        };
        let seeds: &[&[u8]] = &[
            b"escrow_state",
            self.maker.to_account_info().key.as_ref(),
            &self.escrow_state.seed.to_le_bytes()[..],
            &[self.escrow_state.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds
        );
        transfer_checked(cpi_ctx, self.vault.amount, self.mint_a.decimals)?;

        let close_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.taker.to_account_info(),
            authority: self.escrow_state.to_account_info(),
        };

        let close_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            close_accounts,
            signer_seeds
        );

        close_account(close_ctx)?;

        Ok(())
    }
}
