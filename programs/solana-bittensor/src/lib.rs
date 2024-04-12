use anchor_lang::prelude::*;
pub mod errors;
pub mod instructions;
pub mod states;

use crate::instructions::*;

declare_id!("7rzTY9ro4qQtnWZg3kkacYsrh9tBcQ6ueuEBi2n5GdsW");

#[program]
pub mod solana_bittensor {
    use super::*;
    // 初始化主网
    pub fn initialize_bittensor(ctx: Context<InitializeBittensor>) -> Result<()> {
        instructions::initialize_bittensor(ctx)
    }

    // 注册子网
    pub fn initialize_subnet(ctx: Context<InitializeSubnet>) -> Result<()> {
        instructions::initialize_subnet(ctx)
    }

    pub fn mint(ctx: Context<MintTao>) -> Result<()> {
        instructions::mint_tao(ctx)
    }

    // 注册子网验证人
    pub fn initialize_subnet_validator(ctx: Context<InitializeSubnetValidator>) -> Result<()> {
        instructions::initialize_subnet_validator(ctx)
    }

    // 注册子网矿工
    pub fn initialize_subnet_miner(ctx: Context<InitializeSubnetMiner>) -> Result<()> {
        instructions::initialize_subnet_miner(ctx)
    }

    // 1. 注册主网验证人
    pub fn register_bittensor_validator(ctx: Context<RegisterBittensorValidator>) -> Result<()> {
        instructions::register_bittensor_validator(ctx)
    }
    // 2. 给子网打分
    pub fn set_subnet_weights(ctx: Context<SetSubnetWeights>, weights: Vec<u64>) -> Result<()> {
        instructions::set_subnet_weights(ctx, weights)
    }

    // 3. 验证人质押
    pub fn validator_stake(ctx: Context<ValidatorStake>, amount: u64) -> Result<()> {
        instructions::validator_stake(ctx, amount)
    }
    // 4. 矿工质押
    pub fn miner_stake(ctx: Context<MinerStake>, amount: u64) -> Result<()> {
        instructions::miner_stake(ctx, amount)
    }
    // 5. 结束主网周期
    pub fn end_epoch(ctx: Context<EndEpoch>) -> Result<()> {
        instructions::end_epoch(ctx)
    }

    // 6. 结束子网周期
    pub fn end_subnet_epoch(ctx: Context<EndSubnetEpoch>) -> Result<()> {
        instructions::end_subnet_epoch(ctx)
    }

    // 7. 给子网矿工打分
    pub fn set_miner_weights(ctx: Context<SetMinerWeights>, weights: Vec<u64>) -> Result<()> {
        instructions::set_miner_weights(ctx, weights)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
