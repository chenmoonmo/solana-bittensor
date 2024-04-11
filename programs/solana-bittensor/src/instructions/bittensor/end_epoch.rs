use crate::states::*;
use anchor_lang::prelude::*;

// 一个周期发放的奖励总数
pub const REWARD_PER_EPOCH: u64 = 1000 * 1_000_000_000;

pub fn end_epoch(ctx: Context<EndEpoch>) -> Result<()> {
    // 向子网分发奖励
    let validators = ctx.accounts.bittensor_state.load()?.validators;
    let subnets = &mut ctx.accounts.bittensor_state.load_mut()?.subnets;

    let weights = ctx.accounts.bittensor_epoch.load_mut()?.weights;

    let mut subnet_weights = [0; SUBNET_MAX_NUMBER];

    for i in 0..MAX_VALIDATOR_NUMBER {
        let validator = &validators[i];

        for j in 0..SUBNET_MAX_NUMBER {
            subnet_weights[j] += weights[i][j] * validator.stake;
        }
    }

    let total_weight = subnet_weights.iter().sum::<u64>();

    for i in 0..SUBNET_MAX_NUMBER {
        let subnet = &mut subnets[i];

        if subnet.owner != Pubkey::default() {
            let reward = REWARD_PER_EPOCH * subnet_weights[i] / total_weight;

            subnet.distribute_reward += reward;
        }
    }

    // TODO 重置 bittensor_epoch

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
