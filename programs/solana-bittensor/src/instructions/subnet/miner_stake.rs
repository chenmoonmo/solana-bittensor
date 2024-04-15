use crate::states::*;
use anchor_lang::prelude::*;
use anchor_spl::{
    token,
    token::{Mint, Token, TokenAccount, Transfer},
};

pub fn miner_stake(ctx: Context<MinerStake>, amount: u64) -> Result<()> {
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

    let miner_id = ctx.accounts.miner_state.load()?.id;

    ctx.accounts
        .subnet_state
        .load_mut()?
        .miner_add_stake(miner_id, amount);

    ctx.accounts.miner_state.load_mut()?.add_stake(amount);

    Ok(())
}

pub fn miner_unstake(ctx: Context<MinerStake>, amount: u64) -> Result<()> {
    let miner_id = ctx.accounts.miner_state.load()?.id;

    ctx.accounts
        .subnet_state
        .load_mut()?
        .miner_remove_stake(miner_id, amount);

    ctx.accounts.miner_state.load_mut()?.remove_stake(amount);

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.tao_stake.to_account_info(),
                to: ctx.accounts.user_tao_ata.to_account_info(),
                authority: ctx.accounts.tao_stake.to_account_info(),
            },
        ),
        amount,
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct MinerStake<'info> {
    #[account(mut)]
    pub subnet_state: AccountLoader<'info, SubnetState>,
    #[account(
        mut,
        seeds = [b"miner_state",subnet_state.key().as_ref(),owner.key().as_ref()],
        bump
    )]
    pub miner_state: AccountLoader<'info, MinerState>,

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

    // 矿工的 tao token account
    #[account(mut)]
    pub user_tao_ata: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}
