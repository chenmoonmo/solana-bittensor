use crate::states::*;
use anchor_lang::prelude::*;

pub fn initialize_subnet_validator(ctx: Context<InitializeSubnetValidator>) -> Result<()> {
    // TODO:
    // 设置注册费用
    // 注册验证人时 燃烧代币
    // 验证人保护期初始化

    let owner = ctx.accounts.owner.key();

    let validator_id = ctx
        .accounts
        .subnet_state
        .load_mut()?
        .create_validator(owner);

    let validator_state = &mut ctx.accounts.validator_state;
    validator_state.id = validator_id;
    validator_state.owner = owner;

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeSubnetValidator<'info> {
    #[account(mut)]
    pub subnet_state: AccountLoader<'info, SubnetState>,

    #[account(
        init,
        space = 1024 * 10,
        payer = owner,
        seeds = [b"validator_state",subnet_state.key().as_ref(),owner.key().as_ref()],
        bump
    )]
    pub validator_state: Account<'info, ValidatorState>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
