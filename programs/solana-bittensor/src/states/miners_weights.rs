use super::MAX_GROUP_MINER_NUMBER;
use super::MAX_VALIDATOR_NUMBER;
use anchor_lang::prelude::*;

pub const MAX_WEIGHT: u16 = 1000;

#[account(zero_copy(unsafe))]
#[repr(packed)]
#[derive(Debug)]
pub struct MinerWeights {
    pub miner_group_id: u8,
    pub miners_weights: [[u16; MAX_VALIDATOR_NUMBER]; MAX_GROUP_MINER_NUMBER],
    pub miner_total_weights: [u64; MAX_GROUP_MINER_NUMBER],
    pub is_end: bool,
}

impl Default for MinerWeights {
    #[inline]
    fn default() -> Self {
        MinerWeights {
            miner_group_id: 0,
            miners_weights: [[0; MAX_VALIDATOR_NUMBER]; MAX_GROUP_MINER_NUMBER],
            miner_total_weights: [0; MAX_GROUP_MINER_NUMBER],
            is_end: false,
        }
    }
}

impl MinerWeights {
    pub const LEN: usize =
        1 + 1 + 2 * MAX_GROUP_MINER_NUMBER * MAX_VALIDATOR_NUMBER + 8 * MAX_GROUP_MINER_NUMBER; // 2 * 100 * 32 = 6400

    pub fn set_weights(&mut self, validator_id: u8, weights: &Vec<u16>) -> () {
        // 将 Vec<u64> 转换为 [u64; 100]
        for (i, &weight) in weights.iter().enumerate().take(MAX_GROUP_MINER_NUMBER) {
            self.miners_weights[i][validator_id as usize] = weight;
        }
    }

    pub fn remove_weights(&mut self, validator_id: u8) -> () {
        for i in 0..MAX_GROUP_MINER_NUMBER {
            self.miners_weights[i][validator_id as usize] = 0;
        }
    }
}
