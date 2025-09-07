use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use solana_program::clock::Clock;

#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct VaultEvent {
    pub event_type: String,
    pub vault: Pubkey,
    pub timestamp: i64,
    pub authority: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct DepositEvent {
    pub base: VaultEvent,
    pub token_mint: Pubkey,
    pub amount: u64,
    pub user: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct WithdrawEvent {
    pub base: VaultEvent,
    pub token_mint: Pubkey,
    pub amount: u64,
    pub user: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct MultiSigInitializedEvent {
    pub base: VaultEvent,
    pub owners: Vec<Pubkey>,
    pub threshold: u64,
    pub nonce: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct MultiSigTransactionCreatedEvent {
    pub base: VaultEvent,
    pub transaction_id: u64,
    pub proposer: Pubkey,
    pub target_program: Pubkey,
    pub instruction_count: usize,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct MultiSigTransactionApprovedEvent {
    pub base: VaultEvent,
    pub transaction_id: u64,
    pub approver: Pubkey,
    pub current_approvals: usize,
    pub required_approvals: usize,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct MultiSigTransactionExecutedEvent {
    pub base: VaultEvent,
    pub transaction_id: u64,
    pub executor: Pubkey,
    pub target_program: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct MultiSigOwnersUpdatedEvent {
    pub base: VaultEvent,
    pub old_owners: Vec<Pubkey>,
    pub new_owners: Vec<Pubkey>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct MultiSigThresholdUpdatedEvent {
    pub base: VaultEvent,
    pub old_threshold: u64,
    pub new_threshold: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct VaultInitializedEvent {
    pub base: VaultEvent,
    pub bump: u8,
    pub emergency_admin: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct TokenDepositedEvent {
    pub base: VaultEvent,
    pub token_mint: Pubkey,
    pub amount: u64,
    pub fee_amount: u64,
    pub depositor: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct TokenWithdrawnEvent {
    pub base: VaultEvent,
    pub token_mint: Pubkey,
    pub amount: u64,
    pub fee_amount: u64,
    pub recipient: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct ProposalCreatedEvent {
    pub base: VaultEvent,
    pub proposal_id: u64,
    pub instruction_type: String,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct ProposalApprovedEvent {
    pub base: VaultEvent,
    pub proposal_id: u64,
    pub approver: Pubkey,
    pub total_approvals: usize,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct ProposalExecutedEvent {
    pub base: VaultEvent,
    pub proposal_id: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct VaultPausedEvent {
    pub base: VaultEvent,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct VaultUnpausedEvent {
    pub base: VaultEvent,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct EmergencyWithdrawEvent {
    pub base: VaultEvent,
    pub token_mint: Pubkey,
    pub amount: u64,
    pub recipient: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct TokenAddedEvent {
    pub base: VaultEvent,
    pub token_mint: Pubkey,
    pub vault_token_account: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct TimeLockCreatedEvent {
    pub base: VaultEvent,
    pub time_lock_index: usize,
    pub beneficiary: Pubkey,
    pub amount: u64,
    pub duration: i64,
    pub cliff_time: Option<i64>,
    pub is_linear: bool,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct TimeLockClaimedEvent {
    pub base: VaultEvent,
    pub time_lock_index: usize,
    pub beneficiary: Pubkey,
    pub claimed_amount: u64,
    pub remaining_amount: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct YieldStrategySetEvent {
    pub base: VaultEvent,
    pub token_mint: Pubkey,
    pub strategy_program: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct FeeConfigUpdatedEvent {
    pub base: VaultEvent,
    pub deposit_fee_bps: u16,
    pub withdrawal_fee_bps: u16,
    pub fee_recipient: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct AuthorityTransferredEvent {
    pub base: VaultEvent,
    pub new_authority: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct EmergencyAdminUpdatedEvent {
    pub base: VaultEvent,
    pub new_admin: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct GovernanceInitializedEvent {
    pub base: VaultEvent,
    pub voting_token_mint: Pubkey,
    pub quorum_threshold: u16,
    pub proposal_threshold: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct GovernanceProposalCreatedEvent {
    pub base: VaultEvent,
    pub proposal_id: u64,
    pub proposer: Pubkey,
    pub title: String,
    pub end_time: i64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct GovernanceVoteCastEvent {
    pub base: VaultEvent,
    pub proposal_id: u64,
    pub voter: Pubkey,
    pub vote_type: crate::state::VoteType,
    pub voting_power: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct GovernanceProposalQueuedEvent {
    pub base: VaultEvent,
    pub proposal_id: u64,
    pub eta: i64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct GovernanceProposalExecutedEvent {
    pub base: VaultEvent,
    pub proposal_id: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct GovernanceProposalCancelledEvent {
    pub base: VaultEvent,
    pub proposal_id: u64,
}

pub fn create_base_event(
    vault: Pubkey,
    authority: Pubkey,
    event_type: &str,
    clock: &Clock,
) -> VaultEvent {
    VaultEvent {
        event_type: event_type.to_string(),
        vault,
        timestamp: clock.unix_timestamp,
        authority,
    }
}
