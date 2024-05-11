use crate::states::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

pub fn register_subnet(ctx: Context<RegisterSubnet>) -> Result<()> {
    ctx.accounts.subnet_validators.load_init()?;

    ctx.accounts.subnet_state.register(ctx.accounts.owner.key());

    ctx.accounts.subnet_state.miners = [
        ctx.accounts.subnet_miners.key(),
        ctx.accounts.subnet_miners1.key(),
        ctx.accounts.subnet_miners2.key(),
        ctx.accounts.subnet_miners3.key(),
        ctx.accounts.subnet_miners4.key(),
        ctx.accounts.subnet_miners5.key(),
        ctx.accounts.subnet_miners6.key(),
        ctx.accounts.subnet_miners7.key(),
        ctx.accounts.subnet_miners8.key(),
        ctx.accounts.subnet_miners9.key(),
    ];

    ctx.accounts.subnet_miners.load_init()?.group_id = 0;
    ctx.accounts.subnet_miners1.load_init()?.group_id = 1;
    ctx.accounts.subnet_miners2.load_init()?.group_id = 2;
    ctx.accounts.subnet_miners3.load_init()?.group_id = 3;
    ctx.accounts.subnet_miners4.load_init()?.group_id = 4;
    ctx.accounts.subnet_miners5.load_init()?.group_id = 5;
    ctx.accounts.subnet_miners6.load_init()?.group_id = 6;
    ctx.accounts.subnet_miners7.load_init()?.group_id = 7;
    ctx.accounts.subnet_miners8.load_init()?.group_id = 8;
    ctx.accounts.subnet_miners9.load_init()?.group_id = 9;

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
        payer = owner,
        space = 8 + SubnetMiners::LEN,
        seeds = [b"subnet_miners 0",subnet_state.key().as_ref()],
        bump
    )]
    pub subnet_miners: AccountLoader<'info, SubnetMiners>,

    #[account(
        init,
        payer = owner,
        space = 8 + SubnetMiners::LEN,
        seeds = [b"subnet_miners 1",subnet_state.key().as_ref()],
        bump
    )]
    pub subnet_miners1: AccountLoader<'info, SubnetMiners>,

    #[account(
        init,
        payer = owner,
        space = 8 + SubnetMiners::LEN,
        seeds = [b"subnet_miners 2",subnet_state.key().as_ref()],
        bump
    )]
    pub subnet_miners2: AccountLoader<'info, SubnetMiners>,

    #[account(
        init,
        payer = owner,
        space = 8 + SubnetMiners::LEN,
        seeds = [b"subnet_miners 3",subnet_state.key().as_ref()],
        bump
    )]
    pub subnet_miners3: AccountLoader<'info, SubnetMiners>,

    #[account(
        init,
        payer = owner,
        space = 8 + SubnetMiners::LEN,
        seeds = [b"subnet_miners 4",subnet_state.key().as_ref()],
        bump
    )]
    pub subnet_miners4: AccountLoader<'info, SubnetMiners>,

    #[account(
        init,
        payer = owner,
        space = 8 + SubnetMiners::LEN,
        seeds = [b"subnet_miners 5",subnet_state.key().as_ref()],
        bump
    )]
    pub subnet_miners5: AccountLoader<'info, SubnetMiners>,

    #[account(
        init,
        payer = owner,
        space = 8 + SubnetMiners::LEN,
        seeds = [b"subnet_miners 6",subnet_state.key().as_ref()],
        bump
    )]
    pub subnet_miners6: AccountLoader<'info, SubnetMiners>,

    #[account(
        init,
        payer = owner,
        space = 8 + SubnetMiners::LEN,
        seeds = [b"subnet_miners 7",subnet_state.key().as_ref()],
        bump
    )]
    pub subnet_miners7: AccountLoader<'info, SubnetMiners>,

    #[account(
        init,
        payer = owner,
        space = 8 + SubnetMiners::LEN,
        seeds = [b"subnet_miners 8",subnet_state.key().as_ref()],
        bump
    )]
    pub subnet_miners8: AccountLoader<'info, SubnetMiners>,

    #[account(
        init,
        payer = owner,
        space = 8 + SubnetMiners::LEN,
        seeds = [b"subnet_miners 9",subnet_state.key().as_ref()],
        bump
    )]
    pub subnet_miners9: AccountLoader<'info, SubnetMiners>,

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
