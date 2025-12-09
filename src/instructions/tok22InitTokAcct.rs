use core::convert::TryFrom;
use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};
use pinocchio_log::log;
use pinocchio_system::instructions::CreateAccount;

use crate::{check_empty_acct, instructions::check_signer};
use pinocchio_token_2022::{instructions::InitializeAccount3, state::TokenAccount};

/// Token2022 Init Token Account
pub struct Token2022InitTokAcct<'a> {
    pub payer: &'a AccountInfo,
    pub owner: &'a AccountInfo,
    pub mint_account: &'a AccountInfo,
    pub token_account: &'a AccountInfo,
    pub token_program: &'a AccountInfo,
}
impl<'a> Token2022InitTokAcct<'a> {
    pub const DISCRIMINATOR: &'a u8 = &3;

    pub fn process(self) -> ProgramResult {
        let Token2022InitTokAcct {
            payer,
            owner,
            mint_account,
            token_account,
            token_program,
        } = self;
        check_signer(payer)?;
        check_empty_acct(token_account)?;

        log!("Make Token Account");
        CreateAccount {
            from: payer,
            to: token_account,
            owner: token_program.key(),
            lamports: Rent::get()?.minimum_balance(TokenAccount::BASE_LEN),
            space: TokenAccount::BASE_LEN as u64,
        }
        .invoke()?;

        if !token_account.is_writable() {
            return Err(ProgramError::InvalidAccountData);
        }
        log!("Init Token Account");
        InitializeAccount3 {
            account: token_account,
            mint: mint_account,
            owner: owner.key(),
            token_program: token_program.key(),
        };
        Ok(())
    }
    pub fn init_if_needed(self) -> ProgramResult {
        match check_empty_acct(self.token_account) {
            Ok(_) => Self::process(self),
            Err(_) => Ok(()),
        }
    }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for Token2022InitTokAcct<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        let (data, accounts) = value;

        let [payer, owner, mint_account, token_account, token_program, _] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        if data.len() < 1 {
            return Err(ProgramError::AccountDataTooSmall);
        }

        Ok(Self {
            payer,
            owner,
            mint_account,
            token_account,
            token_program,
        })
    }
}
