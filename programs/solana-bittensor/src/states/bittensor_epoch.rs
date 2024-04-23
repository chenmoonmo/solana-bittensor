use anchor_lang::prelude::*;

use super::{BITTENSOR_VALIDATOR_MAX_NUMBER, MAX_SUBNET_NUMBER};

#[account(zero_copy(unsafe))]
#[repr(packed)]
#[derive(Default, Debug)]
pub struct BittensorEpochState {
    pub epoch_number: u64,
    pub epoch_start_timestamp: i64,
    pub weights: [[u64; MAX_SUBNET_NUMBER]; BITTENSOR_VALIDATOR_MAX_NUMBER],
}

impl BittensorEpochState {
    pub fn set_weights(&mut self, validator_id: u8, weights: Vec<u64>) -> () {
        let mut new_weights = [0u64; MAX_SUBNET_NUMBER];
        for i in 0..weights.len() {
            new_weights[i] = weights[i];
        }
        self.weights[validator_id as usize] = new_weights;
    }

    pub fn remove_weights(&mut self, validator_id: u8) -> () {
        self.weights[validator_id as usize] = [0; MAX_SUBNET_NUMBER];
    }

    pub fn remove_subnet_weights(&mut self, subnet_id: u8) -> () {
        for i in 0..BITTENSOR_VALIDATOR_MAX_NUMBER {
            self.weights[i][subnet_id as usize] = 0;
        }
    }

    pub fn initialize_epoch(&mut self, epoch_start_timestamp: i64) -> () {
        self.epoch_start_timestamp = epoch_start_timestamp;
        self.weights = [[0; MAX_SUBNET_NUMBER]; BITTENSOR_VALIDATOR_MAX_NUMBER];
        self.epoch_number += 1;
    }
}
