use crate::errors::ErrorCode;
use crate::states::*;
use anchor_lang::prelude::*;

pub fn register_bittensor_validator(ctx: Context<RegisterBittensorValidator>) -> Result<()> {
    // TODO: 淘汰主网验证人
    let bittensor_state = &mut ctx.accounts.bittensor_state.load_mut()?;
    let validator_state = &mut ctx.accounts.validator_state;

    let validator_id = validator_state.id;
    let stake = validator_state.stake;
    let owner = validator_state.owner;
    let bounds = validator_state.bounds;
    let subnet_id = ctx.accounts.subnet_state.load()?.id;

    let is_existed = bittensor_state
        .validators
        .iter()
        .any(|v| v.validator_state == validator_state.key());

    // 已经主网验证人
    require!(!is_existed, ErrorCode::ValidatorExist);

    if bittensor_state.last_validator_id < i8::try_from(MAX_VALIDATOR_NUMBER - 1).unwrap() {
        bittensor_state.create_bittensor_validator(
            owner,
            validator_state.key(),
            subnet_id,
            validator_id,
            stake,
            bounds,
        );
    } else {
        // 找出工作量最少的验证人
        let min_validator = bittensor_state
            .validators
            .iter_mut()
            .filter(|v| v.protection == 0)
            .min_by_key(|v| v.stake)
            .unwrap();

        // 如果新的验证人的工作量大于最小的验证人，则替换
        require!(
            bounds > min_validator.stake,
            ErrorCode::ValidatorNotEnoughBounds
        );

        min_validator.stake = stake;
        min_validator.owner = owner;
        min_validator.bounds = bounds;
        min_validator.validator_id = validator_id;
        min_validator.subnet_id = subnet_id;
        min_validator.protection = 1;
        min_validator.validator_state = validator_state.key();
    }
    Ok(())
}

#[derive(Accounts)]
pub struct RegisterBittensorValidator<'info> {
    #[account(
        mut,
        seeds = [b"bittensor"],
        bump,
    )]
    pub bittensor_state: AccountLoader<'info, BittensorState>,

    #[account(mut)]
    pub subnet_state: AccountLoader<'info, SubnetState>,

    #[account(
        mut,
        seeds = [b"validator_state",subnet_state.key().as_ref(),owner.key().as_ref()],
        bump
    )]
    pub validator_state: Account<'info, ValidatorState>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
