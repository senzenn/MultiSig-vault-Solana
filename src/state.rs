use solana_program::pubkey::Pubkey;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::Serialize;

// Define VoteType enum
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Copy, PartialEq, Serialize)]
#[borsh(use_discriminant = true)]
pub enum VoteType {
    For = 0,
    Against = 1,
    Abstain = 2,
}

// Define GovernanceInstruction type
pub type GovernanceInstruction = Vec<u8>;

// Fee configuration structure
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Default)]
pub struct FeeConfig {
    pub deposit_fee_bps: u16,
    pub withdrawal_fee_bps: u16,
    pub fee_recipient: Pubkey,
}

// Supported token structure
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Default)]
pub struct SupportedToken {
    pub mint: Pubkey,
    pub bump: u8,
    pub total_deposited: u64,
    pub total_withdrawn: u64,
    pub is_active: bool,
}

// Token balance structure
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Default)]
pub struct TokenBalance {
    pub mint: Pubkey,
    pub balance: u64,
    pub last_updated: i64,
}

// Time lock structure
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Default)]
pub struct TimeLock {
    pub beneficiary: Pubkey,
    pub amount: u64,
    pub start_time: i64,
    pub duration: i64,
    pub cliff_duration: Option<i64>,
    pub is_linear: bool,
    pub claimed_amount: u64,
    pub end_time: i64,
    pub cliff_time: i64,
    pub released_amount: u64,
}

// Proposal structure
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Default)]
pub struct Proposal {
    pub id: u64,
    pub instruction: crate::instruction::VaultInstruction,
    pub approvals: Vec<Pubkey>,
    pub executed: bool,
    pub created_at: i64,
    pub proposer: Pubkey,
}

// Governance proposal structure
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Default)]
pub struct GovernanceProposal {
    pub id: u64,
    pub proposer: Pubkey,
    pub title: String,
    pub description: String,
    pub instructions: Vec<Vec<u8>>,
    pub for_votes: u64,
    pub against_votes: u64,
    pub abstain_votes: u64,
    pub created_at: i64,
    pub end_time: i64,
    pub executed: bool,
    pub queued: bool,
    pub eta: Option<i64>,
    pub start_time: i64,
    pub cancelled: bool,
}

// Vote record structure
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct VoteRecord {
    pub proposal_id: u64,
    pub voter: Pubkey,
    pub vote_type: VoteType,
    pub voting_power: u64,
    pub voted_at: i64,
}

// Voter registry structure
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Default)]
pub struct VoterRegistry {
    pub voter: Pubkey,
    pub voting_power: u64,
    pub registered_at: i64,
}

// Governance configuration structure
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Default)]
pub struct GovernanceConfig {
    pub voting_token_mint: Pubkey,
    pub quorum_threshold: u16,
    pub proposal_threshold: u64,
    pub voting_period: i64,
    pub time_lock_delay: i64,
    pub execution_threshold: u16,
    pub timelock_delay: i64, // Alias for time_lock_delay
}

// Multi-sig structure (adapted from coral-xyz multisig)
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Default)]
pub struct MultiSig {
    pub owners: Vec<Pubkey>,
    pub threshold: u64,
    pub nonce: u8,
    pub bump: u8,
}

// Transaction account for multisig execution
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Default)]
pub struct MultiSigTransaction {
    pub multisig: Pubkey,
    pub program_id: Pubkey,
    pub accounts: Vec<TransactionAccount>,
    pub data: Vec<u8>,
    pub signers: Vec<bool>,
    pub did_execute: bool,
    pub proposer: Pubkey,
    pub created_at: i64,
}

// Transaction account metadata
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct TransactionAccount {
    pub pubkey: Pubkey,
    pub is_signer: bool,
    pub is_writable: bool,
}

// Yield strategy configuration
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Default)]
pub struct YieldStrategyConfig {
    pub token_mint: Pubkey,
    pub strategy_program: Pubkey,
    pub auto_compound: bool,
    pub last_harvested_slot: u64,
}

// Emergency action log entry
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Default)]
pub struct EmergencyActionLog {
    pub timestamp: i64,
    pub admin: Pubkey,
    pub action: u8, // 0: Pause, 1: Unpause, 2: EmergencyWithdraw
    pub details: Vec<u8>,
}

// Main Vault structure (what tests expect)
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Default)]
pub struct Vault {
    pub authority: Pubkey,
    pub bump: u8,
    pub emergency_admin: Pubkey,
    pub paused: bool,
    pub supported_tokens: Vec<SupportedToken>,
    pub token_balances: Vec<TokenBalance>,
    pub time_locks: Vec<TimeLock>,
    pub proposals: Vec<Proposal>,
    pub next_proposal_id: u64,
    pub fee_config: FeeConfig,
    pub total_value_locked: u64,
    pub total_fees_collected: u64,
    pub legacy_mint: Option<Pubkey>,
    pub legacy_total_deposited: u64,
    pub governance_config: Option<GovernanceConfig>,
    pub governance_proposals: Vec<GovernanceProposal>,
    pub next_governance_proposal_id: u64,
    pub vote_records: Vec<VoteRecord>,
    pub voter_registry: Vec<VoterRegistry>,
    pub multi_sig: Option<MultiSig>,
    pub multi_sig_transactions: Vec<MultiSigTransaction>,
    pub yield_strategies: Vec<YieldStrategyConfig>,
    pub emergency_logs: Vec<EmergencyActionLog>,
}

// Vault state structure (simplified version)
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct VaultState {
    pub authority: Pubkey,
    pub emergency_admin: Pubkey,
    pub is_paused: bool,
    pub total_deposits: u64,
    pub total_withdrawals: u64,
    pub fee_recipient: Pubkey,
    pub deposit_fee_bps: u16,
    pub withdrawal_fee_bps: u16,
}

// Multi-sig state structure
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct MultiSigState {
    pub authorities: Vec<Pubkey>,
    pub threshold: u8,
    pub bump: u8,
}

// Time lock state structure
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct TimeLockState {
    pub beneficiary: Pubkey,
    pub amount: u64,
    pub start_time: i64,
    pub duration: i64,
    pub cliff_duration: Option<i64>,
    pub is_linear: bool,
    pub claimed_amount: u64,
}

// Governance state structure
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GovernanceState {
    pub voting_token_mint: Pubkey,
    pub quorum_threshold: u16,
    pub proposal_threshold: u64,
    pub voting_period: i64,
    pub time_lock_delay: i64,
    pub execution_threshold: u16,
}
