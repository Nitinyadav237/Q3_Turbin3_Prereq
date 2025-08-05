use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        Mint,
        TokenAccount,
        TokenInterface,
        TransferChecked,
        transfer_checked,
        CloseAccount,
        close_account,
    },
};

use crate::state::EscrowState;

#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct Refund<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(mint::token_program = token_program)]
    pub mint_a: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint=mint_a,
        associated_token::authority=maker,
        associated_token::token_program=token_program
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint=mint_a,
        associated_token::authority=escrow_state,
        associated_token::token_program=token_program

    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        close=maker,
        has_one=mint_a,
        seeds = [b"escrow_state", maker.key().as_ref(), seed.to_le_bytes().as_ref()], 
        bump=escrow_state.bump
    )]
    pub escrow_state: Account<'info, EscrowState>,

    // program
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Refund<'info> {
    pub fn refund_close_account(&self) -> Result<()> {
        // transfer token back to maker and close the account
        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.maker_ata_a.to_account_info(),
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
            destination: self.maker.to_account_info(),
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
