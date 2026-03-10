use litesvm::LiteSVM;
use solana_keypair::Keypair;
use solana_pubkey::Pubkey;
use solana_signer::Signer;
//use solana_message::Message;
use solana_sdk::{inner_instruction, transaction::Transaction};
//use solana_signer::Signer;
//use solana_system_interface::cpi::transfer;
//use solana_transaction::Transaction;

#[test]
fn create_account() {
  let mut svm = LiteSVM::new();

  let user = Keypair::new();

  let mint_authority = Keypair::new();
  svm
    .airdrop(&mint_authority.pubkey(), 1_000_000_000)
    .unwrap();

  svm.airdrop(&user.pubkey(), 1_000_000_000).unwrap();

  let balance = svm.get_balance(&user.pubkey()).unwrap();
  assert_eq!(balance, 1_000_000_000);

  println!("Account funded with {} SOL", balance as f64 / 1e9);
}
#[test]
fn test_transfer() {
  let mut svm = LiteSVM::new();

  // Create two accounts
  let alice = Keypair::new();
  let bob = Keypair::new();

  // Fund Alice
  svm.airdrop(&alice.pubkey(), 2_000_000_000).unwrap();

  /*https://www.litesvm.com/docs/examples/sol-transfer
  let ix = transfer(&alice.pubkey(), &bob.pubkey(), 1_000_000_000);

  // Build and sign transaction
  let tx = Transaction::new_signed_with_payer(
    &[ix],
    Some(&alice.pubkey()),
    &[&alice],
    svm.latest_blockhash(),
  );

  // Send it (execution happens immediately)
  svm.send_transaction(tx).unwrap();

  // Check new balances
  assert_eq!(svm.get_balance(&bob.pubkey()).unwrap(), 1_000_000_000);
  assert!(svm.get_balance(&alice.pubkey()).unwrap() < 1_000_000_000);*/

  println!("Transfer successful!");
}

#[test]
fn make_mint_ata() {
  let mut svm = LiteSVM::new();

  let mint = Keypair::new();
  let mint_data = Mint {
    mint_authority: None.into(),
    supply: 0,
    decimals: 6,
    is_initialized: true,
    freeze_authority: None.into(),
  };
  let mut mint_account_data = vec![0; Mint::LEN];
  Mint::pack(mint_data, &mut mint_account_data).unwrap();

  svm.set_account(
    mint.pubkey(),
    Account {
      lamports: 100_000_000,
      //    let lamports = svm.minimum_balance_for_rent_exemption(Mint::LEN);
      data: mint_account_data,
      owner: TOKEN_PROGRAM_ID,
      executable: false,
      rent_epoch: 0,
    },
  );

  // Create a new Token Account
  let token_account = Keypair::new();
  let owner = Keypair::new();

  let token_account_data = TokenAccount {
    mint: mint.pubkey(),
    owner: owner.pubkey(),
    amount: 0,
    delegate: None.into(),
    state: spl_token::state::AccountState::Initialized,
    is_native: None.into(),
    delegated_amount: 0,
    close_authority: None.into(),
  };
  let mut token_account_data_bytes = vec![0; TokenAccount::LEN];
  TokenAccount::pack(token_account_data, &mut token_account_data_bytes).unwrap();

  svm.set_account(
    token_account.pubkey(),
    Account {
      lamports: 100_000_000,
      // let lamports = svm.minimum_balance_for_rent_exemption(TokenAccount::LEN);
      data: token_account_data_bytes,
      owner: TOKEN_PROGRAM_ID,
      executable: false,
      rent_epoch: 0,
    },
  );
}
