use core::convert::TryFrom;
use pinocchio::{
  cpi::{Seed, Signer},
  error::ProgramError,
  sysvars::rent::Rent,
  AccountView, ProgramResult,
};
use pinocchio_log::log;
use pinocchio_system::instructions::Transfer as SystemTransfer;

use crate::{
  check_rent_sysvar, check_sysprog,
  instructions::{check_pda, check_signer, derive_pda1, parse_u64},
  none_zero_u64, sol_balc, Ee, PROG_ADDR, VAULT_SEED, VAULT_SIZE,
};

// Deposit SOL to program PDA
// make and rent-funds the vault PDA
// check the PDA exists and is owned by the program
// transfer the SOL amount to the vault

//Deposit Accounts
pub struct DepositSol<'a> {
  pub user: &'a AccountView,
  pub vault: &'a AccountView,
  pub rent_sysvar: &'a AccountView,
  pub amount: u64,
}
impl<'a> DepositSol<'a> {
  pub const DISCRIMINATOR: &'a u8 = &0;

  pub fn process(self) -> ProgramResult {
    let DepositSol {
      user,
      vault,
      rent_sysvar,
      amount,
    } = self;
    log!("DepositSol process");
    check_vault_exists(user, vault, rent_sysvar)?;

    log!("DepositSol 2");
    SystemTransfer {
      from: user,
      to: vault,
      lamports: amount,
    }
    .invoke()?;
    log!("success: {} Lamports deposited to vault", amount);
    Ok(())
  }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountView])> for DepositSol<'a> {
  type Error = ProgramError;

  fn try_from(value: (&'a [u8], &'a [AccountView])) -> Result<Self, Self::Error> {
    log!("DepositSol try_from");
    let (data, accounts) = value;
    log!("accounts len: {}, data len: {}", accounts.len(), data.len());

    let [user, vault, system_program, rent_sysvar] = accounts else {
      return Err(ProgramError::NotEnoughAccountKeys);
    };
    check_signer(user)?;
    check_sysprog(system_program)?;
    check_rent_sysvar(rent_sysvar)?;

    log!("DepositSol data: {}", data);
    let amount = parse_u64(data)?;
    log!("amount: {}", amount);
    none_zero_u64(amount)?;
    sol_balc(user, amount)?;

    Ok(Self {
      user,
      vault,
      rent_sysvar,
      amount,
    })
  }
}

/// Ensure the vault exists; if not, create it with PDA seeds. user must be a signer, vault must be writable, and rent minimum must be respected for creation.
fn check_vault_exists(
  user: &AccountView,
  vault: &AccountView,
  rent_sysvar: &AccountView,
) -> ProgramResult {
  log!("check_vault_exists");
  // Create when empty and fund rent-exempt.
  if vault.lamports() == 0 {
    let (expected_vault_pda, bump) = derive_pda1(user.address(), VAULT_SEED)?;
    if vault.address() != &expected_vault_pda {
      return Ee::VaultPDA.e();
    }
    let signer_seeds = [
      Seed::from(VAULT_SEED),
      Seed::from(user.address().as_ref()),
      Seed::from(core::slice::from_ref(&bump)),
    ];
    let seed_signer = Signer::from(&signer_seeds);

    // Make the account rent-exempt.
    let rent = Rent::from_account_view(rent_sysvar)?;
    let needed_lamports = rent.try_minimum_balance(VAULT_SIZE)?;
    log!("needed_lamports: {}", needed_lamports);
    log!("VAULT_SIZE: {}", VAULT_SIZE);

    pinocchio_system::instructions::CreateAccount {
      from: user,
      to: vault,
      lamports: needed_lamports,
      space: VAULT_SIZE as u64,
      owner: &PROG_ADDR,
    }
    .invoke_signed(&[seed_signer])?;

    log!("Vault created");
  } else {
    // If vault already exists
    check_pda(vault)?;
    log!("Vault already exists");
  }
  Ok(())
}
