use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};
use crate::state::VaultState;
#[derive(Accounts)]
pub struct Initialize<'info>{
    #[account(mut)]
    pub user:Signer<'info>,
    #[account(
        init,
        payer=user,
        space=VaultState::INIT_SPACE,
        seeds=[b"vault_program",user.key().as_ref()],
        bump
    )]
    pub vault_state:Account<'info,VaultState>,

    #[account(
        mut,
       seeds=[b"vault",vault_state.key().as_ref()],
       bump
    )]
    pub vault:SystemAccount<'info>,

    // program
    pub system_program:Program<'info,System>

}

impl<'info> Initialize<'info>{
    pub fn initialize(&mut self,bumps:&InitializeBumps)->Result<()>{
        let rent_exempt=Rent::get()?.minimum_balance(self.vault.to_account_info().data_len());
        // print!("rent ex ",rent_exempt);

        let cpi_program=self.system_program.to_account_info();
        let cpi_accounts=Transfer{
            from:self.user.to_account_info(),
            to:self.vault.to_account_info()
        };
        let cpi_ctx=CpiContext::new(cpi_program, cpi_accounts);
        transfer(cpi_ctx, rent_exempt);

        self.vault_state.vault_state_bump=bumps.vault_state;
        self.vault_state.vault_bump=bumps.vault;
        Ok(())
    }
}
