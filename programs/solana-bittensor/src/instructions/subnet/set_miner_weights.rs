use crate::errors::ErrorCode;
use crate::states::*;
use anchor_lang::prelude::*;

pub fn set_miner_weights(ctx: Context<SetMinerWeights>, weights: Vec<u16>) -> Result<()> {
    let subnet_epoch = &mut ctx.accounts.subnet_epoch.load_mut()?;

    require!(subnet_epoch.end_step == 0, ErrorCode::EpochIsEnded);

    let validator_id = ctx.accounts.validator_state.id;
    let sum_weights = weights.iter().sum::<u16>();

    require!(
        sum_weights <= MAX_WEIGHT,
        ErrorCode::TotalWeightExceedsMaxWeight
    );

    subnet_epoch.set_weights(validator_id, &weights);

    emit!(ValidatorSetWeightsEvent {
        subnet_id: ctx.accounts.subnet_state.id,
        validator_id: validator_id,
        weights,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct SetMinerWeights<'info> {
    #[account(mut)]
    pub subnet_state: Box<Account<'info, SubnetState>>,

    #[account(
        mut,
        seeds = [b"subnet_epoch",subnet_state.key().as_ref()],
        bump
    )]
    pub subnet_epoch: AccountLoader<'info, SubnetEpochState>,

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
