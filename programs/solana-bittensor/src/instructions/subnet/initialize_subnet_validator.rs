use crate::states::*;
use anchor_lang::prelude::*;

pub fn initialize_subnet_validator(ctx: Context<InitializeSubnetValidator>) -> Result<()> {
    // TODO:
    // 设置注册费用
    // 注册验证人时 燃烧代币
    // 验证人保护期初始化
    
    let owner = ctx.accounts.owner.key();
    let subnet_state = &mut ctx.accounts.subnet_state.load_mut()?;

    let validator_state = &mut ctx.accounts.validator_state;

    validator_state.owner = owner;

    subnet_state.create_validator(owner);

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
