use crate::{
  check_data_len, check_pda, instructions::check_signer, parse_u32, parse_u64, writable, Config2,
  Ee,
};
use core::convert::TryFrom;
use pinocchio::{error::ProgramError, AccountView, Address, ProgramResult};
use pinocchio_log::log;

/// Config2Update
pub struct Config2Update<'a> {
  pub authority: &'a AccountView,
  pub config_pda: &'a AccountView,
  pub func_selector: u8,
  pub new_u32: u32,
  pub new_u64: u64,
  pub account1: &'a Address,
  //pub account2: &'a Address,
  //pub bools: [bool; 4],
  //pub u8s: [u8; 4],
  //pub u32s: [u32; 4],
  //pub u64s: [u64; 4],
  //pub str_u8array: [u8; 32],
  pub config2: &'a mut Config2,
}
impl<'a> Config2Update<'a> {
  pub const DISCRIMINATOR: &'a u8 = &20;

  pub fn process(self) -> ProgramResult {
    log!("UpdateConfig2 process()");
    match self.func_selector {
      2 => self.update_admin(),
      3 => self.update1(),
      _ => Ee::FunctionSelector.e(),
    }
  }

  pub fn update1(self) -> ProgramResult {
    log!("UpdateConfig update1()");
    self.config2.set_new_u32(self.new_u32);
    self.config2.set_new_u64(self.new_u64);
    self.config2.set_new_account1(self.account1);
    //self.config2.set_str_u8array(self.str_u8array);
    Ok(())
  }

  pub fn only_owner(&self) -> ProgramResult {
    if self.config2.prog_owner() != self.authority.address() {
      return Ee::OnlyProgOwner.e();
    }
    Ok(())
  }
  pub fn update_admin(self) -> ProgramResult {
    self.only_owner()?;
    self.config2.set_admin(self.account1);
    Ok(())
  }
  pub fn update_prog_owner(self) -> ProgramResult {
    self.only_owner()?;
    self.config2.set_prog_owner(self.account1);
    Ok(())
  }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountView])> for Config2Update<'a> {
  type Error = ProgramError;

  fn try_from(value: (&'a [u8], &'a [AccountView])) -> Result<Self, Self::Error> {
    log!("Config2Update try_from");
    let (data, accounts) = value;
    log!("accounts len: {}, data len: {}", accounts.len(), data.len());
    let data_size1 = 88;
    check_data_len(data, data_size1)?; //56+32

    let [authority, config_pda, account1, _account2] = accounts else {
      return Err(ProgramError::NotEnoughAccountKeys);
    };
    check_signer(authority)?;
    writable(config_pda)?;
    check_pda(config_pda)?;

    /*log!("parse booleans");
    let b0 = u8_to_bool(data[0])?;
    let b1 = u8_to_bool(data[1])?;
    let b2 = u8_to_bool(data[2])?;
    let b3 = u8_to_bool(data[3])?;
    let bools = [b0, b1, b2, b3];*/

    log!("parse u8 array");
    let func_selector = data[4];
    //let u8s = [data[4], data[5], data[6], data[7]];
    log!("func_selector: {}", func_selector);
    log!("parse u32 array");
    let new_u32 = parse_u32(&data[8..12])?;
    /*let u32b = parse_u32(&data[12..16])?;
    let u32c = parse_u32(&data[16..20])?;
    let u32d = parse_u32(&data[20..24])?;
    let u32s = [u32a, u32b, u32c, u32d];*/
    log!("new_u32: {}", new_u32);

    log!("parse u64 array");
    let new_u64 = parse_u64(&data[24..32])?;
    /*let u64b = parse_u64(&data[32..40])?;
    let u64c = parse_u64(&data[40..48])?;
    let u64d = parse_u64(&data[48..56])?;
    let u64s = [u64a, u64b, u64c, u64d];*/
    log!("new_u64: {}", new_u64);

    config_pda.check_borrow_mut()?;
    let config2: &mut Config2 = Config2::from_account_view(&config_pda)?;

    if config2.admin().ne(authority.address()) && config2.prog_owner().ne(authority.address()) {
      return Err(ProgramError::IncorrectAuthority);
    }

    Ok(Self {
      authority,
      config_pda,
      func_selector,
      new_u32,
      new_u64,
      //u8s, bools, u32s, u64s, str_u8array,
      account1: account1.address(),
      //account2: account2.address(),
      config2,
    })
  }
}
