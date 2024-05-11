use crate::errors::ErrorCode;
use crate::states::*;
use anchor_lang::prelude::*;

///  一次计算一个矿工组的权重

pub fn end_epoch_weights(ctx: Context<EndEpochWeights>) -> Result<()> {
    let mut miner_weights = ctx.accounts.miner_weights.load_mut()?;

    let subnet_validators = ctx.accounts.subnet_validators.load_mut()?;

    require!(miner_weights.is_end == false, ErrorCode::InvalidEndStep);

    // miner_weights 的 miners_weights 字段是一个二维数组，每个元素是一个长度为 32 的数组, 代表一个矿工被验证人所打的分

    for i in 0..100 {
        let mut weights = miner_weights.miners_weights[i]
            .into_iter()
            .collect::<Vec<u16>>();

        weights.sort();

        let median = weights[weights.len() / 2];

        let mut total_weights = 0;

        for j in 0..MAX_VALIDATOR_NUMBER {
            let mut weight = miner_weights.miners_weights[i][j];
            if weight > median {
                weight = median;
            }

            let total_stake = subnet_validators.validators[j].stake;
            total_weights += weight as u64 * total_stake;
        }

        miner_weights.miner_total_weights[i] = total_weights;
    }

    // TODO: 将质押运算后的总和加到 epoch 或者什么里

    miner_weights.is_end = true;

    Ok(())
}

#[derive(Accounts)]
pub struct EndEpochWeights<'info> {
    #[account(mut)]
    pub subnet_state: Box<Account<'info, SubnetState>>,
    #[account(
            mut,
            seeds = [b"miner_weights 0",subnet_state.key().as_ref()],
            bump
        )]
    pub miner_weights: AccountLoader<'info, MinerWeights>,

    #[account(
        mut,
        seeds = [b"subnet_validators",subnet_state.key().as_ref()],
        bump
    )]
    pub subnet_validators: AccountLoader<'info, SubnetValidators>,
}
