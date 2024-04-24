use crate::errors::ErrorCode;
use crate::states::*;
use anchor_lang::prelude::*;

pub fn end_subnet_epoch1(ctx: Context<EndSubnetEpoch>) -> Result<()> {
    let subnet_validators = &mut ctx.accounts.subnet_validators.load_mut()?;
    let subnet_epoch = &mut ctx.accounts.subnet_epoch.load_mut()?;

    require!(subnet_epoch.end_step == 2, ErrorCode::InvalidEndStep);

    for i in 0..MAX_VALIDATOR_NUMBER {
        let validator_weight = subnet_epoch.miners_weights[i];
        let total_stake = subnet_validators.validators[i].stake;

        for j in 0..MAX_MINER_NUMBER / 2 {
            subnet_epoch.validator_total_weights[i] += validator_weight[j];
            subnet_epoch.miner_total_weights[j] += validator_weight[j] as u64 * total_stake;
        }
    }

    subnet_epoch.end_step += 1;
    Ok(())
}

pub fn end_subnet_epoch2(ctx: Context<EndSubnetEpoch>) -> Result<()> {
    let subnet_validators = &mut ctx.accounts.subnet_validators.load_mut()?;
    let subnet_epoch = &mut ctx.accounts.subnet_epoch.load_mut()?;

    require!(subnet_epoch.end_step == 3, ErrorCode::InvalidEndStep);

    for i in 0..MAX_VALIDATOR_NUMBER {
        let validator_weight = subnet_epoch.miners_weights[i];
        let total_stake = subnet_validators.validators[i].stake;

        for j in (MAX_MINER_NUMBER / 2)..MAX_MINER_NUMBER {
            subnet_epoch.validator_total_weights[i] += validator_weight[j];
            subnet_epoch.miner_total_weights[j] += validator_weight[j] as u64 * total_stake;
        }
    }

    subnet_epoch.end_step += 1;

    Ok(())
}

pub fn end_subnet_epoch(ctx: Context<EndSubnetEpoch>) -> Result<()> {
    let timestamp = Clock::get()?.unix_timestamp;
    let bittensor_state = &mut ctx.accounts.bittensor_state.load_mut()?;
    let subnet_state = &mut ctx.accounts.subnet_state;
    let subnet_validators = &mut ctx.accounts.subnet_validators.load_mut()?;
    let subnet_miners = &mut ctx.accounts.subnet_miners.load_mut()?;
    let subnet_epoch = &mut ctx.accounts.subnet_epoch.load_mut()?;

    require!(subnet_epoch.end_step == 4, ErrorCode::InvalidEndStep);

    let validator_total_weights = subnet_epoch.validator_total_weights;
    let miner_total_weights = subnet_epoch.miner_total_weights;

    msg!("miner_total_weights {:?}", miner_total_weights);

    let validator_bounds = validator_total_weights
        .iter()
        .enumerate()
        .map(|(i, &w)| w as u64 * subnet_validators.validators[i].stake)
        .collect::<Vec<u64>>();

    let total_weights: u64 = miner_total_weights.iter().sum();
    let total_bounds: u64 = validator_bounds.iter().sum();

    let half_reward = bittensor_state.subnets[subnet_state.id as usize].distribute_reward / 2;

    for i in 0..MAX_MINER_NUMBER {
        let reward = (miner_total_weights[i] as u128)
            .checked_mul(half_reward as u128)
            .unwrap()
            .checked_div(total_weights as u128)
            .unwrap_or(0) as u64;

        subnet_miners.miners[i].reward += reward;

        if subnet_miners.miners[i].protection > 0 {
            subnet_miners.miners[i].protection -= 1;
        }
    }

    for i in 0..MAX_VALIDATOR_NUMBER {
        let reward = (validator_bounds[i] as u128)
            .checked_mul(half_reward as u128)
            .unwrap()
            .checked_div(total_bounds as u128)
            .unwrap_or(0) as u64;

        subnet_validators.validators[i].bounds = validator_bounds[i];
        subnet_validators.validators[i].reward += reward;

        // 更新主网验证人的工作量
        if let Some(v) = bittensor_state
            .validators
            .iter_mut()
            .find(|v| v.validator_state == subnet_validators.validators[i].pubkey)
        {
            v.bounds = validator_bounds[i];
        }

        if subnet_validators.validators[i].protection > 0 {
            subnet_validators.validators[i].protection -= 1;
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
    pub subnet_state: Box<Account<'info, SubnetState>>,

    #[account(
        mut,
        seeds = [b"subnet_miners",subnet_state.key().as_ref()],
        bump
    )]
    pub subnet_miners: AccountLoader<'info, SubnetMiners>,

    #[account(
        mut,
        seeds = [b"subnet_validators",subnet_state.key().as_ref()],
        bump
    )]
    pub subnet_validators: AccountLoader<'info, SubnetValidators>,

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
