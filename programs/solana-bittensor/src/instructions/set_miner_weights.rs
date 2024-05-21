use crate::errors::ErrorCode;
use crate::states::*;
use anchor_lang::prelude::*;

pub fn set_miner_weights(
    ctx: Context<SetMinerWeights>,
    weights: Vec<u16>,
    ids: Vec<u32>,
) -> Result<()> {
    let subnet_validators = &mut ctx.accounts.subnet_validators.load_mut()?;
    let validator_id = ctx.accounts.validator_state.id;
    let miner_weights = &mut ctx.accounts.miner_weights.load_mut()?;

    require!(
        ctx.accounts.subnet_state.end_step == 0,
        ErrorCode::InvalidEndStep
    );

    for (i, miner_id) in ids.into_iter().enumerate() {
        let weight = weights[i];

        let pre_weight = miner_weights.miners_weights[miner_id as usize][validator_id as usize];

        if pre_weight > 0 {
            subnet_validators.validators[validator_id as usize].used_weights -= pre_weight;
        }

        miner_weights.miners_weights[miner_id as usize][validator_id as usize] = weight;
    }

    // 限制周期内验证人可以打出的总权重
    let sum_weights = weights.iter().sum::<u16>();

    let validator_used_weights = subnet_validators.validators[validator_id as usize].used_weights;

    subnet_validators.validators[validator_id as usize].used_weights += sum_weights;

    require!(
        validator_used_weights + sum_weights <= MAX_WEIGHT,
        ErrorCode::TotalWeightExceedsMaxWeight
    );

    // emit!(ValidatorSetWeightsEvent {
    //     subnet_id: ctx.accounts.subnet_state.id,
    //     validator_id: validator_id,
    //     weights,
    // });

    Ok(())
}

#[derive(Accounts)]
pub struct SetMinerWeights<'info> {
    #[account(mut)]
    pub subnet_state: Box<Account<'info, SubnetState>>,

    #[account(
        mut,
        seeds = [b"validator_state",subnet_state.key().as_ref(),owner.key().as_ref()],
        bump
    )]
    pub validator_state: Account<'info, ValidatorState>,

    #[account(
        mut,
        seeds = [b"subnet_validators",subnet_state.key().as_ref()],
        bump
    )]
    pub subnet_validators: AccountLoader<'info, SubnetValidators>,

    #[account(mut)]
    pub miner_weights: AccountLoader<'info, MinerWeights>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
