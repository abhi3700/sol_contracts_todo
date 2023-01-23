use anchor_lang::prelude::*;

pub mod constant;
pub mod error;
pub mod states;

use crate::{constant::*, error::*, states::*};

// need to add the program id after generating during build process
// `$ solana address -k target/deploy/todo-keypair.json`
declare_id!("2eXx4gGsLzQdwNNjvgpkK868LxSSwAfYZ2UYbnsLQd49");

#[program]
pub mod todo {
    use super::*;

    // `ctx` represents the struct
    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        // initialize
        let user_profile = &mut ctx.accounts.user_profile;
        user_profile.authority = ctx.accounts.authority.key();
        user_profile.last_todo = 0; // initialize the value
        user_profile.todo_count = 0; // intialize the value

        Ok(()) // () is returned as what is defined inside `Result<()>`
    }
}

// `'info` is for defining the lifetime
#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(mut)]
    authority: Signer<'info>,

    #[account(
        init,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
        payer = authority,
        // 8 (First 8 bytes are default account discriminator) + max_size(UserProfile)
        space = 8 + std::mem::size_of::<UserProfile>()  
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,
    pub system_program: Program<'info, System>, // it's needed
}
