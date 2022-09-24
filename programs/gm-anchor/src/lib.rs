use anchor_lang::prelude::*;

declare_id!("kUQbXedCVEBGzBCazSVHpg7rayHxXCp7vf9HEgsSBuZ");

#[program]
pub mod gm_anchor {
   use super::*;
   pub fn execute(ctx: Context<Execute>, name: String, age: u8) -> Result<()> {
       let gm_account = &mut ctx.accounts.gm_account;

       gm_account.name = name;
       gm_account.age = age;
       msg!("GM {}, {}", gm_account.name, gm_account.age);
       Ok(())
   }
}

#[derive(Accounts)]
pub struct Execute<'info> {
   #[account(init, payer = user, space = 8 + 32 + 1)]
   pub gm_account: Account<'info, GreetingAccount>,
   #[account(mut)]
   pub user: Signer<'info>,
   pub system_program: Program<'info, System>,
}

#[account]
pub struct GreetingAccount {
   pub name: String,
   pub age: u8,
}