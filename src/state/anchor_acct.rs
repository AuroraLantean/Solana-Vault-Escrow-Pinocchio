use crate::Ee;
use pinocchio::{error::ProgramError, AccountView, Address, ProgramResult};
use pinocchio_log::log; //logger::log_message

#[derive(Clone, Debug)]
#[repr(C)]
pub struct SimpleAcct {
  anchor_discriminator: [u8; 8], // 8 bytes
  price: [u8; 8],                //8 bytes for u64,
                                 //unknown: [u8; 8],
}
impl SimpleAcct {
  pub const LEN: usize = 8 + 8; // 16

  pub fn price(&self) -> u64 {
    u64::from_le_bytes(self.price)
  }
  pub fn check(pda: &AccountView) -> ProgramResult {
    log!("SimpleAcct data_len(): {}", pda.data_len()); // 16
    if pda.data_len() != Self::LEN {
      return Ee::SimpleAcctDataLen.e();
    }
    //check that the accounts are owned by the Pyth Solana Receiver
    unsafe {
      //log_message(&pda.owner().to_bytes());
      if pda.owner().ne(&Address::from_str_const(
        "CgZEcSRPh1Ay1EYR4VJPTJRYcRkTDjjZhBAjZ5M8keGp",
      )) {
        return Ee::SimpleAcctOwner.e();
      }
    }
    Ok(())
  }
  pub fn from_account_view(pda: &AccountView) -> Result<&mut Self, ProgramError> {
    Self::check(pda)?;
    log!("check() in from_account_view() successful");

    unsafe { Ok(&mut *(pda.borrow_unchecked_mut().as_ptr() as *mut Self)) }
  }
}
