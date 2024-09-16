//Token contract
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount};

declare_id!("YOUR_PROGRAM_ID"); // Replace with your program ID

#[program]
pub mod mansa_musa {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, total_supply: u64) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.total_supply = total_supply;
        state.airdrop_amount = 200_000_000_000; // 200 billion
        state.liquidity = 100_000_000_000; // 100 billion
        state.dev_fund = 100_000_000_000; // 100 billion
        state.whitelisted = HashSet::new(); // Initialize empty whitelist
        Ok(())
    }

    // Other functions will be defined here
}

#[account]
pub struct State {
    pub total_supply: u64,
    pub airdrop_amount: u64,
    pub liquidity: u64,
    pub dev_fund: u64,
    pub whitelisted: HashSet<Pubkey>, // Keep track of whitelisted addresses
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 32)] // 8 bytes for account discriminator + 32 for HashSet storage
    pub state: Account<'info, State>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WhitelistUser<'info> {
    #[account(mut)]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub user: Signer<'info>,
}

pub fn whitelist_user(ctx: Context<WhitelistUser>) -> Result<()> {
    let state = &mut ctx.accounts.state;
    state.whitelisted.insert(ctx.accounts.user.key()); // Mark user as whitelisted
    Ok(())
}

#[derive(Accounts)]
pub struct DistributeAirdrop<'info> {
    #[account(mut)]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub recipient: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub authority: Signer<'info>, // Authority to distribute airdrop
}

pub fn distribute_airdrop(ctx: Context<DistributeAirdrop>, amount: u64) -> Result<()> {
    let state = &mut ctx.accounts.state;

    // Ensure the recipient is whitelisted
    if !state.whitelisted.contains(&ctx.accounts.recipient.owner) {
        return Err(ErrorCode::NotWhitelisted.into());
    }

    // Transfer tokens
    token::transfer(
        ctx.accounts.into_transfer_context().with_signer(&[&[b"authority", &[state.bump_seed]]]),
        amount,
    )?;
    Ok(())
}

#[derive(Accounts)]
pub struct PurchaseTokens<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    pub state: Account<'info, State>,
}

pub fn purchase_tokens(ctx: Context<PurchaseTokens>, amount: u64, price: u64) -> Result<()> {
    let state = &mut ctx.accounts.state;

    // Check if enough liquidity is available
    if state.liquidity < amount {
        return Err(ErrorCode::InsufficientLiquidity.into());
    }

    // Placeholder for payment logic (e.g., deducting from buyer's balance)
    
    // Deduct amount from available liquidity
    state.liquidity -= amount; 
    Ok(())
}

#[derive(Accounts)]
pub struct ReleaseDevFunds<'info> {
    pub admin: Signer<'info>,
    pub state: Account<'info, State>,
}

const VESTING_PERIOD: u64 = 6307200000; // 2 years in seconds for linear vesting

pub fn release_dev_funds(ctx: Context<ReleaseDevFunds>) -> Result<()> {
    let state = &mut ctx.accounts.state;

    // Logic to release funds based on the vesting schedule
    // Implement time logic to check if funds can be released
    Ok(())
}

#[error_code]
pub enum ErrorCode {
    #[msg("The user is not whitelisted.")]
    NotWhitelisted,
    #[msg("Insufficient liquidity available for purchase.")]
    InsufficientLiquidity,
}




