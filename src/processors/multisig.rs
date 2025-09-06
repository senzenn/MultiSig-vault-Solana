use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    clock::Clock,
    sysvar::Sysvar,
};

use crate::instruction::VaultInstruction;
use crate::state::{Vault, MultiSigAuthority, Proposal};
use crate::events::{create_base_event, MultiSigInitializedEvent};
use crate::emit_event;

pub fn process_initialize_multi_sig(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    authorities: Vec<Pubkey>,
    threshold: u8,
    _bump: u8,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_account = next_account_info(account_info_iter)?;
    let initializer = next_account_info(account_info_iter)?;
    let clock_sysvar = next_account_info(account_info_iter)?;

    if !initializer.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if vault_account.owner != _program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    let vault_data = vault_account.data.borrow();
    let mut vault = Vault::try_from_slice(&vault_data)?;

    if vault.authority != *initializer.key {
        return Err(ProgramError::InvalidAccountData);
    }

    vault.multi_sig = Some(MultiSigAuthority {
        authorities: authorities.clone(),
        threshold,
        nonce: 0,
    });

    drop(vault_data);
    vault.serialize(&mut &mut vault_account.data.borrow_mut()[..])?;

    let clock = Clock::from_account_info(clock_sysvar)?;
    let multi_sig_event = MultiSigInitializedEvent {
        base: create_base_event(*vault_account.key, *initializer.key, "multi_sig_initialized", &clock),
        authorities,
        threshold,
    };
    emit_event!(multi_sig_event);

    msg!("Multi-signature initialized with {} authorities and threshold {}", vault.multi_sig.as_ref().unwrap().authorities.len(), threshold);
    Ok(())
}

pub fn process_create_proposal(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction: VaultInstruction,
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

    let is_authorized = if let Some(multi_sig) = &vault.multi_sig {
        multi_sig.authorities.contains(proposer.key)
    } else {
        vault.authority == *proposer.key
    };

    if !is_authorized {
        return Err(ProgramError::InvalidAccountData);
    }

    let clock = Clock::from_account_info(clock_sysvar)?;
    let proposal = Proposal {
        id: vault.next_proposal_id,
        proposer: *proposer.key,
        instruction: instruction.clone(),
        approvals: vec![*proposer.key],
        created_at: clock.unix_timestamp,
        executed: false,
    };

    vault.proposals.push(proposal);
    vault.next_proposal_id += 1;

    drop(vault_data);
    vault.serialize(&mut &mut vault_account.data.borrow_mut()[..])?;

    msg!("Proposal {} created", vault.next_proposal_id - 1);
    Ok(())
}

pub fn process_approve_proposal(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    proposal_id: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_account = next_account_info(account_info_iter)?;
    let approver = next_account_info(account_info_iter)?;

    if !approver.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let vault_data = vault_account.data.borrow();
    let mut vault = Vault::try_from_slice(&vault_data)?;

    if let Some(proposal) = vault.proposals.iter_mut().find(|p| p.id == proposal_id) {
        if proposal.approvals.contains(approver.key) {
            return Err(ProgramError::InvalidAccountData);
        }

        if let Some(multi_sig) = &vault.multi_sig {
            if !multi_sig.authorities.contains(approver.key) {
                return Err(ProgramError::InvalidAccountData);
            }
        } else if vault.authority != *approver.key {
            return Err(ProgramError::InvalidAccountData);
        }

        proposal.approvals.push(*approver.key);
        drop(vault_data);
        vault.serialize(&mut &mut vault_account.data.borrow_mut()[..])?;

        msg!("Proposal {} approved by {}", proposal_id, approver.key);
        Ok(())
    } else {
        Err(ProgramError::InvalidAccountData)
    }
}

pub fn process_execute_proposal(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    proposal_id: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_account = next_account_info(account_info_iter)?;
    let _executor = next_account_info(account_info_iter)?;

    let vault_data = vault_account.data.borrow();
    let mut vault = Vault::try_from_slice(&vault_data)?;

    if let Some(proposal_index) = vault.proposals.iter().position(|p| p.id == proposal_id) {
        let proposal = &vault.proposals[proposal_index];

        if proposal.executed {
            return Err(ProgramError::InvalidAccountData);
        }

        let required_approvals = if let Some(multi_sig) = &vault.multi_sig {
            multi_sig.threshold as usize
        } else {
            1
        };

        if proposal.approvals.len() < required_approvals {
            return Err(ProgramError::InvalidAccountData);
        }

        vault.proposals[proposal_index].executed = true;

        drop(vault_data);
        vault.serialize(&mut &mut vault_account.data.borrow_mut()[..])?;

        msg!("Proposal {} executed", proposal_id);
        Ok(())
    } else {
        Err(ProgramError::InvalidAccountData)
    }
}

pub fn process_reject_proposal(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    proposal_id: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_account = next_account_info(account_info_iter)?;
    let rejector = next_account_info(account_info_iter)?;

    if !rejector.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let vault_data = vault_account.data.borrow();
    let mut vault = Vault::try_from_slice(&vault_data)?;

    if let Some(proposal_index) = vault.proposals.iter().position(|p| p.id == proposal_id) {
        let proposal = &vault.proposals[proposal_index];

        if proposal.executed {
            return Err(ProgramError::InvalidAccountData);
        }

        vault.proposals.remove(proposal_index);

        drop(vault_data);
        vault.serialize(&mut &mut vault_account.data.borrow_mut()[..])?;

        msg!("Proposal {} rejected", proposal_id);
        Ok(())
    } else {
        Err(ProgramError::InvalidAccountData)
    }
}
