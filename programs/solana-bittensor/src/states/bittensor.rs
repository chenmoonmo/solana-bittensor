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
    pub last_validator_id: i8,
    pub last_subnet_id: i8,
    pub subnets: [SubnetInfo; SUBNET_MAX_NUMBER],
    pub validators: [BittensorValidatorInfo; BITTENSOR_VALIDATOR_MAX_NUMBER],
}

impl BittensorState {
    pub fn initialize(&mut self, epoch_start_timestamp: i64) -> () {
        self.epoch_start_timestamp = epoch_start_timestamp;
        self.total_stake = 0;
        self.last_validator_id = -1;
        self.last_subnet_id = -1;
        self.subnets = [SubnetInfo::default(); SUBNET_MAX_NUMBER];
        self.validators = [BittensorValidatorInfo::default(); BITTENSOR_VALIDATOR_MAX_NUMBER];
    }
    pub fn update_epoch_start_timestamp(&mut self, timestamp: i64) -> () {
        self.epoch_start_timestamp = timestamp;
    }

    pub fn create_subnet(&mut self, owner: Pubkey) -> u8 {
        let id = (self.last_subnet_id + 1) as u8;

        self.subnets[id as usize].id = id;
        self.subnets[id as usize].owner = owner;
        self.subnets[id as usize].distribute_reward = 0;
        self.subnets[id as usize].stake = 0;

        self.last_subnet_id += 1;

        id
    }

    pub fn create_bittensor_validator(
        &mut self,
        owner: Pubkey,
        subnet_id: u8,
        validator_id: u8,
    ) -> u8 {
        let id = (self.last_validator_id + 1) as u8;
        self.validators[id as usize] = BittensorValidatorInfo {
            id,
            validator_id,
            subnet_id,
            stake: 0,
            owner,
        };
        self.last_validator_id += 1;
        id
    }

    pub fn validator_add_stake(&mut self, validator_id: u8, subnet_id: u8, amount: u64) -> () {
        for validator in self.validators.iter_mut() {
            if validator.validator_id == validator_id && validator.subnet_id == subnet_id {
                validator.stake += amount;
                break;
            }
        }
    }

    pub fn reward_subnet(&mut self, subnet_id: u8, reward: u64) -> () {
        self.subnets[subnet_id as usize].distribute_reward += reward;
    }
}

#[zero_copy(unsafe)]
#[repr(packed)]
#[derive(Default, Debug)]
pub struct SubnetInfo {
    pub id: u8,
    pub distribute_reward: u64,
    pub stake: u64,
    pub owner: Pubkey,
}

#[zero_copy(unsafe)]
#[repr(packed)]
#[derive(Default, Debug)]
pub struct BittensorValidatorInfo {
    pub id: u8,
    pub validator_id: u8,
    pub subnet_id: u8,
    // 质押数量
    pub stake: u64,
    pub owner: Pubkey,
}

impl BittensorValidatorInfo {
    pub const LEN: usize = 1 + 8 + 32;
}
