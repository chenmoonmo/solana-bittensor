use crate::states::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

pub fn register_subnet(ctx: Context<RegisterSubnet>) -> Result<()> {

    ctx.accounts.subnet_state.register(ctx.accounts.owner.key());

    Ok(())
}

#[derive(Accounts)]
pub struct RegisterSubnet<'info> {
    #[account(
        mut,
        seeds = [b"bittensor"],
        bump,
    )]
    pub bittensor_state: AccountLoader<'info, BittensorState>,

    #[account(
        init,
        payer = owner,
        space = 8 + SubnetState::LEN,
        seeds = [b"subnet_state",owner.key().as_ref()],
        bump
    )]
    pub subnet_state: Box<Account<'info, SubnetState>>,

    // 系统奖励代币
    #[account(
            mut,
            seeds = [b"tao", bittensor_state.key().as_ref()], 
            bump,
        )]
    pub tao_mint: Box<Account<'info, Mint>>,

    // 质押代币存储账户
    #[account(
        init,
        payer = owner,
        seeds=[b"tao_stake", subnet_state.key().as_ref()],
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
