use anchor_lang::prelude::*;

pub const MAX_VALIDATOR_NUMBER: usize = 32;
pub const MAX_MINER_NUMBER: usize = 32;
pub const SUBNET_EPOCH_DURATION: i64 = 60 * 60 * 24;

#[account(zero_copy(unsafe))]
#[repr(packed)]
#[derive(Default, Debug)]
pub struct SubnetState {
    pub id: u8,
    pub miner_total_stake: u64,
    pub validator_total_stake: u64,
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

    pub fn create_validator(&mut self, owner: Pubkey) -> u8 {
        let mut id = 0u8;
        for i in 0..MAX_VALIDATOR_NUMBER {
            if self.validators[i].id == 0 {
                id = i as u8 + 1;
                self.validators[i].id = id;
                self.validators[i].owner = owner;
                break;
            }
        }
        id
    }

    pub fn create_miner(&mut self, owner: Pubkey) -> u8 {
        let mut id = 0u8;

        for i in 0..MAX_MINER_NUMBER {
            if self.miners[i].id == 0 {
                id = i as u8 + 1;
                self.miners[i].id = id;
                self.miners[i].owner = owner;
                break;
            }
        }
        id
    }

    pub fn miner_add_stake(&mut self, miner_id: u8, amount: u64) -> () {
        for i in 0..MAX_MINER_NUMBER {
            if self.miners[i].id == miner_id {
                self.miners[i].stake += amount;
                break;
            }
        }
        self.miner_total_stake += amount;
    }

    pub fn validator_add_stake(&mut self, validator_id: u8, amount: u64) -> () {
        for i in 0..MAX_VALIDATOR_NUMBER {
            if self.validators[i].id == validator_id {
                self.validators[i].stake += amount;
                break;
            }
        }
        self.validator_total_stake += amount;
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
    // 上一个周期的工作量
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
    // 待提取奖励
    pub reward: u64,
}

impl MinerInfo {
    pub const LEN: usize = 8 + 32 + 1 + 8 + 8;
}
