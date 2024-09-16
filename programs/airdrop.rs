use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount};
use std::collections::HashSet;

declare_id!("YOUR_PROGRAM_ID"); // Replace with your program ID

#[account]
pub struct AirdropState {
    pub total_tokens: u64,
    pub distributed_tokens: u64,
    pub distribution_time: i64,
    pub whitelisted: HashSet<Pubkey>, // Track whitelisted addresses
}

#[program]
pub mod airdrop {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, total_tokens: u64) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.total_tokens = total_tokens; // Total airdrop tokens (200 billion)
        state.distributed_tokens = 0; // Initially no tokens distributed
        state.distribution_time = Clock::get()?.unix_timestamp + 11 * 24 * 60 * 60; // Set distribution time to 11 days from now
        state.whitelisted = HashSet::new(); // Initialize empty whitelist
        Ok(())
    }

    pub fn whitelist_user(ctx: Context<WhitelistUser>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.whitelisted.insert(ctx.accounts.user.key()); // Mark user as whitelisted
        Ok(())
    }

    pub fn distribute_airdrop(ctx: Context<DistributeAirdrop>, amount: u64) -> Result<()> {
        let state = &mut ctx.accounts.state;

        // Ensure the distribution time has passed
        let current_time = Clock::get()?.unix_timestamp;
        if current_time < state.distribution_time {
            return Err(ErrorCode::DistributionNotReady.into());
        }

        // Ensure the recipient is whitelisted
        if !state.whitelisted.contains(&ctx.accounts.recipient.owner) {
            return Err(ErrorCode::NotWhitelisted.into());
        }

        // Calculate the total number of whitelisted users
        let num_users = state.whitelisted.len() as u64;
        if num_users == 0 {
            return Err(ErrorCode::NoWhitelistedUsers.into());
        }

        // Calculate tokens per user
        let tokens_per_user = state.total_tokens / num_users;

        // Transfer tokens to the recipient
        token::transfer(
            ctx.accounts.into_transfer_context(),
            tokens_per_user,
        )?;

        // Update distributed tokens
        state.distributed_tokens += tokens_per_user;

        Ok(())
    }

    impl<'info> DistributeAirdrop<'info> {
        fn into_transfer_context(self) -> CpiContext<'_, '_, '_, 'info, token::Transfer<'info>> {
            let cpi_accounts = token::Transfer {
                from: ctx.accounts.state.to_account_info(),
                to: self.recipient.to_account_info(),
                authority: self.authority.to_account_info(),
            };
            CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
        }
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 32)] // Adjust space as needed
    pub state: Account<'info, AirdropState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WhitelistUser<'info> {
    #[account(mut)]
    pub state: Account<'info, AirdropState>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct DistributeAirdrop<'info> {
    #[account(mut)]
    pub state: Account<'info, AirdropState>,
    #[account(mut)]
    pub recipient: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub authority: Signer<'info>, // Authority to distribute airdrop
}