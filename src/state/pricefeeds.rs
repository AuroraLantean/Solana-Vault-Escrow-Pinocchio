use crate::Ee;
use pinocchio::{error::ProgramError, sysvars::clock::Clock, AccountView, Address, ProgramResult};
use pinocchio_log::log; //logger::log_message

//----------------== Pyth
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
    log!("PythPriceUpdateV2 data_len(): {}", data.len()); // 134
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
    Self::check(pda)?;
    log!("check() in from_account_view() successful");
    unsafe {
      let reinterpreted = &*(pda.try_borrow().unwrap().as_ptr() as *const Self);
      Ok(reinterpreted)
    }
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
  pub fn check(pda: &AccountView) -> ProgramResult {
    log!("PythPriceUpdateV2 data_len(): {}", pda.data_len()); // 134
    if pda.data_len() != Self::LEN {
      return Ee::PythPriceUpdateV2DataLen.e();
    }
    //check that the accounts are owned by the Pyth Solana Receiver
    unsafe {
      //log_message(&pda.owner().to_bytes());
      if pda.owner().ne(&Address::from_str_const(
        "rec5EKMGg6MxZYaMdyBfgwp4d5rB9T1VQH5pJv5LtFJ",
      )) {
        return Ee::PythPDA.e();
      }
    }
    Ok(())
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
    feed_id: &[u8; 32],
  ) -> Result<(), ProgramError> {
    //PriceFeedMessage
    log!("get_price(): feed_id input: {}", feed_id);
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
      .publish_time()
      .saturating_add(maximum_age.try_into().unwrap())
      >= clock.unix_timestamp
    {
      return Err(Ee::OraclePriceTooOld.into());
    }
    if self.price_message.price() <= 0 {
      return Err(Ee::OraclePriceInvalid.into());
    }
    Ok(())
    //Ok(self.price_message)
  }
}
