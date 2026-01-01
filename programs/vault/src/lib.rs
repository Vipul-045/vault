use anchor_lang::prelude::*;

declare_id!("69TavcFc8E4cjPi5z7kds5YzP4UZJjFCxKX32b1nY8m4");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<InitializeVault>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeVault {}
