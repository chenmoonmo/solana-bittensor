use anchor_lang::prelude::*;
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

    pub fn set_miner_weight(ctx: Context<SetMinerWeight>, weight: u64) -> Result<()> {
        instructions::set_miner_weight(ctx, weight)
    }

    // 1. 注册主网验证人
    // 2. 给子网打分
    // 3. 验证人质押
    // 4. 矿工质押

}

#[derive(Accounts)]
pub struct Initialize {}
