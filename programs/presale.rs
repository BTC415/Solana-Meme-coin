use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

#[program]
pub mod presale {
    use super::*;

    pub fn buy_tokens(ctx: Context<BuyTokens>, amount: u64) -> Result<()> {
        let presale_account = &mut ctx.accounts.presale_account;

        // Ensure that the amount requested is greater than zero
        if amount == 0 {
            return Err(ErrorCode::InvalidAmount.into());
        }

        // Determine the price based on the amount being purchased
        let price_per_token = if presale_account.total_sold < 50_000_000_000 {
            0.0001 // Price for first 50 billion tokens
        } else if presale_account.total_sold < 100_000_000_000 {
            0.0002 // Price for next 50 billion tokens
        } else if presale_account.total_sold < 150_000_000_000 {
            0.0003 // Price for last 50 billion tokens
        } else {
            return Err(ErrorCode::PresaleEnded.into());
        };

        // Calculate total cost in lamports (1 SOL = 1_000_000_000 lamports)
        let total_cost = (amount as f64 * price_per_token * 1_000_000_000.0) as u64;

        //Check if buyer has sent enough SOL
        let buyer_balance = ctx.accounts.authority.lamports();
        if buyer_balance < total_cost {
            return Err(ErrorCode::InsufficientFunds.into());
        }

        // Mint tokens to buyer's account
        let cpi_accounts = token::MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.buyer_token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };

        token::mint_to(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            amount,
        )?;

        // Update total sold safely
        presale_account.total_sold = presale_account
            .total_sold
            .checked_add(amount)
            .ok_or(ErrorCode::OverflowError)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct BuyTokens<'info> {
    #[account(mut)]
    pub presale_account: Account<'info, PresaleAccount>,
    #[account(mut)]
    pub buyer_token_account: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct PresaleAccount {
    pub total_sold: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Presale has ended.")]
    PresaleEnded,
    #[msg("Invalid amount provided for purchase.")]
    InvalidAmount,
    #[msg("Insufficient funds provided.")]
    InsufficientFunds,
    #[msg("Overflow occured when updating total sold.")]
    OverflowError
  }
