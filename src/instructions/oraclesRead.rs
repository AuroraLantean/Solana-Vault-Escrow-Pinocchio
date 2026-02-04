use crate::{
  check_data_len, check_mint0a, check_pda, instructions::check_signer, parse_u32, parse_u64,
  writable, Ee,
};
use core::convert::TryFrom;
use pinocchio::{error::ProgramError, AccountView, Address, ProgramResult};
use pinocchio_log::log;

/// OraclesRead
pub struct OraclesRead<'a> {
  pub signer: &'a AccountView,
  pub config_pda: &'a AccountView,
  pub func_selector: u8,
  pub num_u32: u32,
  pub num_u64: u64,
  pub oracle_program: &'a Address,
}
impl<'a> OraclesRead<'a> {
  pub const DISCRIMINATOR: &'a u8 = &21;

  pub fn process(self) -> ProgramResult {
    log!("UpdateConfig2 process()");
    match self.func_selector {
      0 => self.pyth(),
      _ => Ee::FunctionSelector.e(),
    }
  }
  //TODO: Read Oracle Token Prices!
  //https://solana.stackexchange.com/questions/22293/how-to-convert-a-solana-program-account-info-into-a-pinocchio-account-info

  pub fn pyth(self) -> ProgramResult {
    log!("OraclesRead Pyth ()");
    Ok(())
  }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountView])> for OraclesRead<'a> {
  type Error = ProgramError;

  fn try_from(value: (&'a [u8], &'a [AccountView])) -> Result<Self, Self::Error> {
    log!("OraclesRead try_from");
    let (data, accounts) = value;
    log!("accounts len: {}, data len: {}", accounts.len(), data.len());
    let data_size1 = 16;
    check_data_len(data, data_size1)?;

    let [signer, config_pda, oracle_program, token_mint, token_program] = accounts else {
      return Err(ProgramError::NotEnoughAccountKeys);
    };
    check_signer(signer)?;
    writable(config_pda)?;
    check_pda(config_pda)?;
    check_mint0a(token_mint, token_program)?;

    log!("parse u8 array");
    let func_selector = data[0];
    log!("func_selector: {}", func_selector);
    //let func_selector = data[1];
    //let func_selector = data[2];
    //let func_selector = data[3];

    let num_u32 = parse_u32(&data[4..8])?;
    log!("num_u32: {}", num_u32);

    let num_u64 = parse_u64(&data[8..data_size1])?;
    log!("num_u64: {}", num_u64);

    Ok(Self {
      signer,
      config_pda,
      func_selector,
      num_u32,
      num_u64,
      oracle_program: oracle_program.address(),
    })
  }
}
