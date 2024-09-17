// use anchor_lang::prelude::*;
// use anchor_spl::token::{self, Mint, TokenAccount};

// declare_id!("YOUR_PROGRAM_ID"); // Replace with your program ID

// #[account]
// pub struct DevFundState {
//     pub total_fund: u64,
//     pub released_fund: u64,
//     pub release_time: i64,
// }

// #[program]
// pub mod dev_fund {
//     use super::*;

//     pub fn initialize(ctx: Context<Initialize>, total_fund: u64) -> Result<()> {
//         let state = &mut ctx.accounts.state;
//         state.total_fund = total_fund; // Total development fund (100 billion)
//         state.released_fund = 0; // Initially no funds released
//         state.release_time = Clock::get()?.unix_timestamp; // Set initial release time
//         Ok(())
//     }

//     pub fn release_dev_funds(ctx: Context<ReleaseDevFunds>, amount: u64) -> Result<()> {
//         let state = &mut ctx.accounts.state;

//         // Check if funds can be released based on vesting logic
//         if state.released_fund + amount > state.total_fund {
//             return Err(ErrorCode::ExceedsTotalFund.into());
//         }

//         // Logic to release funds
//         state.released_fund += amount; 
//         Ok(())
//     }
// }

// #[derive(Accounts)]
// pub struct Initialize<'info> {
//     #[account(init, payer = user, space = 8 + 64)] // Adjust space as needed
//     pub state: Account<'info, DevFundState>,
//     #[account(mut)]
//     pub user: Signer<'info>,
//     pub system_program: Program<'info, System>,
// }

// #[derive(Accounts)]
// pub struct ReleaseDevFunds<'info> {
//     pub admin: Signer<'info>,
//     pub state: Account<'info, DevFundState>,
// }

use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Mint};

#[account]
pub struct FundAccount {
    pub total_allocated: u64,
    pub total_released: u64,
    pub vesting_start_time: i64,
}

#[derive(Accounts)]
pub struct AllocateTokens<'info> {
    #[account(init, payer = authority, space = 8 + FundAccount::LEN)]
    pub fund_account: Account<'info, FundAccount>,
    #[account(mut)]
    pub fund_token_account: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, token::Token>,
}

#[derive(Accounts)]
pub struct ReleaseTokens<'info> {
    #[account(mut)]
    pub fund_account: Account<'info, FundAccount>,
    #[account(mut)]
    pub recipient_token_account: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, token::Token>,
}

#[program]
pub mod development_fund {
    use super::*;

    const VESTING_PERIOD_MONTHS: u64 = 24;
    const TOKENS_PER_MONTH: u64 = 100_000_000_000 / VESTING_PERIOD_MONTHS; // 100 billion total

    pub fn allocate_tokens(ctx: Context<AllocateTokens>, amount: u64) -> Result<()> {
        let fund_account = &mut ctx.accounts.fund_account;

        // Ensure that the allocation amount is greater than zero
        if amount == 0 {
            return Err(ErrorCode::InvalidAmount.into());
        }

        fund_account.total_allocated = amount;
        fund_account.total_released = 0;
        fund_account.vesting_start_time = Clock::get()?.unix_timestamp;

        Ok(())
    }

    pub fn release_tokens(ctx: Context<ReleaseTokens>) -> Result<()> {
        let fund_account = &mut ctx.accounts.fund_account;

        // Calculate how many months have passed since vesting started
        let current_time = Clock::get()?.unix_timestamp;
        let elapsed_months = (current_time - fund_account.vesting_start_time) / 2_592_000; // seconds in a month

        // Ensure we do not release more than allocated
        if elapsed_months > VESTING_PERIOD_MONTHS {
            elapsed_months = VESTING_PERIOD_MONTHS; // Cap at max months
        }

        let total_releasable = elapsed_months * TOKENS_PER_MONTH;

        // Ensure we do not exceed already released tokens
        if total_releasable <= fund_account.total_released {
            return Err(ErrorCode::NoTokensAvailable.into());
        }

        let amount_to_release = total_releasable - fund_account.total_released;

        // Transfer tokens to recipient's account
        let cpi_accounts = token::Transfer {
            from: ctx.accounts.fund_token_account.to_account_info(),
            to: ctx.accounts.recipient_token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };

        token::transfer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            amount_to_release,
        )?;

        // Update released amount
        fund_account.total_released += amount_to_release;

        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid allocation amount provided.")]
    InvalidAmount,

    #[msg("No tokens available for release.")]
    NoTokensAvailable,
}