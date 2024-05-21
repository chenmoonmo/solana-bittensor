use super::{MAX_MINER_NUMBER, MINER_PROTECTION};
use anchor_lang::prelude::*;

#[account(zero_copy(unsafe))]
#[repr(packed)]
pub struct SubnetMiners {
    //  85 * MAX_MINER_NUMBER = 4 + 85 * 10000 = 850004
    pub last_miner_id: i32,
    pub miners: [MinerInfo; MAX_MINER_NUMBER],
}

impl Default for SubnetMiners {
    #[inline]
    fn default() -> Self {
        Self {
            last_miner_id: -1,
            miners: [MinerInfo::default(); MAX_MINER_NUMBER],
        }
    }
}

impl SubnetMiners {
    pub fn create_miner(&mut self, owner: Pubkey, pubkey: Pubkey) -> u32 {
        let id: u32 = self.last_miner_id as u32 + 1;

        self.miners[id as usize].id = id;
        self.miners[id as usize].pubkey = pubkey;
        self.miners[id as usize].owner = owner;
        self.miners[id as usize].protection = MINER_PROTECTION;

        self.last_miner_id = id as i32;

        id
    }
}

#[zero_copy(unsafe)]
#[repr(packed)]
pub struct MinerInfo {
    pub id: u32,
    pub owner: Pubkey,
    pub pubkey: Pubkey,
    // 待提取奖励
    pub reward: u64,
    // 上一个周期的权重
    pub last_weight: u64,
    // 保护期
    pub protection: u8,
}

impl Default for MinerInfo {
    #[inline]
    fn default() -> Self {
        Self {
            id: 0,
            owner: Pubkey::default(),
            pubkey: Pubkey::default(),
            protection: MINER_PROTECTION,
            reward: 0,
            last_weight: 0,
        }
    }
}

impl MinerInfo {
    pub const LEN: usize = 4 + 32 + 32 + 8 + 8 + 1; // = 85
}
