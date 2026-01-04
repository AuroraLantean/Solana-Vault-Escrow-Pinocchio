//use num_derive::FromPrimitive;
use pinocchio::{
  account_info::AccountInfo,
  program_error::{ProgramError, ToStr},
  pubkey::{find_program_address, try_find_program_address, Pubkey},
  sysvars::{clock::Clock, rent::Rent, Sysvar},
};
use pinocchio_log::log;
use pinocchio_token_2022::state::{Mint as Mint22, TokenAccount as TokenAccount22};
use thiserror::Error;

use crate::Status;

//TODO: put errors in error.rs ... https://learn.blueshift.gg/en/courses/pinocchio-for-dummies/pinocchio-errors
#[derive(Clone, Debug, Eq, Error, PartialEq)] //FromPrimitive
pub enum Ee {
  #[error("InvalidDiscriminator")]
  InvalidDiscriminator,
  #[error("NotSigner")]
  NotSigner,
  #[error("OnlyOwner")]
  OnlyOwner,
  #[error("OnlyAdmin")]
  OnlyAdmin,
  #[error("OnlyUser")]
  OnlyUser,
  #[error("Xyz005")]
  Xyz005,
  #[error("Xyz006")]
  Xyz006,
  #[error("NotWritable")]
  NotWritable,
  #[error("NotExecutable")]
  NotExecutable,
  #[error("TokenProgram")]
  TokenProgram,
  #[error("AtokenGPvbd")]
  AtokenGPvbd,
  #[error("SystemProgram")]
  SystemProgram,
  #[error("InputDataUnderMin")]
  InputDataUnderMin,
  #[error("InputDataOverMax")]
  InputDataOverMax,
  #[error("Xyz009")]
  Xyz009,
  //Bytes for Numerical
  #[error("ZeroAsU128")]
  ZeroAsU128,
  #[error("ZeroAsU64")]
  ZeroAsU64,
  #[error("ZeroAsU32")]
  ZeroAsU32,
  #[error("ZeroAsU16")]
  ZeroAsU16,
  #[error("ZeroAsU8")]
  ZeroAsU8,

  //Inputs
  #[error("StrOverMax")]
  StrOverMax,
  #[error("StrUnderMin")]
  StrUnderMin,
  #[error("ArrayLength")]
  ArrayLength,
  #[error("ByteForStatus")]
  ByteForStatus,
  #[error("ByteForSBool")]
  ByteForSBool,
  #[error("Xyz010")]
  Xyz010,
  //Bytes Sizes
  #[error("ByteSizeForU128")]
  ByteSizeForU128,
  #[error("ByteSizeForU64")]
  ByteSizeForU64,
  #[error("ByteSizeForU32")]
  ByteSizeForU32,
  #[error("ByteSizeForU16")]
  ByteSizeForU16,
  #[error("ByteSizeForU8")]
  ByteSizeForU8,
  #[error("Xyz020")]
  Xyz020,
  //Byte Slice Sizes
  #[error("ByteSliceSize64")]
  ByteSliceSize64,
  #[error("ByteSliceSize32")]
  ByteSliceSize32,
  #[error("ByteSliceSize10")]
  ByteSliceSize10,
  #[error("ByteSliceSize6")]
  ByteSliceSize6,
  #[error("Xyz030")]
  Xyz030,
  //PDA
  #[error("PdaNoLamport")]
  PdaNoLamport,
  #[error("ForeignPDA")]
  ForeignPDA,
  #[error("ConfigPDA")]
  ConfigPDA,
  #[error("VaultPDA")]
  VaultPDA,
  #[error("AdminPDA")]
  AdminPDA,
  #[error("UserPDA")]
  UserPDA,
  #[error("EscrowPDA")]
  EscrowPDA,
  #[error("ActionPDA")]
  ActionPDA,
  #[error("Xyz040")]
  Xyz040,
  //PDA Data Length
  #[error("ConfigDataLengh")]
  ConfigDataLengh,
  #[error("VaultDataLengh")]
  VaultDataLengh,
  #[error("AdminDataLengh")]
  AdminDataLengh,
  #[error("UserDataLengh")]
  UserDataLengh,
  #[error("EscrowDataLengh")]
  EscrowDataLengh,
  #[error("ActionDataLengh")]
  ActionDataLengh,
  #[error("Xyz050")]
  Xyz050,
  //Mint Account
  #[error("DecimalsValue")]
  DecimalsValue,
  #[error("MintDataLen")]
  MintDataLen,
  #[error("MintOrMintAuthority")]
  MintOrMintAuthority,
  #[error("MintOrTokenProgram")]
  MintOrTokenProgram,
  #[error("Xyz060")]
  Xyz060,
  //ATA
  #[error("TokAcctDataLen")]
  TokAcctDataLen,
  #[error("Tok22AcctDataLen")]
  Tok22AcctDataLen,
  #[error("AtaOrOwner")]
  AtaOrOwner,
  #[error("AtaOrMint")]
  AtaOrMint,
  #[error("AtaCheckX1")]
  AtaCheckX1,
  #[error("ForeignAta")]
  ForeignAta,
  #[error("Xyz070")]
  Xyz070,

  //Token 2022
  #[error("Acct22Type")]
  Acct22Type,
  #[error("Tok22AcctDiscOffset")]
  Tok22AcctDiscOffset,
  #[error("NotRentExamptMint22")]
  NotRentExamptMint22,
  #[error("NotRentExamptTokAcct22")]
  NotRentExamptTokAcct22,
  #[error("Xyz080")]
  Xyz080,

  //Withdraw
  #[error("PdaToBeBelowRentExampt")]
  PdaToBeBelowRentExampt,
  #[error("ToWallet")]
  ToWallet,
  //Math
  #[error("MathOverflow")]
  MathOverflow,
  #[error("MathUnderflow")]
  MathUnderflow,
  #[error("Xyz090")]
  Xyz090,
  //Misc...
  #[error("EmptyData")]
  EmptyData,
  #[error("FunctionSelector")]
  FunctionSelector,
  #[error("ClockGet")]
  ClockGet,
  #[error("ErrorNotMapped")]
  ErrorNotMapped,
}
impl From<Ee> for ProgramError {
  fn from(e: Ee) -> Self {
    ProgramError::Custom(e as u32)
  }
}
//Deserialize Errors from Raw Values
impl TryFrom<u32> for Ee {
  type Error = ProgramError;
  fn try_from(error: u32) -> Result<Self, Self::Error> {
    match error {
      0 => Ok(Ee::InvalidDiscriminator),
      1 => Ok(Ee::NotSigner),
      2 => Ok(Ee::NotWritable),
      3 => Ok(Ee::NotExecutable),
      4 => Ok(Ee::ZeroAsU64),
      5 => Ok(Ee::DecimalsValue),
      6 => Ok(Ee::MintDataLen),
      7 => Ok(Ee::TokAcctDataLen),
      8 => Ok(Ee::Tok22AcctDataLen),
      9 => Ok(Ee::TokenProgram),
      10 => Ok(Ee::SystemProgram),
      11 => Ok(Ee::AtaOrOwner),
      12 => Ok(Ee::AtaOrMint),
      13 => Ok(Ee::AtaCheckX1),
      14 => Ok(Ee::ForeignAta),
      15 => Ok(Ee::ForeignPDA),
      17 => Ok(Ee::EmptyData),
      18 => Ok(Ee::Acct22Type),
      19 => Ok(Ee::StrOverMax),
      20 => Ok(Ee::StrUnderMin),
      21 => Ok(Ee::InputDataUnderMin),
      22 => Ok(Ee::InputDataOverMax),
      23 => Ok(Ee::PdaNoLamport),
      25 => Ok(Ee::Tok22AcctDiscOffset),
      26 => Ok(Ee::ByteForStatus),
      27 => Ok(Ee::ByteForSBool),
      28 => Ok(Ee::ByteSizeForU64),
      29 => Ok(Ee::ByteSizeForU32),
      30 => Ok(Ee::ByteSizeForU16),
      31 => Ok(Ee::ByteSizeForU8),
      32 => Ok(Ee::VaultPDA),
      33 => Ok(Ee::ConfigDataLengh),
      34 => Ok(Ee::FunctionSelector),
      35 => Ok(Ee::ConfigPDA),

      37 => Ok(Ee::MathOverflow),
      38 => Ok(Ee::MathUnderflow),
      39 => Ok(Ee::NotRentExamptMint22),
      40 => Ok(Ee::NotRentExamptTokAcct22),
      42 => Ok(Ee::MintOrMintAuthority),
      43 => Ok(Ee::MintOrTokenProgram),
      45 => Ok(Ee::PdaToBeBelowRentExampt),
      46 => Ok(Ee::ToWallet),

      48 => Ok(Ee::ByteSliceSize32),
      49 => Ok(Ee::ByteSliceSize10),
      50 => Ok(Ee::ByteSliceSize6),
      51 => Ok(Ee::AtokenGPvbd),
      52 => Ok(Ee::ClockGet),
      53 => Ok(Ee::OnlyOwner),
      54 => Ok(Ee::UserPDA),
      55 => Ok(Ee::ZeroAsU32),
      56 => Ok(Ee::EscrowDataLengh),
      _ => Err(Ee::ErrorNotMapped.into()),
    }
  }
}
//Human Readable Errors; TODO: arrange below
impl ToStr for Ee {
  fn to_str<E>(&self) -> &'static str {
    match self {
      Ee::Xyz009 => "Xyz009",
      Ee::Xyz010 => "Xyz010",
      Ee::Xyz020 => "Xyz020",
      Ee::Xyz030 => "Xyz030",
      Ee::Xyz040 => "Xyz040",
      Ee::Xyz050 => "Xyz050",
      Ee::Xyz060 => "Xyz060",
      Ee::Xyz070 => "Xyz070",
      Ee::Xyz080 => "Xyz080",
      Ee::Xyz090 => "Xyz090",
      Ee::EscrowPDA => "EscrowPDA",
      Ee::EscrowDataLengh => "EscrowDataLengh",
      Ee::ActionDataLengh => "ActionDataLengh",
      Ee::UserDataLengh => "UserDataLengh",
      Ee::AdminDataLengh => "AdminDataLengh",
      Ee::VaultDataLengh => "VaultDataLengh",
      Ee::ByteSliceSize64 => "ByteSliceSize64",
      Ee::ArrayLength => "ArrayLength",
      Ee::AdminPDA => "AdminPDA",
      Ee::ActionPDA => "ActionPDA",
      Ee::ErrorNotMapped => "ErrorNotMapped",
      Ee::InvalidDiscriminator => "InvalidDiscriminator",
      Ee::NotSigner => "NotSigner",
      Ee::NotWritable => "NotWritable",
      Ee::NotExecutable => "NotExecutable",
      Ee::ZeroAsU128 => "ZeroAsU128",
      Ee::ZeroAsU64 => "ZeroAsU64",
      Ee::ZeroAsU32 => "ZeroAsU32",
      Ee::ZeroAsU16 => "ZeroAsU16",
      Ee::ZeroAsU8 => "ZeroAsU8",
      Ee::DecimalsValue => "DecimalsValue",
      Ee::MintDataLen => "MintDataLen",
      Ee::TokAcctDataLen => "TokAcctDataLen",
      Ee::Tok22AcctDataLen => "Tok22AcctDataLen",
      Ee::TokenProgram => "TokenProgram",
      Ee::SystemProgram => "SystemProgram",
      Ee::AtaOrOwner => "AtaOrOwner",
      Ee::AtaOrMint => "AtaOrMint",
      Ee::AtaCheckX1 => "AtaCheckX1",
      Ee::ForeignAta => "ForeignAta",
      Ee::ForeignPDA => "ForeignPDA",
      Ee::EmptyData => "EmptyData",
      Ee::Acct22Type => "Acct22Type",
      Ee::StrOverMax => "StrOverMax",
      Ee::StrUnderMin => "StrUnderMin",
      Ee::InputDataUnderMin => "InputDataUnderMin",
      Ee::InputDataOverMax => "InputDataOverMax",
      Ee::PdaNoLamport => "PdaNoLamport",
      Ee::Tok22AcctDiscOffset => "Tok22AcctDiscOffset",
      Ee::ByteForStatus => "ByteForStatus",
      Ee::ByteForSBool => "ByteForSBool",
      Ee::ByteSizeForU128 => "ByteSizeForU128",
      Ee::ByteSizeForU64 => "ByteSizeForU64",
      Ee::ByteSizeForU32 => "ByteSizeForU32",
      Ee::ByteSizeForU16 => "ByteSizeForU16",
      Ee::ByteSizeForU8 => "ByteSizeForU8",
      Ee::VaultPDA => "VaultPDA",
      Ee::ConfigDataLengh => "ConfigDataLengh",
      Ee::FunctionSelector => "FunctionSelector",
      Ee::ConfigPDA => "ConfigPDA",
      Ee::UserPDA => "UserPDA",
      Ee::MathOverflow => "MathOverflow",
      Ee::MathUnderflow => "MathUnderflow",
      Ee::NotRentExamptMint22 => "NotRentExamptMint22",
      Ee::NotRentExamptTokAcct22 => "NotRentExamptTokAcct22",
      Ee::MintOrMintAuthority => "MintOrMintAuthority",
      Ee::MintOrTokenProgram => "MintOrTokenProgram",
      Ee::PdaToBeBelowRentExampt => "PdaToBeBelowRentExampt",
      Ee::ToWallet => "ToWallet",

      Ee::ByteSliceSize32 => "ByteSliceSize32",
      Ee::ByteSliceSize10 => "ByteSliceSize10",
      Ee::ByteSliceSize6 => "ByteSliceSize6",
      Ee::AtokenGPvbd => "AtokenGPvbd",
      Ee::ClockGet => "ClockGet",
      Ee::OnlyOwner => "OnlyOwner",
      Ee::OnlyAdmin => "OnlyAdmin",
      Ee::OnlyUser => "OnlyUser",
      Ee::Xyz005 => "Xyz005",
      Ee::Xyz006 => "Xyz006",
    }
  }
}

//----------------== Account Verification Functions
pub fn check_signer(account: &AccountInfo) -> Result<(), ProgramError> {
  if !account.is_signer() {
    return Err(Ee::NotSigner.into());
  }
  Ok(())
}
pub fn check_mint0a(mint: &AccountInfo, token_program: &AccountInfo) -> Result<(), ProgramError> {
  //if !mint.is_owned_by(mint_authority)
  if mint.data_len() != pinocchio_token::state::Mint::LEN {
    return Err(Ee::MintDataLen.into());
  }
  if !token_program.key().eq(&pinocchio_token::ID) {
    return Err(Ee::TokenProgram.into());
  }
  if mint.owner() != &pinocchio_token::ID {
    return Err(Ee::MintOrTokenProgram.into());
  }
  Ok(())
}

pub fn check_mint0b(
  mint: &AccountInfo,
  mint_authority: &AccountInfo,
  token_program: &AccountInfo,
  decimals: u8,
) -> Result<(), ProgramError> {
  let mint_info = pinocchio_token::state::Mint::from_account_info(mint)?;
  if mint_info
    .mint_authority()
    .is_some_and(|authority| !mint_authority.key().eq(authority))
  {
    return Err(Ee::MintOrMintAuthority.into());
  }
  if decimals != mint_info.decimals() {
    return Err(Ee::DecimalsValue.into());
  }
  check_mint0a(mint, token_program)?;
  //TODO: over mint supply?
  Ok(())
}

pub fn check_mint22a(mint: &AccountInfo, token_program: &AccountInfo) -> Result<(), ProgramError> {
  //if !mint.is_owned_by(mint_authority)
  if mint.data_len() != pinocchio_token_2022::state::Mint::BASE_LEN {
    return Err(Ee::MintDataLen.into());
  }
  if !token_program.key().eq(&pinocchio_token_2022::ID) {
    return Err(Ee::TokenProgram.into());
  }
  if mint.owner() != &pinocchio_token_2022::ID {
    return Err(Ee::MintOrTokenProgram.into());
  }
  Ok(())
}
pub fn check_mint22b(
  mint: &AccountInfo,
  mint_authority: &AccountInfo,
  token_program: &AccountInfo,
  decimals: u8,
) -> Result<(), ProgramError> {
  let mint_info = pinocchio_token_2022::state::Mint::from_account_info(mint)?;

  if mint_info
    .mint_authority()
    .is_some_and(|authority| !mint_authority.key().eq(authority))
  {
    return Err(Ee::MintOrMintAuthority.into());
  }
  if decimals != mint_info.decimals() {
    return Err(Ee::DecimalsValue.into());
  }
  check_mint22a(mint, token_program)?;
  //TODO: over mint supply?
  Ok(())
}

pub fn check_ata(
  ata: &AccountInfo,
  owner: &AccountInfo,
  mint: &AccountInfo,
) -> Result<(), ProgramError> {
  if ata
    .data_len()
    .ne(&pinocchio_token::state::TokenAccount::LEN)
  {
    return Err(Ee::TokAcctDataLen.into());
  }
  let ata_info = pinocchio_token::state::TokenAccount::from_account_info(ata)?;
  if !ata_info.owner().eq(owner.key()) {
    return Err(Ee::AtaOrOwner.into());
  }
  if !ata_info.mint().eq(mint.key()) {
    return Err(Ee::AtaOrMint.into());
  }
  Ok(())
}
pub fn check_ata22(
  ata: &AccountInfo,
  owner: &AccountInfo,
  mint: &AccountInfo,
) -> Result<(), ProgramError> {
  // token2022 ata has first 165 bytes the same as the legacy ata, but then some more data //log!("ata22 len:{}", ata.data_len());
  let ata_info = TokenAccount22::from_account_info(ata)?;
  if !ata_info.owner().eq(owner.key()) {
    return Err(Ee::AtaOrOwner.into());
  }
  if !ata_info.mint().eq(mint.key()) {
    return Err(Ee::AtaOrMint.into());
  }
  Ok(())
}
pub fn check_ata_x1(
  authority: &AccountInfo,
  token_program: &AccountInfo,
  mint: &AccountInfo,
  ata: &AccountInfo,
) -> Result<(), ProgramError> {
  if find_program_address(
    &[authority.key(), token_program.key(), mint.key()],
    &pinocchio_associated_token_account::ID,
  )
  .0
  .ne(ata.key())
  {
    return Err(Ee::AtaCheckX1.into());
  }
  Ok(())
}
pub fn check_pda(account: &AccountInfo) -> Result<(), ProgramError> {
  if account.lamports() == 0 {
    return Err(Ee::PdaNoLamport.into());
  }
  if !account.is_owned_by(&crate::ID) {
    return Err(Ee::ForeignPDA.into());
  }
  Ok(())
}
pub fn check_initialized(account: &AccountInfo) -> Result<(), ProgramError> {
  if account.lamports() > 0 {
    return Err(ProgramError::AccountAlreadyInitialized);
  }
  Ok(())
}
pub fn check_sysprog(system_program: &AccountInfo) -> Result<(), ProgramError> {
  if !system_program.key().eq(&pinocchio_system::ID) {
    return Err(Ee::SystemProgram.into());
  }
  Ok(())
}
pub const ATOKENGPVBD: pinocchio_pubkey::reexport::Pubkey = pinocchio_system::ID; //[0, 0];
pub fn check_atoken_gpvbd(atoken_program: &AccountInfo) -> Result<(), ProgramError> {
  if !atoken_program.key().eq(&ATOKENGPVBD) {
    return Err(Ee::AtokenGPvbd.into());
  }
  Ok(())
}

//----------------== Check Account Properties
pub fn writable(account: &AccountInfo) -> Result<(), ProgramError> {
  if !account.is_writable() {
    return Err(Ee::NotWritable.into());
  }
  Ok(())
}
pub fn executable(account: &AccountInfo) -> Result<(), ProgramError> {
  if !account.executable() {
    return Err(Ee::NotExecutable.into());
  }
  Ok(())
}
//TODO: Mint and ATA from TokenLgc works. For mint and ATA from Token2022?
/// acc_type: 0 Mint, 1 TokenAccount
pub fn rent_exempt22(account: &AccountInfo, acc_type: u8) -> Result<(), ProgramError> {
  if acc_type == 0 && account.lamports() < Rent::get()?.minimum_balance(Mint22::BASE_LEN) {
    return Err(Ee::NotRentExamptMint22.into());
  }
  if acc_type == 1 && account.lamports() < Rent::get()?.minimum_balance(TokenAccount22::BASE_LEN) {
    return Err(Ee::NotRentExamptTokAcct22.into());
  }
  if acc_type > 1 {
    return Err(Ee::Acct22Type.into());
  }
  Ok(())
}
pub fn is_rent_exempt(account: &AccountInfo) -> Result<(u64, u64), ProgramError> {
  let min_balance = Rent::get()?.minimum_balance(account.data_len());
  let current = account.lamports();
  if current < min_balance {
    return Err(ProgramError::AccountNotRentExempt);
  }
  Ok((current, min_balance))
}
pub fn empty_lamport(account: &AccountInfo) -> Result<(), ProgramError> {
  if account.lamports() == 0 {
    return Ok(());
  }
  Err(ProgramError::AccountAlreadyInitialized)
}
pub fn empty_data(account: &AccountInfo) -> Result<(), ProgramError> {
  if account.data_len() == 0 {
    return Ok(());
  }
  Err(Ee::EmptyData.into())
}

//----------------== Check Input Values
pub fn min_data_len(data: &[u8], min: usize) -> Result<(), ProgramError> {
  if data.len() < min {
    return Err(Ee::InputDataUnderMin.into());
  }
  Ok(())
}
pub fn max_data_len(data: &[u8], max: usize) -> Result<(), ProgramError> {
  if data.len() > max {
    return Err(Ee::InputDataOverMax.into());
  }
  Ok(())
}

pub fn check_decimals(mint: &AccountInfo, decimals: u8) -> Result<(), ProgramError> {
  let mint_info = pinocchio_token::state::Mint::from_account_info(mint)?;
  if decimals != mint_info.decimals() {
    return Err(Ee::DecimalsValue.into());
  }
  Ok(())
}
pub fn check_decimals_max(decimals: u8, max: u8) -> Result<(), ProgramError> {
  if decimals > max {
    return Err(Ee::DecimalsValue.into());
  }
  Ok(())
}
pub fn check_str_len(s: &str, min_len: usize, max_len: usize) -> Result<(), ProgramError> {
  if s.len() < min_len {
    return Err(Ee::StrOverMax.into());
  }
  if s.len() > max_len {
    return Err(Ee::StrUnderMin.into());
  }
  Ok(())
}

//----------------== Parse Functions
/// Parse a u64 from u8 array
pub fn parse_u64(data: &[u8]) -> Result<u64, ProgramError> {
  let bytes: [u8; 8] = data.try_into().or_else(|_e| Err(Ee::ByteSizeForU64))?;

  let amt = u64::from_le_bytes(bytes);
  // let amount = u64::from_le_bytes([data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7]]);
  if amt == 0 {
    return Err(Ee::ZeroAsU64.into());
  }
  Ok(amt)
}
pub fn parse_u32(data: &[u8]) -> Result<u32, ProgramError> {
  let bytes: [u8; 4] = data.try_into().or_else(|_e| Err(Ee::ByteSizeForU32))?;

  let amt = u32::from_le_bytes(bytes);
  // let amount = u64::from_le_bytes([data[0], data[1], data[2], data[3]]);
  if amt == 0 {
    return Err(Ee::ZeroAsU32.into());
  }
  Ok(amt)
}
pub fn to32bytes(byte_slice: &[u8]) -> Result<&[u8; 32], ProgramError> {
  let bytes: &[u8; 32] = byte_slice.try_into().map_err(|_| Ee::ByteSliceSize32)?;
  //let mut str_u8array = [0u8; 32];
  //str_u8array.copy_from_slice(&data[10..42]);
  return Ok(bytes);
}
pub fn to10bytes(byte_slice: &[u8]) -> Result<&[u8; 10], ProgramError> {
  let bytes: &[u8; 10] = byte_slice.try_into().map_err(|_| Ee::ByteSliceSize10)?;
  return Ok(bytes);
}
pub fn to6bytes(byte_slice: &[u8]) -> Result<&[u8; 6], ProgramError> {
  let bytes: &[u8; 6] = byte_slice.try_into().map_err(|_| Ee::ByteSliceSize6)?;
  return Ok(bytes);
}
pub fn u8_to_bool(v: u8) -> Result<bool, ProgramError> {
  match v {
    0 => Ok(false),
    1 => Ok(true),
    _ => Err(Ee::ByteForSBool.into()),
  }
}
pub fn u8_to_status(v: u8) -> Result<Status, ProgramError> {
  match v {
    0 => Ok(Status::Waiting),
    1 => Ok(Status::Active),
    2 => Ok(Status::Expired),
    3 => Ok(Status::Paused),
    4 => Ok(Status::Canceled),
    _ => Err(Ee::ByteForStatus.into()),
  }
}

//----------------== Derive Functions
pub fn derive_pda1(user: &AccountInfo, bstr: &[u8]) -> Result<(Pubkey, u8), ProgramError> {
  log!("derive_pda1");
  //find_program_address(&[b"vault", user.key().as_ref()], &crate::ID)
  // let (pda, _bump) =
  try_find_program_address(&[bstr, user.key().as_ref()], &crate::ID)
    .ok_or_else(|| ProgramError::InvalidSeeds)
}
/*let pda = pubkey::create_program_address(
    &[PDA_SEED, &[self.datas.bump as u8]],
    &crate::ID,
) */

//----------------== Token 2022 Interface
/// [4 (extension discriminator) + 32 (update_authority) + 32 (metadata)]
pub const METADATA_POINTER_SIZE: usize = 4 + 32 + 32;
/// [4 (extension discriminator) + 32 (update_authority) + 32 (mint) + 4 (size of name ) + 4 (size of symbol) + 4 (size of uri) + 4 (size of additional_metadata)]
pub const METADATA_EXTENSION_BASE_SIZE: usize = 4 + 32 + 32 + 4 + 4 + 4 + 4;
/// Padding used so that Mint and Account extensions start at the same index
pub const EXTENSIONS_PADDING_AND_OFFSET: usize = 84;
const TOKEN_2022_ACCOUNT_DISCRIMINATOR_OFFSET: usize = 165;
pub const TOKEN_2022_MINT_DISCRIMINATOR: u8 = 0x01;
pub const TOKEN_2022_TOKEN_ACCOUNT_DISCRIMINATOR: u8 = 0x02;

pub fn check_mint_interface(mint: &AccountInfo) -> Result<(), ProgramError> {
  if !mint.is_owned_by(&pinocchio_token_2022::ID) {
    //legacy token
    if !mint.is_owned_by(&pinocchio_token::ID) {
      return Err(Ee::MintOrTokenProgram.into());
    } else {
      if mint.data_len().ne(&pinocchio_token::state::Mint::LEN) {
        return Err(Ee::MintDataLen.into());
      }
    }
  } else {
    //Token2022
    let data = mint.try_borrow_data()?;

    if data.len().ne(&pinocchio_token::state::Mint::LEN) {
      if data.len().le(&TOKEN_2022_ACCOUNT_DISCRIMINATOR_OFFSET) {
        return Err(Ee::Tok22AcctDataLen.into());
      }
      if data[TOKEN_2022_ACCOUNT_DISCRIMINATOR_OFFSET].ne(&TOKEN_2022_MINT_DISCRIMINATOR) {
        return Err(Ee::Tok22AcctDiscOffset.into());
      }
    }
  }
  Ok(())
}

pub fn check_tokacct_interface(ata: &AccountInfo) -> Result<(), ProgramError> {
  if !ata.is_owned_by(&pinocchio_token_2022::ID) {
    //Legacy ATA
    if !ata.is_owned_by(&pinocchio_token::ID) {
      return Err(Ee::ForeignAta.into());
    } else {
      if ata
        .data_len()
        .ne(&pinocchio_token::state::TokenAccount::LEN)
      {
        return Err(Ee::TokAcctDataLen.into());
      }
    }
  } else {
    //Token2022 ATA
    let data = ata.try_borrow_data()?;

    if data.len().ne(&pinocchio_token::state::TokenAccount::LEN) {
      if data.len().le(&TOKEN_2022_ACCOUNT_DISCRIMINATOR_OFFSET) {
        return Err(Ee::Tok22AcctDataLen.into());
      }
      if data[TOKEN_2022_ACCOUNT_DISCRIMINATOR_OFFSET].ne(&TOKEN_2022_TOKEN_ACCOUNT_DISCRIMINATOR) {
        return Err(Ee::Tok22AcctDiscOffset.into());
      }
    }
  }
  Ok(())
}
pub fn get_time() -> Result<u32, ProgramError> {
  let clock = Clock::get().map_err(|_| Ee::ClockGet)?;
  let time = clock.unix_timestamp as u32;
  log!("Solana time: {}", time);
  Ok(time)
}
