use anchor_lang::prelude::*;
#[account]
pub struct VaultState{
    pub vault_bump:u8,
    pub vault_state_bump:u8
}

impl Space for VaultState{
    const INIT_SPACE:usize=8+1+1;
}