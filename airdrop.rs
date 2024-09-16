///Airdrop smart contract

use solana_program::{
  account_info::{AccountInfo, next_account_info},
  entrypoint,
  entrypoint::ProgramResult,
  msg,
  pubkey::Pubkey,
  program_error::ProgramError,
  sysvar::Sysvar,
  clock::Clock,
};
use std::collections::HashSet;

pub struct Airdrop {
  pub whitelist: HashSet<Pubkey>,
  pub total_tokens: u64,
  pub distributed_tokens: u64,
  pub distribution_time: u64,
}

impl Airdrop {
  pub fn new(total_tokens: u64) -> Self {
      Airdrop {
          whitelist: HashSet::new(),
          total_tokens,
          distributed_tokens: 0,
          distribution_time: 0,
      }
  }

  pub fn add_to_whitelist(&mut self, user: Pubkey) {
      self.whitelist.insert(user);
  }

  pub fn is_whitelisted(&self, user: &Pubkey) -> bool {
      self.whitelist.contains(user)
  }

  pub fn set_distribution_time(&mut self, time: u64) {
      self.distribution_time = time;
  }

  pub fn distribute_tokens(&mut self) -> Result<u64, ProgramError> {
      if self.distributed_tokens > 0 {
          return Err(ProgramError::InvalidInstructionData);
      }

      let current_time = Clock::get()?.unix_timestamp as u64;
      if current_time < self.distribution_time {
          return Err(ProgramError::InvalidInstructionData);
      }

      let num_users = self.whitelist.len() as u64;
      if num_users == 0 {
          return Ok(0);
      }

      let tokens_per_user = self.total_tokens / num_users;
      self.distributed_tokens = self.total_tokens;

      msg!("Distributing {} tokens to {} users", tokens_per_user, num_users);
      Ok(tokens_per_user)
  }
}

entrypoint!(process_instruction);

fn process_instruction(
  program_id: &Pubkey,
  accounts: &[AccountInfo],
  instruction_data: &[u8],
) -> ProgramResult {
  let account_info_iter = &mut accounts.iter();
  let user_account = next_account_info(account_info_iter)?;

  let mut airdrop = Airdrop::new(200_000_000_000); // Total tokens for airdrop

  // Example usage
  if instruction_data[0] == 1 { // Add to whitelist
      airdrop.add_to_whitelist(*user_account.key);
  } else if instruction_data[0] == 2 { // Distribute tokens
      airdrop.set_distribution_time(Clock::get()?.unix_timestamp as u64 + 11 * 24 * 60 * 60); // 11 days
      let tokens_per_user = airdrop.distribute_tokens()?;
      msg!("Each user receives: {}", tokens_per_user);
  }

  Ok(())
}
