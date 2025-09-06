# Vault Solana Program - API Reference

## Overview

This API reference provides comprehensive documentation for all instructions, data structures, and functions available in the Vault Solana program. Use this reference when building integrations or understanding program capabilities.

## Instruction Set

### Core Instructions

#### 0: Initialize
Initializes a new vault instance with basic configuration.

**Instruction Data:**
```rust
struct InitializeData {
    bump: u8,  // PDA bump seed
}
```

**Required Accounts:**
1. `vault_account` (writable) - Vault PDA account
2. `mint_account` (readonly) - Token mint for the vault
3. `authority` (signer, writable) - Vault authority
4. `vault_token_account` (writable) - Vault's associated token account
5. `token_program` (readonly) - SPL Token program
6. `associated_token_program` (readonly) - Associated Token program
7. `system_program` (readonly) - System program
8. `rent_sysvar` (readonly) - Rent sysvar

**Response:** Vault initialized successfully

#### 1: Deposit
Deposits tokens into the vault.

**Instruction Data:**
```rust
struct DepositData {
    amount: u64,  // Amount to deposit
}
```

**Required Accounts:**
1. `vault_account` (writable) - Vault state account
2. `user_token_account` (writable) - User's token account
3. `vault_token_account` (writable) - Vault's token account
4. `authority` (signer, readonly) - Deposit authority
5. `token_program` (readonly) - SPL Token program
6. `clock_sysvar` (readonly) - Clock sysvar

**Events Emitted:**
- `DepositEvent` - Contains deposit details

#### 2: Withdraw
Withdraws tokens from the vault.

**Instruction Data:**
```rust
struct WithdrawData {
    amount: u64,  // Amount to withdraw
}
```

**Required Accounts:**
1. `vault_account` (writable) - Vault state account
2. `vault_token_account` (writable) - Vault's token account
3. `user_token_account` (writable) - User's token account
4. `authority` (signer, readonly) - Withdrawal authority
5. `token_program` (readonly) - SPL Token program
6. `clock_sysvar` (readonly) - Clock sysvar

**Events Emitted:**
- `WithdrawEvent` - Contains withdrawal details

### Multi-Signature Instructions

#### 3: InitializeMultiSig
Sets up multi-signature authorization for the vault.

**Instruction Data:**
```rust
struct InitializeMultiSigData {
    authorities: Vec<Pubkey>,  // List of authorized signers
    threshold: u8,            // Required signatures
    bump: u8,                 // PDA bump seed
}
```

**Required Accounts:**
1. `vault_account` (writable) - Vault state account
2. `initializer` (signer, readonly) - Account initializing multi-sig
3. `clock_sysvar` (readonly) - Clock sysvar

**Events Emitted:**
- `MultiSigInitializedEvent` - Contains multi-sig configuration

#### 4: CreateProposal
Creates a new multi-signature proposal.

**Instruction Data:**
```rust
struct CreateProposalData {
    instruction: Box<VaultInstruction>,  // Instruction to execute
}
```

**Required Accounts:**
1. `vault_account` (writable) - Vault state account
2. `proposer` (signer, readonly) - Account creating proposal
3. `clock_sysvar` (readonly) - Clock sysvar

**Events Emitted:**
- `ProposalCreatedEvent` - Contains proposal details

#### 5: ApproveProposal
Approves a pending multi-signature proposal.

**Instruction Data:**
```rust
struct ApproveProposalData {
    proposal_id: u64,  // ID of proposal to approve
}
```

**Required Accounts:**
1. `vault_account` (writable) - Vault state account
2. `approver` (signer, readonly) - Account approving proposal

#### 6: ExecuteProposal
Executes an approved multi-signature proposal.

**Instruction Data:**
```rust
struct ExecuteProposalData {
    proposal_id: u64,  // ID of proposal to execute
}
```

**Required Accounts:**
1. `vault_account` (writable) - Vault state account
2. `executor` (signer, readonly) - Account executing proposal

**Events Emitted:**
- `ProposalExecutedEvent` - Contains execution details

#### 7: RejectProposal
Rejects a pending multi-signature proposal.

**Instruction Data:**
```rust
struct RejectProposalData {
    proposal_id: u64,  // ID of proposal to reject
}
```

**Required Accounts:**
1. `vault_account` (writable) - Vault state account
2. `rejector` (signer, readonly) - Account rejecting proposal

### Emergency Instructions

#### 8: PauseVault
Pauses all vault operations for emergency situations.

**Instruction Data:** None

**Required Accounts:**
1. `vault_account` (writable) - Vault state account
2. `authority` (signer, readonly) - Emergency admin authority

**Events Emitted:**
- `VaultPausedEvent` - Contains pause details

#### 9: UnpauseVault
Resumes vault operations after emergency pause.

**Instruction Data:** None

**Required Accounts:**
1. `vault_account` (writable) - Vault state account
2. `authority` (signer, readonly) - Emergency admin authority

**Events Emitted:**
- `VaultUnpausedEvent` - Contains unpause details

#### 10: EmergencyWithdraw
Allows emergency withdrawal of specific tokens.

**Instruction Data:**
```rust
struct EmergencyWithdrawData {
    token_mint: Pubkey,  // Token mint to withdraw
    amount: u64,         // Amount to withdraw
}
```

**Required Accounts:**
1. `vault_account` (writable) - Vault state account
2. `vault_token_account` (writable) - Vault's token account
3. `user_token_account` (writable) - User's token account
4. `authority` (signer, readonly) - Emergency admin authority
5. `token_program` (readonly) - SPL Token program

**Events Emitted:**
- `EmergencyWithdrawEvent` - Contains emergency withdrawal details

### Multi-Token Instructions

#### 11: AddSupportedToken
Adds support for a new token type.

**Instruction Data:**
```rust
struct AddSupportedTokenData {
    mint: Pubkey,  // Token mint to add
    bump: u8,      // PDA bump seed
}
```

**Required Accounts:**
1. `vault_account` (writable) - Vault state account
2. `token_mint` (readonly) - Token mint account
3. `vault_token_account` (writable) - Vault's associated token account
4. `authority` (signer, readonly) - Vault authority
5. `token_program` (readonly) - SPL Token program
6. `associated_token_program` (readonly) - Associated Token program
7. `system_program` (readonly) - System program
8. `rent_sysvar` (readonly) - Rent sysvar

**Events Emitted:**
- `TokenAddedEvent` - Contains token addition details

#### 12: DepositMultiToken
Deposits a specific token type.

**Instruction Data:**
```rust
struct DepositMultiTokenData {
    mint: Pubkey,  // Token mint to deposit
    amount: u64,   // Amount to deposit
}
```

**Required Accounts:**
1. `vault_account` (writable) - Vault state account
2. `user_token_account` (writable) - User's token account
3. `vault_token_account` (writable) - Vault's token account
4. `authority` (signer, readonly) - Deposit authority
5. `token_program` (readonly) - SPL Token program
6. `clock_sysvar` (readonly) - Clock sysvar

#### 13: WithdrawMultiToken
Withdraws a specific token type.

**Instruction Data:**
```rust
struct WithdrawMultiTokenData {
    mint: Pubkey,  // Token mint to withdraw
    amount: u64,   // Amount to withdraw
}
```

**Required Accounts:**
1. `vault_account` (writable) - Vault state account
2. `vault_token_account` (writable) - Vault's token account
3. `user_token_account` (writable) - User's token account
4. `authority` (signer, readonly) - Withdrawal authority
5. `token_program` (readonly) - SPL Token program
6. `clock_sysvar` (readonly) - Clock sysvar

### Time-Lock Instructions

#### 14: CreateTimeLock
Creates a time-locked deposit with vesting schedule.

**Instruction Data:**
```rust
struct CreateTimeLockData {
    beneficiary: Pubkey,        // Recipient of locked tokens
    amount: u64,               // Amount to lock
    duration: i64,             // Lock duration in seconds
    cliff_duration: Option<i64>, // Cliff period in seconds
    is_linear: bool,           // Linear vs cliff-based release
}
```

**Required Accounts:**
1. `vault_account` (writable) - Vault state account
2. `beneficiary_token_account` (writable) - Beneficiary's token account
3. `vault_token_account` (writable) - Vault's token account
4. `authority` (signer, readonly) - Vault authority
5. `token_program` (readonly) - SPL Token program
6. `clock_sysvar` (readonly) - Clock sysvar

**Events Emitted:**
- `TimeLockCreatedEvent` - Contains time-lock details

#### 15: ClaimTimeLock
Claims released tokens from a time lock.

**Instruction Data:**
```rust
struct ClaimTimeLockData {
    time_lock_index: usize,  // Index of time lock to claim
}
```

**Required Accounts:**
1. `vault_account` (writable) - Vault state account
2. `beneficiary_token_account` (writable) - Beneficiary's token account
3. `vault_token_account` (writable) - Vault's token account
4. `beneficiary` (signer, readonly) - Beneficiary account
5. `token_program` (readonly) - SPL Token program
6. `clock_sysvar` (readonly) - Clock sysvar

**Events Emitted:**
- `TimeLockClaimedEvent` - Contains claim details

#### 16: CancelTimeLock
Cancels a time lock (authority only).

**Instruction Data:**
```rust
struct CancelTimeLockData {
    time_lock_index: usize,  // Index of time lock to cancel
}
```

**Required Accounts:**
1. `vault_account` (writable) - Vault state account
2. `authority` (signer, readonly) - Vault authority
3. `clock_sysvar` (readonly) - Clock sysvar

### Yield Farming Instructions

#### 17: SetYieldStrategy
Sets a yield farming strategy for a token.

**Instruction Data:**
```rust
struct SetYieldStrategyData {
    token_mint: Pubkey,       // Token mint for strategy
    strategy_program: Pubkey, // Yield farming program ID
}
```

**Required Accounts:**
1. `vault_account` (writable) - Vault state account
2. `strategy_program` (readonly) - Strategy program account
3. `authority` (signer, readonly) - Vault authority

**Events Emitted:**
- `YieldStrategySetEvent` - Contains strategy details

#### 18: HarvestYield
Harvests yield from a yield farming strategy.

**Instruction Data:**
```rust
struct HarvestYieldData {
    token_mint: Pubkey,  // Token mint to harvest
}
```

**Required Accounts:**
1. `vault_account` (writable) - Vault state account
2. `vault_token_account` (writable) - Vault's token account
3. `reward_token_account` (writable) - Reward token account
4. `strategy_account` (writable) - Strategy account
5. `authority` (signer, readonly) - Vault authority
6. `strategy_program` (readonly) - Strategy program
7. `token_program` (readonly) - SPL Token program

#### 19: CompoundYield
Compounds harvested yield back into strategy.

**Instruction Data:**
```rust
struct CompoundYieldData {
    token_mint: Pubkey,  // Token mint to compound
}
```

**Required Accounts:**
1. `vault_account` (writable) - Vault state account
2. `vault_token_account` (writable) - Vault's token account
3. `strategy_account` (writable) - Strategy account
4. `authority` (signer, readonly) - Vault authority
5. `strategy_program` (readonly) - Strategy program
6. `token_program` (readonly) - SPL Token program

### Jupiter DEX Instructions

#### 20: JupiterSwap
Performs a token swap via Jupiter aggregator.

**Instruction Data:**
```rust
struct JupiterSwapData {
    input_mint: Pubkey,   // Input token mint
    output_mint: Pubkey,  // Output token mint
    amount: u64,          // Amount to swap
    slippage_bps: u16,    // Slippage tolerance in basis points
}
```

**Required Accounts:**
- Vault account and token accounts
- Jupiter program and related accounts
- Authority and token program accounts

#### 21: JupiterRoute
Performs a multi-hop token swap via Jupiter.

**Instruction Data:**
```rust
struct JupiterRouteData {
    input_mint: Pubkey,   // Input token mint
    output_mint: Pubkey,  // Output token mint
    amount: u64,          // Amount to swap
    route: Vec<u8>,       // Encoded swap route
}
```

### Fee Management Instructions

#### 22: UpdateFeeConfig
Updates fee configuration for the vault.

**Instruction Data:**
```rust
struct UpdateFeeConfigData {
    deposit_fee_bps: u16,     // Deposit fee in basis points
    withdrawal_fee_bps: u16,  // Withdrawal fee in basis points
    fee_recipient: Pubkey,    // Fee recipient address
}
```

**Required Accounts:**
1. `vault_account` (writable) - Vault state account
2. `authority` (signer, readonly) - Vault authority

**Events Emitted:**
- `FeeConfigUpdatedEvent` - Contains fee configuration details

#### 23: CollectFees
Collects accumulated fees to fee recipient.

**Instruction Data:** None

**Required Accounts:**
1. `vault_account` (writable) - Vault state account
2. `fee_recipient_token_account` (writable) - Fee recipient's token account
3. `vault_token_account` (writable) - Vault's token account
4. `authority` (signer, readonly) - Vault authority
5. `token_program` (readonly) - SPL Token program

### Administrative Instructions

#### 24: TransferAuthority
Transfers vault authority to new address.

**Instruction Data:**
```rust
struct TransferAuthorityData {
    new_authority: Pubkey,  // New authority address
}
```

**Required Accounts:**
1. `vault_account` (writable) - Vault state account
2. `current_authority` (signer, readonly) - Current vault authority

**Events Emitted:**
- `AuthorityTransferredEvent` - Contains authority transfer details

#### 25: UpdateEmergencyAdmin
Updates emergency administrator address.

**Instruction Data:**
```rust
struct UpdateEmergencyAdminData {
    new_admin: Pubkey,  // New emergency admin address
}
```

**Required Accounts:**
1. `vault_account` (writable) - Vault state account
2. `current_authority` (signer, readonly) - Current vault authority

**Events Emitted:**
- `EmergencyAdminUpdatedEvent` - Contains emergency admin update details

### Governance Instructions

#### 26: InitializeGovernance
Initializes the governance system for the vault.

**Instruction Data:**
```rust
struct InitializeGovernanceData {
    voting_token_mint: Pubkey,    // Token used for voting
    quorum_threshold: u16,        // Minimum participation percentage
    proposal_threshold: u64,      // Minimum tokens to create proposal
    voting_period: i64,           // Voting duration in seconds
    timelock_delay: i64,          // Delay before execution
    execution_threshold: u16,     // Minimum approval percentage
}
```

**Required Accounts:**
1. `vault_account` (writable) - Vault state account
2. `authority` (signer, readonly) - Vault authority

**Events Emitted:**
- `GovernanceInitializedEvent` - Contains governance configuration

#### 27: CreateGovernanceProposal
Creates a new governance proposal.

**Instruction Data:**
```rust
struct CreateGovernanceProposalData {
    title: String,                      // Proposal title
    description: String,                // Proposal description
    instructions: Vec<GovernanceInstruction>, // Instructions to execute
}
```

**Required Accounts:**
1. `vault_account` (writable) - Vault state account
2. `proposer` (signer, readonly) - Account creating proposal
3. `proposer_token_account` (readonly) - Proposer's token account
4. `clock_sysvar` (readonly) - Clock sysvar

**Events Emitted:**
- `GovernanceProposalCreatedEvent` - Contains proposal details

#### 28: CastVote
Casts a vote on a governance proposal.

**Instruction Data:**
```rust
struct CastVoteData {
    proposal_id: u64,     // Proposal to vote on
    vote_type: VoteType,  // Type of vote (For/Against/Abstain)
}
```

**Required Accounts:**
1. `vault_account` (writable) - Vault state account
2. `voter` (signer, readonly) - Account casting vote
3. `voter_token_account` (readonly) - Voter's token account
4. `clock_sysvar` (readonly) - Clock sysvar

**Events Emitted:**
- `GovernanceVoteCastEvent` - Contains vote details

#### 29: QueueProposal
Queues an approved proposal for execution.

**Instruction Data:**
```rust
struct QueueProposalData {
    proposal_id: u64,  // Proposal to queue
}
```

**Required Accounts:**
1. `vault_account` (writable) - Vault state account
2. `authority` (signer, readonly) - Vault authority
3. `clock_sysvar` (readonly) - Clock sysvar

**Events Emitted:**
- `GovernanceProposalQueuedEvent` - Contains queue details

#### 30: ExecuteGovernanceProposal
Executes a queued governance proposal.

**Instruction Data:**
```rust
struct ExecuteGovernanceProposalData {
    proposal_id: u64,  // Proposal to execute
}
```

**Required Accounts:**
1. `vault_account` (writable) - Vault state account
2. `executor` (signer, readonly) - Account executing proposal
3. `clock_sysvar` (readonly) - Clock sysvar

**Events Emitted:**
- `GovernanceProposalExecutedEvent` - Contains execution details

#### 31: CancelGovernanceProposal
Cancels a pending governance proposal.

**Instruction Data:**
```rust
struct CancelGovernanceProposalData {
    proposal_id: u64,  // Proposal to cancel
}
```

**Required Accounts:**
1. `vault_account` (writable) - Vault state account
2. `canceller` (signer, readonly) - Account cancelling proposal

**Events Emitted:**
- `GovernanceProposalCancelledEvent` - Contains cancellation details

#### 32: UpdateGovernanceConfig
Updates governance system configuration.

**Instruction Data:**
```rust
struct UpdateGovernanceConfigData {
    quorum_threshold: u16,        // Minimum participation percentage
    proposal_threshold: u64,      // Minimum tokens to create proposal
    voting_period: i64,           // Voting duration in seconds
    timelock_delay: i64,          // Delay before execution
    execution_threshold: u16,     // Minimum approval percentage
}
```

**Required Accounts:**
1. `vault_account` (writable) - Vault state account
2. `authority` (signer, readonly) - Vault authority

## Data Structures

### Core Data Types

#### Vault
```rust
pub struct Vault {
    pub authority: Pubkey,
    pub bump: u8,
    pub multi_sig: Option<MultiSigAuthority>,
    pub paused: bool,
    pub emergency_admin: Pubkey,
    pub supported_tokens: Vec<TokenBalance>,
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
    pub voter_registry: Vec<VoterInfo>,
}
```

#### MultiSigAuthority
```rust
pub struct MultiSigAuthority {
    pub authorities: Vec<Pubkey>,
    pub threshold: u8,
    pub nonce: u8,
}
```

#### GovernanceConfig
```rust
pub struct GovernanceConfig {
    pub voting_token_mint: Pubkey,
    pub quorum_threshold: u16,
    pub proposal_threshold: u64,
    pub voting_period: i64,
    pub timelock_delay: i64,
    pub execution_threshold: u16,
}
```

### Enums

#### VoteType
```rust
pub enum VoteType {
    For,
    Against,
    Abstain,
}
```

#### VaultInstruction
```rust
pub enum VaultInstruction {
    Initialize { bump: u8 },
    Deposit { amount: u64 },
    Withdraw { amount: u64 },
    // ... all instruction variants
}
```

## Error Codes

### Program Error Codes
- `0x0` - Invalid instruction data
- `0x1` - Missing required signature
- `0x2` - Incorrect program ID
- `0x3` - Invalid account data
- `0x4` - Insufficient funds
- `0x5` - Invalid argument
- `0x6` - Account already initialized
- `0x7` - Vault paused
- `0x8` - Time lock not mature
- Custom errors defined in `VaultError` enum

## Event Structures

### Base Event
```rust
pub struct VaultEvent {
    pub event_type: String,
    pub vault: Pubkey,
    pub timestamp: i64,
    pub authority: Pubkey,
}
```

### Specific Events
- `DepositEvent`, `WithdrawEvent`
- `MultiSigInitializedEvent`, `ProposalCreatedEvent`
- `GovernanceInitializedEvent`, `GovernanceProposalCreatedEvent`
- `TimeLockCreatedEvent`, `TimeLockClaimedEvent`
- `FeeConfigUpdatedEvent`, `AuthorityTransferredEvent`

## Constants

### Program Constants
```rust
pub const VAULT_PROGRAM_ID: &str = "VAULT11111111111111111111111111111111111111";
pub const MAX_PROPOSAL_TITLE_LENGTH: usize = 200;
pub const MAX_PROPOSAL_DESCRIPTION_LENGTH: usize = 1000;
pub const MAX_TIME_LOCKS_PER_VAULT: usize = 100;
pub const MIN_VOTING_PERIOD: i64 = 86400; // 1 day
pub const MAX_VOTING_PERIOD: i64 = 2592000; // 30 days
```

### Fee Constants
```rust
pub const MAX_DEPOSIT_FEE_BPS: u16 = 1000; // 10%
pub const MAX_WITHDRAWAL_FEE_BPS: u16 = 1000; // 10%
pub const BASIS_POINTS_DENOMINATOR: u16 = 10000;
```

## Account Sizes

### PDA Account Sizes
- Vault account: ~10,000 bytes (variable based on vectors)
- Associated token accounts: ~165 bytes
- Program accounts: Variable based on data structure

### Rent Calculations
```rust
// Minimum rent for vault account
let vault_size = std::mem::size_of::<Vault>() as u64;
let min_rent = Rent::default().minimum_balance(vault_size as usize);
```

## Rate Limits

### Operation Limits
- Maximum 100 proposals per vault
- Maximum 100 time locks per vault
- Maximum 1000 governance proposals per vault
- Maximum 10,000 vote records per vault

### Time-based Limits
- Minimum 1 day voting period
- Maximum 30 days voting period
- Minimum 1 hour timelock delay
- Maximum 7 days timelock delay

This API reference provides complete specifications for integrating with the Vault Solana program. All instructions, data structures, error codes, and operational limits are documented for reliable implementation.
