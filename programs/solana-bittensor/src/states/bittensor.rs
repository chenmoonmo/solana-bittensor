use anchor_lang::prelude::*;

pub const SUBNET_MAX_NUMBER: usize = 32;
pub const BITTENSOR_VALIDATOR_MAX_NUMBER: usize = 32;
pub const MAX_EPOCH_NUMBER: usize = 10;

pub const EPOCH_DURATION: i64 = 60 * 60 * 1;

#[account(zero_copy(unsafe))]
#[repr(packed)]
#[derive(Default, Debug)]
pub struct BittensorState {
    pub epoch_start_timestamp: i64,
    pub total_stake: u64,
    pub subnets: [SubnetInfo; SUBNET_MAX_NUMBER],
    pub validators: [BittensorValidatorInfo; BITTENSOR_VALIDATOR_MAX_NUMBER],
}

impl BittensorState {
    pub const LEN: usize = 8
        + 8
        + SubnetInfo::LEN * SUBNET_MAX_NUMBER
        + BittensorValidatorInfo::LEN * BITTENSOR_VALIDATOR_MAX_NUMBER;

    pub fn new() -> Self {
        BittensorState {
            epoch_start_timestamp: 0,
            total_stake: 0,
            subnets: [SubnetInfo::default(); SUBNET_MAX_NUMBER],
            validators: [BittensorValidatorInfo::default(); BITTENSOR_VALIDATOR_MAX_NUMBER],
        }
    }
    pub fn update_epoch_start_timestamp(&mut self, timestamp: i64) -> () {
        self.epoch_start_timestamp = timestamp;
    }

    pub fn create_subnet(&mut self, owner: Pubkey) -> () {
        for i in 0..SUBNET_MAX_NUMBER {
            if self.subnets[i].id == 0 {
                self.subnets[i].id = i as u8 + 1;
                self.subnets[i].distribute_reward = 0;
                self.subnets[i].stake = 0;
                self.subnets[i].owner = owner;
            }
        }
    }

    pub fn create_bittensor_validator(&mut self, owner: Pubkey) -> () {
        for i in 0..BITTENSOR_VALIDATOR_MAX_NUMBER {
            if self.validators[i].id == 0 {
                self.validators[i].id = i as u8 + 1;
                self.validators[i].stake = 0;
                self.validators[i].owner = owner;
            }
        }
    }
}

#[zero_copy(unsafe)]
#[repr(packed)]
#[derive(Default, Debug)]
pub struct WeightInfo {
    pub validator_id: u8,
    pub weight: u8,
}

impl WeightInfo {
    pub const LEN: usize = 1 + 1;
}

#[zero_copy(unsafe)]
#[repr(packed)]
#[derive(Default, Debug)]
pub struct SubnetInfo {
    pub id: u8,
    pub distribute_reward: u64,
    pub stake: u64,
    pub owner: Pubkey,
    // 仅保存本周期内的打分
    pub weights: [WeightInfo; BITTENSOR_VALIDATOR_MAX_NUMBER],
}

impl SubnetInfo {
    pub const LEN: usize = 1 + 8 + 8 + 32 + WeightInfo::LEN * BITTENSOR_VALIDATOR_MAX_NUMBER;
    // 计算权重
    pub fn calculate_weight(
        &self,
        validators: [BittensorValidatorInfo; BITTENSOR_VALIDATOR_MAX_NUMBER],
    ) -> u8 {
        let mut total_stake = 0u64;

        for i in 0..SUBNET_MAX_NUMBER {
            let validator = validators
                .iter()
                .find(|v| v.id == self.weights[i].validator_id);
            if let Some(validator) = validator {
                total_stake += validator.stake;
            }
        }

        let mut weight = 0u8;

        for i in 0..SUBNET_MAX_NUMBER {
            let validator = validators
                .iter()
                .find(|v| v.id == self.weights[i].validator_id);
            if let Some(validator) = validator {
                weight = validator
                    .stake
                    .checked_mul(100)
                    .unwrap()
                    .checked_div(total_stake)
                    .unwrap() as u8;
            }
        }

        weight
    }
}

#[zero_copy(unsafe)]
#[repr(packed)]
#[derive(Default, Debug)]
pub struct BittensorValidatorInfo {
    pub id: u8,
    // 质押数量
    pub stake: u64,
    pub owner: Pubkey,
}

impl BittensorValidatorInfo {
    pub const LEN: usize = 1 + 8 + 32;
}
