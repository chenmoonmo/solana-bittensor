use super::{MAX_MINER_NUMBER, MINER_PROTECTION};
use anchor_lang::prelude::*;

#[account(zero_copy(unsafe))]
#[repr(packed)]
#[derive(Debug)]
pub struct SubnetMiners {
    pub id: u8,
    pub group_id: u8,
    pub last_miner_id: i8,
    pub miners: [MinerInfo; 100],
}

impl Default for SubnetMiners {
    #[inline]
    fn default() -> Self {
        Self {
            id: 0,
            group_id: 0,
            last_miner_id: -1,
            miners: [MinerInfo::default(); 100],
        }
    }
}

impl SubnetMiners {
    pub const LEN: usize = 1 + 1 + 1 + MAX_MINER_NUMBER * MinerInfo::LEN; // 1 + 1 + 32 * 89 = 2849 10240kb

    pub fn create_miner(&mut self, owner: Pubkey, pubkey: Pubkey) -> u8 {
        let id = (self.last_miner_id + 1) as u8;
        self.miners[id as usize].id = id;
        self.miners[id as usize].owner = owner;
        self.miners[id as usize].pubkey = pubkey;
        self.last_miner_id = id as i8;
        id
    }

    pub fn miner_add_stake(&mut self, miner_id: u8, amount: u64) -> () {
        self.miners[miner_id as usize].stake += amount;
    }

    pub fn miner_remove_stake(&mut self, miner_id: u8, amount: u64) -> () {
        self.miners[miner_id as usize].stake -= amount;
    }
}

#[zero_copy(unsafe)]
#[repr(packed)]
#[derive(Debug)]
pub struct MinerInfo {
    pub id: u8,
    pub owner: Pubkey,
    pub pubkey: Pubkey,
    pub stake: u64,
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
            stake: 0,
            reward: 0,
            last_weight: 0,
        }
    }
}

impl MinerInfo {
    pub const LEN: usize = 1 + 32 + 32 + 1 + 8 + 8 + 8; // 89
}
