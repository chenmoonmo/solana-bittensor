use crate::states::*;
use anchor_lang::prelude::*;

// 一个周期发放的奖励总数
pub const REWARD_PER_EPOCH: u64 = 1000 * 1_000_000_000;

pub fn end_epoch(ctx: Context<EndEpoch>) -> Result<()> {
    // 向子网分发奖励
    let validators = ctx.accounts.bittensor_state.load()?.validators;

    let subnets = &mut ctx.accounts.bittensor_state.load_mut()?.subnets;

    for subnet in subnets.iter_mut() {
        let weight = subnet.calculate_weight(validators);
        let reward_amount = REWARD_PER_EPOCH
            .checked_div(weight as u64)
            .unwrap()
            .checked_mul(100)
            .unwrap();

        subnet.distribute_reward += reward_amount;
        subnet.initialize_weights();
    }

    Ok(())
}

#[derive(Accounts)]
pub struct EndEpoch<'info> {
    #[account(mut)]
    pub bittensor_state: AccountLoader<'info, BittensorState>,

    pub system_program: Program<'info, System>,
}
