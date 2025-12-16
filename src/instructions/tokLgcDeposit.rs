use core::convert::TryFrom;
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};
use pinocchio_log::log;

use crate::{
    check_decimals, executable, instructions::check_signer, parse_u64, rent_exempt, writable,
};
use pinocchio_token::state::TokenAccount;

/// TokLgc Deposit Tokens
pub struct TokLgcDeposit<'a> {
    pub authority: &'a AccountInfo, //signer
    pub from_ata: &'a AccountInfo,
    pub to_ata: &'a AccountInfo,
    pub to_wallet: &'a AccountInfo,
    pub mint: &'a AccountInfo,
    pub token_program: &'a AccountInfo,
    pub system_program: &'a AccountInfo,
    pub atoken_program: &'a AccountInfo,
    pub decimals: u8,
    pub amount: u64,
}
impl<'a> TokLgcDeposit<'a> {
    pub const DISCRIMINATOR: &'a u8 = &8;

    pub fn process(self) -> ProgramResult {
        let TokLgcDeposit {
            authority,
            from_ata,
            to_ata,
            to_wallet,
            mint,
            token_program,
            system_program,
            atoken_program: _,
            decimals,
            amount,
        } = self;
        log!("TokLgcDeposit process()");
        check_signer(authority)?;
        executable(token_program)?;

        log!("TokLgcDeposit 1");
        rent_exempt(mint, 0)?;
        //writable(mint)?;
        check_decimals(mint, decimals)?;

        log!("TokLgcDeposit 4");
        if !mint.is_owned_by(token_program.key()) {
            return Err(ProgramError::InvalidAccountData);
        }

        log!("TokLgcDeposit 5");
        if !system_program.key().eq(&pinocchio_system::ID) {
            return Err(ProgramError::IncorrectProgramId);
        }

        if to_ata.data_is_empty() {
            log!("Make to_ata");
            pinocchio_associated_token_account::instructions::Create {
                funding_account: authority,
                account: to_ata,
                wallet: to_wallet,
                mint,
                system_program,
                token_program,
            }
            .invoke()?;
            //Please upgrade to SPL Token 2022 for immutable owner support
        } else {
            log!("to_ata has data");
            let to_ata_info = TokenAccount::from_account_info(to_ata)?;
            if !to_ata_info.owner().eq(to_wallet.key()) {
                return Err(ProgramError::InvalidAccountData);
            }
        }
        writable(to_ata)?;
        rent_exempt(to_ata, 1)?;
        log!("T=_ATA is found/verified");

        log!("Transfer Tokens");
        pinocchio_token::instructions::TransferChecked {
            from: from_ata,
            mint,
            to: to_ata,
            authority,
            amount, // unsafe { *(data.as_ptr().add(1 + 8) as *const u64)}
            decimals,
        }
        .invoke()?;
        /*  pinocchio_token::instructions::Transfer {
            from: vault,
            to: to_ata,
            authority: escrow,
            amount: vault_account.amount(),
        }.invoke_signed(&[seeds.clone()])?; */
        Ok(())
    }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for TokLgcDeposit<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        log!("TokLgcDeposit try_from");
        let (data, accounts) = value;
        log!("accounts len: {}, data len: {}", accounts.len(), data.len());

        let [authority, from_ata, to_ata, to_wallet, mint, token_program, system_program, atoken_program] =
            accounts
        else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        //1+8: u8 takes 1, u64 takes 8 bytes
        if data.len() < 9 {
            return Err(ProgramError::AccountDataTooSmall);
        }
        let decimals = data[0];
        let amount = parse_u64(&data[1..])?;
        log!("decimals: {}, amount: {}", decimals, amount);
        Ok(Self {
            authority,
            from_ata,
            to_ata,
            to_wallet,
            mint,
            token_program,
            system_program,
            atoken_program,
            decimals,
            amount,
        })
    }
}
/*  pinocchio_token::instructions::CloseAccount {
    account: vault,
    destination: maker,
    authority: escrow,
}.invoke_signed(&[seeds])?;  */
