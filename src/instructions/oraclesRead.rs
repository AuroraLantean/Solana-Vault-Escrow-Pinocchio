use crate::{
  check_data_len, check_mint0a, check_pda, instructions::check_signer, parse_u64, read_oracle_pda,
  to32bytes, writable,
};
use core::convert::TryFrom;
use pinocchio::{error::ProgramError, AccountView, ProgramResult};
use pinocchio_log::log;

/// OraclesRead
pub struct OraclesRead<'a> {
  pub signer: &'a AccountView,
  pub config_pda: &'a AccountView,
  pub oracle_pda: &'a AccountView,
  pub oracle_vendor: u8,
  pub num_u64: u64,
  pub feed_id: [u8; 32],
}
impl<'a> OraclesRead<'a> {
  pub const DISCRIMINATOR: &'a u8 = &21;

  pub fn process(self) -> ProgramResult {
    log!("OraclesRead process()");
    let price = read_oracle_pda(self.oracle_vendor, self.oracle_pda, self.feed_id)?;
    log!("process():price: {}", price);
    Ok(())
  }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountView])> for OraclesRead<'a> {
  type Error = ProgramError;

  fn try_from(value: (&'a [u8], &'a [AccountView])) -> Result<Self, Self::Error> {
    log!("OraclesRead try_from");
    let (data, accounts) = value;
    log!("accounts len: {}, data len: {}", accounts.len(), data.len());
    let data_size1 = 44;
    check_data_len(data, data_size1)?;

    let [signer, config_pda, oracle_pda, token_mint, token_program] = accounts else {
      return Err(ProgramError::NotEnoughAccountKeys);
    };
    check_signer(signer)?;
    writable(config_pda)?;
    check_pda(config_pda)?;
    check_mint0a(token_mint, token_program)?;

    log!("parse u8 array");
    let oracle_vendor = data[0];
    log!("oracle_vendor: {}", oracle_vendor);
    let num_u64 = parse_u64(&data[4..12])?;
    log!("num_u64: {}", num_u64);
    let feed_id = *to32bytes(&data[12..data_size1])?;
    log!("feed_id: {}", &feed_id);
    Ok(Self {
      signer,
      config_pda,
      oracle_pda,
      oracle_vendor,
      num_u64,
      feed_id,
    })
  }
}
