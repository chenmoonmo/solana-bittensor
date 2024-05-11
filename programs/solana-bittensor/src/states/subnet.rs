use anchor_lang::prelude::*;

pub const MAX_VALIDATOR_NUMBER: usize = 32;
// TODO: max_group_miner_number = 100 MAX_MINER_NUMBER = 1000
pub const MAX_GROUP_MINER_NUMBER: usize = 100;
pub const MAX_MINER_NUMBER: usize = 1000;
pub const SUBNET_EPOCH_DURATION: i64 = 60 * 60 * 24;
// 保护期
pub const MINER_PROTECTION: u8 = 1;
pub const VALIDATOR_PROTECTION: u64 = 1;

#[account]
#[derive(Default, Debug)]
pub struct SubnetState {
    pub id: u8,
    pub owner: Pubkey,
    pub miners: [Pubkey; 10],
    pub validators: Pubkey,
    pub stake: u64,
    pub distribute_reward: u64,
    pub epoch_number: u64,
    pub epoch_start_timestamp: i64,
    pub end_step: u8,
    pub epoch_total_weights: u64,
}

impl SubnetState {
    pub const LEN: usize = 1 + 32 * 10 + 32 * 10 + 32 + 8 + 8 + 8 + 8 + 8 + 1; // 1 + 32 * 32 + 32 * 10 + 32 + 32 + 8 + 8 + 8 + 8 + 1 = 738
    pub fn register(&mut self, owner: Pubkey) -> () {
        self.owner = owner;
    }

    pub fn initialize(&mut self, id: u8) -> () {
        self.id = id;
    }
}
