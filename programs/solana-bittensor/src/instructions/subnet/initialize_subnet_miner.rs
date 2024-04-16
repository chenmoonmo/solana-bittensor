use crate::states::*;
use crate::errors::ErrorCode;
use anchor_lang::prelude::*;

use anchor_spl::{
    token,
    token::{Burn, Mint, Token, TokenAccount},
};

const MINER_REGISTER_FEE: u64 = 1 * 1_000_000_000;

pub fn initialize_subnet_miner(ctx: Context<InitializeSubnetMiner>) -> Result<()> {
    // TODO:
    // 矿工保护期初始化

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

    if subnet_state.last_miner_id == i8::try_from(MAX_MINER_NUMBER - 1).unwrap() {
        // 淘汰 前一个周期 bounds 最低且不在保护期的矿工

        let mut min_miner_id = 0;

        for miner in subnet_state.miners {
            // TODO: 矿工保护期
            if miner.last_weight < subnet_state.miners[min_miner_id as usize].last_weight {
                min_miner_id = miner.id;
            }
        }

        // 在 remaining accounts 中找到对应的矿工账户
        // 修改该矿工的状态
        // 如果用户没有传入账户，那么报错
        let mut is_find_current_account = false;

        for account in ctx.remaining_accounts.iter() {
            let mut data = account.try_borrow_mut_data()?;
            let mut account_to_write: MinerState =
                MinerState::try_deserialize(&mut data.as_ref()).expect("Error Deserializing Data");

            if account_to_write.id == min_miner_id {
                account_to_write.is_active = false;
                account_to_write.try_serialize(&mut data.as_mut())?;
                is_find_current_account = true;
                break;
            }
        }

        require!(is_find_current_account, ErrorCode::CantFindAtRemainingAccounts);

        ctx.accounts.miner_state.id = min_miner_id;
        ctx.accounts.miner_state.subnet_id = subnet_state.id;
        ctx.accounts.miner_state.owner = ctx.accounts.owner.key();
        ctx.accounts.miner_state.is_active = true;

        subnet_state.miners[min_miner_id as usize].stake = 0;
        subnet_state.miners[min_miner_id as usize].last_weight = 0;
        // TODO: 没领取的奖励怎么办
        subnet_state.miners[min_miner_id as usize].reward = 0;
        subnet_state.miners[min_miner_id as usize].owner = ctx.accounts.owner.key();
    } else {
        let owner = ctx.accounts.owner.key();

        let miner_id = subnet_state.create_miner(owner);

        ctx.accounts
            .miner_state
            .initialize(miner_id, subnet_state.id, owner);
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
        init,
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
