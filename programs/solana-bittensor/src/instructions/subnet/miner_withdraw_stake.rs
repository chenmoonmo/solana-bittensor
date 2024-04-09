use crate::errors::ErrorCode;
use crate::states::*;
use anchor_lang::prelude::*;

use anchor_spl::{
    token,
    token::{Mint, Token, TokenAccount, Transfer},
};

pub fn miner_withdraw_stake(ctx: Context<MinerWithdrawStake>, amount: u64) -> Result<()> {
    let miner_stake = ctx.accounts.miner_state.load()?.stake;
    let miner_id = ctx.accounts.miner_state.load()?.id;
    // 提取 stake 不能超过 miner 的 stake
    require!(miner_stake >= amount, ErrorCode::NotEnoughStake);

    let bump = ctx.bumps.bittensor_state;
    let pda_sign: &[&[u8]; 2] = &[b"bittensor", &[bump]];

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.tao_stake.to_account_info(),
                to: ctx.accounts.user_tao_ata.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            },
        )
        .with_signer(&[pda_sign]),
        amount,
    )?;

    // 1. 从 miner_state 中减去 stake
    ctx.accounts.miner_state.load_mut()?.remove_stake(amount);
    // 2. 从总的质押中减去 stake
    ctx.accounts
        .subnet_state
        .load_mut()?
        .miner_remove_stake(miner_id, amount);

    Ok(())
}

#[derive(Accounts)]
pub struct MinerWithdrawStake<'info> {
    #[account(
        mut,
        seeds = [b"bittensor"],
        bump,
    )]
    pub bittensor_state: AccountLoader<'info, BittensorState>,

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
