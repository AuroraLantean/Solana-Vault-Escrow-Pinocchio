use core::convert::TryFrom;
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};
use pinocchio_log::log;

use crate::{
  ata_balc, check_ata, check_atoken_gpvbd, check_decimals, check_mint0a, check_sysprog,
  check_vault, data_len, executable, instructions::check_signer, none_zero_u64, parse_u64,
  rent_exempt_mint, rent_exempt_tokacct, writable, Config, Ee,
};

/// Make Escrow Token Offer
pub struct EscrowTokMake<'a> {
  pub user_x: &'a AccountInfo, //signer
  pub user_x_ata: &'a AccountInfo,
  pub vault_ata: &'a AccountInfo, //as to_ata
  pub vault: &'a AccountInfo,     //PDA or to_wallet
  pub mint_x: &'a AccountInfo,
  pub mint_y: &'a AccountInfo,
  pub token_program: &'a AccountInfo,
  pub system_program: &'a AccountInfo,
  pub atoken_program: &'a AccountInfo,
  pub decimal_x: u8,
  pub decimal_y: u8,
  pub amount_x: u64,
  pub amount_y: u64,
}
impl<'a> EscrowTokMake<'a> {
  pub const DISCRIMINATOR: &'a u8 = &15;

  pub fn process(self) -> ProgramResult {
    let EscrowTokMake {
      user_x,
      user_x_ata,
      vault_ata,
      vault,
      mint_x,
      mint_y,
      token_program,
      system_program,
      atoken_program: _,
      decimal_x,
      decimal_y,
      amount_x,
      amount_y,
    } = self;
    log!("EscrowTokMake process()");
    /*Make a valid program derived address without searching for a bump seed:
    let seed = [(b"escrow"), user_x.key().as_slice(), bump.as_ref()];
    let seeds = &seed[..];
    let pda = pubkey::checked_create_program_address(seeds, &crate::ID).unwrap();*/
    if vault_ata.data_is_empty() {
      log!("Make vault_ata");
      pinocchio_associated_token_account::instructions::Create {
        funding_account: user_x,
        account: vault_ata,
        wallet: vault,
        mint: mint_x,
        system_program,
        token_program,
      }
      .invoke()?;
      //Please upgrade to SPL Token 2022 for immutable owner support
    } else {
      log!("vault_ata has data");
      check_ata(vault_ata, vault, mint_x)?;
    }
    writable(vault_ata)?;
    rent_exempt_tokacct(vault_ata)?;
    log!("EscrowTokMake 7: ToATA is found/verified");

    log!("EscrowTokMake 8: Transfer Tokens");
    pinocchio_token::instructions::TransferChecked {
      from: user_x_ata,
      mint: mint_x,
      to: vault_ata,
      authority: user_x,
      amount: amount_x,
      decimals: decimal_x,
    }
    .invoke()?;
    /*  pinocchio_token::instructions::Transfer {
        from: vault,
        to: vault_ata,
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

    let [user_x, user_x_ata, vault_ata, vault, mint_x, mint_y, config_pda, token_program, system_program, atoken_program] =
      accounts
    else {
      return Err(ProgramError::NotEnoughAccountKeys);
    };
    check_signer(user_x)?;
    executable(token_program)?;
    check_sysprog(system_program)?;
    check_atoken_gpvbd(atoken_program)?;

    writable(user_x_ata)?;
    check_ata(user_x_ata, user_x, mint_x)?;
    log!("EscrowTokMake try_from 5");

    //2x u8 takes 2 + 2x u64 takes 16 bytes
    data_len(data, 18)?;
    let decimal_x = data[0];
    let amount_x = parse_u64(&data[1..9])?;
    log!("decimal_x: {}, amount_x: {}", decimal_x, amount_x);
    let decimal_y = data[10];
    let amount_y = parse_u64(&data[11..19])?;
    log!("decimal_y: {}, amount_y: {}", decimal_y, amount_y);

    none_zero_u64(amount_x)?;
    ata_balc(user_x_ata, amount_x)?;
    none_zero_u64(amount_y)?;

    log!("EscrowTokMake try_from 9");
    config_pda.can_borrow_mut_data()?;
    let config: &mut Config = Config::from_account_info(&config_pda)?;

    check_vault(vault, config.vault())?;

    log!("EscrowTokMake try_from 14");
    rent_exempt_mint(mint_x)?;
    rent_exempt_mint(mint_y)?;
    //TODO: fee is part of exchange amount

    log!("EscrowTokMake try_from 16");
    check_decimals(mint_x, decimal_x)?;
    check_decimals(mint_y, decimal_y)?;

    log!("EscrowTokMake try_from 18");
    check_mint0a(mint_x, token_program)?;
    check_mint0a(mint_y, token_program)?;

    /*let seeds = &[User::SEED_PREFIX, self.accounts.payer.key().as_ref()];
    let (t_account, bump) = find_program_address(
        seeds, &crate::ID);
    if t_account.ne(self.accounts.target_account.key()) { return Err(ProgramError::InvalidAccountData); }*/
    Ok(Self {
      user_x,
      user_x_ata,
      vault_ata,
      vault,
      mint_x,
      mint_y,
      token_program,
      system_program,
      atoken_program,
      decimal_x,
      decimal_y,
      amount_x,
      amount_y,
    })
  }
}
