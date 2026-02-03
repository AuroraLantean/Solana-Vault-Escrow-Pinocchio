use pinocchio::{error::ProgramError, AccountView, Address, ProgramResult};

use crate::{none_zero_u64, Status};

//TODO: Bytemuck is a great library that makes it easy to read and write byte arrays as structs.
#[derive(Clone, Debug)]
#[repr(C)] //0..8 	Discriminator 	8 bytes
pub struct Config2 {
  mint0: Address,         // 32 bytes
  mint1: Address,         // 32
  mint2: Address,         // 32
  mint3: Address,         // 32
  vault: Address,         // 32
  prog_owner: Address,    // 32
  admin: Address,         // 32
  str_u8array: [u8; 32],  // 32
  fee: [u8; 8],           // 8 for u64,
  sol_balance: [u8; 8],   // 8
  token_balance: [u8; 8], // 8
  updated_at: [u8; 4],    // 4 for u32
  is_authorized: bool,    // 1
  status: u8,             // 1
  vault_bump: u8,         // 1
  bump: u8,               // 1
  new_u32: [u8; 4],       // 4 for u32
  new_u64: [u8; 8],       // 8 for u64
  new_account1: Address,  // 32
} // padding: [u8; 6] if the struct size needs to be aligned to 32 bytes.

impl Config2 {
  pub const INIT_LEN: usize = core::mem::size_of::<Self>();
  pub const SEED: &[u8] = b"config";
  //Getters or Accessors: Safe Direct value copy, no reference created
  pub fn mint0(&self) -> &Address {
    &self.mint0
  }
  pub fn mint1(&self) -> &Address {
    &self.mint1
  }
  pub fn mint2(&self) -> &Address {
    &self.mint2
  }
  pub fn mint3(&self) -> &Address {
    &self.mint3
  }
  pub fn mints(&self) -> [&Address; 4] {
    [&self.mint0, &self.mint1, &self.mint2, &self.mint3]
  }
  pub fn vault(&self) -> &Address {
    &self.vault
  }
  pub fn prog_owner(&self) -> &Address {
    &self.prog_owner
  }
  pub fn admin(&self) -> &Address {
    &self.admin
  }
  pub fn str_u8array(&self) -> &[u8; 32] {
    &self.str_u8array
  }
  pub fn status(&self) -> Status {
    self.status.into()
  }
  pub fn is_authorized(&self) -> bool {
    self.is_authorized
  }
  pub fn vault_bump(&self) -> u8 {
    self.vault_bump
  }
  pub fn bump(&self) -> u8 {
    self.bump
  }
  pub fn fee(&self) -> u64 {
    u64::from_le_bytes(self.fee)
  }
  pub fn sol_balance(&self) -> u64 {
    u64::from_le_bytes(self.sol_balance)
  }
  pub fn token_balance(&self) -> u64 {
    u64::from_le_bytes(self.token_balance)
  }
  pub fn updated_at(&self) -> u32 {
    u32::from_le_bytes(self.updated_at)
  }
  pub fn new_u32(&self) -> u32 {
    u32::from_le_bytes(self.new_u32)
  }
  pub fn new_u64(&self) -> u64 {
    u64::from_le_bytes(self.new_u64)
  }
  //----------== Load from AccountView
  //For Config2 PDA
  pub fn from_account_view(pda: &AccountView) -> Result<&mut Self, ProgramError> {
    unsafe { Ok(&mut *(pda.borrow_unchecked_mut().as_ptr() as *mut Self)) }
    /*Ok(Ref::map(account_info.try_borrow_data()?, |data| unsafe {
        Self::from_bytes_unchecked(data)
    })) */
  }
  //----------== Setters
  pub fn set_mint0(&mut self, pkey: &Address) {
    self.mint0 = pkey.clone();
  }
  pub fn set_mint1(&mut self, pkey: &Address) {
    self.mint1 = pkey.clone();
  }
  pub fn set_mint2(&mut self, pkey: &Address) {
    self.mint2 = pkey.clone();
  }
  pub fn set_mint3(&mut self, pkey: &Address) {
    self.mint3 = pkey.clone();
  }
  pub fn set_mints(&mut self, mints: [&Address; 4]) {
    self.mint0 = mints[0].clone();
    self.mint1 = mints[1].clone();
    self.mint2 = mints[2].clone();
    self.mint3 = mints[3].clone();
  }
  pub fn set_vault(&mut self, pkey: &Address) {
    self.vault = pkey.clone();
  }
  pub fn set_prog_owner(&mut self, pkey: &Address) {
    self.prog_owner = pkey.clone();
  }
  pub fn set_admin(&mut self, pkey: &Address) {
    self.admin = pkey.clone();
  }
  pub fn set_str_u8array(&mut self, str_u8array: [u8; 32]) {
    self.str_u8array = str_u8array;
  }
  pub fn set_fee(&mut self, amt: u64) -> ProgramResult {
    none_zero_u64(amt)?;
    self.fee = amt.to_le_bytes();
    Ok(())
  }
  pub fn set_sol_balance(&mut self, amt: u64) {
    self.sol_balance = amt.to_le_bytes();
  }
  pub fn set_token_balance(&mut self, amt: u64) {
    self.token_balance = amt.to_le_bytes();
  }
  pub fn set_updated_at(&mut self, amt: u32) {
    self.updated_at = amt.to_le_bytes();
  }
  pub fn set_vault_bump(&mut self, amt: u8) {
    self.vault_bump = amt;
  }
  pub fn set_bump(&mut self, amt: u8) {
    self.bump = amt;
  }
  pub fn set_status(&mut self, status: u8) {
    self.status = status;
  }
  pub fn set_is_authorized(&mut self, boo: bool) {
    self.is_authorized = boo;
  }
  pub fn set_new_u32(&mut self, amt: u32) {
    self.new_u32 = amt.to_le_bytes();
  }
  pub fn set_new_u64(&mut self, amt: u64) {
    self.new_u64 = amt.to_le_bytes();
  }
}
