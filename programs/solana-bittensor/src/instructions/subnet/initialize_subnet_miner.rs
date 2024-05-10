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
    let subnet_miners = &mut ctx.accounts.subnet_miners.load_mut()?;
    let owner = ctx.accounts.owner.key();
    let pubkey = ctx.accounts.miner_state.key();

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

    let mut event = MinerRegisterEvent {
        id: 0,
        subnet_id: subnet_state.id,
        owner,
        stake: 0,
        pubkey,
        pre_pubkey: Pubkey::default(),
    };

    if subnet_miners.last_miner_id < i8::try_from(MAX_MINER_NUMBER - 1).unwrap() {
        let miner_id = subnet_miners.create_miner(owner, pubkey);

        event.id = miner_id;

        ctx.accounts
            .miner_state
            .initialize(miner_id, subnet_state.id, owner);
        
    } else {
        // 淘汰 前一个周期 bounds 最低且不在保护期的矿工

        match subnet_miners
            .miners
            .iter_mut()
            .filter(|v| v.protection == 0)
            .min_by_key(|v| v.last_weight)
        {
            Some(min_miner) => {

                event.pre_pubkey = min_miner.pubkey;
                event.id = min_miner.id;

                ctx.accounts.miner_state.id = min_miner.id;
                ctx.accounts.miner_state.subnet_id = subnet_state.id;
                ctx.accounts.miner_state.owner = owner;

                min_miner.stake = 0;
                min_miner.last_weight = 0;
                min_miner.reward = 0;
                min_miner.protection = 1;
                min_miner.owner = owner;
                min_miner.pubkey = pubkey;

                // 将矿工的得分清零
                ctx.accounts
                    .subnet_epoch
                    .load_mut()?
                    .remove_miner_weights(min_miner.id);
            }
            None => {
                require!(false, ErrorCode::NoMinerCanReplace)
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
    pub subnet_epoch: AccountLoader<'info, SubnetEpochState>,

    #[account(
        mut,
        seeds = [b"subnet_miners",subnet_state.key().as_ref()],
        bump
    )]
    pub subnet_miners: AccountLoader<'info, SubnetMiners>,

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
