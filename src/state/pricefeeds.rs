use crate::{get_time_i64, Ee};
use pinocchio::{error::ProgramError, AccountView, Address};
use pinocchio_log::log; //logger::log_message

//----------------== Pyth
pub fn read_oracle_pda(
  oracle_vendor: u8,
  pda: &AccountView,
  feed_id: [u8; 32],
) -> Result<u64, ProgramError> {
  let price = match oracle_vendor {
    0 | 1 => pyth_network(pda, feed_id)?,
    //255 => simple_acct(pda, feed_id)?,
    _ => return Err(Ee::OracleNum.into()),
  };
  Ok(price)
}
pub const MAX_PRICE_AGE: u64 = 60; // in seconds

pub fn pyth_network(pda: &AccountView, feed_id: [u8; 32]) -> Result<u64, ProgramError> {
  log!("pyth_network");
  //Pyth Devnet or Mainnet https://docs.pyth.network/price-feeds/core/contract-addresses/solana
  //check that the accounts are owned by the Pyth Solana Receiver according to https://docs.pyth.network/price-feeds/core/contract-addresses/solana
  log!("PythPriceUpdateV2 data_len(): {}", pda.data_len()); // 134
  unsafe {
    //log_message(&pda.owner().to_bytes());
    if pda.owner().ne(&Address::from_str_const(
      "rec5EKMGg6MxZYaMdyBfgwp4d5rB9T1VQH5pJv5LtFJ",
    )) {
      return Err(Ee::PythPDA.into());
    }
  }
  pda.check_borrow()?;
  let data = pda.try_borrow()?;
  let price_update: &PriceUpdateV2 = PriceUpdateV2::from_account_data(&data)?;
  //let price_update: &PriceUpdateV2 = PriceUpdateV2::from_account_view(&pda)?;
  log!("pyth_network reading price_update success");

  //Anchor
  //let price = price_update.get_price_no_older_than(&Clock::get()?, maximum_age, &feed_id)?;

  /*if price_update.write_authority().ne(&Address::from_str_const(
    "4cSM2e6rvbGQUFiJbqytoVMi5GgghSMr8LwVrT9VPSPo",
  )) { log!("write_authority incorrect!!!");}*/

  if !price_update.is_fully_verified() {
    log!("verification_level: not verified!!!");
    return Err(Ee::PythPriceVerification.into());
  }

  let price_mesg = price_update.price_message();
  if !price_mesg.feed_id().eq(&feed_id) {
    log!("feed_id is NOT correct");
    log!("feed_id: {}", &feed_id);
    log!("price_mesg.feed_id(): {}", price_mesg.feed_id());
    return Err(Ee::PythMismatchedFeedId.into());
  }
  log!("posted_slot: {}", price_update.posted_slot());

  let asset_price = price_update.get_price_no_older_than(MAX_PRICE_AGE, &feed_id)?;

  Ok(asset_price as u64)
}
// pyth-crosschain-main/pythnet/pythnet_sdk/src/messages.rs
//#[derive(Debug, Copy, Clone, PartialEq)] //Serialize, Deserialize, BorshSchema
#[repr(C)]
pub struct PriceFeedMessage {
  feed_id: [u8; 32],
  price: [u8; 8],        //i64,
  conf: [u8; 8],         //u64,
  exponent: [u8; 4],     //i32,
  publish_time: [u8; 8], //i64, in seconds
  /// for any time t, the unique update is the one such that prev_publish_time < t <= publish_time.
  /// Note that there may not be such an update while we are migrating to the new message-sending logic,
  /// as some price updates on pythnet may not be sent to other chains (because the message-sending
  /// logic may not have triggered). We can solve this problem by making the message-sending mandatory (which we can do once publishers have migrated over).
  ///
  /// Additionally, this field may be equal to publish_time if the message is sent on a slot where where the aggregation was unsuccesful. This problem will go away once all publishers have migrated over to a recent version of pyth-agent.
  prev_publish_time: [u8; 8], //i64,
  ema_price: [u8; 8],    //i64,
  ema_conf: [u8; 8],     //u64,
}
impl PriceFeedMessage {
  pub fn feed_id(&self) -> &[u8; 32] {
    &self.feed_id
  }
  pub fn price(&self) -> i64 {
    i64::from_le_bytes(self.price)
  }
  pub fn conf(&self) -> u64 {
    u64::from_le_bytes(self.conf)
  }
  pub fn exponent(&self) -> i32 {
    i32::from_le_bytes(self.exponent)
  }
  pub fn publish_time(&self) -> i64 {
    i64::from_le_bytes(self.publish_time)
  }
  pub fn prev_publish_time(&self) -> i64 {
    i64::from_le_bytes(self.prev_publish_time)
  }
  pub fn ema_price(&self) -> i64 {
    i64::from_le_bytes(self.ema_price)
  }
  pub fn ema_conf(&self) -> u64 {
    u64::from_le_bytes(self.ema_conf)
  }
}
// pyth-crosschain-main/target_chains/solana/pyth_solana_receiver_sdk/src/price_update.rs
/// This enum represents how much a price update has been verified:
/// - If `Full`, we have verified the signatures for two thirds of the current guardians.
/// - If `Partial`, only `num_signatures` guardian signatures have been checked.
/*#[derive(Debug, Clone, PartialEq)]
pub enum VerificationLevel {
  Partial {
    #[allow(unused)]
    num_signatures: u8,
  },
  Full,
}*/
#[repr(C)] //#[derive(Clone, Debug)]
pub struct PriceUpdateV2 {
  anchor_discriminator: [u8; 8], // 8 bytes
  write_authority: Address,      // 32 bytes
  /// `[0]` = variant (0 = Partial, 1 = Full), `[1]` = num_signatures when Partial.
  verification_level: u8, // 1 bytes
  //verification_level: [u8; 2], // 2 bytes
  price_message: PriceFeedMessage, // 32 + 8 + 8 + 4 + 8 + 8
  posted_slot: [u8; 8],            //8 bytes for u64, the last unknown 8 bytes are Anchor padding
}
impl PriceUpdateV2 {
  /// Total serialized size in bytes — verified at compile time via `size_of`.
  pub const LEN: usize = core::mem::size_of::<PriceUpdateV2>();
  //pub const LEN: usize = 8 + 32 + 2 + 32 + 8 + 8 + 4 + 8 + 8 + 8 + 8 + 8; // 134

  /// Anchor discriminator: first 8 bytes of `sha256("account:PriceUpdateV2")`.
  pub const DISCRIMINATOR: [u8; 8] = [0x22, 0xf1, 0x23, 0x63, 0x9d, 0x7e, 0xf4, 0xcd];

  /// Zero-copy borrow of `bytes` as a `PriceUpdateV2`.
  ///
  /// # Safety
  /// `bytes` must be at least `Self::LEN` bytes long and contain a valid Borsh-serialised `PriceUpdateV2`.  No alignment constraint beyond 1 byte.
  #[inline(always)]
  pub unsafe fn from_bytes_unchecked(bytes: &[u8]) -> &Self {
    &*(bytes.as_ptr() as *const Self)
  }
  /// Borrow `data` as a `PriceUpdateV2`, checking length and discriminator.
  /// Returns `None` when either check fails.
  #[inline]
  pub fn from_account_data(data: &[u8]) -> Result<&Self, ProgramError> {
    log!("PythPriceUpdateV2 data_len() should be 134: {}", data.len());
    if data.len() < Self::LEN {
      return Err(Ee::PythPriceUpdateV2DataLen.into());
    }
    if data[..8] != Self::DISCRIMINATOR {
      return Err(Ee::PythPriceUpdateV2Discriminator.into());
    }
    //check that the accounts are owned by the Pyth Solana Receiver
    unsafe { Ok(Self::from_bytes_unchecked(data)) }
  }
  pub fn from_account_view(pda: &AccountView) -> Result<&Self, ProgramError> {
    if pda.data_len() < Self::LEN {
      return Err(Ee::PythPriceUpdateV2DataLen.into());
    }
    unsafe { Ok(&*(pda.try_borrow().unwrap().as_ptr() as *const Self)) }
  }
  pub fn write_authority(&self) -> &Address {
    &self.write_authority
  }
  /// `true` when all guardian signatures have been verified.
  #[inline(always)]
  pub fn is_fully_verified(&self) -> bool {
    self.verification_level == 1
    //self.verification_level[0] == 1
  }
  /// Number of guardian signatures checked for a `Partial` verification. Returns `None` when the verification level is `Full`.
  /*#[inline(always)]
  pub fn num_signatures(&self) -> Option<u8> {
    if self.verification_level[0] == 0 {
      Some(self.verification_level[1])
    } else {
      None
    }
  }*/
  pub fn price_message(&self) -> &PriceFeedMessage {
    &self.price_message
  }
  pub fn posted_slot(&self) -> u64 {
    //error message: Program failed: account data too small for instruction
    u64::from_le_bytes(self.posted_slot)
  }
  // target_chains/solana/pyth_solana_receiver_sdk/src/price_update.rs
  pub fn get_price_no_older_than(
    &self,
    maximum_age: u64,
    feed_id: &[u8; 32],
  ) -> Result<f64, ProgramError> {
    log!("get_price(): feed_id input: {}", feed_id);
    // target_chains/solana/pyth_solana_receiver_sdk/src/error.rs
    let price_mesg = self.price_message();
    log!("price: {}", price_mesg.price());
    log!("conf: {}", price_mesg.conf());
    log!("exponent: {}", price_mesg.exponent());
    log!("publish_time: {}", price_mesg.publish_time());
    log!("prev_publish_time: {}", price_mesg.prev_publish_time());
    log!(
      "delta_time:  {}",
      price_mesg.publish_time() - price_mesg.prev_publish_time()
    );

    // The actual price is `(price ± conf)* 10^exponent`.
    log!(
      "The price is ({} ± {}) * 10^{}",
      price_mesg.price(),
      price_mesg.conf(), //confidence_interval
      price_mesg.exponent()
    );
    if price_mesg.price() <= 0 {
      return Err(Ee::OraclePriceInvalid.into());
    }
    let asset_price = price_mesg.price() as f64 * 10f64.powi(price_mesg.exponent());
    log!("asset_price = {}", asset_price as i64);

    //check if price feed update's age exceeds the requested maximum age"
    let max_time = price_mesg
      .publish_time()
      .saturating_add(maximum_age.try_into().unwrap());
    log!("max_time:    {}", max_time);
    let time = get_time_i64()?;
    if price_mesg
      .publish_time()
      .saturating_add(maximum_age.try_into().unwrap())
      <= time
    {
      return Err(Ee::OraclePriceTooOld.into());
    }
    Ok(asset_price)
  }
}
