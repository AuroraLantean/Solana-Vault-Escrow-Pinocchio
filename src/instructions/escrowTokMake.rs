use core::convert::TryFrom;
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};
use pinocchio_log::log;

use crate::{
  check_ata, check_decimals, check_mint0a, check_sysprog, executable, instructions::check_signer,
  min_data_len, parse_u64, rent_exempt, writable,
};

/// Escrow Token Make Offer
pub struct EscrowTokMake<'a> {
  pub maker: &'a AccountInfo, //signer
  pub from_ata: &'a AccountInfo,
  pub to_ata: &'a AccountInfo,
  pub to_wallet: &'a AccountInfo,
  pub mint_maker: &'a AccountInfo,
  pub mint_taker: &'a AccountInfo,
  pub token_program: &'a AccountInfo,
  pub system_program: &'a AccountInfo,
  pub atoken_program: &'a AccountInfo,
  pub amount: u64,
  pub decimals: u8,
}
impl<'a> EscrowTokMake<'a> {
  pub const DISCRIMINATOR: &'a u8 = &15;

  pub fn process(self) -> ProgramResult {
    let EscrowTokMake {
      maker,
      from_ata,
      to_ata,
      to_wallet,
      mint_maker,
      mint_taker,
      token_program,
      system_program,
      atoken_program: _,
      amount,
      decimals,
    } = self;
    log!("EscrowTokMake process()");
    check_signer(maker)?;
    executable(token_program)?;
    writable(from_ata)?;

    log!("EscrowTokMake 1");
    rent_exempt(mint_maker, 0)?;
    rent_exempt(mint_taker, 0)?;
    log!("EscrowTokMake 2");
    check_ata(from_ata, maker, mint_maker)?;

    log!("EscrowTokMake 3");
    check_decimals(mint_maker, decimals)?;
    check_mint0a(mint_maker, token_program)?;

    log!("EscrowTokMake 5");
    check_sysprog(system_program)?;

    if to_ata.data_is_empty() {
      log!("Make to_ata");
      pinocchio_associated_token_account::instructions::Create {
        funding_account: maker,
        account: to_ata,
        wallet: to_wallet,
        mint: mint_maker,
        system_program,
        token_program,
      }
      .invoke()?;
      //Please upgrade to SPL Token 2022 for immutable owner support
    } else {
      log!("to_ata has data");
      check_ata(to_ata, to_wallet, mint_maker)?;
    }
    writable(to_ata)?;
    rent_exempt(to_ata, 1)?;
    log!("ToATA is found/verified");

    log!("Transfer Tokens");
    pinocchio_token::instructions::TransferChecked {
      from: from_ata,
      mint: mint_maker,
      to: to_ata,
      authority: maker,
      amount, // unsafe { *(data.as_ptr().add(1 + 8) as *const u64)}
      decimals,
    }
    .invoke()?;
    /*  pinocchio_token::instructions::Transfer {
        from: vault,
        to: to_ata,
        authority: escrow,
        amount: vault_account.amount(),
    }.invoke_signed(&[seeds.clone()])?; */
    Ok(())
  }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for EscrowTokMake<'a> {
  type Error = ProgramError;

  fn try_from(value: (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
    log!("EscrowTokMake try_from");
    let (data, accounts) = value;
    log!("accounts len: {}, data len: {}", accounts.len(), data.len());

    let [maker, from_ata, to_ata, to_wallet, mint_maker, mint_taker, token_program, system_program, atoken_program] =
      accounts
    else {
      return Err(ProgramError::NotEnoughAccountKeys);
    };

    //u8 takes 1 + u64 takes 8 bytes
    min_data_len(data, 9)?;

    let decimals = data[0];
    let amount = parse_u64(&data[1..])?;
    log!("decimals: {}, amount: {}", decimals, amount);
    Ok(Self {
      maker,
      from_ata,
      to_ata,
      to_wallet,
      mint_maker,
      mint_taker,
      token_program,
      system_program,
      atoken_program,
      decimals,
      amount,
    })
  }
}
