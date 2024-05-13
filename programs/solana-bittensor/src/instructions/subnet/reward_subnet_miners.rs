use crate::errors::ErrorCode;
use crate::states::*;
use anchor_lang::prelude::*;

pub fn reward_subnet_miners(ctx: Context<RewardSubnetMiners>) -> Result<()> {
    let miner_weights = &mut ctx.accounts.miner_weights.load_mut()?;
    let subnet_miners = &mut ctx.accounts.subnet_miners.load_mut()?;

    // 只有所有的矿工组的打分都被结算了，才能进行奖励
    require!(
        ctx.accounts
            .subnet_state
            .weights_staus
            .into_iter()
            .all(|i| i == 2),
        ErrorCode::InvalidEndStep
    );

    require!(
        miner_weights.miner_group_id == subnet_miners.group_id,
        ErrorCode::InvalidMinerGroupId
    );

    require!(miner_weights.end_step == 2, ErrorCode::InvalidEndStep);

    let epoch_total_weights = ctx.accounts.subnet_state.epoch_total_weights;

    for i in 0..MAX_GROUP_MINER_NUMBER {
        let weight = miner_weights.miner_total_weights[i];

        let reward = (weight as u128)
            .checked_mul(10_000_000_000)
            .unwrap()
            .checked_div(epoch_total_weights as u128)
            .unwrap_or(0) as u64;

        subnet_miners.miners[i].reward += reward;
    }

    ctx.accounts.subnet_state.weights_staus[ctx.accounts.subnet_miners.load()?.group_id as usize] =
        3;
        
    miner_weights.end_epoch();

    Ok(())
}

#[derive(Accounts)]
pub struct RewardSubnetMiners<'info> {
    #[account(mut)]
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
