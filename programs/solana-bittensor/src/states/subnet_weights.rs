use anchor_lang::prelude::*;

use crate::states::subnet::MAX_MINER_NUMBER;
use crate::states::subnet::MAX_VALIDATOR_NUMBER;

#[account(zero_copy(unsafe))]
#[repr(packed)]
#[derive(Default, Debug)]
pub struct SubnetWeightsState {
    pub subnet_weights: [SubnetWeightInfo; MAX_VALIDATOR_NUMBER],
    pub miners_weights: [MinerWeightInfo; MAX_MINER_NUMBER],
}

#[zero_copy(unsafe)]
#[repr(packed)]
#[derive(Default, Debug)]
pub struct SubnetWeightInfo {
    // default to false
    pub is_initialized: bool,
    pub validator_id: u8,
    pub weight: u64,
}

impl SubnetWeightInfo {
    pub const LEN: usize = 8 + 1 + 8;
}


#[zero_copy(unsafe)]
#[repr(packed)]
#[derive(Default, Debug)]
pub struct MinerWeightInfo {
    // default to false
    pub is_initialized: bool,
    pub validator_id: u8,
    pub miner_id: u8,
    pub weight: u64,
}

impl MinerWeightInfo {
    pub const LEN: usize = 8 + 1 + 1 + 8;
}
