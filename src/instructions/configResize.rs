use core::convert::TryFrom;
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};
use pinocchio_log::log;
use pinocchio_system::instructions::AllocateWithSeed;

use crate::{
  check_pda, data_len, instructions::check_signer, none_zero_u64, parse_u64, writable, Config, ID,
};

/// Close PDA
pub struct ConfigResize<'a> {
  pub authority: &'a AccountInfo,
  pub config_pda: &'a AccountInfo,
  pub system_program: &'a AccountInfo,
  pub new_size: u64,
}
impl<'a> ConfigResize<'a> {
  pub const DISCRIMINATOR: &'a u8 = &19;

  pub fn process(self) -> ProgramResult {
    let ConfigResize {
      authority,
      config_pda,
      system_program: _,
      new_size,
    } = self;
    log!("ConfigResize process()");
    AllocateWithSeed {
      account: config_pda,
      base: authority,
      seed: "config",
      space: new_size,
      owner: &ID,
    }
    .invoke()?;
    // Realloc {
    //   account: config_pda,
    //   space: new_size,
    //   payer: authority,
    //   system_program: system_program,
    // }
    // .invoke()?;
    Ok(())
  }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for ConfigResize<'a> {
  type Error = ProgramError;

  fn try_from(value: (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
    log!("ConfigResize try_from");
    let (data, accounts) = value;
    log!("accounts len: {}, data len: {}", accounts.len(), data.len());
    data_len(data, 8)?;

    let [authority, config_pda, system_program] = accounts else {
      return Err(ProgramError::NotEnoughAccountKeys);
    };
    check_signer(authority)?;
    writable(config_pda)?;
    check_pda(config_pda)?;

    config_pda.can_borrow_mut_data()?;
    let config: &mut Config = Config::from_account_info(&config_pda)?;
    if config.admin().ne(authority.key()) && config.prog_owner().ne(authority.key()) {
      return Err(ProgramError::IncorrectAuthority);
    }
    let new_size = parse_u64(&data[0..8])?;
    log!("new_size: {}", new_size);
    none_zero_u64(new_size)?;

    Ok(Self {
      authority,
      config_pda,
      system_program,
      new_size,
    })
  }
}
