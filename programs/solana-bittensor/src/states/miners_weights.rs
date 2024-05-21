use super::MAX_GROUP_MINER_NUMBER;
use super::MAX_VALIDATOR_NUMBER;
use anchor_lang::prelude::*;

pub const MAX_WEIGHT: u16 = 1000;

#[account(zero_copy(unsafe))]
#[repr(packed)]
pub struct MinerWeights {
    // 8 + 1 + 32 + (2 * 32 + 2) * 1000 = 66041
    // 363次
    pub miners_weights: [[u16; MAX_VALIDATOR_NUMBER]; 1000],
    pub miner_total_weights: [u64; 1000],
    pub validator_status: [bool; MAX_VALIDATOR_NUMBER],
    pub last_calculate_id: u32,
    pub last_reward_id: u32,
    pub end_step: u8,
}

impl MinerWeights {
    // pub fn set_weight(&mut self, miner_id: usize, validator_id: usize, weight: u16) -> () {
    //     self.miners_weights[miner_id as usize].data[validator_id as usize] = weight;
    // }

    pub fn set_weights(&mut self, validator_id: u8, weights: &Vec<u16>) -> () {
        // 将 Vec<u64> 转换为 [u64; 100]
        for (i, &weight) in weights.iter().enumerate() {
            self.miners_weights[i][validator_id as usize] = weight;
        }
    }

    pub fn remove_weights(&mut self, validator_id: u8) -> () {
        for i in 0..MAX_GROUP_MINER_NUMBER {
            self.miners_weights[i][validator_id as usize] = 0;
        }
    }

    pub fn end_epoch(&mut self) -> () {
        // self.miner_total_weights = [0; MAX_GROUP_MINER_NUMBER];
        // self.miners_weights = [[0; MAX_VALIDATOR_NUMBER]; MAX_GROUP_MINER_NUMBER];
        // self.validator_status = [false; MAX_VALIDATOR_NUMBER];
        // self.end_step = 0;
    }
}

// #[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
// pub struct Weights {
//     pub data: [u16; MAX_VALIDATOR_NUMBER],
// }
