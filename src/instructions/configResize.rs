use crate::{
  check_data_len, check_pda, check_rent_sysvar, check_sysprog, get_rent_exempt,
  instructions::check_signer, none_zero_u64, parse_u64, writable, Config,
};
use core::convert::TryFrom;
use pinocchio::{error::ProgramError, AccountView, ProgramResult};
use pinocchio_log::log;
use pinocchio_system::instructions::Transfer as SystemTransfer;

/// Resize PDA
pub struct ConfigResize<'a> {
  pub authority: &'a AccountView,
  pub config_pda: &'a AccountView,
  pub system_program: &'a AccountView,
  pub rent_sysvar: &'a AccountView,
  pub new_len: usize,
}
impl<'a> ConfigResize<'a> {
  pub const DISCRIMINATOR: &'a u8 = &19;

  pub fn process(self) -> ProgramResult {
    let ConfigResize {
      authority,
      config_pda,
      system_program: _,
      rent_sysvar,
      new_len,
    } = self;
    log!("ConfigResize process()");
    config_pda.resize(new_len)?;

    log!("ConfigResize 1");
    let min_lamport = get_rent_exempt(config_pda, rent_sysvar, new_len)?;

    let prev_lamport = config_pda.lamports();
    if min_lamport > prev_lamport {
      log!("deposit lamports");
      SystemTransfer {
        from: authority,
        to: config_pda,
        lamports: min_lamport - prev_lamport,
      }
      .invoke()?;
    } else if min_lamport < prev_lamport {
      log!("withdraw lamports");
      SystemTransfer {
        from: config_pda,
        to: authority,
        lamports: prev_lamport - min_lamport,
      }
      .invoke()?;
    }
    Ok(())
  }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountView])> for ConfigResize<'a> {
  type Error = ProgramError;

  fn try_from(value: (&'a [u8], &'a [AccountView])) -> Result<Self, Self::Error> {
    log!("ConfigResize try_from");
    let (data, accounts) = value;
    log!("accounts len: {}, data len: {}", accounts.len(), data.len());
    check_data_len(data, 8)?;

    let [authority, config_pda, system_program, rent_sysvar] = accounts else {
      return Err(ProgramError::NotEnoughAccountKeys);
    };
    check_signer(authority)?;
    writable(config_pda)?;
    check_pda(config_pda)?;
    check_sysprog(system_program)?;
    check_rent_sysvar(rent_sysvar)?;

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
