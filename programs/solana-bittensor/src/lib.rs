use anchor_lang::prelude::*;
pub mod errors;
pub mod instructions;
pub mod states;
mod allocator;

use crate::instructions::*;

declare_id!("7rzTY9ro4qQtnWZg3kkacYsrh9tBcQ6ueuEBi2n5GdsW");

#[program]
pub mod solana_bittensor {
    use super::*;
    // 初始化主网
    pub fn initialize_subnet(ctx: Context<InitializeSubnet>) -> Result<()> {
        instructions::initialize_subnet(ctx)
    }
    // 扩容矿工
    pub fn increase_miners(_ctx: Context<IncreaseMiners>, _len: u32) -> Result<()> {
        Ok(())
    }

    //  扩容权重矩阵
    pub fn increase_miner_weights(_ctx: Context<IncreaseMinerWeights>, _len: u32) -> Result<()> {
        Ok(())
    }

    // 测试用 mint
    pub fn mint(ctx: Context<MintTao>) -> Result<()> {
        instructions::mint_tao(ctx)
    }

    // 注册子网验证人
    pub fn initialize_subnet_validator(
        ctx: Context<InitializeSubnetValidator>,
        stake_amount: u64,
    ) -> Result<()> {
        instructions::initialize_subnet_validator(ctx, stake_amount)
    }

    // 注册子网矿工
    pub fn initialize_subnet_miner(ctx: Context<InitializeSubnetMiner>) -> Result<()> {
        instructions::initialize_subnet_miner(ctx)
    }

    // 验证人质押
    pub fn validator_stake(ctx: Context<ValidatorStake>, amount: u64) -> Result<()> {
        instructions::validator_stake(ctx, amount)
    }

    // 给子网矿工打分
    pub fn set_miner_weights(
        ctx: Context<SetMinerWeights>,
        weights: Vec<u16>,
        ids: Vec<u32>,
    ) -> Result<()> {
        instructions::set_miner_weights(ctx, weights, ids)
    }

    // 开始结束周期 每50个矿工为一组 进行中位数计算和区在权重计算
    pub fn end_epoch_weights(ctx: Context<EndEpochWeights>) -> Result<()> {
        instructions::end_epoch_weights(ctx)
    }

    // 结束周期 每50个矿工为一组 进行奖励
    pub fn reward_subnet_miners(ctx: Context<RewardSubnetMiners>) -> Result<()> {
        instructions::reward_subnet_miners(ctx)
    }

    // 结束周期 奖励验证人
    pub fn reward_subnet_validators(ctx: Context<RewardSubnetValidators>) -> Result<()> {
        instructions::reward_subnet_validators(ctx)
    }

    // 验证人提取质押
    pub fn validator_unstakes(ctx: Context<ValidatorStake>, amount: u64) -> Result<()> {
        instructions::validator_unstake(ctx, amount)
    }

    // 8. 矿工提取奖励
    pub fn miner_reward(ctx: Context<MinerReward>) -> Result<()> {
        instructions::miner_reward(ctx)
    }
    // 9. 验证人提取奖励
    pub fn validator_reward(ctx: Context<ValidatorReward>) -> Result<()> {
        instructions::validator_reward(ctx)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
