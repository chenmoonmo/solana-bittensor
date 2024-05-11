use crate::errors::ErrorCode;
use crate::states::*;
use anchor_lang::prelude::*;

pub fn set_miner_weights(ctx: Context<SetMinerWeights>, weights: Vec<u16>) -> Result<()> {
    let miner_weights = &mut ctx.accounts.miner_weights.load_mut()?;

    let subnet_validators = &mut ctx.accounts.subnet_validators.load_mut()?;

    let validator_id = ctx.accounts.validator_state.id;

    // 限制周期内验证人可以打出的总权重
    let sum_weights = weights.iter().sum::<u16>();
    let validator_used_weights = subnet_validators.validators[validator_id as usize].used_weights;

    require!(
        validator_used_weights + sum_weights <= MAX_WEIGHT,
        ErrorCode::TotalWeightExceedsMaxWeight
    );

    miner_weights.set_weights(validator_id, &weights);

    subnet_validators.validators[validator_id as usize].used_weights += sum_weights;

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

    /// 验证者每次只能给一个矿工组进行打分
    #[account(
        mut,
        seeds = [b"miner_weights 0",subnet_state.key().as_ref()],
        bump
    )]
    pub miner_weights: AccountLoader<'info, MinerWeights>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
