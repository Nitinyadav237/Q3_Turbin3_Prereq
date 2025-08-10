use anchor_lang::{ prelude::*, system_program::{ Transfer, transfer } };
use crate::{ error::DiceError, state::BetConfig };
#[derive(Accounts)]
#[instruction(seed:u128)]
pub struct Refund<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    pub casino: SystemAccount<'info>,
    #[account(
        mut,
        seeds=[b"vault",casino.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    #[account(
        mut,
        close=player,
        seeds = [b"bet", vault.key().as_ref(), seed.to_le_bytes().as_ref()], 
        bump=bet_account.bump
    )]
    pub bet_account: Account<'info, BetConfig>,

    pub system_program: Program<'info, System>,
}

impl<'info> Refund<'info> {
    pub fn refund(&mut self, bumps: &RefundBumps) -> Result<()> {
        let slot = Clock::get()?.slot;
        require!(self.bet_account.slot - slot > 1000, DiceError::TimeoutNotReached);
        let accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.player.to_account_info(),
        };

        let seeds = [b"vault", &self.casino.key().to_bytes()[..], &[bumps.vault]];
        let signer_seeds = &[&seeds[..]][..];

        let ctx = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            accounts,
            signer_seeds
        );

        transfer(ctx, self.bet_account.bet_amount)
    }
}
