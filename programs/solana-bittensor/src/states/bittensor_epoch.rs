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
    pub const LEN: usize = 8 + 8 + 8 * MAX_SUBNET_NUMBER * BITTENSOR_VALIDATOR_MAX_NUMBER; // 8 + 8 + 8 * 32 * 32 = 8224

    pub fn set_weights(&mut self, validator_id: u8, weights: &Vec<u64>) -> () {
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

#[event]
#[cfg_attr(feature = "client", derive(Debug))]
pub struct BittensorEpochEndEvent {
    pub epoch_number: u64,
    pub epoch_start_timestamp: i64,
    pub weights: [[u64; MAX_SUBNET_NUMBER]; BITTENSOR_VALIDATOR_MAX_NUMBER],
    pub rewards: [u64; MAX_SUBNET_NUMBER],
}

//event when validator set weights
#[event]
#[cfg_attr(feature = "client", derive(Debug))]
pub struct BittensorValidatorSetWeightsEvent {
    pub validator_id: u8,
    pub validator_state: Pubkey,
    pub weights: Vec<u64>,
}
