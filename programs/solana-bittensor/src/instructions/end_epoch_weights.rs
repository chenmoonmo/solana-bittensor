use crate::errors::ErrorCode;
use crate::states::*;
use anchor_lang::prelude::*;

pub fn end_epoch_weights(ctx: Context<EndEpochWeights>) -> Result<()> {
    let miner_weights = &mut ctx.accounts.miner_weights.load_mut()?;
    let subnet_validators = ctx.accounts.subnet_validators.load_mut()?;
    let subnet_miners = ctx.accounts.subnet_miners.load_mut()?;

    require!(
        ctx.accounts.subnet_state.end_step == 0,
        ErrorCode::InvalidEndStep
    );

    let start_index: usize = miner_weights.last_calculate_id as usize;
    let mut end_index: usize = miner_weights.last_calculate_id as usize + 50;

    if end_index >= subnet_miners.last_miner_id as usize {
        end_index = subnet_miners.last_miner_id as usize;
        ctx.accounts.subnet_state.end_step = 1;
    }

    // miner_weights 的 miners_weights 字段是一个二维数组，每个元素是一个长度为 32 的数组, 代表一个矿工被验证人所打的分

    for i in start_index..end_index {
        let mut weights = miner_weights.miners_weights[i as usize]
            .into_iter()
            .collect::<Vec<u16>>();

        weights.sort();

        let median = weights[weights.len() / 2];

        let mut total_weights: u64 = 0;

        for j in 0..MAX_VALIDATOR_NUMBER {
            let mut weight = miner_weights.miners_weights[i][j];

            if weight > median {
                miner_weights.miners_weights[i][j] = median;
                weight = median;
            }

            let total_stake = subnet_validators.validators[j].stake;
            total_weights += weight as u64 * total_stake;
        }

        miner_weights.miner_total_weights[i as usize] = total_weights;
        ctx.accounts.subnet_state.epoch_total_weights += total_weights;
    }

    miner_weights.last_calculate_id = end_index as u32;

    Ok(())
}

#[derive(Accounts)]
pub struct EndEpochWeights<'info> {
    #[account(mut)]
    pub subnet_state: Box<Account<'info, SubnetState>>,

    #[account(
        mut,
        seeds = [b"subnet_miners",subnet_state.key().as_ref()],
        bump,
    )]
    pub subnet_miners: AccountLoader<'info, SubnetMiners>,

    #[account(mut)]
    pub miner_weights: AccountLoader<'info, MinerWeights>,

    #[account(
        mut,
        seeds = [b"subnet_validators",subnet_state.key().as_ref()],
        bump
    )]
    pub subnet_validators: AccountLoader<'info, SubnetValidators>,
}
