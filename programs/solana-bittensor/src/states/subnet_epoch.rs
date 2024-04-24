use super::{MAX_MINER_NUMBER, MAX_VALIDATOR_NUMBER};
use anchor_lang::prelude::*;

pub const MAX_WEIGHT: u16 = 1000;

#[account(zero_copy(unsafe))]
#[repr(packed)]
#[derive(Debug)]
pub struct SubnetEpochState {
    pub id: u8,
    pub epoch_number: u64,
    pub epoch_start_timestamp: i64,
    pub miners_weights: [[u16; MAX_MINER_NUMBER]; MAX_VALIDATOR_NUMBER],
    pub miner_total_weights: [u64; MAX_MINER_NUMBER],
    pub validator_total_weights: [u16; MAX_VALIDATOR_NUMBER],
    pub end_step: u8,
}

impl Default for SubnetEpochState {
    #[inline]
    fn default() -> Self {
        SubnetEpochState {
            id: 0,
            epoch_number: 0,
            epoch_start_timestamp: 0,
            miners_weights: [[0; MAX_MINER_NUMBER]; MAX_VALIDATOR_NUMBER],
            miner_total_weights: [0; MAX_MINER_NUMBER],
            validator_total_weights: [0; MAX_VALIDATOR_NUMBER],
            end_step: 0,
        }
    }
}

impl SubnetEpochState {
    pub fn reset(&mut self, epoch_start_timestamp: i64) -> () {
        self.epoch_start_timestamp = epoch_start_timestamp;
        self.miners_weights = [[0; MAX_MINER_NUMBER]; MAX_VALIDATOR_NUMBER];
        self.epoch_number += 1;
    }

    pub fn end_epoch(&mut self, epoch_start_timestamp: i64) -> () {
        self.epoch_start_timestamp = epoch_start_timestamp;
        self.miners_weights = [[0; MAX_MINER_NUMBER]; MAX_VALIDATOR_NUMBER];
        self.miner_total_weights = [0; MAX_MINER_NUMBER];
        self.validator_total_weights = [0; MAX_VALIDATOR_NUMBER];
        self.end_step = 0;
        self.epoch_number += 1;
    }

    pub fn set_weights(&mut self, validator_id: u8, weights: Vec<u16>) -> () {
        // 将 Vec<u64> 转换为 [u64; MAX_MINER_NUMBER]
        let mut weights_array = [0; MAX_MINER_NUMBER];
        for (i, weight) in weights.into_iter().enumerate() {
            weights_array[i] = weight;
        }
        self.miners_weights[validator_id as usize] = weights_array;
    }

    pub fn remove_weights(&mut self, validator_id: u8) -> () {
        self.miners_weights[validator_id as usize] = [0; MAX_MINER_NUMBER];
    }

    pub fn remove_miner_weights(&mut self, miner_id: u8) -> () {
        for i in 0..MAX_VALIDATOR_NUMBER {
            self.miners_weights[i][miner_id as usize] = 0;
        }
    }
}
