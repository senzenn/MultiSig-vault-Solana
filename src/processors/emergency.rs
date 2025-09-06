use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    program::invoke,
};
use spl_token::instruction as token_instruction;

use crate::state::Vault;

pub fn process_pause_vault(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_account = next_account_info(account_info_iter)?;
    let authority = next_account_info(account_info_iter)?;

    if !authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let vault_data = vault_account.data.borrow();
    let mut vault = Vault::try_from_slice(&vault_data)?;

    if vault.emergency_admin != *authority.key {
        return Err(ProgramError::InvalidAccountData);
    }

    vault.paused = true;

    drop(vault_data);
    vault.serialize(&mut &mut vault_account.data.borrow_mut()[..])?;

    msg!("Vault paused by emergency admin");
    Ok(())
}

pub fn process_unpause_vault(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_account = next_account_info(account_info_iter)?;
    let authority = next_account_info(account_info_iter)?;

    if !authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let vault_data = vault_account.data.borrow();
    let mut vault = Vault::try_from_slice(&vault_data)?;

    if vault.emergency_admin != *authority.key {
        return Err(ProgramError::InvalidAccountData);
    }

    vault.paused = false;

    drop(vault_data);
    vault.serialize(&mut &mut vault_account.data.borrow_mut()[..])?;

    msg!("Vault unpaused by emergency admin");
    Ok(())
}

pub fn process_emergency_withdraw(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    token_mint: Pubkey,
    amount: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_account = next_account_info(account_info_iter)?;
    let vault_token_account = next_account_info(account_info_iter)?;
    let user_token_account = next_account_info(account_info_iter)?;
    let emergency_admin = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;

    if !emergency_admin.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let vault_data = vault_account.data.borrow();
    let mut vault = Vault::try_from_slice(&vault_data)?;

    if vault.emergency_admin != *emergency_admin.key {
        return Err(ProgramError::InvalidAccountData);
    }

    invoke(
        &token_instruction::transfer(
            token_program.key,
            vault_token_account.key,
            user_token_account.key,
            vault_account.key,
            &[vault_account.key],
            amount,
        )?,
        &[
            vault_token_account.clone(),
            user_token_account.clone(),
            vault_account.clone(),
            token_program.clone(),
        ],
    )?;

    if let Some(token_balance) = vault.supported_tokens.iter_mut().find(|tb| tb.mint == token_mint) {
        token_balance.balance = token_balance.balance.saturating_sub(amount);
    }
    vault.total_value_locked = vault.total_value_locked.saturating_sub(amount);

    drop(vault_data);
    vault.serialize(&mut &mut vault_account.data.borrow_mut()[..])?;

    msg!("Emergency withdrawal of {} tokens executed", amount);
    Ok(())
}
