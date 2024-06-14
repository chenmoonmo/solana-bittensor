use crate::states::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

pub fn initialize_subnet(ctx: Context<InitializeSubnet>) -> Result<()> {
    ctx.accounts.subnet_state.register(ctx.bumps.subnet_state, ctx.accounts.owner.key());
    ctx.accounts.subnet_validators.load_init()?.last_validator_id = -1;
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeSubnet<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + SubnetState::LEN,
        seeds = [b"subnet_state"],
        bump
    )]
    pub subnet_state: Box<Account<'info, SubnetState>>,

    #[account(
        init,
        payer = owner,
        space = 8 + SubnetValidators::LEN,
        seeds = [b"subnet_validators",subnet_state.key().as_ref()],
        bump
    )]
    pub subnet_validators: AccountLoader<'info, SubnetValidators>,

    #[account(
        init,
        space = 10 * 1024,
        payer = owner,
        seeds = [b"subnet_miners",subnet_state.key().as_ref()],
        bump
    )]
    pub subnet_miners: AccountLoader<'info, SubnetMiners>,

    #[account(
        init,
        payer = owner,
        space = 10 * 1024,
        seeds = [b"miner_weights",subnet_state.key().as_ref()],
        bump
    )]
    pub miner_weights: AccountLoader<'info, MinerWeights>,

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

    #[account(
        init,
        payer = owner,
        seeds=[b"tao_stake", subnet_state.key().as_ref()],
        bump,
        token::mint = tao_mint,
        token::authority = subnet_state
    )]
    pub tao_stake: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
#[instruction(len: u32)]
pub struct IncreaseMiners<'info> {
    #[account(
        mut,
        realloc = len as usize, 
        realloc::zero = true, 
        realloc::payer = signer
    )]
    pub subnet_miners: AccountLoader<'info, SubnetMiners>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(len: u32)]
pub struct IncreaseMinerWeights<'info> {
    #[account(
        mut,
        realloc = len as usize, 
        realloc::zero = true, 
        realloc::payer= signer
    )]
    pub miner_weights: AccountLoader<'info, MinerWeights>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}