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
        init,
        payer = owner,
        space = 8 + SubnetState::LEN,
        seeds = [b"subnet_state"],
        bump
    )]
    pub subnet_state: Box<Account<'info, SubnetState>>,

    // 系统奖励代币
    #[account(
        init,
        payer = owner,
        seeds = [b"tao", subnet_state.key().as_ref()], 
        bump,
        mint::decimals = 9,
        mint::authority = subnet_state
     )]
    pub tao_mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}
