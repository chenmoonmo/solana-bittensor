use anchor_lang::prelude::*;

use crate::states::subnet::MAX_MINER_NUMBER;
// use crate::states::subnet::MAX_VALIDATOR_NUMBER;

#[account(zero_copy(unsafe))]
#[repr(packed)]
#[derive(Default, Debug)]
pub struct SubnetWeightsState {
    // pub subnet_weights: [SubnetWeightInfo; MAX_VALIDATOR_NUMBER],
    pub miners_weights: [MinerWeightInfo; MAX_MINER_NUMBER],
}

impl SubnetWeightsState {
    // pub fn set_subnet_weight(&mut self, validator_id: u8, weight: u64) -> () {
    //     // 如果已经存在 validator_id 的打分，则报错

    //     for i in 0..MAX_VALIDATOR_NUMBER {
    //         if self.subnet_weights[i].validator_id == validator_id {
    //             // TODO: 报错
    //             panic!("Validator {} has already been scored", validator_id);
    //         }
    //     }

    //     for i in 0..MAX_VALIDATOR_NUMBER {
    //         if self.subnet_weights[i].validator_id == 0 {
    //             self.subnet_weights[i].validator_id = validator_id;
    //             self.subnet_weights[i].weight = weight;
    //             break;
    //         }
    //     }
    // }

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
