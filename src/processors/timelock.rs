use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    program::invoke,
    clock::Clock,
    sysvar::Sysvar,
};
use spl_token::instruction as token_instruction;

use crate::state::Vault;

pub fn process_create_time_lock(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    beneficiary: Pubkey,
    amount: u64,
    duration: i64,
    cliff_duration: Option<i64>,
    is_linear: bool,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_account = next_account_info(account_info_iter)?;
    let authority = next_account_info(account_info_iter)?;
    let clock_sysvar = next_account_info(account_info_iter)?;

    if !authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let vault_data = vault_account.data.borrow();
    let mut vault = Vault::try_from_slice(&vault_data)?;

    let is_authorized = if let Some(multi_sig) = &vault.multi_sig {
        multi_sig.authorities.contains(authority.key)
    } else {
        vault.authority == *authority.key
    };

    if !is_authorized || vault.paused {
        return Err(ProgramError::InvalidAccountData);
    }

    let clock = Clock::from_account_info(clock_sysvar)?;
    let start_time = clock.unix_timestamp;
    let end_time = start_time + duration;
    let cliff_time = cliff_duration.map(|d| start_time + d).unwrap_or(end_time);

    let time_lock = crate::state::TimeLock {
        beneficiary,
        amount,
        start_time,
        end_time,
        cliff_time,
        released_amount: 0,
        is_linear,
    };

    vault.time_locks.push(time_lock);

    drop(vault_data);
    vault.serialize(&mut &mut vault_account.data.borrow_mut()[..])?;

    msg!("Time lock created for {} tokens to {}", amount, beneficiary);
    Ok(())
}

pub fn process_claim_time_lock(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    time_lock_index: usize,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_account = next_account_info(account_info_iter)?;
    let vault_token_account = next_account_info(account_info_iter)?;
    let user_token_account = next_account_info(account_info_iter)?;
    let beneficiary = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;
    let clock_sysvar = next_account_info(account_info_iter)?;

    if !beneficiary.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let vault_data = vault_account.data.borrow();
    let mut vault = Vault::try_from_slice(&vault_data)?;

    if time_lock_index >= vault.time_locks.len() {
        return Err(ProgramError::InvalidAccountData);
    }

    let time_lock = &mut vault.time_locks[time_lock_index];

    if time_lock.beneficiary != *beneficiary.key {
        return Err(ProgramError::InvalidAccountData);
    }

    let clock = Clock::from_account_info(clock_sysvar)?;
    let current_time = clock.unix_timestamp;

    if current_time < time_lock.cliff_time {
        return Err(ProgramError::InvalidAccountData);
    }

    let releasable_amount = if current_time >= time_lock.end_time {
        time_lock.amount - time_lock.released_amount
    } else if time_lock.is_linear {
        let total_duration = time_lock.end_time - time_lock.start_time;
        let elapsed = current_time - time_lock.start_time;
        let vested_amount = (time_lock.amount as i64 * elapsed / total_duration) as u64;
        vested_amount - time_lock.released_amount
    } else {
        time_lock.amount - time_lock.released_amount
    };

    if releasable_amount == 0 {
        return Err(ProgramError::InvalidAccountData);
    }

    if vault.legacy_total_deposited < releasable_amount {
        return Err(ProgramError::InsufficientFunds);
    }

    invoke(
        &token_instruction::transfer(
            token_program.key,
            vault_token_account.key,
            user_token_account.key,
            vault_account.key,
            &[vault_account.key],
            releasable_amount,
        )?,
        &[
            vault_token_account.clone(),
            user_token_account.clone(),
            vault_account.clone(),
            token_program.clone(),
        ],
    )?;

    time_lock.released_amount += releasable_amount;
    vault.legacy_total_deposited -= releasable_amount;
    vault.total_value_locked -= releasable_amount;

    if time_lock.released_amount >= time_lock.amount {
        vault.time_locks.remove(time_lock_index);
    }

    drop(vault_data);
    vault.serialize(&mut &mut vault_account.data.borrow_mut()[..])?;

    msg!("Claimed {} tokens from time lock", releasable_amount);
    Ok(())
}

pub fn process_cancel_time_lock(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    time_lock_index: usize,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_account = next_account_info(account_info_iter)?;
    let authority = next_account_info(account_info_iter)?;

    if !authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let vault_data = vault_account.data.borrow();
    let mut vault = Vault::try_from_slice(&vault_data)?;

    let is_authorized = if let Some(multi_sig) = &vault.multi_sig {
        multi_sig.authorities.contains(authority.key)
    } else {
        vault.authority == *authority.key
    };

    if !is_authorized {
        return Err(ProgramError::InvalidAccountData);
    }

    if time_lock_index >= vault.time_locks.len() {
        return Err(ProgramError::InvalidAccountData);
    }

    vault.time_locks.remove(time_lock_index);

    drop(vault_data);
    vault.serialize(&mut &mut vault_account.data.borrow_mut()[..])?;

    msg!("Time lock {} cancelled", time_lock_index);
    Ok(())
}
