use crate::errors::ErrorCode;
use crate::states::*;
use anchor_lang::prelude::*;

pub fn register_bittensor_validator(ctx: Context<RegisterBittensorValidator>) -> Result<()> {
    let bittensor_state = &mut ctx.accounts.bittensor_state.load_mut()?;

    let validator_state = &mut ctx.accounts.validator_state;
    let validator_id = validator_state.id;
    let stake = validator_state.stake;
    let owner = validator_state.owner;
    let subnet_id = ctx.accounts.subnet_state.load()?.id;

    let is_exist = bittensor_state
        .validators
        .iter()
        .any(|validator: &BittensorValidatorInfo| {
            validator.validator_id == validator_id
                && validator.subnet_id == subnet_id
                && validator.owner != Pubkey::default()
        });

    // 已经主网验证人
    require!(!is_exist, ErrorCode::ValidatorExist);

    // TODO: id
    bittensor_state.create_bittensor_validator(owner, subnet_id, validator_id, stake);

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
