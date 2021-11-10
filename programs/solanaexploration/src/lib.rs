use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod solanaexploration {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        // Get a reference to the account
        let base_account = &mut ctx.accounts.base_account;
        // initialize total_gifs
        base_account.total_gifs = 0;
        Ok(())
    }

    pub fn add_gif(ctx: Context<AddGif>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.total_gifs += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    // init - tell solana to create new account
    // payer = user - payer of the account is user
    // space = 9000 - allocate 9000 bytes for account
    //
    // note that solana users pay to rent their accounts. If you don't pay
    // rent then validators will clear the account.
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,

    // data passed to the program that proves that the user calling
    // actually owns their wallet account
    #[account(mut)]
    pub user: Signer<'info>,

    // reference to SystemProgram which is responsible to create accounts
    // in Solana.
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
}
