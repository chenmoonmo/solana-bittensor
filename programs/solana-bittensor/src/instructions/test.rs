use crate::states::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Test<'info> {
    #[account(mut)]
    pub subnet_state: AccountLoader<'info, SubnetState>,
}

pub fn test(ctx: Context<Test>) -> Result<()> {
    for account in ctx.remaining_accounts.iter() {
        // let _account_key: Pubkey = account.key();
        let mut data = account.try_borrow_mut_data()?;
        let mut account_to_write =
        ValidatorState::try_deserialize(&mut data.as_ref()).expect("Error Deserializing Data");


        account_to_write.id = 100;
        account_to_write.try_serialize(&mut data.as_mut())?;
    }

    Ok(())
}
