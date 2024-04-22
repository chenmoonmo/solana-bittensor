use crate::errors::ErrorCode;
use crate::states::*;
use anchor_lang::prelude::*;

pub fn end_subnet_epoch(ctx: Context<EndSubnetEpoch>) -> Result<()> {
    let timestamp = Clock::get()?.unix_timestamp;
    let bittensor_state = &mut ctx.accounts.bittensor_state.load_mut()?;
    let subnet_state = &mut ctx.accounts.subnet_state.load_mut()?;
    let subnet_epoch = &mut ctx.accounts.subnet_epoch.load_mut()?;
    require!(
        subnet_epoch.is_calculated,
        ErrorCode::SubnetEpochNotCalculated
    );

    let mut miner_weights = Box::new([0; MAX_MINER_NUMBER]);
    let mut validator_bounds = Box::new([0; MAX_VALIDATOR_NUMBER]);

    let mut total_bounds = 0u64;
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
        total_bounds += bond;
        validator_bounds[i] = bond;
    }

    let half_reward = bittensor_state.subnets[subnet_state.id as usize].distribute_reward / 2;

    for i in 0..MAX_MINER_NUMBER {
        let reward = (miner_weights[i] as u128)
            .checked_mul(half_reward as u128)
            .unwrap()
            .checked_div(total_weights as u128)
            .unwrap() as u64;
        subnet_state.miners[i].reward += reward;

        if subnet_state.miners[i].protection > 0 {
            subnet_state.miners[i].protection -= 1;
        }
    }

    for i in 0..MAX_VALIDATOR_NUMBER {
        let reward = (validator_bounds[i] as u128)
            .checked_mul(half_reward as u128)
            .unwrap()
            .checked_div(total_bounds as u128)
            .unwrap() as u64;

        subnet_state.validators[i].bounds = validator_bounds[i];
        subnet_state.validators[i].reward += reward;

        // 更新主网验证人的工作量
        if let Some(v) = bittensor_state.validators.iter_mut().find(|v| {
            v.validator_id == subnet_state.validators[i].id && v.subnet_id == subnet_state.id
        }) {
            v.bounds = validator_bounds[i];
        }

        if subnet_state.validators[i].protection > 0 {
            subnet_state.validators[i].protection -= 1;
        }
    }

    // subnet_state.distribute_reward 好像就没用
    subnet_state.distribute_reward = 0;
    bittensor_state.subnets[subnet_state.id as usize].distribute_reward = 0;

    subnet_epoch.end_epoch(timestamp);

    Ok(())
}

pub struct Validatorbounds {
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
