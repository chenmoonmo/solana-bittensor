use crate::{states::*, MAX_MINER_WEIGHT};
use anchor_lang::prelude::*;

pub fn end_subnet_epoch(ctx: Context<EndSubnetEpoch>) -> Result<()> {
    let timestamp = Clock::get()?.unix_timestamp;
    let subnet_state = &mut ctx.accounts.subnet_state.load_mut()?;
    let distribute_reward = ctx.accounts.subnet_state.load()?.distribute_reward;
    let subnet_epoch = &mut ctx.accounts.subnet_epoch.load_mut()?;
    let validators = &mut ctx.accounts.subnet_state.load_mut()?.validators;
    let miners = &mut ctx.accounts.subnet_state.load_mut()?.miners;

    // let mut miner_weights = vec![];
    // let mut validator_bonds = vec![];

    // let mut total_bonds = 0u64;
    // let mut total_weights = 0u64;

    // for validator_weight in subnet_epoch.validator_weights {
    //     // 1. 计算 validator 的工作量
    //     // 2. 计算 miner 的 weight
    //     let validator_id = validator_weight.validator_id;
    //     let total_stake = validators
    //         .iter()
    //         .find(|x| x.id == validator_id)
    //         .unwrap()
    //         .stake;

    //     let mut total_weight = 0;

    //     for miner_weight in validator_weight.weights {
    //         total_weight += miner_weight.weight;
    //         // 如果在 miner_weights 中找到 miner_id，则累加 weight * total_stake
    //         if let Some(miner_weight) = miner_weights
    //             .iter_mut()
    //             .find(|x: &&mut MinerWeight| x.miner_id == miner_weight.miner_id)
    //         {
    //             miner_weight.weight += miner_weight.weight * total_stake;
    //         } else {
    //             miner_weights.push(MinerWeight {
    //                 miner_id: miner_weight.miner_id,
    //                 weight: miner_weight.weight * total_stake,
    //             });
    //         }
    //     }

    //     // 当前验证者的 bond
    //     let bond = total_stake
    //         .checked_mul(total_weight)
    //         .unwrap()
    //         .checked_div(MAX_MINER_WEIGHT)
    //         .unwrap();

    //     validator_bonds.push(ValidatorBonds { validator_id, bond });

    //     total_weights += total_weight;
    //     total_bonds += bond;
    // }

    // let half_reward = distribute_reward.checked_div(2).unwrap();

    // for miner_weight in miner_weights.iter_mut() {
    //     let reward = miner_weight
    //         .weight
    //         .checked_mul(half_reward)
    //         .unwrap()
    //         .checked_div(total_weights)
    //         .unwrap();

    //     let miner = miners
    //         .iter_mut()
    //         .find(|x| x.id == miner_weight.miner_id)
    //         .unwrap();

    //     miner.reward += reward;
    // }

    // for validator_bond in validator_bonds.iter_mut() {
    //     let validator = validators
    //         .iter_mut()
    //         .find(|x| x.id == validator_bond.validator_id)
    //         .unwrap();

    //     // validator.reward += validator_bond.bond;
    //     let reward = validator_bond
    //         .bond
    //         .checked_mul(half_reward)
    //         .unwrap()
    //         .checked_div(total_bonds)
    //         .unwrap();

    //     validator.bonds = validator_bond.bond;
    //     validator.reward += reward;
    // }

    // // TODO: reset subnet_epoch
    // // subnet_epoch.initialize(timestamp);
    // subnet_state.distribute_reward = 0;

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
