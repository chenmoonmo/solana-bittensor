use crate::states::*;
use anchor_lang::prelude::*;

pub fn end_subnet_epoch(ctx: Context<EndSubnetEpoch>) -> Result<()> {
    let timestamp = Clock::get()?.unix_timestamp;
    let subnet_state = &mut ctx.accounts.subnet_state.load_mut()?;
    let distribute_reward = ctx.accounts.subnet_state.load()?.distribute_reward;
    let subnet_epoch = &mut ctx.accounts.subnet_epoch.load_mut()?;
    let validators = &mut ctx.accounts.subnet_state.load_mut()?.validators;
    let miners = &mut ctx.accounts.subnet_state.load_mut()?.miners;

    let mut miner_weights = [0; MAX_MINER_NUMBER];
    let mut validator_bonds = [0; MAX_VALIDATOR_NUMBER];

    let mut total_bonds = 0u64;
    let mut total_weights = 0u64;

    for i in 0..MAX_VALIDATOR_NUMBER {
        let validator_weight = subnet_epoch.miners_weights[i];
        let total_stake = validators[i].stake;

        let mut total_weight = 0;

        for j in 0..MAX_MINER_NUMBER {
            total_weight += validator_weight[j];
            miner_weights[j] += validator_weight[j] * total_stake;
            total_weights += validator_weight[j] * total_stake;
        }

        let bond = total_stake * total_weight / MAX_WEIGHT;
        total_bonds += bond;
        validator_bonds[i] = bond;
    }

    let half_reward = distribute_reward / 2;

    for i in 0..MAX_MINER_NUMBER {
        let reward = miner_weights[i] * half_reward / total_weights;
        miners[i].reward += reward;
    }

    for i in 0..MAX_VALIDATOR_NUMBER {
        let reward = validator_bonds[i] * half_reward / total_bonds;
        validators[i].bonds = validator_bonds[i];
        validators[i].reward += reward;
    }

    // // TODO: reset subnet_epoch
    subnet_state.distribute_reward = 0;
    subnet_epoch.initialize(timestamp);

    Ok(())
}

pub struct ValidatorBonds {
    pub validator_id: u8,
    pub bond: u64,
}

pub struct MinerWeight {
    pub miner_id: u8,
    pub weight: u64,
}

#[derive(Accounts)]
pub struct EndSubnetEpoch<'info> {
    #[account(mut)]
    pub subnet_state: AccountLoader<'info, SubnetState>,

    #[account(
        mut,
        seeds = [b"subnet_epoch",subnet_state.key().as_ref()],
        bump
    )]
    pub subnet_epoch: AccountLoader<'info, SubnetEpochState>,

    pub system_program: Program<'info, System>,
}
