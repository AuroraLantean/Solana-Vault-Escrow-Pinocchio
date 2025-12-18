use pinocchio::{account_info::AccountInfo, program_error::ProgramError};

const TOKEN_2022_ACCOUNT_DISCRIMINATOR_OFFSET: usize = 165;
pub const TOKEN_2022_MINT_DISCRIMINATOR: u8 = 0x01;
pub const TOKEN_2022_TOKEN_ACCOUNT_DISCRIMINATOR: u8 = 0x02;

pub fn check_mint_interface(mint: &AccountInfo) -> Result<(), ProgramError> {
    if !mint.is_owned_by(&pinocchio_token_2022::ID) {
        //legacy token
        if !mint.is_owned_by(&pinocchio_token::ID) {
            return Err(ProgramError::Custom(440));
        } else {
            if mint.data_len().ne(&pinocchio_token::state::Mint::LEN) {
                return Err(ProgramError::Custom(441));
            }
        }
    } else {
        //Token2022
        let data = mint.try_borrow_data()?;

        if data.len().ne(&pinocchio_token::state::Mint::LEN) {
            if data.len().le(&TOKEN_2022_ACCOUNT_DISCRIMINATOR_OFFSET) {
                return Err(ProgramError::Custom(442));
            }
            if data[TOKEN_2022_ACCOUNT_DISCRIMINATOR_OFFSET].ne(&TOKEN_2022_MINT_DISCRIMINATOR) {
                return Err(ProgramError::Custom(443));
            }
        }
    }
    Ok(())
}

pub fn check_tokacct_interface(ata: &AccountInfo) -> Result<(), ProgramError> {
    if !ata.is_owned_by(&pinocchio_token_2022::ID) {
        //Legacy ATA
        if !ata.is_owned_by(&pinocchio_token::ID) {
            return Err(ProgramError::Custom(444));
        } else {
            if ata
                .data_len()
                .ne(&pinocchio_token::state::TokenAccount::LEN)
            {
                return Err(ProgramError::Custom(445));
            }
        }
    } else {
        //Token2022 ATA
        let data = ata.try_borrow_data()?;

        if data.len().ne(&pinocchio_token::state::TokenAccount::LEN) {
            if data.len().le(&TOKEN_2022_ACCOUNT_DISCRIMINATOR_OFFSET) {
                return Err(ProgramError::Custom(446));
            }
            if data[TOKEN_2022_ACCOUNT_DISCRIMINATOR_OFFSET]
                .ne(&TOKEN_2022_TOKEN_ACCOUNT_DISCRIMINATOR)
            {
                return Err(ProgramError::Custom(447));
            }
        }
    }
    Ok(())
}
