use crate::errors::ErrorCode;
use crate::states::*;
use anchor_lang::prelude::*;

use anchor_spl::{
    token,
    token::{Burn, Mint, Token, TokenAccount},
};

pub const SUBNET_REGISTER_FEE: u64 = 10 * 1_000_000_000;

pub fn initialize_subnet(ctx: Context<InitializeSubnet>) -> Result<()> {
    let bittensor_state = &mut ctx.accounts.bittensor_state.load_mut()?;

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

    let owner = *ctx.accounts.owner.key;

    let last_subnet_id = bittensor_state.last_subnet_id;

    if last_subnet_id < i8::try_from(SUBNET_MAX_NUMBER - 1).unwrap() {
        let subnet_id = bittensor_state.create_subnet(owner);

        ctx.accounts
            .subnet_state
            .load_init()?
            .initialize(subnet_id, owner);

        ctx.accounts.subnet_epoch.load_init()?.epoch_start_timestamp = Clock::get()?.unix_timestamp;
    } else {
        // 找到不在保护期内的上个周期内得分最低的子网
        match bittensor_state
            .subnets
            .iter_mut()
            .filter(|s| s.protection == 0)
            .min_by_key(|s| s.last_weight)
        {
            Some(min_subnet) => {
                let subnet_id = min_subnet.id;
                min_subnet.owner = owner;
                min_subnet.distribute_reward = 0;
                min_subnet.stake = 0;
                min_subnet.protection = 1;

                ctx.accounts
                    .subnet_state
                    .load_init()?
                    .initialize(subnet_id, owner);

                ctx.accounts.subnet_epoch.load_init()?.epoch_start_timestamp =
                    Clock::get()?.unix_timestamp;
            }
            None => {}
        }
    }
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeSubnet<'info> {
    #[account(
        mut,
        seeds = [b"bittensor"],
        bump,
    )]
    pub bittensor_state: AccountLoader<'info, BittensorState>,

    #[account(
        init_if_needed,
        payer = owner,
        space = 10 * 1024,
        seeds = [b"subnet_state",owner.key().as_ref()],
        bump
    )]
    pub subnet_state: AccountLoader<'info, SubnetState>,

    #[account(
        init_if_needed,
        payer = owner,
        space = 10 * 1024,
        seeds = [b"subnet_epoch",subnet_state.key().as_ref()],
        bump
    )]
    pub subnet_epoch: AccountLoader<'info, SubnetEpochState>,

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
    pub user_tao_ata: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}
