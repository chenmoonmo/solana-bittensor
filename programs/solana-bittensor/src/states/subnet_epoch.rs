use anchor_lang::prelude::*;

use super::{MAX_MINER_NUMBER, MAX_VALIDATOR_NUMBER};

#[account(zero_copy(unsafe))]
#[repr(packed)]
#[derive(Debug)]
pub struct SubnetEpochState {
    pub epoch_start_timestamp: i64,
    pub validator_weights: [ValidatorWeightInfo; MAX_VALIDATOR_NUMBER],
}

impl SubnetEpochState {
    pub fn initialize(&mut self, epoch_start_timestamp: i64) -> () {
        self.validator_weights = [ValidatorWeightInfo::new(); MAX_VALIDATOR_NUMBER];
        self.epoch_start_timestamp = epoch_start_timestamp;
    }

    pub fn set_miner_weights(
        &mut self,
        validator_id: u8,
        miner_ids: Vec<u64>,
        weights: Vec<u64>,
    ) -> () {
        for validator_info in self.validator_weights.iter_mut() {
            if validator_info.validator_id == 0 {
                validator_info.validator_id = validator_id;
                for i in 0..miner_ids.len() {
                    validator_info.weights[i].miner_id = miner_ids[i] as u8;
                    validator_info.weights[i].weight = weights[i];
                }
            }
            break;
        }
    }
}

#[zero_copy(unsafe)]
#[repr(packed)]
#[derive(Debug)]
pub struct ValidatorWeightInfo {
    pub validator_id: u8,
    pub weights: [MinerWeightInfo; MAX_MINER_NUMBER],
}

impl ValidatorWeightInfo {
    pub fn new() -> Self {
        ValidatorWeightInfo {
            validator_id: 0,
            weights: [MinerWeightInfo::default(); MAX_MINER_NUMBER],
        }
    }
}

#[zero_copy(unsafe)]
#[repr(packed)]
#[derive(Default, Debug)]
pub struct MinerWeightInfo {
    pub miner_id: u8,
    pub weight: u64,
}

impl MinerWeightInfo {
    pub const LEN: usize = 1 + 1 + 8;
}
