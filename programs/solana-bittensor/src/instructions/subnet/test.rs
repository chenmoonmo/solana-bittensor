use anchor_lang::prelude::*;

use crate::states::*;

pub fn test(ctx: Context<Test>, miner_ids: Vec<u64>, weights: Vec<u64>) -> Result<()> {
    let mut i: usize = 0;
    let subnet_weights = &mut ctx.accounts.subnet_weights.load_mut()?;
    let validator_id = ctx.accounts.validator_state.id;
    for miner_id in miner_ids {
        let weight = weights[i];
        i += 1;
        subnet_weights.set_miner_weight(validator_id, miner_id as u8, weight);
    }

    Ok(())
}

#[derive(Accounts)]
pub struct Test<'info> {
    #[account(mut)]
    pub subnet_state: AccountLoader<'info, SubnetState>,

    #[account(
        mut,
        seeds = [b"weights",subnet_state.key().as_ref()],
        bump
    )]
    pub subnet_weights: AccountLoader<'info, SubnetWeightsState>,

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
