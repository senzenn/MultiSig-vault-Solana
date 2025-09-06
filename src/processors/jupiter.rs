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
use crate::defi_protocols;

pub fn process_jupiter_swap(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    input_mint: Pubkey,
    output_mint: Pubkey,
    amount: u64,
    slippage_bps: u16,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_account = next_account_info(account_info_iter)?;
    let input_token_account = next_account_info(account_info_iter)?;
    let output_token_account = next_account_info(account_info_iter)?;
    let jupiter_program = next_account_info(account_info_iter)?;
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

    if *jupiter_program.key != defi_protocols::JUPITER_AGGREGATOR {
        return Err(ProgramError::InvalidAccountData);
    }

    let swap_accounts = vec![
        AccountMeta::new(*input_token_account.key, false),
        AccountMeta::new(*output_token_account.key, false),
        AccountMeta::new_readonly(*authority.key, true),
        AccountMeta::new_readonly(*token_program.key, false),
    ];

    let mut swap_data = vec![0];
    swap_data.extend_from_slice(&amount.to_le_bytes());
    swap_data.extend_from_slice(&slippage_bps.to_le_bytes());

    let swap_ix = Instruction {
        program_id: *jupiter_program.key,
        accounts: swap_accounts,
        data: swap_data,
    };

    invoke(&swap_ix, &[
        input_token_account.clone(),
        output_token_account.clone(),
        authority.clone(),
        token_program.clone(),
    ])?;

    vault.total_value_locked = vault.total_value_locked.saturating_sub(amount);

    drop(vault_data);
    vault.serialize(&mut &mut vault_account.data.borrow_mut()[..])?;

    msg!("Jupiter swap executed: {} {} -> {} with {} bps slippage", amount, input_mint, output_mint, slippage_bps);
    Ok(())
}

pub fn process_jupiter_route(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    input_mint: Pubkey,
    output_mint: Pubkey,
    amount: u64,
    route: Vec<u8>,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_account = next_account_info(account_info_iter)?;
    let input_token_account = next_account_info(account_info_iter)?;
    let output_token_account = next_account_info(account_info_iter)?;
    let jupiter_program = next_account_info(account_info_iter)?;
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

    if *jupiter_program.key != defi_protocols::JUPITER_AGGREGATOR {
        return Err(ProgramError::InvalidAccountData);
    }

    let route_accounts = vec![
        AccountMeta::new(*input_token_account.key, false),
        AccountMeta::new(*output_token_account.key, false),
        AccountMeta::new_readonly(*authority.key, true),
        AccountMeta::new_readonly(*token_program.key, false),
    ];

    let mut route_data = vec![1];
    route_data.extend_from_slice(&amount.to_le_bytes());
    route_data.extend_from_slice(&route);

    let route_ix = Instruction {
        program_id: *jupiter_program.key,
        accounts: route_accounts,
        data: route_data,
    };

    invoke(&route_ix, &[
        input_token_account.clone(),
        output_token_account.clone(),
        authority.clone(),
        token_program.clone(),
    ])?;

    vault.total_value_locked = vault.total_value_locked.saturating_sub(amount);

    drop(vault_data);
    vault.serialize(&mut &mut vault_account.data.borrow_mut()[..])?;

    msg!("Jupiter route executed: {} {} -> {} using custom route", amount, input_mint, output_mint);
    Ok(())
}
