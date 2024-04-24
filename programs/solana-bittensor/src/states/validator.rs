use anchor_lang::prelude::*;
// 考虑 作为主网验证者或子网验证者

#[account]
#[derive(Default, Debug)]
pub struct ValidatorState {
    pub id: u8,
    pub owner: Pubkey,
    pub stake: u64,
}

impl ValidatorState {
    pub const LEN: usize = 1 + 32 + 8; // 1 + 32 + 8 = 41
    pub fn add_stake(&mut self, amount: u64) {
        self.stake += amount;
    }

    pub fn remove_stake(&mut self, amount: u64) {
        self.stake -= amount;
    }
}
