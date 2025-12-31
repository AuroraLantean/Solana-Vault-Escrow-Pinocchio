use solana_signer::Signer;

use crate::litesvm_helpers::setup_escrow_test;
use solana_kite::{
  assert_token_balance, check_account_is_closed, get_pda_and_bump, seeds,
  send_transaction_from_instructions,
};

//TODO: unresolved import in dependencies #273 https://github.com/LiteSVM/litesvm/issues/273
//TODO: you will build a solana program/programs/escrow/src/tests.rs
#[test]
fn test_make_offer_succeeds() {
  let mut test_environment = setup_escrow_test();
}
