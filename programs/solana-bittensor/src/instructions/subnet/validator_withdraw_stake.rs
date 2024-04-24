use crate::errors::ErrorCode;
use crate::states::*;
use anchor_lang::prelude::*;
use anchor_spl::{
    token,
    token::{Mint, Token, TokenAccount, Transfer},
};

pub fn validator_withdraw_stake(ctx: Context<ValidatoWithdrawStake>, amount: u64) -> Result<()> {
    let validator_stake = ctx.accounts.validator_state.stake;
    let validator_id = ctx.accounts.validator_state.id;

    require!(validator_stake >= amount, ErrorCode::NotEnoughStake);

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

    msg!("stake {}", ctx.accounts.validator_state.stake);

    ctx.accounts.validator_state.remove_stake(amount);
    ctx.accounts
        .subnet_validators
        .load_mut()?
        .validator_remove_stake(validator_id, amount);

    Ok(())
}

#[derive(Accounts)]
pub struct ValidatoWithdrawStake<'info> {
    #[account(
        mut,
        seeds = [b"bittensor"],
        bump,
    )]
    pub bittensor_state: AccountLoader<'info, BittensorState>,

    #[account(mut)]
    pub subnet_state: Box<Account<'info, SubnetState>>,

    #[account(
        mut,
        seeds = [b"subnet_epoch",subnet_state.key().as_ref()],
        bump
    )]
    pub subnet_validators: AccountLoader<'info, SubnetValidators>,

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
        token::authority = bittensor_state
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
