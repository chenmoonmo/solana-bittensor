use anchor_lang::prelude::*;

use crate::states::subnet::MAX_VALIDATOR_NUMBER;
use crate::states::subnet_weights::MinerWeightInfo;

#[account(zero_copy(unsafe))]
#[repr(packed)]
#[derive(Default, Debug)]
pub struct MinerState {
    pub id: u8,
    pub owner: Pubkey,
    pub stake: u64,
    // pub rpc_url: String,
    // 矿工的得分 [[验证者ID, 得分], [验证者ID, 得分]...]
    pub weights: [MinerWeightInfo; MAX_VALIDATOR_NUMBER],
}

impl MinerState {
    pub fn set_weight(&mut self, validator_id: u8, weight: u64) -> () {
        // 如果已经存在 validator_id 的打分，则报错
        for i in 0..MAX_VALIDATOR_NUMBER {
            if self.weights[i].validator_id == validator_id {
                panic!("Validator {} has already been scored", validator_id);
            }
        }

        for i in 0..MAX_VALIDATOR_NUMBER {
            if self.weights[i].validator_id == 0 {
                self.weights[i].validator_id = validator_id;
                self.weights[i].miner_id = self.id;
                self.weights[i].weight = weight;
                break;
            }
        }
    }

    pub fn add_stake(&mut self, amount: u64) -> () {
        self.stake += amount;
    }
}
