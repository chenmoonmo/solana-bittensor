use crate::errors::ErrorCode;
use crate::states::*;
use anchor_lang::prelude::*;

use anchor_spl::{
    token,
    token::{Burn, Mint, Token, TokenAccount},
};

const MINER_REGISTER_FEE: u64 = 1 * 1_000_000_000;

pub fn initialize_subnet_miner(ctx: Context<InitializeSubnetMiner>) -> Result<()> {
    let tao_balance = ctx.accounts.user_tao_ata.amount;

    require!(
        tao_balance >= MINER_REGISTER_FEE,
        ErrorCode::NotEnoughBalance
    );

    let subnet_state = &mut ctx.accounts.subnet_state.load_mut()?;

    let bump = ctx.bumps.bittensor_state;
    let pda_sign: &[&[u8]; 2] = &[b"bittensor", &[bump]];

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

    if subnet_state.last_miner_id < i8::try_from(MAX_MINER_NUMBER - 1).unwrap() {
        let owner = ctx.accounts.owner.key();

        let miner_id = subnet_state.create_miner(owner);

        ctx.accounts
            .miner_state
            .initialize(miner_id, subnet_state.id, owner);
    } else {
        // 淘汰 前一个周期 bounds 最低且不在保护期的矿工

        let subnet_id = subnet_state.id;

        match subnet_state
            .miners
            .iter_mut()
            .filter(|v| v.protection == 0)
            .min_by_key(|v| v.last_weight)
        {
            Some(min_miner) => {
                ctx.accounts.miner_state.id = min_miner.id;
                ctx.accounts.miner_state.subnet_id = subnet_id;
                ctx.accounts.miner_state.owner = ctx.accounts.owner.key();
                ctx.accounts.miner_state.is_active = true;

                min_miner.stake = 0;
                min_miner.last_weight = 0;
                min_miner.reward = 0;
                min_miner.owner = ctx.accounts.owner.key();
                min_miner.protection = 1;
                // min_miner.pda = ctx.accounts.miner_state.key();
            }
            None => {
                require!(false, ErrorCode::NoMinerCanReplace)
            }
        }

        // TODO: 没领取的奖励怎么办
    }

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeSubnetMiner<'info> {
    #[account(
        mut,
        seeds = [b"bittensor"],
        bump,
    )]
    pub bittensor_state: AccountLoader<'info, BittensorState>,

    #[account(mut)]
    pub subnet_state: AccountLoader<'info, SubnetState>,

    #[account(
        init_if_needed,
        space = 10 * 1024,
        payer = owner,
        seeds = [b"miner_state",subnet_state.key().as_ref(),owner.key().as_ref()],
        bump
    )]
    pub miner_state: Box<Account<'info, MinerState>>,

    // 系统奖励代币
    #[account(
            mut,
            seeds = [b"tao", bittensor_state.key().as_ref()], 
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
