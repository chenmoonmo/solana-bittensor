use anchor_lang::prelude::*;

#[account]
#[derive(Default, Debug)]
pub struct MinerState {
    pub id: u8,
    pub subnet_id: u8,
    pub owner: Pubkey,
    pub stake: u64,
}

impl MinerState {

    pub const LEN: usize = 1 + 1 + 32 + 8; // 1 + 1 + 32 + 8 = 42


    pub fn initialize(&mut self, id: u8, subnet_id: u8, owner: Pubkey) -> () {
        self.id = id;
        self.subnet_id = subnet_id;
        self.owner = owner;
    }

    pub fn add_stake(&mut self, amount: u64) -> () {
        self.stake += amount;
    }

    pub fn remove_stake(&mut self, amount: u64) -> () {
        self.stake -= amount;
    }
}
