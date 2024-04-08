use crate::states::*;
use anchor_lang::prelude::*;

pub const MAX_MINER_WEIGHT: u64 = 1000;

pub fn set_miner_weights(
    ctx: Context<SetMinerWeights>,
    miner_ids: Vec<u64>,
    weights: Vec<u64>,
) -> Result<()> {
    let subnet_epoch = &mut ctx.accounts.subnet_epoch.load_mut()?;
    let validator_id = ctx.accounts.validator_state.id;
    // TODO: 限制总权重
    // 如果已经打过分则 panic
    // require!(total_weight <= MAX_MINER_WEIGHT, "Total weight exceeds MAX_MINER_WEIGHT");
    subnet_epoch.set_miner_weights(validator_id, miner_ids, weights);
    Ok(())
}

#[derive(Accounts)]
pub struct SetMinerWeights<'info> {
    #[account(mut)]
    pub subnet_state: AccountLoader<'info, SubnetState>,

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
