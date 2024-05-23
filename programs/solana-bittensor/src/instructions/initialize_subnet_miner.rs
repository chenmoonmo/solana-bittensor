use crate::errors::ErrorCode;
use crate::states::*;
use anchor_lang::prelude::*;

use anchor_spl::{
    token,
    token::{Burn, Mint, Token, TokenAccount},
};

const MINER_REGISTER_FEE: u64 = 1 * 1_000_000_000;

pub fn initialize_subnet_miner(ctx: Context<InitializeSubnetMiner>) -> Result<()> {
    let subnet_miners = &mut ctx.accounts.subnet_miners.load_mut()?;
    let tao_balance = ctx.accounts.user_tao_ata.amount;

    require!(
        tao_balance >= MINER_REGISTER_FEE,
        ErrorCode::NotEnoughBalance
    );

    let bump = ctx.accounts.subnet_state.bump;
    let pda_sign: &[&[u8]; 2] = &[b"subnet_state", &[bump]];

    // 燃烧注册费用
    token::burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.tao_mint.to_account_info(),
                from: ctx.accounts.user_tao_ata.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            },
        )
        .with_signer(&[pda_sign]),
        MINER_REGISTER_FEE,
    )?;

    let miner_id =
        subnet_miners.create_miner(ctx.accounts.owner.key(), ctx.accounts.miner_state.key());

    ctx.accounts
        .miner_state
        .initialize(&miner_id, ctx.accounts.owner.key());

    subnet_miners.last_miner_id = miner_id as i32;

    emit!(MinerRegisterEvent {
        id: miner_id,
        owner: ctx.accounts.owner.key(),
        stake: 0,
        pubkey: ctx.accounts.miner_state.key(),
        pre_pubkey: Pubkey::default(),
    });

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeSubnetMiner<'info> {
    #[account(
        seeds = [b"subnet_state"],
        bump = subnet_state.bump
    )]
    pub subnet_state: Box<Account<'info, SubnetState>>,

    #[account(
        mut,
        seeds = [b"subnet_miners",subnet_state.key().as_ref()],
        bump,
    )]
    pub subnet_miners: AccountLoader<'info, SubnetMiners>,

    #[account(
        mut,
        seeds = [b"miner_weights",subnet_state.key().as_ref()],
        bump,
    )]
    pub miner_weights: AccountLoader<'info, MinerWeights>,

    #[account(
        init_if_needed,
        space = 8 + MinerState::LEN,
        payer = owner,
        seeds = [b"miner_state",subnet_state.key().as_ref(),owner.key().as_ref()],
        bump
    )]
    pub miner_state: Box<Account<'info, MinerState>>,

    // 系统奖励代币
    #[account(
            mut,
            seeds = [b"tao", subnet_state.key().as_ref()], 
            bump,
        )]
    pub tao_mint: Box<Account<'info, Mint>>,

    // mine's tao token account
    #[account(mut)]
    pub user_tao_ata: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}
