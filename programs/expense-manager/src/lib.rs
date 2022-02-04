use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod expense_manager {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let base_account=&mut ctx.accounts.base_account;
        base_account.total_amount=0;
        Ok(())
    }

    pub fn add_amt(ctx:Context<AddAmt>,expense_amt:i32) ->ProgramResult {
        let base_account=&mut ctx.accounts.base_account;
        let user=&mut ctx.accounts.user;
        println!("hello integer{}",expense_amt);
        let calculate=ItemStruct {
            expense_amt:expense_amt,
            user_address:*user.to_account_info().key,
        };
        base_account.expense_list.push(calculate);
        base_account.total_amount+=expense_amt;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account:Account<'info,BaseAccount>,
    #[account(mut)]
    user:Signer<'info>,
    pub system_program:Program<'info,System>
}

#[derive(Accounts)]
pub struct AddAmt<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  pub user:Signer<'info>
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub expense_amt: i32,
    pub user_address: Pubkey,
}

#[account]
pub struct BaseAccount{
    pub total_amount: i32,
    pub expense_list:Vec<ItemStruct>
}
