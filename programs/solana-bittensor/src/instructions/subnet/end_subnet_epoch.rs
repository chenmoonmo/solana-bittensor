use crate::states::*;
use anchor_lang::prelude::*;

pub fn end_subnet_epoch(ctx: Context<EndSubnetEpoch>) -> Result<()> {
    // 确保所有矿工组都已经结算完毕
    // 复原所有矿工组的权重
    // 复原所有验证人的待分配权重

    ctx.accounts.subnet_validators.load_mut()?.end_epoch();
    // ctx.accounts.subnet_state.end_epoch();

    Ok(())
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
        seeds = [b"subnet_validators",subnet_state.key().as_ref()],
        bump
    )]
    pub subnet_validators: AccountLoader<'info, SubnetValidators>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}
