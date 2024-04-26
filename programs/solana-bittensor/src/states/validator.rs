use anchor_lang::prelude::*;
// 考虑 作为主网验证者或子网验证者

#[account]
#[derive(Default, Debug)]
pub struct ValidatorState {
    pub id: u8,
    pub owner: Pubkey,
    pub stake: u64,
}

impl ValidatorState {
    pub const LEN: usize = 1 + 32 + 8; // 1 + 32 + 8 = 41
    pub fn add_stake(&mut self, amount: u64) {
        self.stake += amount;
    }

    pub fn remove_stake(&mut self, amount: u64) {
        self.stake -= amount;
    }
}

// event when validator register
#[event]
#[cfg_attr(feature = "client", derive(Debug))]
pub struct ValidatorRegisterEvent {
    pub id: u8,
    pub owner: Pubkey,
    pub stake: u64,
    pub pubkey: Pubkey,
    pub pre_pubkey: Pubkey,
}


// event when validator add stake
#[event]
#[cfg_attr(feature = "client", derive(Debug))]
pub struct ValidatorAddStakeEvent {
    pub id: u8,
    pub owner: Pubkey,
    pub stake: u64,
    pub pubkey: Pubkey,
    pub add_amount: u64,
}

// event when validator remove stake
#[event]
#[cfg_attr(feature = "client", derive(Debug))]
pub struct ValidatorRemoveStakeEvent {
    pub id: u8,
    pub owner: Pubkey,
    pub stake: u64,
    pub pubkey: Pubkey,
    pub remove_amount: u64,
}

// event when validator claim reward
#[event]
#[cfg_attr(feature = "client", derive(Debug))]
pub struct ValidatorClaimRewardEvent {
    pub id: u8,
    pub owner: Pubkey,
    pub pubkey: Pubkey,
    pub claim_amount: u64,
}
