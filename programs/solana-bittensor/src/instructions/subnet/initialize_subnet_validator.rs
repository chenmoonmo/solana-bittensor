use crate::errors::ErrorCode;
use crate::states::*;
use anchor_lang::prelude::*;

use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount, Transfer};

pub const VALIDATOR_REGISTER_FEE: u64 = 1 * 1_000_000_000;

pub fn initialize_subnet_validator(
    ctx: Context<InitializeSubnetValidator>,
    stake_amount: u64,
) -> Result<()> {
    let tao_balance = ctx.accounts.user_tao_ata.amount;
    require!(
        tao_balance >= VALIDATOR_REGISTER_FEE,
        ErrorCode::NotEnoughBalance
    );

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
        VALIDATOR_REGISTER_FEE,
    )?;

    // TODO: 从用户账户转移代币到系统账户
    // 如果是淘汰验证人，需要验证质押数量是否大于前64个验证人中最小的质押数量

    if stake_amount > 0 {
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.user_tao_ata.to_account_info(),
                    to: ctx.accounts.tao_stake.to_account_info(),
                    authority: ctx.accounts.owner.to_account_info(),
                },
            ),
            stake_amount,
        );
    }

    let subnet_state = &mut ctx.accounts.subnet_state.load_mut()?;

    // 验证人已经满了
    if subnet_state.last_validator_id == i8::try_from(MAX_VALIDATOR_NUMBER - 1).unwrap() {
        // 淘汰 前一个周期 bounds 最低且不在保护期的验证人

        let mut min_validator_id = 0;

        for validator in subnet_state.validators {
            if validator.bounds < subnet_state.validators[min_validator_id as usize].bounds
                && subnet_state.validators[min_validator_id as usize].protection == 0
            {
                min_validator_id = validator.id;
            }
        }

        // 在 remaining accounts 中找到对应的验证人账户
        // 修改该验证人的状态
        let mut is_find_current_account = false;

        for account in ctx.remaining_accounts.iter() {
            let mut data = account.try_borrow_mut_data()?;
            let mut account_to_write = ValidatorState::try_deserialize(&mut data.as_ref())
                .expect("Error Deserializing Data");

            if account_to_write.id == min_validator_id {
                account_to_write.is_active = false;
                account_to_write.bounds = 0;

                account_to_write.try_serialize(&mut data.as_mut())?;
                is_find_current_account = true;
                break;
            }
        }

        require!(
            is_find_current_account,
            ErrorCode::CantFindAtRemainingAccounts
        );
        // 将 subnet 的验证人替换为新的验证人

        ctx.accounts.validator_state.id = min_validator_id;
        ctx.accounts.validator_state.owner = ctx.accounts.owner.key();
        ctx.accounts.validator_state.is_active = true;

        subnet_state.validators[min_validator_id as usize].bounds = 0;
        subnet_state.validators[min_validator_id as usize].stake = 0;
        subnet_state.validators[min_validator_id as usize].reward = 0;
        subnet_state.validators[min_validator_id as usize].owner = ctx.accounts.owner.key();
    } else {
        let owner = ctx.accounts.owner.key();

        let validator_id = subnet_state.create_validator(owner, stake_amount);

        let validator_state = &mut ctx.accounts.validator_state;
        validator_state.id = validator_id;
        validator_state.owner = owner;
        validator_state.stake = stake_amount;
    }

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeSubnetValidator<'info> {
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
        space = 1024 * 10,
        payer = owner,
        seeds = [b"validator_state",subnet_state.key().as_ref(),owner.key().as_ref()],
        bump
    )]
    pub validator_state: Account<'info, ValidatorState>,

    // 系统奖励代币
    #[account(
        mut,
        seeds = [b"tao", bittensor_state.key().as_ref()], 
        bump,
    )]
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
