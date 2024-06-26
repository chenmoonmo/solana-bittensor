use crate::states::*;
use anchor_lang::prelude::*;

use anchor_spl::{
    token,
    token::{Mint, MintTo, Token, TokenAccount},
};

pub fn validator_reward(ctx: Context<ValidatorReward>) -> Result<()> {
    let validator_id = ctx.accounts.validator_state.id;

    let validators = &mut ctx.accounts.subnet_validators.load_mut()?.validators;

    let validator = validators
        .iter_mut()
        .find(|x| x.id == validator_id)
        .unwrap();

    let amount = validator.reward;

    let bump = ctx.accounts.subnet_state.bump;
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

    validator.reward = 0;

    emit!(ValidatorClaimRewardEvent {
        id: validator_id,
        owner: ctx.accounts.owner.key(),
        pubkey: ctx.accounts.validator_state.key(),
        claim_amount: amount,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct ValidatorReward<'info> {
    #[account(
        mut,
        seeds = [b"subnet_state"],
        bump = subnet_state.bump,
    )]
    pub subnet_state: Box<Account<'info, SubnetState>>,

    #[account(
        mut,
        seeds = [b"subnet_validators",subnet_state.key().as_ref()],
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
