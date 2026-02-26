use crate::{
  check_pda, executable, get_rent_exempt, instructions::check_signer, writable, Ee, LoanDataAcct,
  PROG_ADDR,
};
use core::convert::TryFrom;
use pinocchio::{
  cpi::{Seed, Signer},
  error::ProgramError,
  sysvars::instructions::INSTRUCTIONS_ID,
  AccountView, ProgramResult,
};
use pinocchio_log::log;
use pinocchio_system::instructions::CreateAccount;

/// FlashloanBorrow
pub struct FlashloanBorrow<'a> {
  pub signer: &'a AccountView,
  pub lender_pda: &'a AccountView,
  pub loan_data: &'a AccountView,
  pub mint: &'a AccountView,
  pub instruction_sysvar: &'a AccountView,
  pub token_program: &'a AccountView,
  pub system_program: &'a AccountView,
  pub config_pda: &'a AccountView,
  pub rent_sysvar: &'a AccountView,
  pub token_accounts: &'a [AccountView],
  //pub lender_ata: &'a AccountView,
  //pub user_ata: &'a AccountView,
  pub bump: [u8; 1],
  pub fee: u16,
  pub amounts: &'a [u64],
} /*Flashloan{
  lender_pda, lender_ata,
  user_ata, mint, user(signer),
  config, sysvar_instructions,
  token_program, system_program }*/
impl<'a> FlashloanBorrow<'a> {
  pub const DISCRIMINATOR: &'a u8 = &22;

  pub fn process(self) -> ProgramResult {
    log!("FlashloanBorrow process()");
    let FlashloanBorrow {
      signer,
      lender_pda,
      loan_data,
      mint,
      instruction_sysvar,
      token_program,
      system_program,
      config_pda,
      rent_sysvar,
      token_accounts,
      bump,
      fee,
      amounts,
    } = self;
    let fee_bytes = fee.to_le_bytes();
    let signer_seeds = [
      Seed::from("protocol".as_bytes()),
      Seed::from(&fee_bytes),
      Seed::from(&self.bump),
    ];
    let signer_seeds = [Signer::from(&signer_seeds)];

    // Open the LoanData account and create a mutable slice to push the Loan struct to it
    let size = size_of::<LoanDataAcct>() * amounts.len();
    let lamports = get_rent_exempt(self.loan_data, rent_sysvar, size)?;

    CreateAccount {
      from: signer,
      to: loan_data,
      lamports,
      space: size as u64,
      owner: &PROG_ADDR,
    }
    .invoke()?;

    Ok(())
  }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountView])> for FlashloanBorrow<'a> {
  type Error = ProgramError;

  fn try_from(value: (&'a [u8], &'a [AccountView])) -> Result<Self, Self::Error> {
    log!("FlashloanBorrow try_from");
    let (data, accounts) = value;
    log!("accounts len: {}, data len: {}", accounts.len(), data.len());
    //let instruction_data = LoanInstructionData::try_from(data)?;

    let [signer, lender_pda, loan_data, mint, instruction_sysvar, config_pda, token_program, system_program, rent_sysvar, token_accounts @ ..] =
      accounts
    else {
      return Err(ProgramError::NotEnoughAccountKeys);
    }; //lender_ata, user_ata
    check_signer(signer)?;
    writable(loan_data)?;
    executable(token_program)?;
    writable(config_pda)?;
    check_pda(config_pda)?;
    //check_mint0a(token_mint, token_program)?;

    if instruction_sysvar.address().ne(&INSTRUCTIONS_ID) {
      return Err(ProgramError::UnsupportedSysvar);
    }
    // Each loan requires a protocol_token_account and a borrower_token_account
    if (token_accounts.len() % 2).ne(&0) || token_accounts.len().eq(&0) {
      return Err(Ee::TokenAcctsLength.into());
    }
    if loan_data.try_borrow()?.len().ne(&0) {
      return Err(Ee::LoanDataAcct.into());
    }

    //-------== parse variadic data
    let (bump, data) = data.split_first().ok_or_else(|| Ee::ByteSizeForU8)?;

    let (fee, data) = data
      .split_at_checked(size_of::<u16>())
      .ok_or_else(|| Ee::ByteSizeForU16)?;
    let fee = u16::from_le_bytes(
      fee
        .try_into()
        .map_err(|_| ProgramError::InvalidInstructionData)?,
    );
    log!("fee: {}", fee);
    if data.len() % size_of::<u64>() != 0 {
      return Err(Ee::ByteSizeForU64.into());
    }
    //Deriving the protocol PDA with the fee creates isolated liquidity pools for each fee tier, eliminating the need to store fee data in accounts. This design is both safe and optimal since each PDA with a specific fee owns only the liquidity associated with that fee rate. If someone passes an invalid fee, the corresponding token account for that fee bracket will be empty, automatically causing the transfer to fail with insufficient funds.

    // Get the amount slice
    let amounts: &[u64] = unsafe {
      core::slice::from_raw_parts(data.as_ptr() as *const u64, data.len() / size_of::<u64>())
    };
    log!("amounts: {}", amounts);
    if amounts.len() != token_accounts.len() / 2 {
      return Err(Ee::AmountsLenVsTokenAcctLen.into());
    }
    Ok(Self {
      signer,
      lender_pda,
      loan_data,
      mint,
      instruction_sysvar,
      config_pda,
      token_program,
      system_program,
      rent_sysvar,
      token_accounts,
      //lender_ata, user_ata,
      bump: [*bump],
      fee,
      amounts,
    })
  }
}
