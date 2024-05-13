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

    let subnet_state = &mut ctx.accounts.subnet_state;
    let owner = ctx.accounts.owner.key();
    let pubkey = ctx.accounts.miner_state.key();

    let bump = ctx.bumps.subnet_state;
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

    let mut event = MinerRegisterEvent {
        id: 0,
        owner,
        stake: 0,
        pubkey,
        pre_pubkey: Pubkey::default(),
    };

    // 首先找到一个 last_miner_id 为 <  100 的 subnet_miners 矿工组
    match [
        ctx.accounts.subnet_miners.load_mut()?,
        ctx.accounts.subnet_miners1.load_mut()?,
        ctx.accounts.subnet_miners2.load_mut()?,
        ctx.accounts.subnet_miners3.load_mut()?,
        ctx.accounts.subnet_miners4.load_mut()?,
        ctx.accounts.subnet_miners5.load_mut()?,
        ctx.accounts.subnet_miners6.load_mut()?,
        ctx.accounts.subnet_miners7.load_mut()?,
        ctx.accounts.subnet_miners8.load_mut()?,
        ctx.accounts.subnet_miners9.load_mut()?,
    ]
    .iter_mut()
    .find(|v| v.last_miner_id < i8::try_from(99).unwrap())
    {
        Some(miners) => {
            let miner_id = miners.create_miner(owner, pubkey);

            event.id = miner_id;

            ctx.accounts.miner_state.initialize(miner_id, owner);
        }
        None => {
            // 如果没找到，则从全部的矿工组中淘汰一个得分最低的矿工
            match [
                ctx.accounts.subnet_miners.load_mut()?,
                ctx.accounts.subnet_miners1.load_mut()?,
                ctx.accounts.subnet_miners2.load_mut()?,
                ctx.accounts.subnet_miners3.load_mut()?,
                ctx.accounts.subnet_miners4.load_mut()?,
                ctx.accounts.subnet_miners5.load_mut()?,
                ctx.accounts.subnet_miners6.load_mut()?,
                ctx.accounts.subnet_miners7.load_mut()?,
                ctx.accounts.subnet_miners8.load_mut()?,
                ctx.accounts.subnet_miners9.load_mut()?,
            ]
            .iter_mut()
            .flat_map(|v| v.miners.iter_mut())
            .filter(|v| v.protection == 0)
            .min_by_key(|v| v.last_weight)
            {
                Some(min_miner) => {
                    event.pre_pubkey = min_miner.pubkey;
                    event.id = min_miner.id;

                    ctx.accounts.miner_state.id = min_miner.id;
                    ctx.accounts.miner_state.owner = owner;

                    min_miner.stake = 0;
                    min_miner.last_weight = 0;
                    min_miner.reward = 0;
                    min_miner.protection = 1;
                    min_miner.owner = owner;
                    min_miner.pubkey = pubkey;

                    // TODO: 将矿工的得分清零
                }
                None => {
                    require!(false, ErrorCode::NoMinerCanReplace)
                }
            }
        }
    }

    emit!(event);
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeSubnetMiner<'info> {
    #[account(
        mut,
        seeds = [b"subnet_state"],
        bump,
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
