use anchor_lang::prelude::*;

use crate::states::subnet::MAX_MINER_NUMBER;

use super::{ValidatorInfo, MAX_VALIDATOR_NUMBER};
// use crate::states::subnet::MAX_VALIDATOR_NUMBER;

#[account(zero_copy(unsafe))]
#[repr(packed)]
#[derive(Default, Debug)]
pub struct SubnetWeightsState {
    // pub subnet_weights: [SubnetWeightInfo; MAX_VALIDATOR_NUMBER],
    pub miners_weights: [MinerWeightInfo; MAX_MINER_NUMBER],
}

impl SubnetWeightsState {
    pub fn set_miner_weight(&mut self, miner_id: u8, validator_id: u8, weight: u64) -> () {
        // 如果已经存在 validator_id 对 miner_id 的打分，则报错
        for i in 0..MAX_MINER_NUMBER {
            if self.miners_weights[i].miner_id == miner_id
                && self.miners_weights[i].validator_id == validator_id
            {
                // TODO: 报错
                panic!(
                    "Miner {} has already been scored by Validator {}",
                    miner_id, validator_id
                );
            }
        }

        for i in 0..MAX_MINER_NUMBER {
            if self.miners_weights[i].miner_id == 0 {
                self.miners_weights[i].miner_id = miner_id;
                self.miners_weights[i].validator_id = validator_id;
                self.miners_weights[i].weight = weight;
                break;
            }
        }
    }

    pub fn get_miner_weights(
        &self,
        validators: [ValidatorInfo; MAX_VALIDATOR_NUMBER],
        validator_total_stake: u64,
    ) -> Vec<(u8, u64)> {
        // 将 miners_weights reduce 为 Vec<(u8, u64)>
        // 1. 遍历 miners_weights，如果 miner_id 不在 weights 中，则添加
        // 2. 如果 miner_id 在 weights 中，则 weight += weight
        // 2.2. weight 为 验证者的 质押数量 / 总质押数量 * weight
        let mut weights: Vec<(u8, u64)> = vec![];
        for i in 0..MAX_MINER_NUMBER {
            if self.miners_weights[i].miner_id == 0 {
                break;
            }

            let miner_id = self.miners_weights[i].miner_id;
            let validator_id = self.miners_weights[i].validator_id;
            let weight = self.miners_weights[i].weight;

            let mut found = false;
            for j in 0..weights.len() {
                if weights[j].0 == miner_id {
                    weights[j].1 +=
                        weight * validators[validator_id as usize].stake / validator_total_stake;
                    found = true;
                    break;
                }
            }

            if !found {
                weights.push((
                    miner_id,
                    weight * validators[validator_id as usize].stake / validator_total_stake,
                ));
            }
        }

        weights
    }
}

// #[zero_copy(unsafe)]
// #[repr(packed)]
// #[derive(Default, Debug)]
// pub struct SubnetWeightInfo {
//     pub validator_id: u8,
//     pub weight: u64,
// }

// impl SubnetWeightInfo {
//     pub const LEN: usize = 8 + 1 + 8;
// }

#[zero_copy(unsafe)]
#[repr(packed)]
#[derive(Default, Debug)]
pub struct MinerWeightInfo {
    pub validator_id: u8,
    pub miner_id: u8,
    pub weight: u64,
}

impl MinerWeightInfo {
    pub const LEN: usize = 8 + 1 + 1 + 8;
}
