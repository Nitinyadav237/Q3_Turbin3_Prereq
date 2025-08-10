use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};
use crate::error::DiceError;
#[derive(Accounts)]
pub struct Initialize<'info>{
    #[account(mut)]
    pub casino:Signer<'info>,
    #[account(
        mut,
        seeds=[b"vault",casino.key().as_ref()],
        bump
    )]
    pub vault:SystemAccount<'info>,
    pub system_program:Program<'info,System>,

}
impl<'info>Initialize<'info> {
    pub fn initialize(&mut self,amount:u64)->Result<()>{
        
           require!(amount > 0, DiceError::InvalidAmount);

           let cpi_account=Transfer{
            from:self.casino.to_account_info(),
            to:self.vault.to_account_info()
           };
           let cpi_ctx=CpiContext::new(self.system_program.to_account_info(),cpi_account);
           transfer(cpi_ctx, amount)?;
           
        Ok(())
    }
}