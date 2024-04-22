use crate::states::*;
use anchor_lang::prelude::*;
use crate::errors::ErrorCode;

pub fn calculate_weights(ctx: Context<CalculateWeights>) -> Result<()> {
    let subnet_epoch = &mut ctx.accounts.subnet_epoch.load_mut()?;

    require!(
        !subnet_epoch.is_calculated,
        ErrorCode::SubnetEpochAlreadyCalculated
    );

    let mut medians = Box::new([0; MAX_MINER_NUMBER]);
    for i in 0..MAX_MINER_NUMBER {
        let mut weights = Vec::new();
        for j in 0..MAX_VALIDATOR_NUMBER {
            weights.push(subnet_epoch.miners_weights[j][i]);
        }

        weights.sort();
        medians[i] = weights[weights.len() / 2];
    }

    for i in 0..MAX_VALIDATOR_NUMBER {
        for j in 0..MAX_MINER_NUMBER {
            if subnet_epoch.miners_weights[i][j] > medians[j] {
                subnet_epoch.miners_weights[i][j] = medians[j];
            }
        }
    }

    subnet_epoch.is_calculated = true;

    Ok(())
}

#[derive(Accounts)]
pub struct CalculateWeights<'info> {
    #[account(
        mut,
        seeds = [b"bittensor"],
        bump,
    )]
    pub bittensor_state: AccountLoader<'info, BittensorState>,

    #[account(
        mut,
        seeds = [b"subnet_state",owner.key().as_ref()],
        bump
    )]
    pub subnet_state: AccountLoader<'info, SubnetState>,

    #[account(
        mut,
        seeds = [b"subnet_epoch",subnet_state.key().as_ref()],
        bump
    )]
    pub subnet_epoch: AccountLoader<'info, SubnetEpochState>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}
