use crate::states::*;
use anchor_lang::prelude::*;

// 一个周期发放的奖励总数
pub const REWARD_PER_EPOCH: u64 = 1 * 1_000_000_000;

pub fn end_epoch(ctx: Context<EndEpoch>) -> Result<()> {
    // 向子网分发奖励
    let bittensor_state = &mut ctx.accounts.bittensor_state.load_mut()?;

    let bittensor_epoch = &mut ctx.accounts.bittensor_epoch.load_mut()?;

    let mut subnet_weights = Box::new([0u64; MAX_SUBNET_NUMBER]);

    for i in 0..MAX_VALIDATOR_NUMBER {
        for j in 0..MAX_SUBNET_NUMBER {
            subnet_weights[j as usize] += (bittensor_epoch.weights[i][j] as u128)
                .checked_mul(bittensor_state.validators[i].stake as u128)
                .unwrap() as u64;
        }
    }

    let total_weight = subnet_weights.iter().sum::<u64>();
    let mut rewards = Box::new([0u64; MAX_SUBNET_NUMBER]);

    for i in 0..MAX_SUBNET_NUMBER {
        let reward = (REWARD_PER_EPOCH as u128)
            .checked_mul(subnet_weights[i] as u128)
            .unwrap()
            .checked_div(total_weight as u128)
            .unwrap_or(0) as u64;

        rewards[i as usize] = reward;

        bittensor_state.reward_subnet(i as u8, reward, subnet_weights[i])
    }

    // 如果主网验证人的保护期大于0，则减1
    bittensor_state.validators.iter_mut().for_each(|v| {
        if v.protection > 0 {
            v.protection -= 1;
        }
    });

    let timestamp = Clock::get()?.unix_timestamp;

    bittensor_epoch.initialize_epoch(timestamp);

    emit!(BittensorEpochEndEvent {
        epoch_number: bittensor_epoch.epoch_number,
        epoch_start_timestamp: bittensor_epoch.epoch_start_timestamp,
        weights: bittensor_epoch.weights,
        rewards: *rewards,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct EndEpoch<'info> {
    #[account(
        mut,
        seeds = [b"bittensor"],
        bump,
    )]
    pub bittensor_state: AccountLoader<'info, BittensorState>,

    #[account(
        mut,
        seeds = [b"bittensor_epoch", bittensor_state.key().as_ref()],
        bump,
    )]
    pub bittensor_epoch: AccountLoader<'info, BittensorEpochState>,

    pub system_program: Program<'info, System>,
}
