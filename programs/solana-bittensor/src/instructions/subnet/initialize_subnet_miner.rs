use crate::states::*;
use anchor_lang::prelude::*;

pub fn initialize_subnet_miner(ctx: Context<InitializeSubnetMiner>) -> Result<()> {
    // TODO:
    // 设置注册费用
    // 注册矿工时 燃烧代币
    // 矿工保护期初始化

    let owner = ctx.accounts.owner.key();

    let miner_id = ctx.accounts.subnet_state.load_mut()?.create_miner(owner);

    let miner_state = &mut ctx.accounts.miner_state.load_init()?;

    miner_state.owner = owner;
    miner_state.id = miner_id;

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeSubnetMiner<'info> {
    #[account(mut)]
    pub subnet_state: AccountLoader<'info, SubnetState>,

    #[account(
        init,
        space = 1024 * 10,
        payer = owner,
        seeds = [b"miner_state",subnet_state.key().as_ref(),owner.key().as_ref()],
        bump
    )]
    pub miner_state: AccountLoader<'info, MinerState>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
