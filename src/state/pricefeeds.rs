use crate::Ee;
use pinocchio::{error::ProgramError, sysvars::clock::Clock, AccountView, Address, ProgramResult};
use pinocchio_log::log; //logger::log_message

//----------------== Pyth
// pyth-crosschain-main/pythnet/pythnet_sdk/src/messages.rs
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)] //Serialize, Deserialize, BorshSchema
pub struct PriceFeedMessage {
  pub feed_id: [u8; 32],
  price: [u8; 8], //i64,
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
impl PriceFeedMessage {
  pub fn price(&self) -> i64 {
    i64::from_le_bytes(self.price)
  }
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
#[derive(Clone, Debug)]
#[repr(C)]
pub struct PriceUpdateV2 {
  anchor_discriminator: [u8; 8],         // 8 bytes
  write_authority: Address,              // 32 bytes
  verification_level: VerificationLevel, // 2 bytes
  price_message: PriceFeedMessage,       // 32 + 8 + 8 + 4 + 8 + 8
  posted_slot: [u8; 8],                  //8 bytes for u64,
  unknown: [u8; 8],                      //8 bytes
}
impl PriceUpdateV2 {
  pub const LEN: usize = 8 + 32 + 2 + 32 + 8 + 8 + 4 + 8 + 8 + 8 + 8 + 8; // 134

  pub fn write_authority(&self) -> &Address {
    &self.write_authority
  }
  pub fn verification_level(&self) -> &VerificationLevel {
    &self.verification_level
  }
  pub fn price_message(&self) -> &PriceFeedMessage {
    &self.price_message
  }
  pub fn posted_slot(&self) -> u64 {
    log!("posted_slot len: {}", self.posted_slot.len() as u64); // 8
    log!("posted_slot[0]: {}", self.posted_slot[0]);
    //error message: Program A9TPi1RSW5apQcZch9CUz5EnuyfSF773zxndJowMrcK3 failed: account data too small for instruction

    log!("posted_slot: {}", &self.posted_slot);
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
  pub fn from_account_view(pda: &AccountView) -> Result<&mut Self, ProgramError> {
    Self::check(pda)?;
    log!("check() in from_account_view() successful");
    /*
    // 8-bytes aligned account data + 8 bytes of trailing data.
    let mut data = [0u64; size_of::<RuntimeAccount>() / size_of::<u64>() + 1];
    data[0] = NOT_BORROWED as u64;

    let account = data.as_mut_ptr() as *mut RuntimeAccount;
    unsafe { (*account).data_len = 8 };

    let account_view = AccountView { raw: account };     */
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
    feed_id: &[u8; 32],
  ) -> Result<PriceFeedMessage, ProgramError> {
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
      .publish_time
      .saturating_add(maximum_age.try_into().unwrap())
      >= clock.unix_timestamp
    {
      return Err(Ee::OraclePriceTooOld.into());
    }
    if self.price_message.price() <= 0 {
      return Err(Ee::OraclePriceInvalid.into());
    }
    Ok(self.price_message)
  }
}
