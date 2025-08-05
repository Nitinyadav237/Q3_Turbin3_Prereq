use anchor_lang::{ prelude::*, system_program::{ transfer, Transfer } };

use crate::state::VaultState;

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        close=user,
        seeds=[b"vault",user.key().as_ref()],
        bump=vault_state.vault_bump
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds=[b"vault",vault_state.key().as_ref()],
        bump=vault_state.vault_bump
    )]
    pub vault: SystemAccount<'info>,

    // program
    pub system_program: Program<'info, System>,
}

impl<'info> Close<'info> {
    pub fn close(&self) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };
        let seeds=&[b"vault",self.vault_state.to_account_info().key.as_ref(),&[self.vault_state.vault_bump]];
        let signer_seeds=&[&seeds[..]];

        let cpi_ctx=CpiContext::new_with_signer(cpi_program,cpi_accounts, signer_seeds);

        transfer(cpi_ctx, self.vault.lamports())?;
        Ok(())
    }
}
