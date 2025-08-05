use anchor_lang::{ prelude::*, system_program::{ transfer, Transfer } };

use crate::state::VaultState;

#[derive(Accounts)]
pub struct Payment<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(seeds = [b"vault", user.key().as_ref()], bump = vault_state.vault_bump)]
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

impl<'info> Payment<'info> {
    pub fn deposit(&self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer(cpi_ctx, amount)?;

        Ok(())
    }
        pub fn withdraw(&self,amount:u64)->Result<()>{

        let cpi_program=self.system_program.to_account_info();
        let cpi_accounts=Transfer{
            from:self.vault.to_account_info(),
            to:self.user.to_account_info()
        };

        let seeds:&[&[u8]]=&[b"vault",
        self.vault_state.to_account_info().key.as_ref(),
        &[self.vault_state.vault_bump]
        ];

        let signer_seed:&[&[&[u8]]]=&[&seeds[..]];

        let cpi_ctx=CpiContext::new_with_signer(cpi_program, cpi_accounts,signer_seed);
        transfer(cpi_ctx,amount)?;
        
        Ok(())
    }

}
