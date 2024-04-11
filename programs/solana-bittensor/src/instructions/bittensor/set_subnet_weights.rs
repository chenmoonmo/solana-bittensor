use crate::errors::ErrorCode;
use crate::states::*;
use anchor_lang::prelude::*;

pub fn set_subnet_weights(ctx: Context<SetSubnetWeights>, weights: Vec<u64>) -> Result<()> {
    let validators = ctx.accounts.bittensor_state.load_mut()?.validators;
    let subnet_id = ctx.accounts.subnet_state.load()?.id;
    let validator_id = ctx.accounts.validator_state.id;

    let sum_weights = weights.iter().sum::<u64>();
    require!(
        sum_weights <= MAX_WEIGHT,
        ErrorCode::TotalWeightExceedsMaxWeight
    );

    let is_bittensor_validator = validators
        .iter()
        .any(|v| v.validator_id == validator_id && v.subnet_id == subnet_id);
    require!(is_bittensor_validator, ErrorCode::NotBittensorValidator);

    ctx.accounts
        .bittensor_epoch
        .load_mut()?
        .set_weights(validator_id, weights);

    Ok(())
}

#[derive(Accounts)]
pub struct SetSubnetWeights<'info> {
    #[account(
        seeds = [b"bittensor"],
        bump,
    )]
    pub bittensor_state: AccountLoader<'info, BittensorState>,

    #[account(
        seeds = [b"bittensor_epoch", bittensor_state.key().as_ref()],
        bump,
    )]
    pub bittensor_epoch: AccountLoader<'info, BittensorEpochState>,

    #[account(mut)]
    pub subnet_state: AccountLoader<'info, SubnetState>,

    #[account(
        mut,
        seeds = [b"validator_state",subnet_state.key().as_ref(),owner.key().as_ref()],
        bump
    )]
    pub validator_state: Account<'info, ValidatorState>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
