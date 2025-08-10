use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

use crate::{error::DiceError, state::BetConfig};
#[derive(Accounts)]
#[instruction(seed:u128)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    /// CHECK : This is safe
    pub casino: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds=[b"vault",casino.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    #[account(
        init,
        payer = player,
        space = 8 + BetConfig::INIT_SPACE,
        seeds = [b"bet", vault.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump
    )]
    pub bet_account: Account<'info, BetConfig>,

    pub system_program: Program<'info, System>,
}
impl<'info> PlaceBet<'info> {
    pub fn create_bet(
        &mut self,
        bet_amount: u64,
        seed: u128,
        roll: u8,
        bump: &PlaceBetBumps
    ) -> Result<()> {
        self.bet_account.set_inner(BetConfig {
            player: self.player.key(),
            bet_amount,
            seed,
            slot: Clock::get()?.slot,
            roll,
            bump: bump.bet_account,
        });

        Ok(())
    }
    pub fn deposit(&self,bet_amount: u64) -> Result<()> {
        require!(bet_amount > 0, DiceError::InvalidAmount);

        let account = Transfer {
            from: self.player.to_account_info(),
            to: self.vault.to_account_info(),
        };
        let ctx = CpiContext::new(self.system_program.to_account_info(), account);
        transfer(ctx, bet_amount)
    }
}
