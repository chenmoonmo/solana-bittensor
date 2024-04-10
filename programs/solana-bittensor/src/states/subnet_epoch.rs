use super::{MAX_MINER_NUMBER, MAX_VALIDATOR_NUMBER};
use anchor_lang::prelude::*;

pub const MAX_MINER_WEIGHT: u64 = 1000;

#[account(zero_copy(unsafe))]
#[repr(packed)]
#[derive(Debug)]
pub struct SubnetEpochState {
    pub epoch_start_timestamp: i64,
    pub miners_weights: [[u64; MAX_MINER_NUMBER]; MAX_VALIDATOR_NUMBER],
}

impl Default for SubnetEpochState {
    #[inline]
    fn default() -> SubnetEpochState {
        SubnetEpochState {
            epoch_start_timestamp: 0,
            miners_weights: [[0; MAX_MINER_NUMBER]; MAX_VALIDATOR_NUMBER],
        }
    }
}

impl SubnetEpochState {
    pub fn set_miner_weights(&mut self, validator_id: u8, weights: Vec<u64>) -> () {
        // 将 Vec<u64> 转换为 [u64; MAX_MINER_NUMBER]
        let mut weights_array = [0; MAX_MINER_NUMBER];
        for (i, weight) in weights.into_iter().enumerate() {
            weights_array[i] = weight;
        }

        self.miners_weights[validator_id as usize] = weights_array;
    }
}
