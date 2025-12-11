use core::convert::TryFrom;
use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};
use pinocchio_log::log;
use pinocchio_system::instructions::CreateAccount;

use crate::{empty_data, empty_lamport, instructions::check_signer, rent_exempt, writable};
use pinocchio_token_2022::{instructions::InitializeAccount3, state::TokenAccount};

/// Token Legacy Init Token Account
pub struct TokenLgcInitTokAcct<'a> {
    pub payer: &'a AccountInfo,
    pub mint: &'a AccountInfo,
    pub owner: &'a AccountInfo,
    pub token_account: &'a AccountInfo,
    pub token_program: &'a AccountInfo,
}
impl<'a> TokenLgcInitTokAcct<'a> {
    pub const DISCRIMINATOR: &'a u8 = &3;

    pub fn process(self) -> ProgramResult {
        let TokenLgcInitTokAcct {
            payer, //signer
            owner,
            mint,
            token_account,
            token_program,
        } = self;
        log!("TokenLgcInitTokAcct process()");
        check_signer(payer)?;
        empty_lamport(token_account)?;
        empty_data(token_account)?;
        rent_exempt(mint, 0)?;

        /*find token_account from owner & token mint"
        cargo add spl-associated-token-account
        use spl_associated_token_account::get_associated_token_address;
        let ata = get_associated_token_address(&wallet, &mint);
        */
        log!("TokenLgcInitTokAcct 2");
        let lamports = Rent::get()?.minimum_balance(TokenAccount::BASE_LEN);
        let space = TokenAccount::BASE_LEN as u64;
        log!("lamports: {}, space: {}", lamports, space);

        log!("Make Token Account");
        CreateAccount {
            from: payer,
            to: token_account,
            owner: token_program.key(),
            lamports,
            space,
        }
        .invoke()?;
        log!("TokenLgcInitTokAcct 7");
        writable(token_account)?;

        log!("Init Token Account");
        InitializeAccount3 {
            account: token_account,
            mint: mint,
            owner: owner.key(),
            token_program: token_program.key(),
        };
        Ok(())
    }
    pub fn init_if_needed(self) -> ProgramResult {
        match empty_lamport(self.token_account) {
            Ok(_) => Self::process(self),
            Err(_) => Ok(()),
        }
    }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for TokenLgcInitTokAcct<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        log!("TokenLgcInitTokAcct try_from");
        let (data, accounts) = value;
        log!("accounts len: {}, data len: {}", accounts.len(), data.len());

        if accounts.len() < 7 {
            return Err(ProgramError::NotEnoughAccountKeys);
        }
        let payer = &accounts[0];
        let mint = &accounts[1];
        let owner = &accounts[2];
        let token_account = &accounts[3];
        let token_program = &accounts[4];
        //let [payer, mint, _] = accounts else {  does not work };
        log!("TokenLgcInitTokAcct try_from end");
        Ok(Self {
            payer,
            mint,
            owner,
            token_account,
            token_program,
        })
    }
}
