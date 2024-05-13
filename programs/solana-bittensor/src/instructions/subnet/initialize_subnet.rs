use crate::errors::ErrorCode;
use crate::states::*;
use anchor_lang::prelude::*;

use anchor_spl::{
    token,
    token::{Burn, Mint, Token, TokenAccount},
};

pub const SUBNET_REGISTER_FEE: u64 = 10 * 1_000_000_000;

pub fn initialize_subnet(ctx: Context<InitializeSubnet>) -> Result<()> {
    let tao_balance = ctx.accounts.user_tao_ata.amount;

    require!(
        tao_balance >= SUBNET_REGISTER_FEE,
        ErrorCode::NotEnoughBalance
    );

    token::burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.tao_mint.to_account_info(),
                from: ctx.accounts.user_tao_ata.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            },
        ),
        SUBNET_REGISTER_FEE,
    )?;

    let owner = ctx.accounts.owner.key();
    let pubkey = ctx.accounts.subnet_state.key();
    let subnet_state = &mut ctx.accounts.subnet_state;

    // TODO: remove subnet id
    // subnet_state.initialize(subnet_id);
    // ctx.accounts.subnet_miners.load_mut()?.id = subnet_id;
    // ctx.accounts.subnet_validators.load_mut()?.id = subnet_id;

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeSubnet<'info> {
    #[account(
        mut,
        seeds = [b"subnet_state"],
        bump
    )]
    pub subnet_state: Box<Account<'info, SubnetState>>,

    #[account(
        mut,
        seeds = [b"subnet_miners 0",subnet_state.key().as_ref()],
        bump
    )]
    pub subnet_miners: AccountLoader<'info, SubnetMiners>,

    #[account(
        mut,
        seeds = [b"subnet_miners 1",subnet_state.key().as_ref()],
        bump
    )]
    pub subnet_miners1: AccountLoader<'info, SubnetMiners>,

    #[account(
        mut,
        seeds = [b"subnet_miners 2",subnet_state.key().as_ref()],
        bump
    )]
    pub subnet_miners2: AccountLoader<'info, SubnetMiners>,

    #[account(
        mut,
        seeds = [b"subnet_miners 3",subnet_state.key().as_ref()],
        bump
    )]
    pub subnet_miners3: AccountLoader<'info, SubnetMiners>,

    #[account(
        mut,
        seeds = [b"subnet_miners 4",subnet_state.key().as_ref()],
        bump
    )]
    pub subnet_miners4: AccountLoader<'info, SubnetMiners>,

    #[account(
        mut,
        seeds = [b"subnet_miners 5",subnet_state.key().as_ref()],
        bump
    )]
    pub subnet_miners5: AccountLoader<'info, SubnetMiners>,

    #[account(
        mut,
        seeds = [b"subnet_miners 6",subnet_state.key().as_ref()],
        bump
    )]
    pub subnet_miners6: AccountLoader<'info, SubnetMiners>,

    #[account(
        mut,
        seeds = [b"subnet_miners 7",subnet_state.key().as_ref()],
        bump
    )]
    pub subnet_miners7: AccountLoader<'info, SubnetMiners>,

    #[account(
        mut,
        seeds = [b"subnet_miners 8",subnet_state.key().as_ref()],
        bump
    )]
    pub subnet_miners8: AccountLoader<'info, SubnetMiners>,

    #[account(
        mut,
        seeds = [b"subnet_miners 9",subnet_state.key().as_ref()],
        bump
    )]
    pub subnet_miners9: AccountLoader<'info, SubnetMiners>,

    #[account(
        mut,
        seeds = [b"subnet_validators",subnet_state.key().as_ref()],
        bump
    )]
    pub subnet_validators: AccountLoader<'info, SubnetValidators>,

    // 系统奖励代币
    #[account(
        mut,
        seeds = [b"tao", subnet_state.key().as_ref()], 
        bump,
    )]
    pub tao_mint: Box<Account<'info, Mint>>,

    // 质押代币存储账户
    #[account(
        mut,
        seeds=[b"tao_stake", subnet_state.key().as_ref()],
        bump,
        token::mint = tao_mint,
        token::authority = subnet_state
    )]
    pub tao_stake: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub user_tao_ata: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}
