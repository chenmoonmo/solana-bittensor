use crate::states::*;
use anchor_lang::prelude::*;

pub fn register_subnet_weights(ctx: Context<RegisterSubnetWeights>) -> Result<()> {
    ctx.accounts.miner_weights.load_init()?.miner_group_id = 0;
    ctx.accounts.miner_weights1.load_init()?.miner_group_id = 1;
    ctx.accounts.miner_weights2.load_init()?.miner_group_id = 2;
    ctx.accounts.miner_weights3.load_init()?.miner_group_id = 3;
    ctx.accounts.miner_weights4.load_init()?.miner_group_id = 4;
    ctx.accounts.miner_weights5.load_init()?.miner_group_id = 5;
    ctx.accounts.miner_weights6.load_init()?.miner_group_id = 6;
    ctx.accounts.miner_weights7.load_init()?.miner_group_id = 7;
    ctx.accounts.miner_weights8.load_init()?.miner_group_id = 8;
    ctx.accounts.miner_weights9.load_init()?.miner_group_id = 9;
    Ok(())
}

#[derive(Accounts)]
pub struct RegisterSubnetWeights<'info> {
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
        init,
        payer = owner,
        space = 8 + MinerWeights::LEN,
        seeds = [b"miner_weights 0",subnet_state.key().as_ref()],
        bump
    )]
    pub miner_weights: AccountLoader<'info, MinerWeights>,

    #[account(
        init,
        payer = owner,
        space = 8 + MinerWeights::LEN,
        seeds = [b"miner_weights 1",subnet_state.key().as_ref()],
        bump
    )]
    pub miner_weights1: AccountLoader<'info, MinerWeights>,

    #[account(
        init,
        payer = owner,
        space = 8 + MinerWeights::LEN,
        seeds = [b"miner_weights 2",subnet_state.key().as_ref()],
        bump
    )]
    pub miner_weights2: AccountLoader<'info, MinerWeights>,

    #[account(
        init,
        payer = owner,
        space = 8 + MinerWeights::LEN,
        seeds = [b"miner_weights 3",subnet_state.key().as_ref()],
        bump
    )]
    pub miner_weights3: AccountLoader<'info, MinerWeights>,

    #[account(
        init,
        payer = owner,
        space = 8 + MinerWeights::LEN,
        seeds = [b"miner_weights 4",subnet_state.key().as_ref()],
        bump
    )]
    pub miner_weights4: AccountLoader<'info, MinerWeights>,

    #[account(
        init,
        payer = owner,
        space = 8 + MinerWeights::LEN,
        seeds = [b"miner_weights 5",subnet_state.key().as_ref()],
        bump
    )]
    pub miner_weights5: AccountLoader<'info, MinerWeights>,

    #[account(
        init,
        payer = owner,
        space = 8 + MinerWeights::LEN,
        seeds = [b"miner_weights 6",subnet_state.key().as_ref()],
        bump
    )]
    pub miner_weights6: AccountLoader<'info, MinerWeights>,

    #[account(
        init,
        payer = owner,
        space = 8 + MinerWeights::LEN,
        seeds = [b"miner_weights 7",subnet_state.key().as_ref()],
        bump
    )]
    pub miner_weights7: AccountLoader<'info, MinerWeights>,

    #[account(
        init,
        payer = owner,
        space = 8 + MinerWeights::LEN,
        seeds = [b"miner_weights 8",subnet_state.key().as_ref()],
        bump
    )]
    pub miner_weights8: AccountLoader<'info, MinerWeights>,

    #[account(
        init,
        payer = owner,
        space = 8 + MinerWeights::LEN,
        seeds = [b"miner_weights 9",subnet_state.key().as_ref()],
        bump
    )]
    pub miner_weights9: AccountLoader<'info, MinerWeights>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
