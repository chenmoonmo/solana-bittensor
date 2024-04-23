use crate::errors::ErrorCode;
use crate::states::*;
use anchor_lang::prelude::*;
use anchor_spl::{
    token,
    token::{Burn, Mint, Token, TokenAccount, Transfer},
};

pub const VALIDATOR_REGISTER_FEE: u64 = 1 * 1_000_000_000;

pub fn initialize_subnet_validator(
    ctx: Context<InitializeSubnetValidator>,
    stake_amount: u64,
) -> Result<()> {
    let subnet_validators = &mut ctx.accounts.subnet_validators.load_mut()?;
    let owner = ctx.accounts.owner.key();
    let pubkey = ctx.accounts.validator_state.key();

    let tao_balance = ctx.accounts.user_tao_ata.amount;

    require!(
        tao_balance >= VALIDATOR_REGISTER_FEE + stake_amount,
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

    // 如果是淘汰验证人，需要验证质押数量是否大于前64个验证人中最小的质押数量
    let min_stake_amount = subnet_validators.get_min_stake();

    require!(
        stake_amount >= min_stake_amount,
        ErrorCode::StakeAmountTooLow
    );

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
        )?;
    }

    // 验证人没满
    if subnet_validators.last_validator_id < i8::try_from(MAX_VALIDATOR_NUMBER - 1).unwrap() {
        let owner = ctx.accounts.owner.key();

        let validator_id = subnet_validators.create_validator(owner, pubkey, stake_amount);

        let validator_state = &mut ctx.accounts.validator_state;
        validator_state.id = validator_id;
        validator_state.owner = owner;
        validator_state.stake = stake_amount;
    } else {
        // 如果验证人已经满了
        // 淘汰 前一个周期 bounds 最低且不在保护期的验证人

        //TODO: 获取验证人质押排序前10中的最小质押

        match subnet_validators
            .validators
            .iter_mut()
            .filter(|v| v.protection == 0)
            .min_by_key(|v| v.bounds)
        {
            Some(min_validator) => {
                // 修改该验证人的状态
                // 将 subnet 的验证人替换为新的验证人
                ctx.accounts.validator_state.id = min_validator.id;
                ctx.accounts.validator_state.owner = owner;
                ctx.accounts.validator_state.stake += stake_amount;

                min_validator.bounds = 0;
                min_validator.stake = ctx.accounts.validator_state.stake;
                min_validator.reward = 0;
                min_validator.protection = 1;
                min_validator.owner = owner;
                min_validator.pubkey = pubkey;

                // 将验证人的打分清零
                ctx.accounts
                    .subnet_epoch
                    .load_mut()?
                    .remove_weights(min_validator.id);
            }
            None => {
                require!(false, ErrorCode::NoValidatorCanReplace)
            }
        }
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
    pub subnet_state: Box<Account<'info, SubnetState>>,

    #[account(
        mut,
        seeds = [b"subnet_epoch",subnet_state.key().as_ref()],
        bump
    )]
    pub subnet_epoch: AccountLoader<'info, SubnetEpochState>,

    #[account(
        init_if_needed,
        space = 1024 * 10,
        payer = owner,
        seeds = [b"validator_state",subnet_state.key().as_ref(),owner.key().as_ref()],
        bump
    )]
    pub validator_state: Account<'info, ValidatorState>,

    #[account(
        mut,
        seeds = [b"subnet_validators",subnet_state.key().as_ref()],
        bump
    )]
    pub subnet_validators: AccountLoader<'info, SubnetValidators>,

    // 系统奖励代币
    #[account(
        mut,
        seeds = [b"tao", bittensor_state.key().as_ref()], 
        bump,
    )]
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
