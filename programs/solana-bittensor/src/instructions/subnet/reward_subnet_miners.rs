use crate::errors::ErrorCode;
use crate::states::*;
use anchor_lang::prelude::*;

pub fn reward_subnet_miners(ctx: Context<RewardSubnetMiners>) -> Result<()> {
    // TODO: 只有所有的矿工组的打分都被结算了，才能进行奖励

    let miner_weights = ctx.accounts.miner_weights.load_mut()?;
    let subnet_miners = &mut ctx.accounts.subnet_miners.load_mut()?;

    require!(
        miner_weights.miner_group_id == subnet_miners.group_id,
        ErrorCode::InvalidMinerGroupId
    );

    let epoch_total_weights = ctx.accounts.subnet_state.epoch_total_weights;

    for i in 0..100 {
        let weight = miner_weights.miner_total_weights[i];
        let reward = weight * ctx.accounts.subnet_state.distribute_reward / epoch_total_weights;
        subnet_miners.miners[i].reward += reward;
    }

    Ok(())
}

#[derive(Accounts)]
pub struct RewardSubnetMiners<'info> {
    #[account(mut)]
    pub subnet_state: Box<Account<'info, SubnetState>>,

    #[account(
        mut,
        seeds = [b"validator_state",subnet_state.key().as_ref(),owner.key().as_ref()],
        bump
    )]
    pub validator_state: Account<'info, ValidatorState>,

    #[account(mut)]
    pub subnet_miners: AccountLoader<'info, SubnetMiners>,

    /// 验证者每次只能给一个矿工组进行打分
    #[account(mut)]
    pub miner_weights: AccountLoader<'info, MinerWeights>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
