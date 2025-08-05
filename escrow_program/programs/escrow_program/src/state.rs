use anchor_lang::prelude::*;

#[account]
pub struct EscrowState{
    pub maker:Pubkey,
    pub mint_a:Pubkey,
    pub mint_b:Pubkey,
    pub receive:u64,
    pub bump:u8,
    pub seed:u64
}
impl Space for EscrowState{
    const INIT_SPACE: usize = 8 + 32+32+32+8+1+8;
}