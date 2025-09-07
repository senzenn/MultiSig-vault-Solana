use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    clock::Clock,
    program::{invoke_signed, invoke},
    instruction::{AccountMeta, Instruction},
    rent::Rent,
    sysvar::Sysvar,
    system_instruction,
    system_program,
    program_pack::Pack,
};
use borsh::{BorshDeserialize, BorshSerialize};
use spl_token::{instruction as token_instruction, state::Account as TokenAccount};
use spl_associated_token_account::{instruction as ata_instruction, get_associated_token_address};

use crate::instruction::VaultInstruction;
use crate::state::{Vault, MultiSig, MultiSigTransaction, FeeConfig, SupportedToken, TokenBalance};
use crate::events::*;
use crate::VaultError;
use crate::emit_event;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = VaultInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    match instruction {
        VaultInstruction::Initialize { bump } => {
            msg!("Instruction: Initialize Vault");
            process_initialize(program_id, accounts, bump)
        }
        VaultInstruction::Deposite { amount } => {
            msg!("Instruction: Deposit tokens");
            process_deposit(program_id, accounts, amount)
        }
        VaultInstruction::Withdraw { amount } => {
            msg!("Instruction: Withdraw tokens");
            process_withdraw(program_id, accounts, amount)
        }
        VaultInstruction::WithdrawSOL { amount } => {
            msg!("Instruction: Withdraw SOL");
            process_withdraw_sol(program_id, accounts, amount)
        }
        VaultInstruction::Transfer { recipient, amount } => {
            msg!("Instruction: Transfer tokens to {}", recipient);
            process_transfer(program_id, accounts, recipient, amount)
        }
        VaultInstruction::InitializeMultiSig {
            owners,
            threshold,
            nonce,
        } => {
            msg!("Instruction: Initialize Multi-Signature Vault");
            process_initialize_multi_sig(program_id, accounts, owners, threshold, nonce)
        }
        VaultInstruction::CreateMultiSigTransaction {
            program_id: target_program_id,
            accounts: transaction_accounts,
            data,
        } => {
            msg!("Instruction: Create Multi-Sig Transaction");
            process_create_multi_sig_transaction(
                program_id,
                accounts,
                target_program_id,
                transaction_accounts,
                data,
            )
        }
        VaultInstruction::ApproveMultiSigTransaction { transaction_id } => {
            msg!("Instruction: Approve Multi-Sig Transaction");
            process_approve_multi_sig_transaction(program_id, accounts, transaction_id)
        }
        VaultInstruction::ExecuteMultiSigTransaction { transaction_id } => {
            msg!("Instruction: Execute Multi-Sig Transaction");
            process_execute_multi_sig_transaction(program_id, accounts, transaction_id)
        }
        VaultInstruction::SetMultiSigOwners { owners } => {
            msg!("Instruction: Set Multi-Sig Owners");
            process_set_multi_sig_owners(program_id, accounts, owners)
        }
        VaultInstruction::ChangeMultiSigThreshold { threshold } => {
            msg!("Instruction: Change Multi-Sig Threshold");
            process_change_multi_sig_threshold(program_id, accounts, threshold)
        }
        VaultInstruction::CreateProposal { instruction_data } => {
            msg!("Instruction: Create Proposal");
            process_create_proposal(program_id, accounts, instruction_data.clone())
        }
        VaultInstruction::ApproveProposal { proposal_id } => {
            msg!("Instruction: Approve Proposal");
            process_approve_proposal(program_id, accounts, proposal_id)
        }
        VaultInstruction::ExecuteProposal { proposal_id } => {
            msg!("Instruction: Execute Proposal");
            process_execute_proposal(program_id, accounts, proposal_id)
        }
        VaultInstruction::RejectProposal { proposal_id } => {
            msg!("Instruction: Reject Proposal");
            process_reject_proposal(program_id, accounts, proposal_id)
        }
        VaultInstruction::PauseVault => {
            msg!("Instruction: Pause Vault");
            process_pause_vault(program_id, accounts)
        }
        VaultInstruction::UnpauseVault => {
            msg!("Instruction: Unpause Vault");
            process_unpause_vault(program_id, accounts)
        }
        VaultInstruction::EmergencyWithdraw { token_mint, amount } => {
            msg!("Instruction: Emergency Withdraw");
            process_emergency_withdraw(program_id, accounts, token_mint, amount)
        }
        VaultInstruction::AddSupportedToken { mint, bump } => {
            msg!("Instruction: Add Supported Token");
            process_add_supported_token(program_id, accounts, mint, bump)
        }
        VaultInstruction::DepositMultiToken { mint, amount } => {
            msg!("Instruction: Deposit Multi Token");
            process_deposit_multi_token(program_id, accounts, mint, amount)
        }
        VaultInstruction::CreateTimeLock {
            beneficiary,
            amount,
            duration,
            cliff_duration,
            is_linear,
        } => {
            msg!("Instruction: Create Time Lock");
            process_create_time_lock(
                program_id,
                accounts,
                beneficiary,
                amount,
                duration,
                cliff_duration,
                is_linear,
            )
        }
        VaultInstruction::ClaimTimeLock { time_lock_index } => {
            msg!("Instruction: Claim Time Lock");
            process_claim_time_lock(program_id, accounts, time_lock_index)
        }
        VaultInstruction::CancelTimeLock { time_lock_index } => {
            msg!("Instruction: Cancel Time Lock");
            process_cancel_time_lock(program_id, accounts, time_lock_index)
        }
        VaultInstruction::SetYieldStrategy {
            token_mint,
            strategy_program,
        } => {
            msg!("Instruction: Set Yield Strategy");
            process_set_yield_strategy(program_id, accounts, token_mint, strategy_program)
        }
        VaultInstruction::HarvestYield { token_mint } => {
            msg!("Instruction: Harvest Yield");
            process_harvest_yield(program_id, accounts, token_mint)
        }
        VaultInstruction::CompoundYield { token_mint } => {
            msg!("Instruction: Compound Yield");
            process_compound_yield(program_id, accounts, token_mint)
        }
        VaultInstruction::JupiterSwap {
            input_mint,
            output_mint,
            amount,
        } => {
            msg!("Instruction: Jupiter Swap");
            process_jupiter_swap(program_id, accounts, input_mint, output_mint, amount)
        }
        VaultInstruction::JupiterRoute {
            input_mint,
            output_mint,
            amount,
            route,
        } => {
            msg!("Instruction: Jupiter Route");
            process_jupiter_route(program_id, accounts, input_mint, output_mint, amount, route)
        }
        VaultInstruction::CollectFees => {
            msg!("Instruction: Collect Fees");
            process_collect_fees(program_id, accounts)
        }
        VaultInstruction::TransferAuthority { new_authority } => {
            msg!("Instruction: Transfer Authority");
            process_transfer_authority(program_id, accounts, new_authority)
        }
        VaultInstruction::UpdateEmergencyAdmin { new_admin } => {
            msg!("Instruction: Update Emergency Admin");
            process_update_emergency_admin(program_id, accounts, new_admin)
        }
        VaultInstruction::InitializeGovernance {
            voting_token_mint,
            quorum_threshold,
            proposal_threshold,
            voting_period,
            time_lock_delay,
            execution_threshold,
        } => {
            msg!("Instruction: Initialize Governance");
            process_initialize_governance(
                program_id,
                accounts,
                voting_token_mint,
                quorum_threshold,
                proposal_threshold,
                voting_period,
                time_lock_delay,
                execution_threshold,
            )
        }
        VaultInstruction::CreateGovernanceProposal {
            title,
            description,
            instructions,
        } => {
            msg!("Instruction: Create Governance Proposal");
            process_create_governance_proposal(
                program_id,
                accounts,
                title,
                description,
                instructions,
            )
        }
        VaultInstruction::CastVote {
            proposal_id,
            vote_type,
        } => {
            msg!("Instruction: Cast Vote");
            process_cast_vote(program_id, accounts, proposal_id, vote_type)
        }
        VaultInstruction::QueueProposal { proposal_id } => {
            msg!("Instruction: Queue Proposal");
            process_queue_proposal(program_id, accounts, proposal_id)
        }
        VaultInstruction::ExecuteGovernanceProposal { proposal_id } => {
            msg!("Instruction: Execute Governance Proposal");
            process_execute_governance_proposal(program_id, accounts, proposal_id)
        }
        VaultInstruction::UpdateGovernanceConfig {
            quorum_threshold,
            proposal_threshold,
            voting_period,
            time_lock_delay,
            execution_threshold,
        } => {
            msg!("Instruction: Update Governance Config");
            process_update_governance_config(
                program_id,
                accounts,
                quorum_threshold,
                proposal_threshold,
                voting_period,
                time_lock_delay,
                execution_threshold,
            )
        }
    }
}

// Initialize vault with proper PDA creation and setup
fn process_initialize(program_id: &Pubkey, accounts: &[AccountInfo], bump: u8) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_account = next_account_info(account_info_iter)?;
    let authority = next_account_info(account_info_iter)?;
    let emergency_admin = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;
    let rent_sysvar = next_account_info(account_info_iter)?;
    let clock_sysvar = next_account_info(account_info_iter)?;

    // Validate accounts
    if !authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if vault_account.owner != system_program.key {
        return Err(VaultError::InvalidAccountOwner.into());
    }

    // Verify PDA derivation
    let expected_vault_pda =
        Pubkey::create_program_address(&[b"vault", authority.key.as_ref(), &[bump]], program_id)?;

    if expected_vault_pda != *vault_account.key {
        return Err(VaultError::InvalidAccountData.into());
    }

    // Check if vault is already initialized
    if !vault_account.data_is_empty() {
        return Err(VaultError::InvalidAccountData.into());
    }

    // Get rent exemption amount
    let rent = Rent::from_account_info(rent_sysvar)?;
    let vault_size = std::mem::size_of::<Vault>() + 1024; // Extra space for dynamic data
    let required_lamports = rent.minimum_balance(vault_size);

    // Transfer lamports to make vault account rent-exempt
    if vault_account.lamports() < required_lamports {
        let transfer_ix = system_instruction::transfer(
            authority.key,
            vault_account.key,
            required_lamports - vault_account.lamports(),
        );

        invoke_signed(
            &transfer_ix,
            &[
                authority.clone(),
                vault_account.clone(),
                system_program.clone(),
            ],
            &[],
        )?;
    }

    // Allocate space for the vault account
    let allocate_ix = system_instruction::allocate(vault_account.key, vault_size as u64);
    invoke_signed(
        &allocate_ix,
        &[vault_account.clone(), system_program.clone()],
        &[&[b"vault", authority.key.as_ref(), &[bump]]],
    )?;

    // Assign ownership to the vault program
    let assign_ix = system_instruction::assign(vault_account.key, program_id);
    invoke_signed(
        &assign_ix,
        &[vault_account.clone(), system_program.clone()],
        &[&[b"vault", authority.key.as_ref(), &[bump]]],
    )?;

    // Initialize vault state
    let clock = Clock::from_account_info(clock_sysvar)?;
    let mut vault = Vault::default();
    vault.authority = *authority.key;
    vault.emergency_admin = *emergency_admin.key;
    vault.bump = bump;
    vault.paused = false;
    vault.fee_config = FeeConfig {
        deposit_fee_bps: 0,
        withdrawal_fee_bps: 0,
        fee_recipient: *authority.key,
    };
    vault.total_value_locked = 0;
    vault.total_fees_collected = 0;

    // Serialize vault state
    vault.serialize(&mut &mut vault_account.data.borrow_mut()[..])?;

    // Emit initialization event
    let init_event = VaultInitializedEvent {
        base: create_base_event(
            *vault_account.key,
            *authority.key,
            "vault_initialized",
            &clock,
        ),
        bump,
        emergency_admin: *emergency_admin.key,
    };
    emit_event!(init_event, init_event);

    msg!(
        "Vault initialized successfully with PDA: {}",
        vault_account.key
    );
    msg!("Authority: {}", authority.key);
    msg!("Emergency Admin: {}", emergency_admin.key);

    Ok(())
}

fn process_deposit(program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_account = next_account_info(account_info_iter)?;
    let user_token_account = next_account_info(account_info_iter)?;
    let vault_token_account = next_account_info(account_info_iter)?;
    let user_authority = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;
    let clock_sysvar = next_account_info(account_info_iter)?;

    // Validate accounts
    if !user_authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if vault_account.owner != program_id {
        return Err(VaultError::InvalidAccountOwner.into());
    }

    if *token_program.key != spl_token::ID {
        return Err(VaultError::InvalidAccountData.into());
    }

    // Load vault state
    let vault_data = vault_account.data.borrow();
    let mut vault = Vault::try_from_slice(&vault_data)?;

    // Check if vault is paused
    if vault.paused {
        return Err(VaultError::UnauthorizedAccess.into());
    }

    // Get token mint from user's token account
    let user_token_data = user_token_account.data.borrow();
    let user_token = TokenAccount::unpack(&user_token_data)?;
    let token_mint = user_token.mint;

    // Check if token is supported
    let supported_token = vault
        .supported_tokens
        .iter()
        .find(|t| t.mint == token_mint && t.is_active);
    if supported_token.is_none() {
        return Err(VaultError::InvalidAccountData.into());
    }

    // Verify vault token account belongs to vault
    let expected_vault_token_account = get_associated_token_address(vault_account.key, &token_mint);
    if expected_vault_token_account != *vault_token_account.key {
        return Err(VaultError::InvalidAccountData.into());
    }

    // Calculate fees
    let deposit_fee = if amount > 0 {
        (amount as u128 * vault.fee_config.deposit_fee_bps as u128 / 10000) as u64
    } else {
        0
    };
    let net_deposit_amount = amount - deposit_fee;

    // Perform token transfer
    let transfer_ix = token_instruction::transfer(
        token_program.key,
        user_token_account.key,
        vault_token_account.key,
        user_authority.key,
        &[],
        net_deposit_amount,
    )?;

    invoke(
        &transfer_ix,
        &[
            user_token_account.clone(),
            vault_token_account.clone(),
            user_authority.clone(),
            token_program.clone(),
        ],
    )?;

    // Update vault state
    let clock = Clock::from_account_info(clock_sysvar)?;

    // Update supported token totals
    if let Some(supported_token) = vault
        .supported_tokens
        .iter_mut()
        .find(|t| t.mint == token_mint)
    {
        supported_token.total_deposited += net_deposit_amount;
    }

    // Update token balance
    let balance_index = vault
        .token_balances
        .iter()
        .position(|b| b.mint == token_mint);
    if let Some(index) = balance_index {
        vault.token_balances[index].balance += net_deposit_amount;
        vault.token_balances[index].last_updated = clock.unix_timestamp;
    } else {
        vault.token_balances.push(TokenBalance {
            mint: token_mint,
            balance: net_deposit_amount,
            last_updated: clock.unix_timestamp,
        });
    }

    // Update total value locked and fees
    vault.total_value_locked += net_deposit_amount;
    vault.total_fees_collected += deposit_fee;

    // Serialize updated vault state
    drop(vault_data);
    vault.serialize(&mut &mut vault_account.data.borrow_mut()[..])?;

    // Emit deposit event
    let deposit_event = TokenDepositedEvent {
        base: create_base_event(
            *vault_account.key,
            *user_authority.key,
            "token_deposited",
            &clock,
        ),
        token_mint,
        amount: net_deposit_amount,
        fee_amount: deposit_fee,
        depositor: *user_authority.key,
    };
    emit_event!(deposit_event, deposit_event);

    msg!(
        "Successfully deposited {} tokens (fee: {}) to vault",
        net_deposit_amount,
        deposit_fee
    );
    msg!("Token mint: {}", token_mint);
    msg!("Depositor: {}", user_authority.key);

    Ok(())
}

fn process_withdraw(program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_account = next_account_info(account_info_iter)?;
    let vault_token_account = next_account_info(account_info_iter)?;
    let user_token_account = next_account_info(account_info_iter)?;
    let user_authority = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;
    let clock_sysvar = next_account_info(account_info_iter)?;

    // Validate accounts
    if !user_authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if vault_account.owner != program_id {
        return Err(VaultError::InvalidAccountOwner.into());
    }

    if *token_program.key != spl_token::ID {
        return Err(VaultError::InvalidAccountData.into());
    }

    // Load vault state
    let vault_data = vault_account.data.borrow();
    let mut vault = Vault::try_from_slice(&vault_data)?;

    // Check if vault is paused
    if vault.paused {
        return Err(VaultError::UnauthorizedAccess.into());
    }

    // Get token mint from vault's token account
    let vault_token_data = vault_token_account.data.borrow();
    let vault_token = TokenAccount::unpack(&vault_token_data)?;
    let token_mint = vault_token.mint;

    // Check if token is supported
    let supported_token = vault
        .supported_tokens
        .iter()
        .find(|t| t.mint == token_mint && t.is_active);
    if supported_token.is_none() {
        return Err(VaultError::InvalidAccountData.into());
    }

    // Verify user token account belongs to user
    let expected_user_token_account = get_associated_token_address(user_authority.key, &token_mint);
    if expected_user_token_account != *user_token_account.key {
        return Err(VaultError::InvalidAccountData.into());
    }

    // Check vault balance
    let vault_balance = vault
        .token_balances
        .iter()
        .find(|b| b.mint == token_mint)
        .map(|b| b.balance)
        .unwrap_or(0);

    if vault_balance < amount {
        return Err(VaultError::InvalidAmount.into());
    }

    // Calculate fees
    let withdrawal_fee = if amount > 0 {
        (amount as u128 * vault.fee_config.withdrawal_fee_bps as u128 / 10000) as u64
    } else {
        0
    };
    let net_withdrawal_amount = amount - withdrawal_fee;

    // Perform token transfer from vault to user
    let transfer_ix = token_instruction::transfer(
        token_program.key,
        vault_token_account.key,
        user_token_account.key,
        vault_account.key, // Vault is the authority for its token account
        &[],
        net_withdrawal_amount,
    )?;

    // Use invoke_signed since vault is a PDA
    let vault_seeds = &[b"vault", vault.authority.as_ref(), &[vault.bump]];
    invoke_signed(
        &transfer_ix,
        &[
            vault_token_account.clone(),
            user_token_account.clone(),
            vault_account.clone(),
            token_program.clone(),
        ],
        &[vault_seeds],
    )?;

    // Update vault state
    let clock = Clock::from_account_info(clock_sysvar)?;

    // Update supported token totals
    if let Some(supported_token) = vault
        .supported_tokens
        .iter_mut()
        .find(|t| t.mint == token_mint)
    {
        supported_token.total_withdrawn += net_withdrawal_amount;
    }

    // Update token balance
    if let Some(balance) = vault
        .token_balances
        .iter_mut()
        .find(|b| b.mint == token_mint)
    {
        balance.balance -= net_withdrawal_amount;
        balance.last_updated = clock.unix_timestamp;
    }

    // Update total value locked and fees
    vault.total_value_locked -= net_withdrawal_amount;
    vault.total_fees_collected += withdrawal_fee;

    // Serialize updated vault state
    drop(vault_data);
    vault.serialize(&mut &mut vault_account.data.borrow_mut()[..])?;

    // Emit withdrawal event
    let withdrawal_event = TokenWithdrawnEvent {
        base: create_base_event(
            *vault_account.key,
            *user_authority.key,
            "token_withdrawn",
            &clock,
        ),
        token_mint,
        amount: net_withdrawal_amount,
        fee_amount: withdrawal_fee,
        recipient: *user_authority.key,
    };
    emit_event!(withdrawal_event, withdrawal_event);

    msg!(
        "Successfully withdrew {} tokens (fee: {}) from vault",
        net_withdrawal_amount,
        withdrawal_fee
    );
    msg!("Token mint: {}", token_mint);
    msg!("Recipient: {}", user_authority.key);

    Ok(())
}

fn process_withdraw_sol(program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_account = next_account_info(account_info_iter)?;
    let recipient = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;
    let clock_sysvar = next_account_info(account_info_iter)?;

    // Validate accounts
    if vault_account.owner != program_id {
        return Err(VaultError::InvalidAccountOwner.into());
    }

    if *system_program.key != system_program::ID {
        return Err(VaultError::InvalidAccountData.into());
    }

    // Load vault state
    let vault_data = vault_account.data.borrow();
    let mut vault = Vault::try_from_slice(&vault_data)?;

    // Check if vault is paused
    if vault.paused {
        return Err(VaultError::UnauthorizedAccess.into());
    }

    // Check vault SOL balance
    let vault_balance = vault_account.lamports();
    if vault_balance < amount {
        return Err(VaultError::InvalidAmount.into());
    }

    // Calculate fees
    let withdrawal_fee = if amount > 0 {
        (amount as u128 * vault.fee_config.withdrawal_fee_bps as u128 / 10000) as u64
    } else {
        0
    };
    let net_withdrawal_amount = amount - withdrawal_fee;

    // Perform SOL transfer from vault to recipient
    let transfer_ix = system_instruction::transfer(
        vault_account.key,
        recipient.key,
        net_withdrawal_amount,
    );

    // Use invoke_signed since vault is a PDA
    let vault_seeds = &[b"vault", vault.authority.as_ref(), &[vault.bump]];
    invoke_signed(
        &transfer_ix,
        &[
            vault_account.clone(),
            recipient.clone(),
            system_program.clone(),
        ],
        &[vault_seeds],
    )?;

    // Update vault state
    let clock = Clock::from_account_info(clock_sysvar)?;

    // Update total value locked and fees
    vault.total_value_locked -= net_withdrawal_amount;
    vault.total_fees_collected += withdrawal_fee;

    // Serialize updated vault state
    drop(vault_data);
    vault.serialize(&mut &mut vault_account.data.borrow_mut()[..])?;

    // Emit withdrawal event
    let withdrawal_event = TokenWithdrawnEvent {
        base: create_base_event(
            *vault_account.key,
            *recipient.key,
            "sol_withdrawn",
            &clock,
        ),
        token_mint: spl_token::native_mint::id(), // Use native SOL mint
        amount: net_withdrawal_amount,
        fee_amount: withdrawal_fee,
        recipient: *recipient.key,
    };
    emit_event!(withdrawal_event, withdrawal_event);

    msg!(
        "Successfully withdrew {} SOL (fee: {}) from vault",
        net_withdrawal_amount,
        withdrawal_fee
    );
    msg!("Recipient: {}", recipient.key);

    Ok(())
}

fn process_transfer(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    recipient: Pubkey,
    amount: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_account = next_account_info(account_info_iter)?;
    let recipient_account = next_account_info(account_info_iter)?;
    let authority = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;
    let clock_sysvar = next_account_info(account_info_iter)?;

    // Validate accounts
    if !authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if vault_account.owner != program_id {
        return Err(VaultError::InvalidAccountOwner.into());
    }

    if *system_program.key != system_program::ID {
        return Err(VaultError::InvalidAccountData.into());
    }

    // Load vault state
    let vault_data = vault_account.data.borrow();
    let mut vault = Vault::try_from_slice(&vault_data)?;

    // Check if vault is paused
    if vault.paused {
        return Err(VaultError::UnauthorizedAccess.into());
    }

    // Verify authority
    if vault.authority != *authority.key {
        return Err(VaultError::InsufficientAuthority.into());
    }

    // Check vault SOL balance
    let vault_balance = vault_account.lamports();
    if vault_balance < amount {
        return Err(VaultError::InvalidAmount.into());
    }

    // Calculate fees
    let transfer_fee = if amount > 0 {
        (amount as u128 * vault.fee_config.withdrawal_fee_bps as u128 / 10000) as u64
    } else {
        0
    };
    let net_transfer_amount = amount - transfer_fee;

    // Perform SOL transfer from vault to recipient
    let transfer_ix = system_instruction::transfer(
        vault_account.key,
        recipient_account.key,
        net_transfer_amount,
    );

    // Use invoke_signed since vault is a PDA
    let vault_seeds = &[b"vault", vault.authority.as_ref(), &[vault.bump]];
    invoke_signed(
        &transfer_ix,
        &[
            vault_account.clone(),
            recipient_account.clone(),
            system_program.clone(),
        ],
        &[vault_seeds],
    )?;

    // Update vault state
    let clock = Clock::from_account_info(clock_sysvar)?;

    // Update total value locked and fees
    vault.total_value_locked -= net_transfer_amount;
    vault.total_fees_collected += transfer_fee;

    // Serialize updated vault state
    drop(vault_data);
    vault.serialize(&mut &mut vault_account.data.borrow_mut()[..])?;

    // Emit transfer event
    let transfer_event = TokenWithdrawnEvent {
        base: create_base_event(
            *vault_account.key,
            *authority.key,
            "sol_transferred",
            &clock,
        ),
        token_mint: spl_token::native_mint::id(), // Use native SOL mint
        amount: net_transfer_amount,
        fee_amount: transfer_fee,
        recipient: *recipient_account.key,
    };
    emit_event!(transfer_event, transfer_event);

    msg!(
        "Successfully transferred {} SOL (fee: {}) from vault to {}",
        net_transfer_amount,
        transfer_fee,
        recipient
    );
    msg!("Authority: {}", authority.key);
    msg!("Recipient: {}", recipient_account.key);

    Ok(())
}

fn process_initialize_multi_sig(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    owners: Vec<Pubkey>,
    threshold: u64,
    nonce: u8,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_account = next_account_info(account_info_iter)?;
    let initializer = next_account_info(account_info_iter)?;
    let clock_sysvar = next_account_info(account_info_iter)?;

    if !initializer.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if vault_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    let vault_data = vault_account.data.borrow();
    let mut vault = Vault::try_from_slice(&vault_data)?;

    if vault.authority != *initializer.key {
        return Err(VaultError::InsufficientAuthority.into());
    }

    // Validate threshold
    if threshold == 0 || threshold > owners.len() as u64 {
        return Err(VaultError::InvalidThreshold.into());
    }

    // Validate owners (no duplicates)
    let mut unique_owners = owners.clone();
    unique_owners.sort();
    unique_owners.dedup();
    if unique_owners.len() != owners.len() {
        return Err(VaultError::InvalidAccountData.into());
    }

    vault.multi_sig = Some(MultiSig {
        owners: owners.clone(),
        threshold,
        nonce,
        bump: 0, // Will be calculated when needed
    });

    drop(vault_data);
    vault.serialize(&mut &mut vault_account.data.borrow_mut()[..])?;

    let clock = Clock::from_account_info(clock_sysvar)?;

    // Emit event
    let multisig_event = MultiSigInitializedEvent {
        base: create_base_event(
            *vault_account.key,
            *initializer.key,
            "multisig_initialized",
            &clock,
        ),
        owners,
        threshold,
        nonce,
    };
    emit_event!(multisig_event, multisig_event);

    msg!(
        "Multi-signature initialized with {} owners and threshold {}",
        unique_owners.len(),
        threshold
    );
    Ok(())
}

fn process_create_proposal(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: Vec<u8>,
) -> ProgramResult {
    msg!("Processing create proposal");
    Ok(())
}

fn process_approve_proposal(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    proposal_id: u64,
) -> ProgramResult {
    msg!("Processing approve proposal: {}", proposal_id);
    Ok(())
}

fn process_execute_proposal(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    proposal_id: u64,
) -> ProgramResult {
    msg!("Processing execute proposal: {}", proposal_id);
    Ok(())
}

fn process_reject_proposal(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    proposal_id: u64,
) -> ProgramResult {
    msg!("Processing reject proposal: {}", proposal_id);
    Ok(())
}

fn process_pause_vault(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    msg!("Processing pause vault");
    Ok(())
}

fn process_unpause_vault(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    msg!("Processing unpause vault");
    Ok(())
}

fn process_emergency_withdraw(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    token_mint: Pubkey,
    amount: u64,
) -> ProgramResult {
    msg!("Processing emergency withdraw");
    Ok(())
}

fn process_add_supported_token(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    mint: Pubkey,
    bump: u8,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_account = next_account_info(account_info_iter)?;
    let vault_token_account = next_account_info(account_info_iter)?;
    let token_mint = next_account_info(account_info_iter)?;
    let authority = next_account_info(account_info_iter)?;
    let associated_token_program = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;
    let rent_sysvar = next_account_info(account_info_iter)?;
    let clock_sysvar = next_account_info(account_info_iter)?;

    // Validate accounts
    if !authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if vault_account.owner != program_id {
        return Err(VaultError::InvalidAccountOwner.into());
    }

    if *token_mint.key != mint {
        return Err(VaultError::InvalidAccountData.into());
    }

    // Load vault state
    let vault_data = vault_account.data.borrow();
    let mut vault = Vault::try_from_slice(&vault_data)?;

    // Check if authority is vault authority
    if vault.authority != *authority.key {
        return Err(VaultError::InsufficientAuthority.into());
    }

    // Check if token is already supported
    if vault.supported_tokens.iter().any(|t| t.mint == mint) {
        return Err(VaultError::InvalidAccountData.into());
    }

    // Verify vault token account derivation
    let expected_vault_token_account = get_associated_token_address(vault_account.key, &mint);
    if expected_vault_token_account != *vault_token_account.key {
        return Err(VaultError::InvalidAccountData.into());
    }

    // Create associated token account for vault if it doesn't exist
    if vault_token_account.data_is_empty() {
        let create_ata_ix = ata_instruction::create_associated_token_account(
            authority.key,
            vault_account.key,
            &mint,
            &spl_token::ID,
        );

        invoke(
            &create_ata_ix,
            &[
                authority.clone(),
                vault_token_account.clone(),
                vault_account.clone(),
                token_mint.clone(),
                system_program.clone(),
                token_program.clone(),
                rent_sysvar.clone(),
                associated_token_program.clone(),
            ],
        )?;
    }

    // Update vault state
    let clock = Clock::from_account_info(clock_sysvar)?;
    let supported_token = SupportedToken {
        mint,
        bump,
        total_deposited: 0,
        total_withdrawn: 0,
        is_active: true,
    };

    vault.supported_tokens.push(supported_token);

    // Serialize updated vault state
    drop(vault_data);
    vault.serialize(&mut &mut vault_account.data.borrow_mut()[..])?;

    // Emit token added event
    let token_added_event = TokenAddedEvent {
        base: create_base_event(*vault_account.key, *authority.key, "token_added", &clock),
        token_mint: mint,
        vault_token_account: *vault_token_account.key,
    };
    emit_event!(token_added_event, token_added_event);

    msg!("Successfully added token {} to vault", mint);
    msg!("Vault token account: {}", vault_token_account.key);

    Ok(())
}

fn process_deposit_multi_token(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    mint: Pubkey,
    amount: u64,
) -> ProgramResult {
    msg!("Processing deposit multi token");
    Ok(())
}

fn process_create_time_lock(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    beneficiary: Pubkey,
    amount: u64,
    duration: i64,
    cliff_duration: Option<i64>,
    is_linear: bool,
) -> ProgramResult {
    msg!("Processing create time lock");
    Ok(())
}

fn process_claim_time_lock(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    time_lock_index: usize,
) -> ProgramResult {
    msg!("Processing claim time lock");
    Ok(())
}

fn process_cancel_time_lock(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    time_lock_index: usize,
) -> ProgramResult {
    msg!("Processing cancel time lock");
    Ok(())
}

fn process_set_yield_strategy(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    token_mint: Pubkey,
    strategy_program: Pubkey,
) -> ProgramResult {
    msg!("Processing set yield strategy");
    Ok(())
}

fn process_harvest_yield(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    token_mint: Pubkey,
) -> ProgramResult {
    msg!("Processing harvest yield");
    Ok(())
}

fn process_compound_yield(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    token_mint: Pubkey,
) -> ProgramResult {
    msg!("Processing compound yield");
    Ok(())
}

fn process_jupiter_swap(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input_mint: Pubkey,
    output_mint: Pubkey,
    amount: u64,
) -> ProgramResult {
    msg!("Processing jupiter swap");
    Ok(())
}

fn process_jupiter_route(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input_mint: Pubkey,
    output_mint: Pubkey,
    amount: u64,
    route: Vec<u8>,
) -> ProgramResult {
    msg!("Processing jupiter route");
    Ok(())
}

fn process_collect_fees(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    msg!("Processing collect fees");
    Ok(())
}

fn process_transfer_authority(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    new_authority: Pubkey,
) -> ProgramResult {
    msg!("Processing transfer authority");
    Ok(())
}

fn process_update_emergency_admin(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    new_admin: Pubkey,
) -> ProgramResult {
    msg!("Processing update emergency admin");
    Ok(())
}

fn process_initialize_governance(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    voting_token_mint: Pubkey,
    quorum_threshold: u16,
    proposal_threshold: u64,
    voting_period: i64,
    time_lock_delay: i64,
    execution_threshold: u16,
) -> ProgramResult {
    msg!("Processing initialize governance");
    Ok(())
}

fn process_create_governance_proposal(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    title: String,
    description: String,
    instructions: Vec<Vec<u8>>,
) -> ProgramResult {
    msg!("Processing create governance proposal");
    Ok(())
}

fn process_cast_vote(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    proposal_id: u64,
    vote_type: crate::state::VoteType,
) -> ProgramResult {
    msg!("Processing cast vote");
    Ok(())
}

fn process_queue_proposal(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    proposal_id: u64,
) -> ProgramResult {
    msg!("Processing queue proposal");
    Ok(())
}

fn process_execute_governance_proposal(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    proposal_id: u64,
) -> ProgramResult {
    msg!("Processing execute governance proposal");
    Ok(())
}

fn process_update_governance_config(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    quorum_threshold: u16,
    proposal_threshold: u64,
    voting_period: i64,
    time_lock_delay: i64,
    execution_threshold: u16,
) -> ProgramResult {
    msg!("Processing update governance config");
    Ok(())
}

// Multi-sig processor functions
fn process_create_multi_sig_transaction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    target_program_id: Pubkey,
    transaction_accounts: Vec<crate::state::TransactionAccount>,
    data: Vec<u8>,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_account = next_account_info(account_info_iter)?;
    let proposer = next_account_info(account_info_iter)?;
    let clock_sysvar = next_account_info(account_info_iter)?;

    if !proposer.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let vault_data = vault_account.data.borrow();
    let mut vault = Vault::try_from_slice(&vault_data)?;

    // Check if multisig is initialized
    let multi_sig = vault
        .multi_sig
        .as_ref()
        .ok_or(VaultError::MultisigNotInitialized)?;

    // Check if proposer is authorized
    if !multi_sig.owners.contains(proposer.key) {
        return Err(VaultError::InvalidOwner.into());
    }

    let clock = Clock::from_account_info(clock_sysvar)?;
    let transaction_id = vault.multi_sig_transactions.len() as u64;

    // Find owner index
    let owner_index = multi_sig
        .owners
        .iter()
        .position(|owner| owner == proposer.key)
        .ok_or(VaultError::InvalidOwner)?;

    let mut signers = vec![false; multi_sig.owners.len()];
    signers[owner_index] = true;

    // Validate transaction data
    if transaction_accounts.is_empty() {
        return Err(VaultError::InvalidTransactionData.into());
    }

    let transaction = MultiSigTransaction {
        multisig: *vault_account.key,
        program_id: target_program_id,
        accounts: transaction_accounts.clone(),
        data: data.clone(),
        signers,
        did_execute: false,
        proposer: *proposer.key,
        created_at: clock.unix_timestamp,
    };

    vault.multi_sig_transactions.push(transaction);

    drop(vault_data);
    vault.serialize(&mut &mut vault_account.data.borrow_mut()[..])?;

    // Emit event
    let transaction_event = MultiSigTransactionCreatedEvent {
        base: create_base_event(
            *vault_account.key,
            *proposer.key,
            "multisig_transaction_created",
            &clock,
        ),
        transaction_id,
        proposer: *proposer.key,
        target_program: target_program_id,
        instruction_count: transaction_accounts.len(),
    };
    emit_event!(transaction_event, transaction_event);

    msg!(
        "Multi-sig transaction {} created by {}",
        transaction_id,
        proposer.key
    );
    Ok(())
}

fn process_approve_multi_sig_transaction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    transaction_id: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_account = next_account_info(account_info_iter)?;
    let approver = next_account_info(account_info_iter)?;
    let clock_sysvar = next_account_info(account_info_iter)?;

    if !approver.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let vault_data = vault_account.data.borrow();
    let mut vault = Vault::try_from_slice(&vault_data)?;

    // Check if multisig is initialized
    let multi_sig = vault
        .multi_sig
        .as_ref()
        .ok_or(VaultError::MultisigNotInitialized)?;

    // Check if transaction exists
    if transaction_id as usize >= vault.multi_sig_transactions.len() {
        return Err(VaultError::TransactionNotFound.into());
    }

    let transaction = &mut vault.multi_sig_transactions[transaction_id as usize];

    // Check if transaction is already executed
    if transaction.did_execute {
        return Err(VaultError::TransactionAlreadyExecuted.into());
    }

    // Find approver in owners list
    let owner_index = multi_sig
        .owners
        .iter()
        .position(|owner| owner == approver.key)
        .ok_or(VaultError::InvalidOwner)?;

    // Check if already approved
    if transaction.signers[owner_index] {
        return Err(VaultError::TransactionAlreadySigned.into());
    }

    // Approve the transaction
    transaction.signers[owner_index] = true;

    let clock = Clock::from_account_info(clock_sysvar)?;
    let current_approvals = transaction.signers.iter().filter(|&&signed| signed).count();

    drop(vault_data);
    vault.serialize(&mut &mut vault_account.data.borrow_mut()[..])?;

    // Emit event
    let approval_event = MultiSigTransactionApprovedEvent {
        base: create_base_event(
            *vault_account.key,
            *approver.key,
            "multisig_transaction_approved",
            &clock,
        ),
        transaction_id,
        approver: *approver.key,
        current_approvals,
        required_approvals: multi_sig.threshold as usize,
    };
    emit_event!(approval_event, approval_event);

    msg!(
        "Multi-sig transaction {} approved by {} ({} of {} approvals)",
        transaction_id,
        approver.key,
        current_approvals,
        multi_sig.threshold
    );
    Ok(())
}

fn process_execute_multi_sig_transaction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    transaction_id: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_account = next_account_info(account_info_iter)?;
    let multisig_signer = next_account_info(account_info_iter)?;
    let executor = next_account_info(account_info_iter)?;
    let clock_sysvar = next_account_info(account_info_iter)?;

    if !executor.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let vault_data = vault_account.data.borrow();
    let mut vault = Vault::try_from_slice(&vault_data)?;

    // Check if multisig is initialized
    let multi_sig = vault
        .multi_sig
        .as_ref()
        .ok_or(VaultError::MultisigNotInitialized)?;

    // Check if transaction exists
    if transaction_id as usize >= vault.multi_sig_transactions.len() {
        return Err(VaultError::TransactionNotFound.into());
    }

    let transaction = &vault.multi_sig_transactions[transaction_id as usize];

    // Check if transaction is already executed
    if transaction.did_execute {
        return Err(VaultError::TransactionAlreadyExecuted.into());
    }

    // Check if we have enough approvals
    let current_approvals = transaction.signers.iter().filter(|&&signed| signed).count();
    if current_approvals < multi_sig.threshold as usize {
        return Err(VaultError::NotEnoughSigners.into());
    }

    // Create the instruction to execute
    let mut ix = Instruction {
        program_id: transaction.program_id,
        accounts: transaction
            .accounts
            .iter()
            .map(|acc| {
                if &acc.pubkey == multisig_signer.key {
                    AccountMeta::new_readonly(acc.pubkey, true)
                } else if acc.is_writable {
                    AccountMeta::new(acc.pubkey, acc.is_signer)
                } else {
                    AccountMeta::new_readonly(acc.pubkey, acc.is_signer)
                }
            })
            .collect(),
        data: transaction.data.clone(),
    };

    // Get remaining accounts for the CPI
    let remaining_accounts = account_info_iter.as_slice();

    // Derive the multisig signer PDA
    let (expected_signer, bump) = Pubkey::find_program_address(
        &[vault_account.key.as_ref(), &[multi_sig.nonce]],
        program_id,
    );

    if expected_signer != *multisig_signer.key {
        return Err(VaultError::InvalidAccountData.into());
    }

    let seeds = &[vault_account.key.as_ref(), &[bump]];
    let signer_seeds = &[&seeds[..]];

    // Execute the transaction
    invoke_signed(&ix, remaining_accounts, signer_seeds)?;

    // Mark transaction as executed
    drop(vault_data);
    let mut vault = Vault::try_from_slice(&vault_account.data.borrow())?;
    vault.multi_sig_transactions[transaction_id as usize].did_execute = true;
    vault.serialize(&mut &mut vault_account.data.borrow_mut()[..])?;

    let clock = Clock::from_account_info(clock_sysvar)?;

    // Emit event
    let execution_event = MultiSigTransactionExecutedEvent {
        base: create_base_event(
            *vault_account.key,
            *executor.key,
            "multisig_transaction_executed",
            &clock,
        ),
        transaction_id,
        executor: *executor.key,
        target_program: transaction.program_id,
    };
    emit_event!(execution_event, execution_event);

    msg!(
        "Multi-sig transaction {} executed by {}",
        transaction_id,
        executor.key
    );
    Ok(())
}

fn process_set_multi_sig_owners(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    owners: Vec<Pubkey>,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_account = next_account_info(account_info_iter)?;
    let multisig_signer = next_account_info(account_info_iter)?;
    let authority = next_account_info(account_info_iter)?;
    let clock_sysvar = next_account_info(account_info_iter)?;

    if !authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let vault_data = vault_account.data.borrow();
    let mut vault = Vault::try_from_slice(&vault_data)?;

    let multi_sig = vault
        .multi_sig
        .as_mut()
        .ok_or(VaultError::MultisigNotInitialized)?;

    // Validate new owners (no duplicates)
    let mut unique_owners = owners.clone();
    unique_owners.sort();
    unique_owners.dedup();
    if unique_owners.len() != owners.len() {
        return Err(VaultError::InvalidAccountData.into());
    }

    // Store old owners for event
    let old_owners = multi_sig.owners.clone();

    // Adjust threshold if necessary
    if (owners.len() as u64) < multi_sig.threshold {
        multi_sig.threshold = owners.len() as u64;
    }

    multi_sig.owners = owners.clone();

    drop(vault_data);
    vault.serialize(&mut &mut vault_account.data.borrow_mut()[..])?;

    let clock = Clock::from_account_info(clock_sysvar)?;

    // Emit event
    let owners_event = MultiSigOwnersUpdatedEvent {
        base: create_base_event(
            *vault_account.key,
            *authority.key,
            "multisig_owners_updated",
            &clock,
        ),
        old_owners: old_owners.clone(),
        new_owners: owners.clone(),
    };
    emit_event!(owners_event, owners_event);

    msg!(
        "Multi-sig owners updated from {:?} to {:?}",
        old_owners,
        owners
    );
    Ok(())
}

fn process_change_multi_sig_threshold(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    threshold: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_account = next_account_info(account_info_iter)?;
    let multisig_signer = next_account_info(account_info_iter)?;
    let authority = next_account_info(account_info_iter)?;
    let clock_sysvar = next_account_info(account_info_iter)?;

    if !authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let vault_data = vault_account.data.borrow();
    let mut vault = Vault::try_from_slice(&vault_data)?;

    let multi_sig = vault
        .multi_sig
        .as_mut()
        .ok_or(VaultError::MultisigNotInitialized)?;

    // Validate threshold
    if threshold == 0 || threshold > multi_sig.owners.len() as u64 {
        return Err(VaultError::InvalidThreshold.into());
    }

    // Store old threshold for event
    let old_threshold = multi_sig.threshold;

    multi_sig.threshold = threshold;

    drop(vault_data);
    vault.serialize(&mut &mut vault_account.data.borrow_mut()[..])?;

    let clock = Clock::from_account_info(clock_sysvar)?;

    // Emit event
    let threshold_event = MultiSigThresholdUpdatedEvent {
        base: create_base_event(
            *vault_account.key,
            *authority.key,
            "multisig_threshold_updated",
            &clock,
        ),
        old_threshold,
        new_threshold: threshold,
    };
    emit_event!(threshold_event, threshold_event);

    msg!(
        "Multi-sig threshold changed from {} to {}",
        old_threshold,
        threshold
    );
    Ok(())
}

// Validation helper functions
fn validate_vault_authority(vault: &Vault, authority: &Pubkey) -> Result<(), VaultError> {
    if vault.authority != *authority {
        return Err(VaultError::InsufficientAuthority);
    }
    Ok(())
}

fn validate_emergency_admin(vault: &Vault, admin: &Pubkey) -> Result<(), VaultError> {
    if vault.emergency_admin != *admin {
        return Err(VaultError::InsufficientAuthority);
    }
    Ok(())
}

fn validate_token_supported(vault: &Vault, token_mint: &Pubkey) -> Result<(), VaultError> {
    let supported = vault
        .supported_tokens
        .iter()
        .any(|t| t.mint == *token_mint && t.is_active);

    if !supported {
        return Err(VaultError::InvalidAccountData);
    }
    Ok(())
}

fn validate_vault_balance(
    vault: &Vault,
    token_mint: &Pubkey,
    required_amount: u64,
) -> Result<(), VaultError> {
    let balance = vault
        .token_balances
        .iter()
        .find(|b| b.mint == *token_mint)
        .map(|b| b.balance)
        .unwrap_or(0);

    if balance < required_amount {
        return Err(VaultError::InvalidAmount);
    }
    Ok(())
}

fn calculate_fee(amount: u64, fee_bps: u16) -> u64 {
    if amount == 0 {
        return 0;
    }
    (amount as u128 * fee_bps as u128 / 10000) as u64
}

fn update_token_balance(vault: &mut Vault, token_mint: &Pubkey, amount_change: i64, clock: &Clock) {
    let balance_index = vault
        .token_balances
        .iter()
        .position(|b| b.mint == *token_mint);

    if let Some(index) = balance_index {
        let balance = &mut vault.token_balances[index];
        balance.balance = (balance.balance as i64 + amount_change) as u64;
        balance.last_updated = clock.unix_timestamp;
    } else if amount_change > 0 {
        vault.token_balances.push(TokenBalance {
            mint: *token_mint,
            balance: amount_change as u64,
            last_updated: clock.unix_timestamp,
        });
    }
}

fn update_supported_token_totals(
    vault: &mut Vault,
    token_mint: &Pubkey,
    deposited: u64,
    withdrawn: u64,
) {
    if let Some(supported_token) = vault
        .supported_tokens
        .iter_mut()
        .find(|t| t.mint == *token_mint)
    {
        supported_token.total_deposited += deposited;
        supported_token.total_withdrawn += withdrawn;
    }
}

