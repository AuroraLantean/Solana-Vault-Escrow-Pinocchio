use core::convert::TryFrom;
use pinocchio::{
  account_info::AccountInfo,
  instruction::{Seed, Signer},
  program_error::ProgramError,
  pubkey::find_program_address,
  sysvars::{rent::Rent, Sysvar},
  ProgramResult,
};
use pinocchio_log::log;

use crate::{
  ata_balc, check_ata, check_ata_escrow, check_atoken_gpvbd, check_decimals, check_escrow_mints,
  check_mint0a, check_sysprog, data_len, executable, instructions::check_signer, none_zero_u64,
  parse_u64, rent_exempt_mint, rent_exempt_tokacct, writable, Config, Ee, Escrow,
};

/// Make Escrow Token Offer
pub struct EscrowTokMake<'a> {
  pub maker0: &'a AccountInfo, //signer
  pub maker0_ata: &'a AccountInfo,
  pub escrow_ata: &'a AccountInfo, //as to_ata
  pub escrow: &'a AccountInfo,     //PDA as to_wallet
  pub mint0: &'a AccountInfo,
  pub mint1: &'a AccountInfo,
  pub token_program: &'a AccountInfo,
  pub system_program: &'a AccountInfo,
  pub atoken_program: &'a AccountInfo,
  pub decimal0: u8,
  pub decimal1: u8,
  pub amount0: u64,
  pub amount1: u64,
  pub id: u64,
}
impl<'a> EscrowTokMake<'a> {
  pub const DISCRIMINATOR: &'a u8 = &15;

  pub fn process(self) -> ProgramResult {
    let EscrowTokMake {
      maker0,
      maker0_ata,
      escrow_ata,
      escrow,
      mint0,
      mint1,
      token_program,
      system_program,
      atoken_program: _,
      decimal0,
      decimal1: _,
      amount0,
      amount1,
      id,
    } = self;
    log!("EscrowTokMake process()");

    /*let bump = unsafe { *(data.as_ptr() as *const u8) }.to_le_bytes();
    if bump.len() != 1 { return Err(..);  };   bump.as_ref()*/
    let seed = [Escrow::SEED, maker0.key().as_slice(), &id.to_le_bytes()];
    let seeds = &seed[..];

    let (expected_escrow, bump) = find_program_address(seeds, &crate::ID); //TODO: may incur unknown cost
    if expected_escrow.ne(escrow.key()) {
      return Ee::EscrowPDA.e();
    }
    //let expected_escrow = checked_create_program_address(seeds, &crate::ID)?;
    log!("EscrowTokMake EscrowPDA verified");

    if escrow.lamports() > 0 {
      //escrow.owner() != &crate::ID
      return Err(ProgramError::AccountAlreadyInitialized);
    } else {
      log!("Make Escrow PDA 1");
      let lamports = Rent::get()?.minimum_balance(Escrow::LEN);

      log!("Make Escrow PDA 2");
      let id_seed = &id.to_le_bytes();
      //let seed = [Escrow::SEED, maker0.key().as_slice(), &id.to_le_bytes()];
      //let seeds = &seed[..];
      let seeds: [Seed<'_>; 4] = [
        Seed::from(Escrow::SEED),
        Seed::from(maker0.key().as_ref()),
        Seed::from(id_seed),
        Seed::from(core::slice::from_ref(&bump)),
      ];
      let seed_signer = [Signer::from(&seeds)];
      log!("Make Escrow PDA 3");

      pinocchio_system::instructions::CreateAccount {
        from: maker0,
        to: escrow,
        lamports,
        space: Escrow::LEN as u64,
        owner: &crate::ID,
      }
      .invoke_signed(&seed_signer)?;
    }

    log!("Escrow is made");
    if escrow_ata.data_is_empty() {
      log!("Make escrow_ata");
      pinocchio_associated_token_account::instructions::Create {
        funding_account: maker0,
        account: escrow_ata,
        wallet: escrow,
        mint: mint0,
        system_program,
        token_program,
      }
      .invoke()?;
      //Please upgrade to SPL Token 2022 for immutable owner support
    } else {
      log!("escrow_ata has data");
      check_ata_escrow(escrow_ata, escrow, mint0)?;
    }
    writable(escrow_ata)?;
    rent_exempt_tokacct(escrow_ata)?;
    log!("Vault ATA is found/verified");

    log!("Transfer token_x from maker0_ata");
    pinocchio_token::instructions::TransferChecked {
      from: maker0_ata,
      mint: mint0,
      to: escrow_ata,
      authority: maker0,
      amount: amount0, // *(data.as_ptr().add(1 + 8) as *const u64)
      decimals: decimal0,
    }
    .invoke()?;
    /*  pinocchio_token::instructions::Transfer {
        from: escrow,
        to: escrow_ata,
        authority: escrow,
        amount: vault_account.amount(),
    }.invoke_signed(&[seeds.clone()])?; */

    log!("Fill Escrow PDA AFTER payment");
    let escrow: &mut Escrow = Escrow::from_account_info(&escrow)?;
    escrow.set_maker0(maker0.key());
    escrow.set_mint0(mint0.key());
    escrow.set_mint1(mint1.key());
    escrow.set_id(id)?;
    escrow.set_amount1(amount1)?; // unsafe { *(data.as_ptr().add(1) as *const u64) };
    escrow.set_bump(bump); // unsafe { *data.as_ptr() };

    Ok(())
  }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for EscrowTokMake<'a> {
  type Error = ProgramError;

  fn try_from(value: (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
    log!("EscrowTokMake try_from");
    let (data, accounts) = value;
    log!("accounts len: {}, data len: {}", accounts.len(), data.len());

    let [maker0, maker0_ata, escrow_ata, escrow, mint0, mint1, config_pda, token_program, system_program, atoken_program] =
      accounts
    else {
      return Err(ProgramError::NotEnoughAccountKeys);
    };
    check_signer(maker0)?;
    executable(token_program)?;
    check_sysprog(system_program)?;
    check_atoken_gpvbd(atoken_program)?;

    writable(maker0_ata)?;
    check_ata(maker0_ata, maker0, mint0)?;
    writable(escrow)?;
    log!("EscrowTokMake try_from 5");

    //2x u8 takes 2 + 2x u64 takes 16 bytes
    data_len(data, 26)?;
    let decimal0 = data[0];
    let amount0 = parse_u64(&data[1..9])?;
    log!("decimal0: {}, amount0: {}", decimal0, amount0);
    none_zero_u64(amount0)?;
    ata_balc(maker0_ata, amount0)?;

    let decimal1 = data[9];
    let amount1 = parse_u64(&data[10..18])?;
    log!("decimal1: {}, amount1: {}", decimal1, amount1);
    none_zero_u64(amount1)?;

    let id = parse_u64(&data[18..26])?;
    log!("id: {}", id);

    log!("EscrowTokMake try_from: config");
    config_pda.can_borrow_mut_data()?;
    let _config: &mut Config = Config::from_account_info(&config_pda)?;

    log!("EscrowTokMake try_from 5");
    check_escrow_mints(mint0, mint1)?;
    rent_exempt_mint(mint0)?;
    rent_exempt_mint(mint1)?;
    //TODO: fee is part of exchange amount

    log!("EscrowTokMake try_from 6");
    check_decimals(mint0, decimal0)?;
    check_mint0a(mint0, token_program)?;
    check_mint0a(mint1, token_program)?; // Not needed since CPI since deposit will fail if not owned by token program

    Ok(Self {
      maker0,
      maker0_ata,
      escrow_ata,
      escrow,
      mint0,
      mint1,
      token_program,
      system_program,
      atoken_program,
      decimal0,
      decimal1,
      amount0,
      amount1,
      id,
    })
  }
}
