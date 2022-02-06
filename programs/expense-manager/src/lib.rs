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

    pub fn income_amt(ctx:Context<IncomeAmt>,inc_amt:i32) ->ProgramResult {
        let base_account=&mut ctx.accounts.base_account;
        let user=&mut ctx.accounts.user;
        let calculate=IncomeStruct {
            amt:inc_amt,
            user_address:*user.to_account_info().key,
        };
        base_account.inc_list.push(calculate);
        base_account.total_amount+=inc_amt;
        Ok(())
    }

    pub fn spend_amt(ctx:Context<SpendAmt>,spend_amt:i32) ->ProgramResult {
        let base_account=&mut ctx.accounts.base_account;
        let user=&mut ctx.accounts.user;
        let calculate=SpendStruct {
            amt:spend_amt,
            user_address:*user.to_account_info().key,
        };
        base_account.spend_list.push(calculate);
        base_account.total_amount-=spend_amt;
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
pub struct IncomeAmt<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  pub user:Signer<'info>
}

#[derive(Accounts)]
pub struct SpendAmt<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    pub user:Signer<'info>
  }

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct IncomeStruct {
    pub amt: i32,
    pub user_address: Pubkey,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct SpendStruct {
    pub amt: i32,
    pub user_address: Pubkey,
}

#[account]
pub struct BaseAccount{
    pub total_amount: i32,
    pub inc_list:Vec<IncomeStruct>,
    pub spend_list:Vec<SpendStruct>
}
