use anchor_lang::prelude::*;

pub const MAX_VALIDATOR_NUMBER: usize = 32;
pub const MAX_MINER_NUMBER: usize = 32;
pub const SUBNET_EPOCH_DURATION: i64 = 60 * 60 * 24;

#[account(zero_copy(unsafe))]
#[repr(packed)]
#[derive(Default, Debug)]
pub struct SubnetState {
    pub id: u8,

    pub epoch_start_timestamp: i64,

    pub total_stake: u64,
    pub stake: u64,
    pub distribute_reward: u64,

    pub validators: [ValidatorInfo; MAX_VALIDATOR_NUMBER],
    pub miners: [MinerInfo; MAX_MINER_NUMBER],
}

impl SubnetState {
    pub const LEN: usize = 1
        + 8
        + 8
        + 8
        + ValidatorInfo::LEN * MAX_VALIDATOR_NUMBER
        + MinerInfo::LEN * MAX_MINER_NUMBER;

    pub fn update_epoch_start_timestamp(&mut self, timestamp: i64) -> () {
        self.epoch_start_timestamp = timestamp;
    }

    pub fn create_validator(&mut self, owner: Pubkey) -> () {
        for i in 0..MAX_VALIDATOR_NUMBER {
            if self.validators[i].id == 0 {
                self.validators[i].id = i as u8 + 1;
                self.validators[i].stake = 0;
                self.validators[i].bonds = 0;
                self.validators[i].reward = 0;
                self.validators[i].owner = owner;
            }
        }
    }

    pub fn create_miner(&mut self, owner: Pubkey) -> () {
        for i in 0..MAX_MINER_NUMBER {
            if self.miners[i].id == 0 {
                self.miners[i].id = i as u8 + 1;
                self.miners[i].stake = 0;
                self.miners[i].owner = owner;
            }
        }
    }
}

#[zero_copy(unsafe)]
#[repr(packed)]
#[derive(Default, Debug)]
pub struct ValidatorInfo {
    pub owner: Pubkey,
    pub id: u8,
    // 质押数量
    pub stake: u64,
    // 工作量
    pub bonds: u64,
    // 待提取奖励
    pub reward: u64,
}

impl ValidatorInfo {
    pub const LEN: usize = 8 + 32 + 1 + 8 + 8;
}

#[zero_copy(unsafe)]
#[repr(packed)]
#[derive(Default, Debug)]
pub struct MinerInfo {
    pub owner: Pubkey,
    pub id: u8,
    pub stake: u64,
}

impl MinerInfo {
    pub const LEN: usize = 8 + 32 + 1 + 8;
}
