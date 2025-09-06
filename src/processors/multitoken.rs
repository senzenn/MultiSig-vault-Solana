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
use spl_associated_token_account::instruction as ata_instruction;

use crate::state::Vault;

pub fn process_add_supported_token(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    mint: Pubkey,
    _bump: u8,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_account = next_account_info(account_info_iter)?;
    let authority = next_account_info(account_info_iter)?;
    let vault_token_account = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;
    let associated_token_program = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;
    let rent_sysvar = next_account_info(account_info_iter)?;

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

    invoke(
        &ata_instruction::create_associated_token_account(
            authority.key,
            vault_account.key,
            &mint,
            token_program.key,
        ),
        &[
            authority.clone(),
            vault_account.clone(),
            vault_token_account.clone(),
            token_program.clone(),
            associated_token_program.clone(),
            system_program.clone(),
            rent_sysvar.clone(),
        ],
    )?;

    vault.supported_tokens.push(crate::state::TokenBalance {
        mint,
        balance: 0,
        yield_strategy: None,
    });

    drop(vault_data);
    vault.serialize(&mut &mut vault_account.data.borrow_mut()[..])?;

    msg!("Token {} added to vault", mint);
    Ok(())
}

pub fn process_deposit_multi_token(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    mint: Pubkey,
    amount: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_account = next_account_info(account_info_iter)?;
    let user_token_account = next_account_info(account_info_iter)?;
    let vault_token_account = next_account_info(account_info_iter)?;
    let authority = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;

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

    if !vault.supported_tokens.iter().any(|tb| tb.mint == mint) {
        return Err(ProgramError::InvalidAccountData);
    }

    invoke(
        &token_instruction::transfer(
            token_program.key,
            user_token_account.key,
            vault_token_account.key,
            authority.key,
            &[authority.key],
            amount,
        )?,
        &[
            user_token_account.clone(),
            vault_token_account.clone(),
            authority.clone(),
            token_program.clone(),
        ],
    )?;

    if let Some(token_balance) = vault.supported_tokens.iter_mut().find(|tb| tb.mint == mint) {
        token_balance.balance = token_balance.balance.checked_add(amount)
            .ok_or(ProgramError::InvalidArgument)?;
    }
    vault.total_value_locked = vault.total_value_locked.checked_add(amount)
        .ok_or(ProgramError::InvalidArgument)?;

    drop(vault_data);
    vault.serialize(&mut &mut vault_account.data.borrow_mut()[..])?;

    msg!("Deposited {} of token {}", amount, mint);
    Ok(())
}

pub fn process_withdraw_multi_token(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    mint: Pubkey,
    amount: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_account = next_account_info(account_info_iter)?;
    let vault_token_account = next_account_info(account_info_iter)?;
    let user_token_account = next_account_info(account_info_iter)?;
    let authority = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;

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

    if let Some(token_balance) = vault.supported_tokens.iter().find(|tb| tb.mint == mint) {
        if token_balance.balance < amount {
            return Err(ProgramError::InsufficientFunds);
        }
    } else {
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

    if let Some(token_balance) = vault.supported_tokens.iter_mut().find(|tb| tb.mint == mint) {
        token_balance.balance = token_balance.balance.checked_sub(amount)
            .ok_or(ProgramError::InvalidArgument)?;
    }
    vault.total_value_locked = vault.total_value_locked.checked_sub(amount)
        .ok_or(ProgramError::InvalidArgument)?;

    drop(vault_data);
    vault.serialize(&mut &mut vault_account.data.borrow_mut()[..])?;

    msg!("Withdrew {} of token {}", amount, mint);
    Ok(())
}
