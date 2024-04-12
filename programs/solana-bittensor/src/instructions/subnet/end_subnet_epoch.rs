use crate::states::*;
use anchor_lang::prelude::*;

pub fn end_subnet_epoch(ctx: Context<EndSubnetEpoch>) -> Result<()> {
    let timestamp = Clock::get()?.unix_timestamp;
    let bittensor_state = &mut ctx.accounts.bittensor_state.load_mut()?;
    let subnet_state = &mut ctx.accounts.subnet_state.load_mut()?;
    let subnet_epoch = &mut ctx.accounts.subnet_epoch.load_mut()?;

    let mut miner_weights = Box::new([0; MAX_MINER_NUMBER]);
    let mut validator_bonds = Box::new([0; MAX_VALIDATOR_NUMBER]);

    let mut total_bonds = 0u64;
    let mut total_weights = 0u64;

    for i in 0..MAX_VALIDATOR_NUMBER {
        let validator_weight = subnet_epoch.miners_weights[i];
        let total_stake = subnet_state.validators[i].stake;

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

    let half_reward = bittensor_state.subnets[subnet_state.id as usize].distribute_reward / 2;

    for i in 0..MAX_MINER_NUMBER {
        let reward = (miner_weights[i] as u128)
            .checked_mul(half_reward as u128)
            .unwrap()
            .checked_div(total_weights as u128)
            .unwrap() as u64;
        subnet_state.miners[i].reward += reward;
    }

    for i in 0..MAX_VALIDATOR_NUMBER {
        let reward = (validator_bonds[i] as u128)
            .checked_mul(half_reward as u128)
            .unwrap()
            .checked_div(total_bonds as u128)
            .unwrap() as u64;
        subnet_state.validators[i].bonds = validator_bonds[i];
        subnet_state.validators[i].reward += reward;
    }

    // subnet_state.distribute_reward 好像就没用
    subnet_state.distribute_reward = 0;
    bittensor_state.subnets[subnet_state.id as usize].distribute_reward = 0;
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
    #[account(
        mut,
        seeds = [b"bittensor"],
        bump,
    )]
    pub bittensor_state: AccountLoader<'info, BittensorState>,
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
