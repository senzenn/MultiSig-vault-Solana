use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
    program::invoke,
    clock::Clock,
};
use spl_token::instruction as token_instruction;
use spl_associated_token_account::instruction as ata_instruction;

use crate::state::Vault;
use crate::events::{DepositEvent, WithdrawEvent, create_base_event};
use crate::{emit_event};

pub fn process_initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    bump: u8,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_account = next_account_info(account_info_iter)?;
    let mint_account = next_account_info(account_info_iter)?;
    let authority = next_account_info(account_info_iter)?;
    let vault_token_account = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;
    let associated_token_program = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;
    let rent_sysvar = next_account_info(account_info_iter)?;

    if !authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let vault_size = std::mem::size_of::<Vault>() as u64;
    let rent = &Rent::from_account_info(rent_sysvar)?;
    let required_lamports = rent.minimum_balance(vault_size as usize);

    invoke(
        &system_instruction::create_account(
            authority.key,
            vault_account.key,
            required_lamports,
            vault_size,
            program_id,
        ),
        &[
            authority.clone(),
            vault_account.clone(),
            system_program.clone(),
        ],
    )?;

    invoke(
        &ata_instruction::create_associated_token_account(
            authority.key,
            vault_account.key,
            mint_account.key,
            token_program.key,
        ),
        &[
            authority.clone(),
            vault_account.clone(),
            mint_account.clone(),
            vault_token_account.clone(),
            token_program.clone(),
            associated_token_program.clone(),
            system_program.clone(),
            rent_sysvar.clone(),
        ],
    )?;

    let vault = Vault {
        authority: *authority.key,
        bump,
        multi_sig: None,
        paused: false,
        emergency_admin: *authority.key,
        supported_tokens: vec![crate::state::TokenBalance {
            mint: *mint_account.key,
            balance: 0,
            yield_strategy: None,
        }],
        time_locks: vec![],
        proposals: vec![],
        next_proposal_id: 0,
        fee_config: crate::state::FeeConfig {
            deposit_fee_bps: 0,
            withdrawal_fee_bps: 0,
            fee_recipient: *authority.key,
        },
        total_value_locked: 0,
        total_fees_collected: 0,
        legacy_mint: Some(*mint_account.key),
        legacy_total_deposited: 0,
        governance_config: None,
        governance_proposals: vec![],
        next_governance_proposal_id: 0,
        vote_records: vec![],
        voter_registry: vec![],
    };

    vault.serialize(&mut &mut vault_account.data.borrow_mut()[..])?;

    msg!("Enhanced vault initialized successfully with legacy support");
    Ok(())
}

pub fn process_deposit(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_account = next_account_info(account_info_iter)?;
    let user_token_account = next_account_info(account_info_iter)?;
    let vault_token_account = next_account_info(account_info_iter)?;
    let authority = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;
    let clock_sysvar = next_account_info(account_info_iter)?;

    if !authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if vault_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
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

    if vault.paused {
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

    if let Some(legacy_mint) = vault.legacy_mint {
        vault.legacy_total_deposited = vault.legacy_total_deposited.checked_add(amount)
            .ok_or(ProgramError::InvalidArgument)?;
    }

    if let Some(legacy_mint) = vault.legacy_mint {
        if let Some(token_balance) = vault.supported_tokens.iter_mut().find(|tb| tb.mint == legacy_mint) {
            token_balance.balance = token_balance.balance.checked_add(amount)
                .ok_or(ProgramError::InvalidArgument)?;
        }
    }

    vault.total_value_locked = vault.total_value_locked.checked_add(amount)
        .ok_or(ProgramError::InvalidArgument)?;

    drop(vault_data);
    vault.serialize(&mut &mut vault_account.data.borrow_mut()[..])?;

    let clock = Clock::from_account_info(clock_sysvar)?;
    let token_mint = vault.legacy_mint.unwrap_or_default();
    let deposit_event = crate::DepositEvent {
        base: crate::create_base_event(*vault_account.key, *authority.key, "deposit", &clock),
        token_mint,
        amount,
        user: *authority.key,
    };
    crate::emit_event!(deposit_event);

    msg!("Deposited {} tokens successfully", amount);
    Ok(())
}

pub fn process_withdraw(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_account = next_account_info(account_info_iter)?;
    let vault_token_account = next_account_info(account_info_iter)?;
    let user_token_account = next_account_info(account_info_iter)?;
    let authority = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;
    let clock_sysvar = next_account_info(account_info_iter)?;

    if !authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if vault_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
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

    if vault.paused {
        return Err(ProgramError::InvalidAccountData);
    }

    if vault.legacy_total_deposited < amount {
        return Err(ProgramError::InsufficientFunds);
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

    if let Some(legacy_mint) = vault.legacy_mint {
        vault.legacy_total_deposited = vault.legacy_total_deposited.checked_sub(amount)
            .ok_or(ProgramError::InvalidArgument)?;
    }

    if let Some(legacy_mint) = vault.legacy_mint {
        if let Some(token_balance) = vault.supported_tokens.iter_mut().find(|tb| tb.mint == legacy_mint) {
            token_balance.balance = token_balance.balance.checked_sub(amount)
                .ok_or(ProgramError::InvalidArgument)?;
        }
    }

    vault.total_value_locked = vault.total_value_locked.checked_sub(amount)
        .ok_or(ProgramError::InvalidArgument)?;

    drop(vault_data);
    vault.serialize(&mut &mut vault_account.data.borrow_mut()[..])?;

    let clock = Clock::from_account_info(clock_sysvar)?;
    let token_mint = vault.legacy_mint.unwrap_or_default();
    let withdraw_event = crate::WithdrawEvent {
        base: crate::create_base_event(*vault_account.key, *authority.key, "withdraw", &clock),
        token_mint,
        amount,
        user: *authority.key,
    };
    crate::emit_event!(withdraw_event);

    msg!("Withdrew {} tokens successfully", amount);
    Ok(())
}
