use crate::{
  check_data_len, check_mint0a, check_pda, instructions::check_signer, parse_u64, read_oracle_pda,
  to32bytes, writable,
};
use core::convert::TryFrom;
use pinocchio::{
  error::ProgramError, sysvars::instructions::INSTRUCTIONS_ID, AccountView, ProgramResult,
};
use pinocchio_log::log;

/// FlashloanBorrow
pub struct FlashloanBorrow<'a> {
  pub signer: &'a AccountView,
  pub lender_pda: &'a AccountView,
  pub lender_ata: &'a AccountView,
  pub loan: &'a AccountView,
  pub instruction_sysvar: &'a AccountView,
  pub token_program: &'a AccountView,
  pub system_program: &'a AccountView,
  pub token_accounts: &'a [AccountView],
  pub config_pda: &'a AccountView,
  pub num_u64: u64,
}
impl<'a> FlashloanBorrow<'a> {
  pub const DISCRIMINATOR: &'a u8 = &22;

  pub fn process(self) -> ProgramResult {
    log!("FlashloanBorrow process()");
    Ok(())
  }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountView])> for FlashloanBorrow<'a> {
  type Error = ProgramError;

  fn try_from(value: (&'a [u8], &'a [AccountView])) -> Result<Self, Self::Error> {
    log!("FlashloanBorrow try_from");
    let (data, accounts) = value;
    log!("accounts len: {}, data len: {}", accounts.len(), data.len());
    let data_size1 = 44;
    check_data_len(data, data_size1)?;

    let [signer, lender_pda, lender_ata, loan, instruction_sysvar, config_pda, token_program, system_program, token_accounts @ ..] =
      accounts
    else {
      return Err(ProgramError::NotEnoughAccountKeys);
    };
    check_signer(signer)?;
    writable(config_pda)?;
    check_pda(config_pda)?;
    //check_mint0a(token_mint, token_program)?;
    if instruction_sysvar.address().ne(&INSTRUCTIONS_ID) {
      return Err(ProgramError::UnsupportedSysvar);
    }
    // Verify that the number of token accounts is valid
    if (token_accounts.len() % 2).ne(&0) || token_accounts.len().eq(&0) {
      return Err(ProgramError::InvalidAccountData);
    }

    if loan.try_borrow()?.len().ne(&0) {
      return Err(ProgramError::InvalidAccountData);
    }
    log!("parse u8 array");
    let oracle_vendor = data[0];
    log!("oracle_vendor: {}", oracle_vendor);
    let num_u64 = parse_u64(&data[4..12])?;
    log!("num_u64: {}", num_u64);
    let feed_id = *to32bytes(&data[12..data_size1])?;
    log!("feed_id: {}", &feed_id);
    Ok(Self {
      signer,
      lender_pda,
      lender_ata,
      loan,
      instruction_sysvar,
      config_pda,
      token_accounts,
      token_program,
      system_program,
      num_u64,
    })
  }
}
