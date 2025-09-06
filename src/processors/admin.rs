use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::state::Vault;

pub fn process_transfer_authority(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    new_authority: Pubkey,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_account = next_account_info(account_info_iter)?;
    let current_authority = next_account_info(account_info_iter)?;

    if !current_authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let vault_data = vault_account.data.borrow();
    let mut vault = Vault::try_from_slice(&vault_data)?;

    if vault.authority != *current_authority.key {
        return Err(ProgramError::InvalidAccountData);
    }

    vault.authority = new_authority;
    vault.emergency_admin = new_authority;

    drop(vault_data);
    vault.serialize(&mut &mut vault_account.data.borrow_mut()[..])?;

    msg!("Authority transferred to {}", new_authority);
    Ok(())
}

pub fn process_update_emergency_admin(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    new_admin: Pubkey,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_account = next_account_info(account_info_iter)?;
    let current_admin = next_account_info(account_info_iter)?;

    if !current_admin.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let vault_data = vault_account.data.borrow();
    let mut vault = Vault::try_from_slice(&vault_data)?;

    if vault.emergency_admin != *current_admin.key {
        return Err(ProgramError::InvalidAccountData);
    }

    vault.emergency_admin = new_admin;

    drop(vault_data);
    vault.serialize(&mut &mut vault_account.data.borrow_mut()[..])?;

    msg!("Emergency admin updated to {}", new_admin);
    Ok(())
}
