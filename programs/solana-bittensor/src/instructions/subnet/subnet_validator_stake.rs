use crate::states::*;
use anchor_lang::prelude::*;
use anchor_spl::{
    token,
    token::{Mint, Token, TokenAccount, Transfer},
};

pub fn subnet_validator_stake(ctx: Context<SubnetValidatorStake>, amount: u64) -> Result<()> {
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user_tao_ata.to_account_info(),
                to: ctx.accounts.tao_stake.to_account_info(),
                authority: ctx.accounts.user_tao_ata.to_account_info(),
            },
        ),
        amount,
    )?;

    let miner_id = ctx.accounts.validator_state.id;

    ctx.accounts
        .subnet_state
        .load_mut()?
        .validator_add_stake(miner_id, amount);

    ctx.accounts.validator_state.add_stake(amount);

    Ok(())
}

#[derive(Accounts)]
pub struct SubnetValidatorStake<'info> {
    #[account(mut)]
    pub subnet_state: AccountLoader<'info, SubnetState>,
    #[account(
        seeds = [b"validator_state",subnet_state.key().as_ref(),owner.key().as_ref()],
        bump
    )]
    pub validator_state: Account<'info, ValidatorState>,

    // 系统奖励代币
    #[account(mut)]
    pub tao_mint: Box<Account<'info, Mint>>,

    // 质押代币存储账户
    #[account(
        seeds=[b"tao_stake", subnet_state.key().as_ref()],
        bump,
        token::mint = tao_mint,
        token::authority = subnet_state
    )]
    pub tao_stake: Box<Account<'info, TokenAccount>>,

    // 验证者的 tao token account
    #[account(mut)]
    pub user_tao_ata: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}
