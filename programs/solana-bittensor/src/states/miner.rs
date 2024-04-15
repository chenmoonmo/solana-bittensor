use anchor_lang::prelude::*;

#[account]
pub struct MinerState {
    pub id: u8,
    pub subnet_id: u8,
    pub owner: Pubkey,
    pub stake: u64,
    pub is_active: bool,
}

impl MinerState {
    pub fn initialize(&mut self, id: u8, subnet_id: u8, owner: Pubkey) -> () {
        self.id = id;
        self.subnet_id = subnet_id;
        self.owner = owner;
        self.is_active = true;
    }

    pub fn add_stake(&mut self, amount: u64) -> () {
        self.stake += amount;
    }

    pub fn remove_stake(&mut self, amount: u64) -> () {
        self.stake -= amount;
    }
}
