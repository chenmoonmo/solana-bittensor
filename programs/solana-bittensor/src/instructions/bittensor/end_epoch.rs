use crate::states::*;
use anchor_lang::prelude::*;

// 一个周期发放的奖励总数
pub const REWARD_PER_EPOCH: u64 = 1000 * 1_000_000_000;

pub fn end_epoch(ctx: Context<EndEpoch>) -> Result<()> {
    // 向子网分发奖励
    let bittensor_state = &mut ctx.accounts.bittensor_state.load_mut()?;
    let weights = ctx.accounts.bittensor_epoch.load()?.weights;

    let mut subnet_weights = [0u64; SUBNET_MAX_NUMBER];

    {
        let validators = &bittensor_state.validators;

        for i in 0..MAX_VALIDATOR_NUMBER {
            let validator_weights = *weights.get(i).unwrap();

            for j in 0..SUBNET_MAX_NUMBER {
                let weight = *validator_weights.get(j).unwrap();

                subnet_weights[j as usize] += (weight as u128)
                    .checked_mul(validators[i].stake as u128)
                    .unwrap() as u64;
            }
        }
    }

    let total_weight = subnet_weights.iter().sum::<u64>();

    for i in 0..SUBNET_MAX_NUMBER {
        let subnet = bittensor_state.subnets[i];

        if subnet.owner != Pubkey::default() {
            let reward = REWARD_PER_EPOCH
                .checked_mul(subnet_weights[i])
                .unwrap()
                .checked_div(total_weight)
                .unwrap_or(0);

            bittensor_state.reward_subnet(i as u8, reward);
        }
    }

    let timestamp = Clock::get()?.unix_timestamp;

    ctx.accounts
        .bittensor_epoch
        .load_mut()?
        .initialize_epoch(timestamp);

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
