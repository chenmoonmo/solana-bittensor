use anchor_lang::prelude::*;

use crate::states::*;

pub fn set_miner_weight(ctx: Context<SetMinerWeight>, weight: u64) -> Result<()> {
    let miner_id = ctx.accounts.miner_state.load()?.id;
    let validator_id = ctx.accounts.miner_state.load()?.id;

    ctx.accounts
        .subnet_weights
        .load_mut()?
        .set_miner_weight(miner_id, validator_id, weight);

    ctx.accounts
        .miner_state
        .load_mut()?
        .set_weight(validator_id, weight);

    Ok(())
}

#[derive(Accounts)]
pub struct SetMinerWeight<'info> {
    #[account(
        mut,
        seeds = [b"subnet_state",owner.key().as_ref()],
        bump
    )]
    pub subnet_state: AccountLoader<'info, SubnetState>,

    #[account(
        mut,
        seeds = [b"weights",subnet_state.key().as_ref()],
        bump
    )]
    pub subnet_weights: AccountLoader<'info, SubnetWeightsState>,

    #[account(mut)]
    pub miner_state: AccountLoader<'info, MinerState>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}
