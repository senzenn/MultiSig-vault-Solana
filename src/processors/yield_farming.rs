use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    program::invoke,
    instruction::{AccountMeta, Instruction},
};

use crate::state::Vault;

pub fn process_set_yield_strategy(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    token_mint: Pubkey,
    strategy_program: Pubkey,
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

    if let Some(token_balance) = vault.supported_tokens.iter_mut().find(|tb| tb.mint == token_mint) {
        token_balance.yield_strategy = Some(strategy_program);
    } else {
        return Err(ProgramError::InvalidAccountData);
    }

    drop(vault_data);
    vault.serialize(&mut &mut vault_account.data.borrow_mut()[..])?;

    msg!("Yield strategy set for token {}", token_mint);
    Ok(())
}

pub fn process_harvest_yield(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    token_mint: Pubkey,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_account = next_account_info(account_info_iter)?;
    let vault_token_account = next_account_info(account_info_iter)?;
    let reward_token_account = next_account_info(account_info_iter)?;
    let strategy_program = next_account_info(account_info_iter)?;
    let strategy_account = next_account_info(account_info_iter)?;
    let authority = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;
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

    let token_balance = vault.supported_tokens.iter()
        .find(|tb| tb.mint == token_mint)
        .ok_or(ProgramError::InvalidAccountData)?;

    let strategy_program_id = token_balance.yield_strategy
        .ok_or(ProgramError::InvalidAccountData)?;

    if strategy_program_id != *strategy_program.key {
        return Err(ProgramError::InvalidAccountData);
    }

    let harvest_accounts = vec![
        AccountMeta::new(*vault_token_account.key, false),
        AccountMeta::new(*reward_token_account.key, false),
        AccountMeta::new(*strategy_account.key, false),
        AccountMeta::new_readonly(*authority.key, true),
        AccountMeta::new_readonly(*token_program.key, false),
        AccountMeta::new_readonly(*clock_sysvar.key, false),
    ];

    let harvest_ix = Instruction {
        program_id: strategy_program_id,
        accounts: harvest_accounts,
        data: vec![2],
    };

    invoke(&harvest_ix, &[
        vault_token_account.clone(),
        reward_token_account.clone(),
        strategy_account.clone(),
        authority.clone(),
        token_program.clone(),
        clock_sysvar.clone(),
    ])?;

    msg!("Yield harvested for token {} using strategy {}", token_mint, strategy_program_id);
    Ok(())
}

pub fn process_compound_yield(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    token_mint: Pubkey,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_account = next_account_info(account_info_iter)?;
    let vault_token_account = next_account_info(account_info_iter)?;
    let strategy_program = next_account_info(account_info_iter)?;
    let strategy_account = next_account_info(account_info_iter)?;
    let authority = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;
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

    let token_balance = vault.supported_tokens.iter_mut()
        .find(|tb| tb.mint == token_mint)
        .ok_or(ProgramError::InvalidAccountData)?;

    let strategy_program_id = token_balance.yield_strategy
        .ok_or(ProgramError::InvalidAccountData)?;

    if strategy_program_id != *strategy_program.key {
        return Err(ProgramError::InvalidAccountData);
    }

    let harvest_accounts = vec![
        AccountMeta::new(*vault_token_account.key, false),
        AccountMeta::new(*strategy_account.key, false),
        AccountMeta::new_readonly(*authority.key, true),
        AccountMeta::new_readonly(*token_program.key, false),
        AccountMeta::new_readonly(*clock_sysvar.key, false),
    ];

    let harvest_ix = Instruction {
        program_id: strategy_program_id,
        accounts: harvest_accounts,
        data: vec![1],
    };

    invoke(&harvest_ix, &[
        vault_token_account.clone(),
        strategy_account.clone(),
        authority.clone(),
        token_program.clone(),
        clock_sysvar.clone(),
    ])?;

    drop(vault_data);
    vault.serialize(&mut &mut vault_account.data.borrow_mut()[..])?;

    msg!("Yield compounded for token {} using strategy {}", token_mint, strategy_program_id);
    Ok(())
}
