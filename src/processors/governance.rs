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

use crate::state::{Vault, VoteType};

pub fn process_initialize_governance(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    voting_token_mint: Pubkey,
    quorum_threshold: u16,
    proposal_threshold: u64,
    voting_period: i64,
    timelock_delay: i64,
    execution_threshold: u16,
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

    if !is_authorized {
        return Err(ProgramError::InvalidAccountData);
    }

    vault.governance_config = Some(crate::state::GovernanceConfig {
        voting_token_mint,
        quorum_threshold,
        proposal_threshold,
        voting_period,
        timelock_delay,
        execution_threshold,
    });

    drop(vault_data);
    vault.serialize(&mut &mut vault_account.data.borrow_mut()[..])?;

    let clock = Clock::from_account_info(clock_sysvar)?;
    msg!("Governance initialized with voting token {}", voting_token_mint);
    Ok(())
}

pub fn process_create_governance_proposal(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    title: String,
    description: String,
    instructions: Vec<crate::state::GovernanceInstruction>,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_account = next_account_info(account_info_iter)?;
    let proposer = next_account_info(account_info_iter)?;
    let voter_token_account = next_account_info(account_info_iter)?;
    let clock_sysvar = next_account_info(account_info_iter)?;

    if !proposer.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let vault_data = vault_account.data.borrow();
    let mut vault = Vault::try_from_slice(&vault_data)?;

    let governance_config = vault.governance_config.as_ref()
        .ok_or(ProgramError::InvalidAccountData)?;

    if vault.legacy_total_deposited < governance_config.proposal_threshold {
        return Err(ProgramError::InvalidAccountData);
    }

    let clock = Clock::from_account_info(clock_sysvar)?;
    let proposal = crate::state::GovernanceProposal {
        id: vault.next_governance_proposal_id,
        proposer: *proposer.key,
        title,
        description,
        instructions,
        start_time: clock.unix_timestamp,
        end_time: clock.unix_timestamp + governance_config.voting_period,
        for_votes: 0,
        against_votes: 0,
        abstain_votes: 0,
        executed: false,
        cancelled: false,
        eta: 0,
    };

    vault.governance_proposals.push(proposal);
    vault.next_governance_proposal_id += 1;

    drop(vault_data);
    vault.serialize(&mut &mut vault_account.data.borrow_mut()[..])?;

    msg!("Governance proposal {} created", vault.next_governance_proposal_id - 1);
    Ok(())
}

pub fn process_cast_vote(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    proposal_id: u64,
    vote_type: VoteType,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_account = next_account_info(account_info_iter)?;
    let voter = next_account_info(account_info_iter)?;
    let voter_token_account = next_account_info(account_info_iter)?;
    let clock_sysvar = next_account_info(account_info_iter)?;

    if !voter.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let vault_data = vault_account.data.borrow();
    let mut vault = Vault::try_from_slice(&vault_data)?;

    let proposal = vault.governance_proposals.iter_mut()
        .find(|p| p.id == proposal_id)
        .ok_or(ProgramError::InvalidAccountData)?;

    let clock = Clock::from_account_info(clock_sysvar)?;

    if clock.unix_timestamp < proposal.start_time || clock.unix_timestamp > proposal.end_time {
        return Err(ProgramError::InvalidAccountData);
    }

    if vault.vote_records.iter().any(|v| v.voter == *voter.key && v.proposal_id == proposal_id) {
        return Err(ProgramError::InvalidAccountData);
    }

    let voting_power = 100;

    match vote_type {
        VoteType::For => proposal.for_votes += voting_power,
        VoteType::Against => proposal.against_votes += voting_power,
        VoteType::Abstain => proposal.abstain_votes += voting_power,
    }

    let vote_record = crate::state::VoteRecord {
        voter: *voter.key,
        proposal_id,
        vote_type: vote_type.clone(),
        voting_power,
        voted_at: clock.unix_timestamp,
    };

    vault.vote_records.push(vote_record);

    drop(vault_data);
    vault.serialize(&mut &mut vault_account.data.borrow_mut()[..])?;

    msg!("Vote cast for proposal {}: {:?}", proposal_id, vote_type);
    Ok(())
}

pub fn process_queue_proposal(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    proposal_id: u64,
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

    let governance_config = vault.governance_config.as_ref()
        .ok_or(ProgramError::InvalidAccountData)?;

    let proposal_index = vault.governance_proposals.iter()
        .position(|p| p.id == proposal_id)
        .ok_or(ProgramError::InvalidAccountData)?;

    let clock = Clock::from_account_info(clock_sysvar)?;

    if clock.unix_timestamp <= vault.governance_proposals[proposal_index].end_time {
        return Err(ProgramError::InvalidAccountData);
    }

    let proposal = &vault.governance_proposals[proposal_index];
    let total_votes = proposal.for_votes + proposal.against_votes + proposal.abstain_votes;
    let quorum_reached = total_votes >= governance_config.quorum_threshold as u64;

    if !quorum_reached {
        return Err(ProgramError::InvalidAccountData);
    }

    vault.governance_proposals[proposal_index].eta = clock.unix_timestamp + governance_config.timelock_delay;

    drop(vault_data);
    vault.serialize(&mut &mut vault_account.data.borrow_mut()[..])?;

    msg!("Proposal {} queued for execution at {}", proposal_id, vault.governance_proposals[proposal_index].eta);
    Ok(())
}

pub fn process_execute_governance_proposal(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    proposal_id: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_account = next_account_info(account_info_iter)?;
    let executor = next_account_info(account_info_iter)?;
    let clock_sysvar = next_account_info(account_info_iter)?;

    if !executor.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let vault_data = vault_account.data.borrow();
    let mut vault = Vault::try_from_slice(&vault_data)?;

    let governance_config = vault.governance_config.as_ref()
        .ok_or(ProgramError::InvalidAccountData)?;

    let proposal = vault.governance_proposals.iter_mut()
        .find(|p| p.id == proposal_id)
        .ok_or(ProgramError::InvalidAccountData)?;

    let clock = Clock::from_account_info(clock_sysvar)?;

    if clock.unix_timestamp < proposal.eta {
        return Err(ProgramError::InvalidAccountData);
    }

    let total_votes = proposal.for_votes + proposal.against_votes;
    let execution_threshold_reached = if total_votes > 0 {
        (proposal.for_votes * 10000 / total_votes) >= governance_config.execution_threshold as u64
    } else {
        false
    };

    if !execution_threshold_reached {
        return Err(ProgramError::InvalidAccountData);
    }

    proposal.executed = true;

    drop(vault_data);
    vault.serialize(&mut &mut vault_account.data.borrow_mut()[..])?;

    msg!("Governance proposal {} executed successfully", proposal_id);
    Ok(())
}

pub fn process_cancel_governance_proposal(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    proposal_id: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_account = next_account_info(account_info_iter)?;
    let canceller = next_account_info(account_info_iter)?;

    if !canceller.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let vault_data = vault_account.data.borrow();
    let mut vault = Vault::try_from_slice(&vault_data)?;

    let proposal = vault.governance_proposals.iter_mut()
        .find(|p| p.id == proposal_id)
        .ok_or(ProgramError::InvalidAccountData)?;

    if proposal.proposer != *canceller.key && vault.emergency_admin != *canceller.key {
        return Err(ProgramError::InvalidAccountData);
    }

    proposal.cancelled = true;

    drop(vault_data);
    vault.serialize(&mut &mut vault_account.data.borrow_mut()[..])?;

    msg!("Governance proposal {} cancelled", proposal_id);
    Ok(())
}

pub fn process_update_governance_config(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    quorum_threshold: u16,
    proposal_threshold: u64,
    voting_period: i64,
    timelock_delay: i64,
    execution_threshold: u16,
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

    if let Some(governance_config) = &mut vault.governance_config {
        governance_config.quorum_threshold = quorum_threshold;
        governance_config.proposal_threshold = proposal_threshold;
        governance_config.voting_period = voting_period;
        governance_config.timelock_delay = timelock_delay;
        governance_config.execution_threshold = execution_threshold;
    } else {
        return Err(ProgramError::InvalidAccountData);
    }

    drop(vault_data);
    vault.serialize(&mut &mut vault_account.data.borrow_mut()[..])?;

    msg!("Governance configuration updated");
    Ok(())
}
