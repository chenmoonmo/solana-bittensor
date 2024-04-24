use crate::errors::ErrorCode;
use crate::states::*;
use anchor_lang::prelude::*;

pub fn end_subnet_medians1(ctx: Context<EndSubnetMedians>) -> Result<()> {
    let subnet_epoch = &mut ctx.accounts.subnet_epoch.load_mut()?;

    require!(subnet_epoch.end_step == 0, ErrorCode::InvalidEndStep);

    let mut medians = Box::new([0; MAX_MINER_NUMBER / 2]);
    for i in 0..MAX_MINER_NUMBER / 2 {
        let mut weights = Vec::new();
        for j in 0..MAX_VALIDATOR_NUMBER {
            weights.push(subnet_epoch.miners_weights[j][i]);
        }

        weights.sort();
        medians[i] = weights[weights.len() / 2];
    }

    for i in 0..MAX_VALIDATOR_NUMBER {
        for j in 0..MAX_MINER_NUMBER / 2 {
            if subnet_epoch.miners_weights[i][j] > medians[j] {
                subnet_epoch.miners_weights[i][j] = medians[j];
            }
        }
    }

    subnet_epoch.end_step += 1;

    Ok(())
}

pub fn end_subnet_medians2(ctx: Context<EndSubnetMedians>) -> Result<()> {
    let subnet_epoch = &mut ctx.accounts.subnet_epoch.load_mut()?;
    require!(subnet_epoch.end_step == 1, ErrorCode::InvalidEndStep);

    let mut medians = Box::new([0; MAX_MINER_NUMBER / 2]);

    for i in 0..MAX_MINER_NUMBER / 2 {
        let mut weights = Vec::new();
        for j in 0..MAX_VALIDATOR_NUMBER {
            weights.push(subnet_epoch.miners_weights[j][i]);
        }

        weights.sort();
        medians[i] = weights[weights.len() / 2];
    }

    for i in 0..MAX_VALIDATOR_NUMBER {
        for j in (MAX_MINER_NUMBER / 2)..MAX_MINER_NUMBER {
            if subnet_epoch.miners_weights[i][j] > medians[j - MAX_MINER_NUMBER / 2] {
                subnet_epoch.miners_weights[i][j] = medians[j - MAX_MINER_NUMBER / 2];
            }
        }
    }

    subnet_epoch.end_step += 1;

    Ok(())
}

#[derive(Accounts)]
pub struct EndSubnetMedians<'info> {
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
    pub subnet_state: Box<Account<'info, SubnetState>>,

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
