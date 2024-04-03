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
