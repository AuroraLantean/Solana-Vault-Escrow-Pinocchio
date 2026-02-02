use crate::{
  check_data_len, check_pda, get_rent_exempt, instructions::check_signer, none_zero_u64, parse_u64,
  writable, Config,
};
use core::convert::TryFrom;
use pinocchio::{error::ProgramError, AccountView, ProgramResult};
use pinocchio_log::log;

/// Config2Write
pub struct Config2Write<'a> {
  pub authority: &'a AccountView,
  pub config_pda: &'a AccountView,
  pub system_program: &'a AccountView,
  pub rent_sysvar: &'a AccountView,
  pub new_len: usize,
}
impl<'a> Config2Write<'a> {
  pub const DISCRIMINATOR: &'a u8 = &19;

  pub fn process(self) -> ProgramResult {
    let Config2Write {
      authority,
      config_pda,
      system_program: _,
      rent_sysvar,
      new_len,
    } = self;
    log!("Config2Write process()");

    log!("Config2Write 1");
    let min_lamport = get_rent_exempt(config_pda, rent_sysvar, new_len)?;

    Ok(())
  }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountView])> for Config2Write<'a> {
  type Error = ProgramError;

  fn try_from(value: (&'a [u8], &'a [AccountView])) -> Result<Self, Self::Error> {
    log!("Config2Write try_from");
    let (data, accounts) = value;
    log!("accounts len: {}, data len: {}", accounts.len(), data.len());
    check_data_len(data, 8)?;

    let [authority, config_pda, system_program, rent_sysvar] = accounts else {
      return Err(ProgramError::NotEnoughAccountKeys);
    };
    check_signer(authority)?;
    writable(config_pda)?;
    check_pda(config_pda)?;

    config_pda.check_borrow_mut()?;
    let config: &mut Config = Config::from_account_view(&config_pda)?;
    if config.admin().ne(authority.address()) && config.prog_owner().ne(authority.address()) {
      return Err(ProgramError::IncorrectAuthority);
    }
    let new_len = parse_u64(&data[0..8])?;
    //let bump = config.bump(); //data[9];
    log!("new_len: {}", new_len);
    none_zero_u64(new_len)?;
    //if new_len == old_len => returns Ok(())
    //if new_len < old_len => truncates
    //if new_len - old_len > MAX_PERMITTED_DATA_INCREASE => InvalidRealloc

    Ok(Self {
      authority,
      config_pda,
      system_program,
      rent_sysvar,
      new_len: new_len as usize,
    })
  }
}
