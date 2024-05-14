use crate::errors::ErrorCode;
use crate::states::*;
use anchor_lang::prelude::*;

// 奖励验证人
pub fn reward_subnet_validators(ctx: Context<RewardSubnetValidators>) -> Result<()> {
    let timestamp = Clock::get()?.unix_timestamp;
    let subnet_validators = &mut ctx.accounts.subnet_validators.load_mut()?;

    require!(
        ctx.accounts
            .subnet_state
            .weights_staus
            .into_iter()
            .all(|i| i == 3),
        ErrorCode::InvalidEndStep
    );

    let validator_bounds = subnet_validators
        .validators
        .iter()
        .map(|v| v.used_weights as u64 * v.stake)
        .collect::<Vec<u64>>();

    let total_bounds: u64 = validator_bounds.iter().sum();

    for i in 0..MAX_VALIDATOR_NUMBER {
        let reward = (validator_bounds[i] as u128)
            .checked_mul(10_000_000_000 as u128)
            .unwrap()
            .checked_div(total_bounds as u128)
            .unwrap_or(0) as u64;

        subnet_validators.validators[i].bounds = validator_bounds[i];
        subnet_validators.validators[i].reward += reward;

        if subnet_validators.validators[i].protection > 0 {
            subnet_validators.validators[i].protection -= 1;
        }
    }

    subnet_validators.end_epoch();
    ctx.accounts.subnet_state.end_epoch(timestamp);

    Ok(())
}

#[derive(Accounts)]
pub struct RewardSubnetValidators<'info> {
    #[account(
        mut,
        seeds = [b"subnet_state"],
        bump
    )]
    pub subnet_state: Box<Account<'info, SubnetState>>,

    #[account(
        mut,
        seeds = [b"subnet_validators",subnet_state.key().as_ref()],
        bump
    )]
    pub subnet_validators: AccountLoader<'info, SubnetValidators>,
}
