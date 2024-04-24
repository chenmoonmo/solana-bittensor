use anchor_lang::prelude::*;

pub const MAX_VALIDATOR_NUMBER: usize = 32;
pub const MAX_MINER_NUMBER: usize = 108;
pub const SUBNET_EPOCH_DURATION: i64 = 60 * 60 * 24;
// 保护期
pub const MINER_PROTECTION: u8 = 1;
pub const VALIDATOR_PROTECTION: u64 = 1;

#[account]
#[derive(Default, Debug)]
pub struct SubnetState {
    pub id: u8,
    pub owner: Pubkey,
    pub epoch: Pubkey,
    pub miners: Pubkey,
    pub validators: Pubkey,
    pub stake: u64,
    pub distribute_reward: u64,
}

impl SubnetState {
    pub const LEN: usize = 1 + 32 + 32 + 32 + 32 + 8 + 8; // 1 + 32 + 32 + 32 + 32 + 8 + 8 = 137
    pub fn register(
        &mut self,
        owner: Pubkey,
        epoch: Pubkey,
        miners: Pubkey,
        validators: Pubkey,
    ) -> () {
        self.owner = owner;
        self.epoch = epoch;
        self.miners = miners;
        self.validators = validators;
    }

    pub fn initialize(&mut self, id: u8) -> () {
        self.id = id;
    }
}
