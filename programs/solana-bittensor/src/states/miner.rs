use anchor_lang::prelude::*;

#[account]
#[derive(Default, Debug)]
pub struct MinerState {
    pub id: u32,
    pub owner: Pubkey,
    pub stake: u64,
}

impl MinerState {
    pub const LEN: usize = 1 + 4 + 32 + 8 + 32; // 1 + 1 + 32 + 8 = 42

    pub fn initialize(&mut self, id: &u32, owner: Pubkey) -> () {
        self.id = *id;
        self.owner = owner;
    }

    pub fn add_stake(&mut self, amount: u64) -> () {
        self.stake += amount;
    }

    pub fn remove_stake(&mut self, amount: u64) -> () {
        self.stake -= amount;
    }
}

// event when miner register
#[event]
#[cfg_attr(feature = "client", derive(Debug))]
pub struct MinerRegisterEvent {
    pub id: u32,
    pub owner: Pubkey,
    pub stake: u64,
    pub pubkey: Pubkey,
    pub pre_pubkey: Pubkey,
}

// event when miner claim reward
#[event]
#[cfg_attr(feature = "client", derive(Debug))]
pub struct MinerClaimRewardEvent {
    pub id: u32,
    pub owner: Pubkey,
    pub pubkey: Pubkey,
    pub claim_amount: u64,
}
