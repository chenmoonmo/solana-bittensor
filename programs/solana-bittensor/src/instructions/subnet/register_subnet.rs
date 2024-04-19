use crate::states::*;
use anchor_lang::prelude::*;

pub fn register_subnet(ctx: Context<RegisterSubnet>) -> Result<()> {
    ctx.accounts
        .subnet_state
        .load_init()?
        .register(ctx.accounts.owner.key());

    ctx.accounts.subnet_epoch.load_init()?;

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
        space = 10 * 1024,
        seeds = [b"subnet_state",owner.key().as_ref()],
        bump
    )]
    pub subnet_state: AccountLoader<'info, SubnetState>,

    #[account(
        init,
        payer = owner,
        space = 10 * 1024,
        seeds = [b"subnet_epoch",subnet_state.key().as_ref()],
        bump
    )]
    pub subnet_epoch: AccountLoader<'info, SubnetEpochState>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
