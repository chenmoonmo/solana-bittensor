use crate::errors::ErrorCode;
use crate::states::*;
use anchor_lang::prelude::*;

pub fn register_bittensor_validator(ctx: Context<RegisterBittensorValidator>) -> Result<()> {
    let validators = &mut ctx.accounts.bittensor_state.load_mut()?.validators;

    let validator_id = ctx.accounts.validator_state.id;
    let stake = ctx.accounts.validator_state.stake;
    let subnet_id = ctx.accounts.subnet_state.load()?.id;

    let is_exist = validators
        .iter()
        .any(|validator| validator.id == validator_id && validator.subnet_id == subnet_id);

    // 已经主网验证人
    require!(!is_exist, ErrorCode::ValidatorExist);

    for validator in validators.iter_mut() {
        if validator.id == 0 {
            validator.id = validator_id;
            validator.subnet_id = subnet_id;
            validator.stake = stake;
            validator.owner = *ctx.accounts.owner.key;
            break;
        }
    }

    Ok(())
}

#[derive(Accounts)]
pub struct RegisterBittensorValidator<'info> {
    #[account(
        seeds = [b"bittensor"],
        bump,
    )]
    pub bittensor_state: AccountLoader<'info, BittensorState>,

    #[account(mut)]
    pub subnet_state: AccountLoader<'info, SubnetState>,

    #[account(
        seeds = [b"validator_state",subnet_state.key().as_ref(),owner.key().as_ref()],
        bump
    )]
    pub validator_state: Account<'info, ValidatorState>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}