use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount};
use std::collections::HashSet;

declare_id!("YOUR_PROGRAM_ID"); // Replace with your program ID

#[account]
pub struct TokenState {
    pub total_supply: u64,
    pub airdrop_amount: u64,
    pub liquidity_amount: u64,
    pub dev_fund_amount: u64,
}

#[program]
pub mod token {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, total_supply: u64) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.total_supply = total_supply; // Total supply of 400 billion
        state.airdrop_amount = 200_000_000_000; // 200 billion for airdrop
        state.liquidity_amount = 100_000_000_000; // 100 billion for liquidity
        state.dev_fund_amount = 100_000_000_000; // 100 billion for development
        state.whitelisted = HashSet::new(); // Initialize empty whitelist
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 64)] // Adjust space as needed
    pub state: Account<'info, TokenState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}