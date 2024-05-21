use crate::errors::ErrorCode;
use crate::states::*;
use anchor_lang::prelude::*;

use anchor_spl::{
    token,
    token::{Mint, MintTo, Token, TokenAccount},
};

pub fn miner_reward(ctx: Context<MinerReward>) -> Result<()> {
    let miner_id = ctx.accounts.miner_state.id;

    let miner = &mut ctx.accounts.subnet_miners.load_mut()?.miners[miner_id as usize];

    require!(
        miner.pubkey == ctx.accounts.miner_state.key(),
        ErrorCode::MinerNotMatch
    );

    let amount = miner.reward;
    let bump = ctx.bumps.subnet_state;
    let pda_sign: &[&[u8]; 2] = &[b"subnet_state", &[bump]];

    token::mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.tao_mint.to_account_info(),
                to: ctx.accounts.user_tao_ata.to_account_info(),
                authority: ctx.accounts.subnet_state.to_account_info(),
            },
        )
        .with_signer(&[pda_sign]),
        amount,
    )?;

    miner.reward = 0;

    emit!(MinerClaimRewardEvent {
        id: miner_id,
        owner: ctx.accounts.owner.key(),
        pubkey: ctx.accounts.user_tao_ata.owner,
        claim_amount: amount,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct MinerReward<'info> {
    #[account(
        mut,
        seeds = [b"subnet_state"],
        bump,
    )]
    pub subnet_state: Box<Account<'info, SubnetState>>,

    #[account(
        mut,
        seeds = [b"miner_state",subnet_state.key().as_ref(),owner.key().as_ref()],
        bump
    )]
    pub miner_state: Box<Account<'info, MinerState>>,

    #[account(mut)]
    pub subnet_miners: AccountLoader<'info, SubnetMiners>,

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
