use crate::states::*;
use anchor_lang::prelude::*;

pub fn end_subnet_epoch(ctx: Context<EndSubnetEpoch>) -> Result<()> {
    let subnet_state = &mut ctx.accounts.subnet_state.load_mut()?;

    let miners = subnet_state.miners;

    let validators = subnet_state.validators;
    let validator_total_stake = subnet_state.validator_total_stake;
    let miners_weights = &mut ctx.accounts.subnet_weights.load_mut()?;

    let weights = miners_weights.get_miner_weights(validators, validator_total_stake);

    for (miner_id, weight) in weights {
        let reward = weight * subnet_state.distribute_reward / 100;
        let mut miner = miners[miner_id as usize - 1];
        miner.reward += reward;
    }

    Ok(())
}

#[derive(Accounts)]
pub struct EndSubnetEpoch<'info> {
    #[account(mut)]
    pub subnet_state: AccountLoader<'info, SubnetState>,

    #[account(
        mut,
        seeds = [b"weights",subnet_state.key().as_ref()],
        bump
    )]
    pub subnet_weights: AccountLoader<'info, SubnetWeightsState>,

    pub system_program: Program<'info, System>,
}
