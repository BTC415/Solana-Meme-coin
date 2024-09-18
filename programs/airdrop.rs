use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use std::collections::HashSet;

#[program]
pub mod airdrop {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, start_time: i64, duration: i64) -> Result<()> {
        let airdrop_account = &mut ctx.accounts.airdrop_account;
        airdrop_account.start_time = start_time; //Airdrop start time
        airdrop_account.end_time = start_time + duration; //Airdrop end time
        airdrop_account.total_tokens = 200_000_000_000; //Total tokens allocated for airdrop
        airdrop_account.distributed_tokens = 0; //Tokens already distributed to each user
        airdrop_account.whitelisted = HashSet::new(); //User Token Accounts for recieveing airdrop tokens -- Pubkey
        Ok(())
    }

    pub fn whitelist_user(ctx: Context<WhitelistUser>) -> Result<()> {
        let airdrop_account = &mut ctx.accounts.airdrop_account;
        airdrop_account.whitelisted.insert(ctx.accounts.user.key());
        Ok(())
    }

    pub fn distribute_airdrop(ctx: Context<DistributeAirdrop>) -> Result<()> {
        let airdrop_account = &mut ctx.accounts.airdrop_account;
        let clock = Clock::get()?;

        if clock.unix_timestamp < airdrop_account.end_time {
            return Err(ErrorCode::AirdropNotEnded.into());
        }

        if !airdrop_account
            .whitelisted
            .contains(&ctx.accounts.recipient.key())
        {
            return Err(ErrorCode::NotWhitelisted.into());
        }

        let tokens_per_user =
            airdrop_account.total_tokens / airdrop_account.whitelisted.len() as u64;

        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.airdrop_token_account.to_account_info(),
                    to: ctx.accounts.recipient_token_account.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                },
            ),
            tokens_per_user,
        )?;

        airdrop_account.distributed_tokens += tokens_per_user;
        Ok(())
    }
}

//defines the struct for the context of the initialize instruction.
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 16 + 16 + 16 + 16 + 32 * 1000)]
    // Adjust space as needed
    pub airdrop_account: Account<'info, AirdropAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

//defines the struct for the context of the whitelist instruction.
#[derive(Accounts)]
pub struct WhitelistUser<'info> {
    #[account(mut)]
    pub airdrop_account: Account<'info, AirdropAccount>,
    pub user: Signer<'info>,
}

//defines the struct for the context of the distribute instruction.
#[derive(Accounts)]
pub struct DistributeAirdrop<'info> {
    #[account(mut)]
    pub airdrop_account: Account<'info, AirdropAccount>,
    #[account(mut)]
    pub airdrop_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recipient_token_account: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

//Public on-chain account for airdrop
#[account]
pub struct AirdropAccount {
    pub start_time: i64,
    pub end_time: i64,
    pub total_tokens: u64,
    pub distributed_tokens: u64,
    pub whitelisted: HashSet<Pubkey>,
}

//Error code and message
#[error_code]
pub enum ErrorCode {
    #[msg("Airdrop has not ended yet")]
    AirdropNotEnded,
    #[msg("User is not whitelisted")]
    NotWhitelisted,
}