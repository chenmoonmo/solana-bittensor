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
    pub fn register_subnet(ctx: Context<RegisterSubnet>) -> Result<()> {
        instructions::register_subnet(ctx)
    }

    pub fn register_subnet_miners(ctx: Context<RegisterSubnetMiners>) -> Result<()> {
        instructions::register_subnet_miners(ctx)
    }

    pub fn register_subnet_weights(ctx: Context<RegisterSubnetWeights>) -> Result<()> {
        instructions::register_subnet_weights(ctx)
    }

    // 注册子网
    pub fn initialize_subnet(ctx: Context<InitializeSubnet>) -> Result<()> {
        instructions::initialize_subnet(ctx)
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


    // 3. 验证人质押
    pub fn validator_stake(ctx: Context<ValidatorStake>, amount: u64) -> Result<()> {
        instructions::validator_stake(ctx, amount)
    }

    // 4. 验证人提取质押
    pub fn validator_unstakes(ctx: Context<ValidatorStake>, amount: u64) -> Result<()> {
        instructions::validator_unstake(ctx, amount)
    }
    // 7. 给子网矿工打分
    pub fn set_miner_weights(ctx: Context<SetMinerWeights>, weights: Vec<u16>) -> Result<()> {
        instructions::set_miner_weights(ctx, weights)
    }
    pub fn end_epoch_weights(ctx: Context<EndEpochWeights>) -> Result<()> {
        instructions::end_epoch_weights(ctx)
    }

    pub fn reward_subnet_miners(ctx: Context<RewardSubnetMiners>) -> Result<()> {
        instructions::reward_subnet_miners(ctx)
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
