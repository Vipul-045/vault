use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};

declare_id!("69TavcFc8E4cjPi5z7kds5YzP4UZJjFCxKX32b1nY8m4");

#[program]
pub mod vault {
    use anchor_spl::token::{self, Transfer};
    use super::*;

    pub fn initialize(ctx: Context<InitializeVault>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.authority = ctx.accounts.authority.key();
        vault.authority_bump = ctx.bump.vault_authority;
        Ok(())
}                               
}

#[derive(Accounts)]
pub struct InitializeVault <'info>{

    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 1,
    )]
    pub vault : Account<'info, Vault>,

    #[account(
        seeds = [ b"vault_authority".as_ref(), vault.key().as_ref()],
        bump
    )]
    pub vault_authority: UncheckedAccount<'info>,

    #[account(
        init,
        payer = authority,
        associated_token::mint = mint,
        associated_token::authority = vault_authority,
    )]

    pub vault_token_Account: Account<'info, TokenAccount>,

    pub mint : Account<'info, Mint>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program : Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program : Program<'info, AssociatedToken>,

    pub rent : Sysvar<'info, Rent>,
}
