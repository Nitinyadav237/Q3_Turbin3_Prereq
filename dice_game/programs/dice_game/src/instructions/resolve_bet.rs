use anchor_lang::{
    prelude::*,
    solana_program::sysvar::instructions::load_instruction_at_checked,
    system_program::{ Transfer, transfer },
};
use anchor_instruction_sysvar::Ed25519InstructionSignatures;
use solana_program::hash::hash;
use solana_program::ed25519_program;

use crate::{ error::DiceError, state::BetConfig };
#[derive(Accounts)]
#[instruction(seed:u128)]
pub struct ResolveBet<'info> {
    pub casino: Signer<'info>,
    #[account(mut)]
    /// CHECK : This is safe
    pub player: UncheckedAccount<'info>,

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
    #[account(address = solana_program::sysvar::instructions::ID)]
    /// CHECK : This is safe
    pub instruction_sysvar: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
impl<'info> ResolveBet<'info> {
    pub fn verify_ed25519_signature(&mut self, sig: &[u8]) -> Result<()> {
        // first take ix[0] out then check program.id  then check account=[] extract signature then comapre length then isveifiable==true then check signature==client(sig) simialy check message

        let ix = load_instruction_at_checked(0, &self.instruction_sysvar.to_account_info())?;

        require_keys_eq!(ix.program_id, ed25519_program::ID, DiceError::Ed25519Program);
        require_eq!(ix.accounts.iter().len(), 0, DiceError::Ed25519Accounts);

        let signatures = Ed25519InstructionSignatures::unpack(&ix.data)?.0;

        require_eq!(signatures.len(), 1, DiceError::Ed25519DataLength);

        let signature = &signatures[0];

        require!(signature.is_verifiable, DiceError::Ed25519Header);

        require_keys_eq!(
            signature.public_key.ok_or(DiceError::Ed25519Pubkey)?,
            self.casino.key(),
            DiceError::Ed25519Pubkey
        );

        require!(
            &signature.signature.ok_or(DiceError::Ed25519Signature)?.eq(sig),
            DiceError::Ed25519Signature
        );

        require!(
            &signature.message
                .as_ref()
                .ok_or(DiceError::Ed25519Signature)?
                .eq(&self.bet_account.to_slice()),
            DiceError::Ed25519Signature
        );

        Ok(())
    }

    pub fn resolve_bet(&mut self, sig: &[u8], bumps: &ResolveBetBumps) -> Result<()> {
        // Payout = Bet Amount × (100% - casino Edge) ÷ (ROLL_CHOSEN - 1) × 100

        let hash = hash(sig).to_bytes();

        let mut hash_16 = [0u8; 16];

        hash_16.copy_from_slice(&hash[0..16]);
        let lower = u128::from_le_bytes(hash_16);

        hash_16.copy_from_slice(&hash[16..32]);
        let higher = u128::from_le_bytes(hash_16);

        let roll = (lower.wrapping_add(higher).wrapping_rem(100) as u8) + 1;

        if self.bet_account.roll > roll {
            let payout = (self.bet_account.bet_amount as u128)
                .checked_mul(10000 - (150 as u128))
                .unwrap()
                .checked_div(self.bet_account.roll as u128)
                .unwrap()
                .checked_div(10000)
                .unwrap();
            let cpi_program = self.system_program.to_account_info();

            let cpi_accounts = Transfer {
                from: self.vault.to_account_info(),
                to: self.player.to_account_info(),
            };
            let seeds = [b"vault", &self.casino.key().to_bytes()[..], &[bumps.vault]];
            let signer_seeds = &[&seeds[..][..]];

            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
            transfer(cpi_ctx, payout as u64)?;
        }

        Ok(())
    }
}
