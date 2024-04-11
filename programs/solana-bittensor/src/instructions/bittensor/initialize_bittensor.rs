use crate::states::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

pub fn initialize_bittensor(ctx: Context<InitializeBittensor>) -> Result<()> {
    let timestamp = Clock::get()?.unix_timestamp;

    ctx.accounts.bittensor_epoch.load_init()?;

    ctx.accounts
        .bittensor_state
        .load_init()?
        .initialize(timestamp);

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeBittensor<'info> {
    #[account(
        init,
        payer = owner,
        space = 10 * 1024,
        seeds = [b"bittensor"],
        bump
    )]
    pub bittensor_state: AccountLoader<'info, BittensorState>,

    #[account(
        init,
        payer = owner,
        space = 10 * 1024,
        seeds = [b"bittensor_epoch", bittensor_state.key().as_ref()],
        bump,
    )]
    pub bittensor_epoch: AccountLoader<'info, BittensorEpochState>,

    // 系统奖励代币
    #[account(
        init,
        payer = owner,
        seeds = [b"tao", bittensor_state.key().as_ref()],
        bump,
        mint::decimals = 9,
        mint::authority = bittensor_state
    )]
    pub tao_mint: Box<Account<'info, Mint>>,
    // 质押代币存储账户
    #[account(
        init,
        payer = owner,
        seeds=[b"tao_stake", bittensor_state.key().as_ref()],
        bump,
        token::mint = tao_mint,
        token::authority = bittensor_state
    )]
    pub tao_stake: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}
