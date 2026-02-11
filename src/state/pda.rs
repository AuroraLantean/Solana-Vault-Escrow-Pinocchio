use pinocchio::{error::ProgramError, sysvars::clock::Clock, AccountView, Address, ProgramResult};

use crate::{none_zero_u64, Ee, PROG_ADDR};

//Vault to hold SOL and control Tokens, and has no struct to be declared
pub const VAULT_SEED: &[u8] = b"vault";
pub const ACCOUNT_DISCRIMINATOR_SIZE: usize = 8;
pub const VAULT_SIZE: usize = ACCOUNT_DISCRIMINATOR_SIZE + size_of::<u64>(); //SOL amount

//TODO: Bytemuck is a great library that makes it easy to read and write byte arrays as structs.
#[derive(Clone, Debug)]
#[repr(C)] //0..8 	Discriminator 	8 bytes
pub struct Config {
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
} // padding: [u8; 6] if the struct size needs to be aligned to 32 bytes.

impl Config {
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
  /*pub fn expected_len(&self) -> u32 {
    u32::from_le_bytes(self.expected_len)
  }*/
  /* pub fn close_authority(&self) -> Option<&Address> {
      if self.has_close_authority() {
          Some(self.close_authority_unchecked())
      } else {
          None
      }
  }*/
  //----------== Load from AccountView
  /* check() is replaced by check_pda
  pub fn check(&self, pda: &AccountView) -> ProgramResult {
    if pda.data_len() != Self::expected_len(&self) as usize {
      return Ee::ConfigDataLengh.e();
    }
    //if pda.data_len() != Self::LEN {}
    // CHECK alignment for the most restrictive field (u64 in this case)... Alignment requirement checking can be removed ONLY IF you know all numbers are using u8 arrays
    /*if (pda.borrow_mut_data_unchecked().as_ptr() as usize) % core::mem::align_of::<Self>() != 0 { return Err();  }*/
    Ok(())
  }*/
  //For Config PDA
  pub fn from_account_view(pda: &AccountView) -> Result<&mut Self, ProgramError> {
    unsafe { Ok(&mut *(pda.borrow_unchecked_mut().as_ptr() as *mut Self)) }
    /*Ok(Ref::map(account_info.try_borrow_data()?, |data| unsafe {
        Self::from_bytes_unchecked(data)
    })) */
  }
  //Must: there are no mutable borrows of the account data
  #[inline]
  pub unsafe fn from_account_info_unchecked(pda: &AccountView) -> Result<&Self, ProgramError> {
    unsafe { Ok(Self::from_bytes_unchecked(pda.borrow_unchecked_mut())) }
    //Ok(&mut *(pda.borrow_mut_data_unchecked().as_ptr() as *mut Self))
  }
  /// The caller must ensure that `bytes` contains a valid representation of `Account`, and
  /// it is properly aligned to be interpreted as an instance of `Account`.
  /// At the moment `Account` has an alignment of 1 byte.
  /// This method does not perform a length validation.
  pub unsafe fn from_bytes_unchecked(bytes: &[u8]) -> &Self {
    unsafe { &*(bytes.as_ptr() as *const &Config) }
  }
  //----------== Setters
  pub fn set_mint0(&mut self, addr: &Address) {
    self.mint0 = addr.clone();
  }
  pub fn set_mint1(&mut self, addr: &Address) {
    self.mint1 = addr.clone();
  }
  pub fn set_mint2(&mut self, addr: &Address) {
    self.mint2 = addr.clone();
  }
  pub fn set_mint3(&mut self, addr: &Address) {
    self.mint3 = addr.clone();
  }
  pub fn set_mints(&mut self, mints: [&Address; 4]) {
    self.mint0 = mints[0].clone();
    self.mint1 = mints[1].clone();
    self.mint2 = mints[2].clone();
    self.mint3 = mints[3].clone();
  }
  pub fn set_vault(&mut self, addr: &Address) {
    self.vault = addr.clone();
  }
  pub fn set_prog_owner(&mut self, addr: &Address) {
    self.prog_owner = addr.clone();
  }
  pub fn set_admin(&mut self, addr: &Address) {
    self.admin = addr.clone();
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
}

//#[repr(C)] keeps the struct layout the same across different architectures
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Status {
  Waiting = 0,
  Active = 1,
  Expired = 2,
  Paused = 3,
  Canceled = 4,
}
impl From<u8> for Status {
  fn from(num: u8) -> Self {
    match num {
      0 => Status::Waiting,
      1 => Status::Active,
      2 => Status::Expired,
      3 => Status::Paused,
      4 => Status::Canceled,
      _ => Status::Canceled,
    }
  }
} //Status::Uninitialized as u8

//------------==
#[derive(Clone, Debug)]
#[repr(C)] //0..8 	Discriminator 	8 bytes
pub struct Escrow {
  maker: Address, //32; PDA needs at least 1 Address to keep PDA addresses from being exhausted by all users using u64. This also gives each user his own Escrow id.
  //taker: Address,   //32 hidden from maker
  mint_x: Address,   //32
  mint_y: Address,   //32
  amount_x: [u8; 8], //8 the offered amount from maker. This field gives taker easier way to view
  amount_y: [u8; 8], //8 the wanted amount to maker. The token_y price in mint_x = this Escrow PDA ATA_X amount / amount_y
  id: [u8; 8],       //8
  decimal_x: u8,     //1
  decimal_y: u8,     //1
  bump: u8,          //1
}
impl Escrow {
  pub const LEN: usize = core::mem::size_of::<Escrow>();
  //pub const LEN: usize = 32 + 32 + 32 + 8 +8+ 1;

  pub const SEED: &[u8] = b"escrow";

  pub fn maker(&self) -> &Address {
    &self.maker
  }
  pub fn mint_x(&self) -> &Address {
    &self.mint_x
  }
  pub fn mint_y(&self) -> &Address {
    &self.mint_y
  }
  pub fn id(&self) -> u64 {
    u64::from_le_bytes(self.id)
  }
  pub fn amount_x(&self) -> u64 {
    u64::from_le_bytes(self.amount_x)
  }
  pub fn amount_y(&self) -> u64 {
    u64::from_le_bytes(self.amount_y)
  }
  pub fn decimal_x(&self) -> u8 {
    self.decimal_x
  }
  pub fn decimal_y(&self) -> u8 {
    self.decimal_y
  }
  pub fn bump(&self) -> u8 {
    self.bump
  }
  pub fn set_maker(&mut self, addr: &Address) {
    self.maker = addr.clone();
  }
  pub fn set_mint_x(&mut self, addr: &Address) {
    self.mint_x = addr.clone();
  }
  pub fn set_mint_y(&mut self, addr: &Address) {
    self.mint_y = addr.clone();
  }
  pub fn set_id(&mut self, amt: u64) -> ProgramResult {
    self.id = amt.to_le_bytes();
    Ok(())
  }
  pub fn set_amount_x(&mut self, amt: u64) -> ProgramResult {
    none_zero_u64(amt)?;
    self.amount_x = amt.to_le_bytes();
    Ok(())
  }
  pub fn set_amount_y(&mut self, amt: u64) -> ProgramResult {
    none_zero_u64(amt)?;
    self.amount_y = amt.to_le_bytes();
    Ok(())
  }
  pub fn set_decimal_x(&mut self, amt: u8) {
    self.decimal_x = amt;
  }
  pub fn set_decimal_y(&mut self, amt: u8) {
    self.decimal_y = amt;
  }
  pub fn set_bump(&mut self, amt: u8) {
    self.bump = amt;
  }
  pub fn check(pda: &AccountView) -> ProgramResult {
    if pda.data_len() != Self::LEN {
      return Ee::EscrowDataLengh.e();
    }
    unsafe {
      if pda.owner().ne(&PROG_ADDR) {
        return Ee::EscrowIsForeign.e();
      }
    }
    Ok(())
  }
  //For Escrow PDA
  pub fn from_account_view(pda: &AccountView) -> Result<&mut Self, ProgramError> {
    Self::check(pda)?;
    unsafe { Ok(&mut *(pda.borrow_unchecked_mut().as_ptr() as *mut Self)) }
  }
  pub unsafe fn from_bytes_unchecked(bytes: &[u8]) -> &Self {
    unsafe { &*(bytes.as_ptr() as *const &&Escrow) }
  }
  #[inline]
  pub unsafe fn from_account_info_unchecked(pda: &AccountView) -> Result<&Self, ProgramError> {
    Self::check(pda)?;
    unsafe { Ok(Self::from_bytes_unchecked(pda.borrow_unchecked_mut())) }
    //unsafe { Ok(&mut *(pda.borrow_mut_data_unchecked().as_ptr() as *mut Self)) }
  }
}

//----------------== Pyth
// pyth-crosschain-main/pythnet/pythnet_sdk/src/messages.rs
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)] //Serialize, Deserialize, BorshSchema
pub struct PriceFeedMessage {
  /// `FeedId` but avoid the type alias because of compatibility issues with Anchor's `idl-build` feature.
  pub feed_id: [u8; 32],
  pub price: i64,
  pub conf: u64,
  pub exponent: i32,
  /// The timestamp of this price update in seconds
  pub publish_time: i64,
  /// The timestamp of the previous price update. This field is intended to allow users to
  /// identify the single unique price update for any moment in time:
  /// for any time t, the unique update is the one such that prev_publish_time < t <= publish_time.
  ///
  /// Note that there may not be such an update while we are migrating to the new message-sending logic,
  /// as some price updates on pythnet may not be sent to other chains (because the message-sending
  /// logic may not have triggered). We can solve this problem by making the message-sending mandatory
  /// (which we can do once publishers have migrated over).
  ///
  /// Additionally, this field may be equal to publish_time if the message is sent on a slot where
  /// where the aggregation was unsuccesful. This problem will go away once all publishers have
  /// migrated over to a recent version of pyth-agent.
  pub prev_publish_time: i64,
  pub ema_price: i64,
  pub ema_conf: u64,
}
// pyth-crosschain-main/target_chains/solana/pyth_solana_receiver_sdk/src/price_update.rs
/// This enum represents how much a price update has been verified:
/// - If `Full`, we have verified the signatures for two thirds of the current guardians.
/// - If `Partial`, only `num_signatures` guardian signatures have been checked.
#[derive(Debug, Clone, PartialEq)]
pub enum VerificationLevel {
  Partial {
    #[allow(unused)]
    num_signatures: u8,
  },
  Full,
}
pub type FeedId = [u8; 32];
#[derive(Clone, Debug)]
#[repr(C)] //0..8 	Discriminator 	8 bytes
pub struct PriceUpdateV2 {
  pub write_authority: Address,
  pub verification_level: VerificationLevel,
  pub price_message: PriceFeedMessage,
  pub posted_slot: u64,
}
impl PriceUpdateV2 {
  pub const LEN: usize = 8 + 32 + 2 + 32 + 8 + 8 + 4 + 8 + 8 + 8 + 8 + 8;

  pub fn check(pda: &AccountView) -> ProgramResult {
    if pda.data_len() != Self::LEN {
      return Ee::PythPriceUpdateV2DataLen.e();
    }
    //check that the accounts are owned by the Pyth Solana Receiver
    unsafe {
      if pda.owner().ne(&Address::from_str_const(
        "rec5EKMGg6MxZYaMdyBfgwp4d5rB9T1VQH5pJv5LtFJ",
      )) {
        return Ee::PythPDA.e();
      }
    }
    Ok(())
  }
  pub fn from_account_view(pda: &AccountView) -> Result<&mut Self, ProgramError> {
    Self::check(pda)?;
    unsafe { Ok(&mut *(pda.borrow_unchecked_mut().as_ptr() as *mut Self)) }
  }

  // target_chains/solana/pyth_solana_receiver_sdk/src/price_update.rs
  /// Ported from get_price_no_older_than_with_custom_verification_level()
  /// Get a `Price` from a `PriceUpdateV2` account for a given `FeedId` no older than `maximum_age` with customizable verification level.
  ///
  /// # Warning
  /// Lowering the verification level from `Full` to `Partial` increases the risk of using a malicious price update.
  /// Please read the documentation for [`VerificationLevel`] for more information.
  ///
  /// # Example
  /// ```ignore
  /// use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, VerificationLevel, PriceUpdateV2};
  /// const MAXIMUM_AGE : u64 = 30;
  /// const FEED_ID: &str = "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d"; // SOL/USD
  ///     let price_update = &mut ctx.accounts.price_update;
  ///     let price = price_update.get_price_no_older_than_with_custom_verification_level(&Clock::get()?, MAXIMUM_AGE, &get_feed_id_from_hex(FEED_ID)?, VerificationLevel::Partial{num_signatures: 5})?;
  ///     Ok(())
  /// }
  ///```
  pub fn get_price(
    &self,
    clock: &Clock,
    maximum_age: u64,
    feed_id: &FeedId,
  ) -> Result<PriceFeedMessage, ProgramError> {
    /*if self.verification_level != VerificationLevel::Full {
      return Err(Ee::PythPriceVerification.into());
    }*/
    // target_chains/solana/pyth_solana_receiver_sdk/src/error.rs

    // if self.price_message.feed_id != *feed_id {
    //   return Err(Ee::PythMismatchedFeedId.into());
    // } //get_price_unchecked(feed_id)?

    //check if price feed update's age exceeds the requested maximum age"
    if self
      .price_message
      .publish_time
      .saturating_add(maximum_age.try_into().unwrap())
      >= clock.unix_timestamp
    {
      return Err(Ee::OraclePriceTooOld.into());
    }
    if self.price_message.price <= 0 {
      return Err(Ee::OraclePriceInvalid.into());
    }
    Ok(self.price_message)
  }
}
