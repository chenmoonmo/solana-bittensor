use crate::states::*;
use anchor_lang::prelude::*;
use anchor_spl::{
    token,
    token::{Mint, Token, TokenAccount, Transfer},
};

pub fn validator_stake(ctx: Context<ValidatorStake>, amount: u64) -> Result<()> {
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user_tao_ata.to_account_info(),
                to: ctx.accounts.tao_stake.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            },
        ),
        amount,
    )?;

    // get account data form publickey
    let validator_id = ctx.accounts.validator_state.id;

    ctx.accounts.validator_state.add_stake(amount);

    ctx.accounts
        .subnet_validators
        .load_mut()?
        .validator_add_stake(validator_id, amount);

    ctx.accounts
        .bittensor_state
        .load_mut()?
        .validator_add_stake(ctx.accounts.validator_state.key(), amount);

    emit!(ValidatorAddStakeEvent {
        id: validator_id,
        owner: ctx.accounts.owner.key(),
        stake: ctx.accounts.validator_state.stake,
        pubkey: ctx.accounts.validator_state.key(),
        add_amount: amount,
    });

    Ok(())
}

pub fn validator_unstake(ctx: Context<ValidatorStake>, amount: u64) -> Result<()> {
    let validator = &mut ctx.accounts.validator_state;
    let pubkey = validator.key();

    validator.remove_stake(amount);

    ctx.accounts
        .subnet_validators
        .load_mut()?
        .validator_remove_stake(validator.id, amount);

    ctx.accounts
        .bittensor_state
        .load_mut()?
        .validator_remove_stake(pubkey, amount);

    let bump = ctx.bumps.bittensor_state;
    let pda_sign: &[&[u8]; 2] = &[b"bittensor", &[bump]];

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.tao_stake.to_account_info(),
                to: ctx.accounts.user_tao_ata.to_account_info(),
                authority: ctx.accounts.bittensor_state.to_account_info(),
            },
        )
        .with_signer(&[pda_sign]),
        amount,
    )?;

    emit!(ValidatorRemoveStakeEvent {
        id: validator.id,
        owner: ctx.accounts.owner.key(),
        stake: validator.stake,
        remove_amount: amount,
        pubkey,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct ValidatorStake<'info> {
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
        mut,
        seeds = [b"validator_state",subnet_state.key().as_ref(),owner.key().as_ref()],
        bump
    )]
    pub validator_state: Account<'info, ValidatorState>,

    // 系统奖励代币
    #[account(mut)]
    pub tao_mint: Box<Account<'info, Mint>>,

    // 质押代币存储账户
    #[account(
        mut,
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
