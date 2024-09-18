use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

#[program]
pub mod token {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, total_supply: u64) -> Result<()> {
       let mint = &mut ctx.accounts.mint;

       //Ensure total supply is not zero
        if total_supply == 0 {
            return Err(ErrorCode::InvalidSupply.into());
        }

       token::initialize_mint(
        ctx.accounts.token_program.to_account_info(),
        mint, 
        &ctx.accounts.authority.key(),
        None,
        9, //Number of decimals
       )?;

       //Mint initial supply to the associated token account
       let cpi_accounts = token::MintTo {
        mint:mint.to_account_info(),
        to:ctx.accounts.token_account.to_account_info(),
        authority:ctx.accounts.authority.to_account_info(),
       };
       token:::mint_to(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        total_supply,
       )?;

       Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + Mint::LEN)]
    pub mint:Account<'info, TokenAccount>,
    #[account(init, payer = authority, space = 8 + TokenAccount::LEN)]
    pub token_account: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    pub token_program:Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid total supply provided.")]
    InvalidSupply,
}
