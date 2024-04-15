use anchor_lang::prelude::*;

pub const MAX_VALIDATOR_NUMBER: usize = 32;
pub const MAX_MINER_NUMBER: usize = 32;
pub const SUBNET_EPOCH_DURATION: i64 = 60 * 60 * 24;

#[account(zero_copy(unsafe))]
#[repr(packed)]
#[derive(Debug)]
pub struct SubnetState {
    pub id: u8,
    pub owner: Pubkey,
    pub stake: u64,
    pub last_validator_id: i8,
    pub last_miner_id: i8,
    pub miner_total_stake: u64,
    pub validator_total_stake: u64,
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

    pub fn initialize(&mut self, id: u8, owner: Pubkey) -> () {
        let validators = [ValidatorInfo::default(); MAX_VALIDATOR_NUMBER];
        let miners = [MinerInfo::default(); MAX_MINER_NUMBER];

        self.id = id;
        self.owner = owner;
        self.stake = 0;
        self.miner_total_stake = 0;
        self.validator_total_stake = 0;
        self.distribute_reward = 0;
        self.validators = validators;
        self.miners = miners;
        self.last_miner_id = -1;
        self.last_validator_id = -1;
    }

    pub fn create_validator(&mut self, owner: Pubkey) -> u8 {
        let id = (self.last_validator_id + 1) as u8;
        self.validators[id as usize].id = id;
        self.validators[id as usize].owner = owner;
        self.last_validator_id = id as i8;
        id
    }

    pub fn create_miner(&mut self, owner: Pubkey) -> u8 {
        let id = (self.last_miner_id + 1) as u8;
        self.miners[id as usize].id = id;
        self.miners[id as usize].owner = owner;
        self.last_miner_id = id as i8;
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

    pub fn miner_remove_stake(&mut self, miner_id: u8, amount: u64) -> () {
        for i in 0..MAX_MINER_NUMBER {
            if self.miners[i].id == miner_id {
                let stake = self.miners[i].stake;
                msg!("{} {}" , stake, amount);
                self.miners[i].stake -= amount;
                break;
            }
        }
        self.miner_total_stake -= amount;
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

    pub fn validator_remove_stake(&mut self, validator_id: u8, amount: u64) -> () {
        for i in 0..MAX_VALIDATOR_NUMBER {
            if self.validators[i].id == validator_id {
                self.validators[i].stake -= amount;
                break;
            }
        }
        self.validator_total_stake -= amount;
    }
}

#[zero_copy(unsafe)]
#[repr(packed)]
#[derive(Default, Debug)]
pub struct ValidatorInfo {
    pub id: u8,
    pub owner: Pubkey,
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
