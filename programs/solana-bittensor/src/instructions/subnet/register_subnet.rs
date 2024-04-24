use crate::states::*;
use anchor_lang::prelude::*;

pub fn register_subnet(ctx: Context<RegisterSubnet>) -> Result<()> {
    ctx.accounts.subnet_state.register(
        ctx.accounts.owner.key(),
        ctx.accounts.subnet_epoch.key(),
        ctx.accounts.subnet_miners.key(),
        ctx.accounts.subnet_validators.key(),
    );
    
    ctx.accounts.subnet_epoch.load_init()?;
    ctx.accounts.subnet_miners.load_init()?;
    ctx.accounts.subnet_validators.load_init()?;
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
        space = 8 + SubnetState::LEN,
        seeds = [b"subnet_state",owner.key().as_ref()],
        bump
    )]
    pub subnet_state: Box<Account<'info, SubnetState>>,

    #[account(
        init,
        payer = owner,
        space = 8 + SubnetEpochState::LEN,
        seeds = [b"subnet_epoch",subnet_state.key().as_ref()],
        bump
    )]
    pub subnet_epoch: AccountLoader<'info, SubnetEpochState>,

    #[account(
        init,
        payer = owner,
        space = 8 + SubnetMiners::LEN,
        seeds = [b"subnet_miners",subnet_state.key().as_ref()],
        bump
    )]
    pub subnet_miners: AccountLoader<'info, SubnetMiners>,

    #[account(
        init,
        payer = owner,
        space = 8 + SubnetValidators::LEN,
        seeds = [b"subnet_validators",subnet_state.key().as_ref()],
        bump
    )]
    pub subnet_validators: AccountLoader<'info, SubnetValidators>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
