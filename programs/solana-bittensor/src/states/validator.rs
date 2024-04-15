use anchor_lang::prelude::*;
// 考虑 作为主网验证者或子网验证者

#[account]
pub struct ValidatorState {
    pub id: u8,
    pub owner: Pubkey,
    pub stake: u64,
    pub bonds: u64,
    pub is_active: bool,
}

impl ValidatorState {
    pub fn add_stake(&mut self, amount: u64) {
        self.stake += amount;
    }

    pub fn remove_stake(&mut self, amount: u64) {
        self.stake -= amount;
    }
}
