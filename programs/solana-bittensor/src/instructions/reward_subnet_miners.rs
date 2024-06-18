use crate::errors::ErrorCode;
use crate::states::*;
use anchor_lang::prelude::*;

pub fn reward_subnet_miners(ctx: Context<RewardSubnetMiners>) -> Result<()> {
    let miner_weights = &mut ctx.accounts.miner_weights.load_mut()?;
    let subnet_miners = &mut ctx.accounts.subnet_miners.load_mut()?;

    require!(
        ctx.accounts.subnet_state.end_step == 1,
        ErrorCode::InvalidEndStep
    );

    let start_index: usize = miner_weights.last_reward_id as usize;
    // could be 400
    let mut end_index: usize = miner_weights.last_reward_id as usize + 400;

    if end_index >= subnet_miners.last_miner_id as usize {
        end_index = subnet_miners.last_miner_id as usize;
        ctx.accounts.subnet_state.end_step = 2;
    }

    let epoch_total_weights = ctx.accounts.subnet_state.epoch_total_weights;

    for i in start_index..end_index {
        let weight = miner_weights.miner_total_weights[i];

        let reward = (weight as u128)
            .checked_mul(MINER_EPOCH_REWARD as u128)
            .unwrap()
            .checked_div(epoch_total_weights as u128)
            .unwrap_or(0) as u64;

        subnet_miners.miners[i].reward += reward;
    }

    miner_weights.last_reward_id = end_index as u32;

    Ok(())
}

#[derive(Accounts)]
pub struct RewardSubnetMiners<'info> {
    #[account(
        mut,
        seeds = [b"subnet_state"],
        bump = subnet_state.bump,
    )]
    pub subnet_state: Box<Account<'info, SubnetState>>,

    #[account(mut)]
    pub subnet_miners: AccountLoader<'info, SubnetMiners>,

    /// 验证者每次只能给一个矿工组进行打分
    #[account(mut)]
    pub miner_weights: AccountLoader<'info, MinerWeights>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
