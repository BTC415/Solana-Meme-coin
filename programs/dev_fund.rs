use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount};

declare_id!("YOUR_PROGRAM_ID"); // Replace with your program ID

#[account]
pub struct DevFundState {
    pub total_fund: u64,
    pub released_fund: u64,
    pub release_time: i64,
}

#[program]
pub mod dev_fund {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, total_fund: u64) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.total_fund = total_fund; // Total development fund (100 billion)
        state.released_fund = 0; // Initially no funds released
        state.release_time = Clock::get()?.unix_timestamp; // Set initial release time
        Ok(())
    }

    pub fn release_dev_funds(ctx: Context<ReleaseDevFunds>, amount: u64) -> Result<()> {
        let state = &mut ctx.accounts.state;

        // Check if funds can be released based on vesting logic
        if state.released_fund + amount > state.total_fund {
            return Err(ErrorCode::ExceedsTotalFund.into());
        }

        // Logic to release funds
        state.released_fund += amount; 
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 64)] // Adjust space as needed
    pub state: Account<'info, DevFundState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ReleaseDevFunds<'info> {
    pub admin: Signer<'info>,
    pub state: Account<'info, DevFundState>,
}