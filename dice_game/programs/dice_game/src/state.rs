use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct BetConfig{
    pub player:Pubkey,
    pub bet_amount:u64,
    pub slot:u64,
    pub seed:u128,
    pub roll:u8,
    pub bump:u8
}

impl BetConfig{
    pub fn to_slice(&mut self)->Vec<u8>{
        let mut s = self.player.to_bytes().to_vec();
        s.extend_from_slice(&self.seed.to_le_bytes());
        s.extend_from_slice(&self.slot.to_le_bytes());
        s.extend_from_slice(&self.bet_amount.to_le_bytes());
        s.extend_from_slice(&[self.roll, self.bump]);
        s

    }
}