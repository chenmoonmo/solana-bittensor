use anchor_lang::prelude::*;

pub const MAX_SUBNET_NUMBER: usize = 32;
pub const BITTENSOR_VALIDATOR_MAX_NUMBER: usize = 32;
pub const MAX_EPOCH_NUMBER: usize = 10;
pub const EPOCH_DURATION: i64 = 60 * 60 * 1;

#[account(zero_copy(unsafe))]
#[repr(packed)]
#[derive(Default, Debug)]
pub struct BittensorState {
    pub total_stake: u64,
    pub last_validator_id: i8,
    pub last_subnet_id: i8,
    pub subnets: [SubnetInfo; MAX_SUBNET_NUMBER],
    pub validators: [BittensorValidatorInfo; BITTENSOR_VALIDATOR_MAX_NUMBER],
}

impl BittensorState {
    pub const LEN: usize = 8
        + 8
        + 1
        + 1
        + SubnetInfo::LEN * MAX_SUBNET_NUMBER
        + BittensorValidatorInfo::LEN * BITTENSOR_VALIDATOR_MAX_NUMBER; // 8 + 8 + 1 + 1 + 32 * 89 + 32 * 91 = 8224

    pub fn initialize(&mut self) -> () {
        self.total_stake = 0;
        self.last_validator_id = -1;
        self.last_subnet_id = -1;
        self.subnets = [SubnetInfo::default(); MAX_SUBNET_NUMBER];
        self.validators = [BittensorValidatorInfo::default(); BITTENSOR_VALIDATOR_MAX_NUMBER];
    }

    pub fn create_subnet(&mut self, owner: Pubkey, subnet_state: Pubkey) -> u8 {
        let id = (self.last_subnet_id + 1) as u8;
        self.subnets[id as usize].id = id;
        self.subnets[id as usize].owner = owner;
        self.subnets[id as usize].distribute_reward = 0;
        self.subnets[id as usize].stake = 0;
        self.subnets[id as usize].protection = 1;
        self.subnets[id as usize].subnet_state = subnet_state;
        self.last_subnet_id += 1;
        id
    }

    pub fn create_bittensor_validator(
        &mut self,
        owner: Pubkey,
        validator_state: Pubkey,
        subnet_id: u8,
        validator_id: u8,
        stake: u64,
        bounds: u64,
    ) -> u8 {
        let id = (self.last_validator_id + 1) as u8;
        self.validators[id as usize] = BittensorValidatorInfo {
            id,
            validator_id,
            subnet_id,
            stake,
            owner,
            bounds,
            protection: 1,
            validator_state,
        };
        self.last_validator_id += 1;
        id
    }

    pub fn validator_add_stake(&mut self, validator_state: Pubkey, amount: u64) -> () {
        if let Some(validator) = self
            .validators
            .iter_mut()
            .find(|v| v.validator_state == validator_state)
        {
            validator.stake += amount;
        }
    }

    pub fn validator_remove_stake(&mut self, validator_state: Pubkey, amount: u64) -> () {
        if let Some(validator) = self
            .validators
            .iter_mut()
            .find(|v| v.validator_state == validator_state)
        {
            validator.stake -= amount;
        }
    }

    pub fn reward_subnet(&mut self, subnet_id: u8, reward: u64, weight: u64) -> () {
        self.subnets[subnet_id as usize].distribute_reward += reward;
        self.subnets[subnet_id as usize].last_weight = weight;
        if self.subnets[subnet_id as usize].protection > 0 {
            self.subnets[subnet_id as usize].protection -= 1;
        }
    }
}

#[zero_copy(unsafe)]
#[repr(packed)]
#[derive(Default, Debug)]
pub struct SubnetInfo {
    pub id: u8,
    pub distribute_reward: u64,
    pub last_weight: u64,
    pub stake: u64,
    pub owner: Pubkey,
    pub subnet_state: Pubkey,
    pub protection: u64,
}

impl SubnetInfo {
    pub const LEN: usize = 1 + 8 + 8 + 8 + 32 + 32 + 8; // 1 + 8 + 8 + 8 + 32 + 32 + 8  = 89
}

#[zero_copy(unsafe)]
#[repr(packed)]
#[derive(Default, Debug)]
pub struct BittensorValidatorInfo {
    pub id: u8,
    pub owner: Pubkey,
    pub validator_state: Pubkey,
    pub validator_id: u8,
    pub subnet_id: u8,
    // 质押数量
    pub stake: u64,
    //工作量
    pub bounds: u64,
    // 保护期
    pub protection: u64,
}

impl BittensorValidatorInfo {
    pub const LEN: usize = 1 + 1 + 1 + 8 + 8 + 32 + 8 + 32; // 1 + 1 + 1 + 8 + 8 + 32 + 8 + 32 = 91
}

// event when new validator is added
#[event]
#[cfg_attr(feature = "client", derive(Debug))]
pub struct RegisterBittensorValidatorEvent {
    pub pre_pubkey: Pubkey,
    pub validator_state: Pubkey,
    pub owner: Pubkey,
    pub id: u8,
    pub validator_id: u8,
    pub subnet_id: u8,
    pub stake: u64,
    pub bounds: u64,
}

// event when new subnet is added
#[event]
#[cfg_attr(feature = "client", derive(Debug))]
pub struct RegisterBittensorSubnetEvent {
    pub pre_pubkey: Pubkey,
    pub subnet_state: Pubkey,
    pub owner: Pubkey,
    pub id: u8,
}
