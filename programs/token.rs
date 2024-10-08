use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, System, Token, TokenAccount};

#[program]
pub mod token {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, total_supply: u64) -> Result<()> {
        let mint = &mut ctx.accounts.mint;

        //Ensure total supply is not zero
        if total_supply == 0 {
            return Err(error!(ErrorCode::InvalidSupply));
        }

        //Token Program Account
        let cpi_program = ctx.accounts.token_program.to_account_info();

        //Create a new mint account
        token::initialize_mint(
            cpi_program, //Token Program Account
            mint,                                         //Mint account being initialized
            &ctx.accounts.authority.key(),                //Mint Authority
            None,                                         //Freeze Authority is set to None
            9,                                            //Number of decimals
        )?;

        //An instance of the MintTo struct
        let cpi_accounts = token::MintTo {
            mint: mint.to_account_info(),                        //Mint Account
            to: ctx.accounts.token_account.to_account_info(), //Destination Account where tokens will be sent
            authority: ctx.accounts.authority.to_account_info(), //Mint Authority
        };

        //Instance of CpiContext
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        //Actually mint the specified total supply of token to the token_account
        token::mint_to(cpi_ctx, total_supply)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    //This account will hold the SPL token mint information.
    //It is initialized with space allocated for a Mint Account and requires payment from the `authority`
    #[account(init, payer = authority, space = 8 + Mint::LEN)]
    pub mint: Account<'info, Mint>,

    //Token Account
    #[account(init, payer = authority, space = 8 + TokenAccount::LEN)]
    pub token_account: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,    //Token Program Account
    pub system_program: Program<'info, System>,  //System Program Account
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid total supply provided.")]
    InvalidSupply,
}