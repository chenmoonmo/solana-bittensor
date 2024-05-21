use anchor_lang::prelude::*;

pub const MAX_VALIDATOR_NUMBER: usize = 32;

pub const MAX_GROUP_MINER_NUMBER: usize = 100;
pub const MAX_MINER_NUMBER: usize = 1000;

pub const SUBNET_EPOCH_DURATION: i64 = 60 * 60 * 24;
// 保护期
pub const MINER_PROTECTION: u8 = 1;
pub const VALIDATOR_PROTECTION: u64 = 1;
// 奖励
pub const MINER_EPOCH_REWARD: u64 = 10_000_000_000;
pub const VALIDATOR_EPOCH_REWARD: u64 = 10_000_000_000;

#[account]
#[derive(Default, Debug)]
pub struct SubnetState {
    pub owner: Pubkey,
    pub stake: u64,
    pub distribute_reward: u64,
    pub epoch_number: u64,
    pub epoch_start_timestamp: i64,
    pub epoch_total_weights: u64,
}

impl SubnetState {
    pub const LEN: usize = 32 + 8 + 8 + 8 + 8 + 8 + 4;

    pub fn register(&mut self, owner: Pubkey) -> () {
        self.owner = owner;
    }

    pub fn end_epoch(&mut self, timestamp: i64) -> () {
        self.epoch_number += 1;
        self.epoch_start_timestamp = timestamp;
        self.epoch_total_weights = 0;
    }
}
