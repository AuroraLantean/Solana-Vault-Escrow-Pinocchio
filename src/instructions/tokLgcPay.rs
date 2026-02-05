use core::convert::TryFrom;
use pinocchio::{error::ProgramError, AccountView, ProgramResult};
use pinocchio_log::log;

use crate::{
  ata_balc, check_ata, check_atoken_gpvbd, check_data_len, check_decimals, check_mint0a,
  check_rent_sysvar, check_sysprog, check_vault, executable, instructions::check_signer,
  none_zero_u64, parse_u64, rent_exempt_mint, rent_exempt_tokacct, writable, Config, Ee,
};

/// TokLgc: Users to Pay Tokens to VaultAdmin
pub struct TokLgcPay<'a> {
  pub user: &'a AccountView, //signer
  pub from_ata: &'a AccountView,
  pub to_ata: &'a AccountView,
  pub vault: &'a AccountView,
  pub mint: &'a AccountView,
  pub config_pda: &'a AccountView,
  pub token_program: &'a AccountView,
  pub system_program: &'a AccountView,
  pub atoken_program: &'a AccountView,
  pub rent_sysvar: &'a AccountView,
  pub decimals: u8,
  pub amount: u64,
}
impl<'a> TokLgcPay<'a> {
  pub const DISCRIMINATOR: &'a u8 = &7;

  pub fn process(self) -> ProgramResult {
    let TokLgcPay {
      user,
      from_ata,
      to_ata,
      vault,
      mint,
      config_pda: _,
      token_program,
      system_program,
      atoken_program: _,
      rent_sysvar,
      decimals,
      amount,
    } = self;
    log!("TokLgcPay process()");

    if to_ata.is_data_empty() {
      log!("Make to_ata");
      pinocchio_associated_token_account::instructions::Create {
        funding_account: user,
        account: to_ata,
        wallet: vault,
        mint,
        system_program,
        token_program,
      }
      .invoke()?;
      //Please upgrade to SPL Token 2022 for immutable owner support
    } else {
      log!("to_ata has data");
      check_ata(to_ata, vault, mint)?;
    }
    writable(to_ata)?;
    rent_exempt_tokacct(to_ata, rent_sysvar)?;
    log!("Vault ATA is found/verified");

    pinocchio_token::instructions::TransferChecked {
      from: from_ata,
      mint,
      to: to_ata,
      authority: user,
      amount,
      decimals,
    }
    .invoke()?;
    Ok(())
  }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountView])> for TokLgcPay<'a> {
  type Error = ProgramError;

  fn try_from(value: (&'a [u8], &'a [AccountView])) -> Result<Self, Self::Error> {
    log!("TokLgcPay try_from");
    let (data, accounts) = value;
    log!("accounts len: {}, data len: {}", accounts.len(), data.len());

    let [user, from_ata, to_ata, vault, mint, config_pda, token_program, system_program, atoken_program, rent_sysvar] =
      accounts
    else {
      return Err(ProgramError::NotEnoughAccountKeys);
    };
    check_signer(user)?;
    executable(token_program)?;
    check_sysprog(system_program)?;
    check_atoken_gpvbd(atoken_program)?;
    check_rent_sysvar(rent_sysvar)?;

    writable(vault)?;
    writable(from_ata)?;
    check_ata(from_ata, user, mint)?;
    log!("TokLgcPay try_from 5");

    //1+8: u8 takes 1, u64 takes 8 bytes
    check_data_len(data, 9)?;
    let decimals = data[0];
    let amount = parse_u64(&data[1..])?;
    log!("decimals: {}, amount: {}", decimals, amount);

    none_zero_u64(amount)?;
    ata_balc(from_ata, amount)?;

    log!("TokLgcPay try_from 9");
    config_pda.check_borrow_mut()?;
    let config: &mut Config = Config::from_account_view(&config_pda)?;

    if !config.mints().contains(&mint.address()) {
      return Err(Ee::MintNotAccepted.into());
    }
    check_vault(vault, config.vault())?;

    log!("LgcPay try_from 10");
    rent_exempt_mint(mint, rent_sysvar, 0)?;
    check_decimals(mint, decimals)?;
    check_mint0a(mint, token_program)?;

    Ok(Self {
      user,
      from_ata,
      to_ata,
      vault,
      mint,
      config_pda,
      token_program,
      system_program,
      atoken_program,
      rent_sysvar,
      decimals,
      amount,
    })
  }
}
