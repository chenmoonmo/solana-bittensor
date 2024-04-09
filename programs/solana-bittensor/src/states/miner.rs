use anchor_lang::prelude::*;

#[account(zero_copy(unsafe))]
#[repr(packed)]
#[derive(Default, Debug)]
pub struct MinerState {
    pub id: u8,
    pub owner: Pubkey,
    pub stake: u64,
}

impl MinerState {
    pub fn add_stake(&mut self, amount: u64) -> () {
        self.stake += amount;
    }

    pub fn remove_stake(&mut self, amount: u64) -> () {
        self.stake -= amount;
    }
}

