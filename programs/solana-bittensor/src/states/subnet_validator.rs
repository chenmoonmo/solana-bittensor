use super::{MAX_VALIDATOR_NUMBER, VALIDATOR_PROTECTION};
use anchor_lang::prelude::*;

#[account(zero_copy(unsafe))]
#[repr(packed)]
#[derive(Debug)]
pub struct SubnetValidators {
    pub last_validator_id: i8,
    pub validators: [ValidatorInfo; MAX_VALIDATOR_NUMBER],
}
// 10240 - 1 - 1 = x * 89

impl Default for SubnetValidators {
    #[inline]
    fn default() -> Self {
        Self {
            last_validator_id: -1,
            validators: [ValidatorInfo::default(); MAX_VALIDATOR_NUMBER],
        }
    }
}

impl SubnetValidators {
    pub const LEN: usize = 1 + 1 + MAX_VALIDATOR_NUMBER * ValidatorInfo::LEN; // 3170
    pub fn create_validator(&mut self, owner: Pubkey, pubkey: Pubkey, stake: u64) -> u8 {
        let id = (self.last_validator_id + 1) as u8;
        self.validators[id as usize].id = id;
        self.validators[id as usize].owner = owner;
        self.validators[id as usize].stake = stake;
        self.validators[id as usize].pubkey = pubkey;
        self.last_validator_id = id as i8;
        id
    }

    pub fn validator_add_stake(&mut self, validator_id: u8, amount: u64) -> () {
        self.validators[validator_id as usize].stake += amount;
    }

    pub fn validator_remove_stake(&mut self, validator_id: u8, amount: u64) -> () {
        self.validators[validator_id as usize].stake -= amount;
    }

    pub fn get_validator_bounds(&self, validator_id: u8) -> u64 {
        self.validators[validator_id as usize].bounds
    }

    pub fn get_min_stake(&self) -> u64 {
        if self.last_validator_id >= i8::try_from(MAX_VALIDATOR_NUMBER - 1).unwrap() {
            let mut stakes = self.validators.map(|v| v.stake);
            stakes.sort_unstable();
            return stakes[10];
        }
        return 0;
    }

    pub fn end_epoch(&mut self) -> () {
        for i in 0..=self.last_validator_id as usize {
            self.validators[i].reset_used_weights();
        }
    }
}
#[zero_copy(unsafe)]
#[repr(packed)]
#[derive(Debug)]
pub struct ValidatorInfo {
    pub id: u8,
    pub used_weights: u16,
    pub owner: Pubkey,
    pub pubkey: Pubkey,
    // 质押数量
    pub stake: u64,
    // 上一个周期的工作量
    pub bounds: u64,
    // 待提取奖励
    pub reward: u64,
    // 保护期
    pub protection: u64,
}

impl Default for ValidatorInfo {
    #[inline]
    fn default() -> Self {
        Self {
            id: 0,
            owner: Pubkey::default(),
            pubkey: Pubkey::default(),
            stake: 0,
            bounds: 0,
            reward: 0,
            used_weights: 0, // max 1000
            protection: VALIDATOR_PROTECTION,
        }
    }
}

impl ValidatorInfo {
    pub const LEN: usize = 1 + 2 + 32 + 32 + 8 + 8 + 8 + 8; // 99

    pub fn reset_used_weights(&mut self) -> () {
        self.used_weights = 0;
    }
}
