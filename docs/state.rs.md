# Vault Solana Program - State Structures Documentation (`state.rs`)

## Overview

The `state.rs` file defines all the core data structures used by the Vault Solana program. These structures represent the program's state, configuration, and operational data stored on-chain.

## Core Data Structures

### Vault Structure

The main vault account structure containing all program state:

```rust
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Vault {
    // Core vault information
    pub authority: Pubkey,              // Primary vault authority
    pub bump: u8,                      // PDA bump seed

    // Multi-signature configuration
    pub multi_sig: Option<MultiSigAuthority>,

    // Operational state
    pub paused: bool,                  // Emergency pause flag
    pub emergency_admin: Pubkey,       // Emergency administrator

    // Token management
    pub supported_tokens: Vec<TokenBalance>, // Supported token balances
    pub time_locks: Vec<TimeLock>,     // Active time locks

    // Governance system
    pub proposals: Vec<Proposal>,      // Multi-sig proposals
    pub next_proposal_id: u64,         // Next proposal ID counter

    // Fee configuration
    pub fee_config: FeeConfig,         // Fee settings

    // Financial metrics
    pub total_value_locked: u64,       // Total TVL across all tokens
    pub total_fees_collected: u64,     // Total fees collected

    // Legacy support (for migration)
    pub legacy_mint: Option<Pubkey>,   // Legacy token mint
    pub legacy_total_deposited: u64,   // Legacy total deposits

    // Advanced governance
    pub governance_config: Option<GovernanceConfig>,
    pub governance_proposals: Vec<GovernanceProposal>,
    pub next_governance_proposal_id: u64,
    pub vote_records: Vec<VoteRecord>,
    pub voter_registry: Vec<VoterInfo>,
}
```

### Multi-Signature Authority

Configuration for multi-signature operations:

```rust
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct MultiSigAuthority {
    pub authorities: Vec<Pubkey>,    // List of authorized signers
    pub threshold: u8,              // Required signatures for approval
    pub nonce: u8,                  // Multi-sig nonce for replay protection
}
```

### Token Balance Management

Structure for tracking individual token balances and strategies:

```rust
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct TokenBalance {
    pub mint: Pubkey,                    // Token mint address
    pub balance: u64,                    // Current token balance
    pub yield_strategy: Option<Pubkey>,  // Associated yield strategy (optional)
}
```

### Fee Configuration

Flexible fee structure for deposits and withdrawals:

```rust
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct FeeConfig {
    pub deposit_fee_bps: u16,       // Deposit fee in basis points (1/100th of 1%)
    pub withdrawal_fee_bps: u16,    // Withdrawal fee in basis points
    pub fee_recipient: Pubkey,      // Address to receive collected fees
}
```

### Time-Locked Deposits

Sophisticated vesting and time-lock mechanism:

```rust
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct TimeLock {
    pub beneficiary: Pubkey,         // Recipient of locked tokens
    pub amount: u64,                 // Total locked amount
    pub start_time: i64,             // Lock start timestamp
    pub end_time: i64,               // Lock end timestamp
    pub cliff_time: i64,             // Cliff period end
    pub released_amount: u64,        // Amount already released
    pub is_linear: bool,             // Linear vs cliff-based release
}
```

## Governance System Structures

### Governance Configuration

Settings for the on-chain governance system:

```rust
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct GovernanceConfig {
    pub voting_token_mint: Pubkey,    // Token used for voting
    pub quorum_threshold: u16,        // Minimum votes required (%)
    pub proposal_threshold: u64,      // Minimum tokens to create proposal
    pub voting_period: i64,           // Voting duration in seconds
    pub timelock_delay: i64,          // Delay before execution
    pub execution_threshold: u16,     // Minimum votes for execution (%)
}
```

### Governance Proposal

Structure for governance proposals:

```rust
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct GovernanceProposal {
    pub id: u64,                           // Unique proposal ID
    pub proposer: Pubkey,                  // Proposal creator
    pub title: String,                     // Proposal title
    pub description: String,               // Detailed description
    pub instructions: Vec<GovernanceInstruction>, // Instructions to execute
    pub start_time: i64,                   // Voting start time
    pub end_time: i64,                     // Voting end time
    pub for_votes: u64,                    // Votes in favor
    pub against_votes: u64,                // Votes against
    pub abstain_votes: u64,                // Abstain votes
    pub executed: bool,                    // Execution status
    pub cancelled: bool,                   // Cancellation status
    pub eta: i64,                          // Execution time after timelock
}
```

### Governance Instruction

Instructions that can be executed via governance:

```rust
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct GovernanceInstruction {
    pub program_id: Pubkey,                    // Target program
    pub accounts: Vec<GovernanceAccountMeta>,  // Required accounts
    pub data: Vec<u8>,                        // Instruction data
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct GovernanceAccountMeta {
    pub pubkey: Pubkey,     // Account public key
    pub is_signer: bool,    // Whether account must sign
    pub is_writable: bool,  // Whether account is writable
}
```

### Voting System

Structures for managing votes and voters:

```rust
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum VoteType {
    For,        // Vote in favor
    Against,    // Vote against
    Abstain,    // Abstain from voting
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct VoteRecord {
    pub voter: Pubkey,          // Voter's public key
    pub proposal_id: u64,       // Proposal being voted on
    pub vote_type: VoteType,    // Type of vote cast
    pub voting_power: u64,      // Voting power used
    pub voted_at: i64,          // Timestamp of vote
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct VoterInfo {
    pub voter: Pubkey,              // Voter's public key
    pub voting_power: u64,          // Current voting power
    pub last_vote_time: i64,        // Last voting activity
    pub proposals_created: u64,     // Proposals created by voter
    pub proposals_voted: u64,       // Proposals voted on by voter
}
```

### Multi-Signature Proposals

Legacy proposal system for multi-sig operations:

```rust
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Proposal {
    pub id: u64,                       // Unique proposal ID
    pub proposer: Pubkey,              // Proposal creator
    pub instruction: VaultInstruction, // Instruction to execute
    pub approvals: Vec<Pubkey>,        // List of approvals
    pub created_at: i64,               // Creation timestamp
    pub executed: bool,                // Execution status
}
```

## State Management

### Account Size Calculations

The vault account size must accommodate all stored data:

```rust
let vault_size = std::mem::size_of::<Vault>() as u64;
```

Key considerations:
- **Dynamic Vectors**: `supported_tokens`, `time_locks`, `proposals`, etc. grow over time
- **String Fields**: `title`, `description` in governance proposals have variable length
- **Nested Structures**: Complex governance structures require careful size planning

### Memory Layout

The structures use Borsh serialization for efficient on-chain storage:

- **Fixed-size fields**: Stored first for consistent access
- **Dynamic fields**: Vectors and strings stored with length prefixes
- **Optional fields**: Use `Option<T>` for nullable data

## Security Considerations

### Access Control
- **Authority validation**: Only authorized addresses can modify state
- **Multi-sig verification**: Threshold-based approval for critical operations
- **Signer requirements**: Ensure proper signature validation

### State Integrity
- **Balance validation**: Prevent underflow/overflow in token operations
- **Time validation**: Ensure timelock periods are respected
- **Proposal validation**: Verify proposal parameters and execution conditions

### Data Validation
- **Account ownership**: Verify program ownership of accounts
- **Mint validation**: Ensure supported tokens are valid SPL tokens
- **Amount validation**: Check for reasonable transaction amounts

## Usage Patterns

### Vault Initialization
```rust
let vault = Vault {
    authority: *authority.key,
    bump,
    multi_sig: None,
    paused: false,
    emergency_admin: *authority.key,
    supported_tokens: vec![TokenBalance {
        mint: *mint_account.key,
        balance: 0,
        yield_strategy: None,
    }],
    time_locks: vec![],
    proposals: vec![],
    next_proposal_id: 0,
    fee_config: FeeConfig {
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
```

### State Updates
- **Immutable operations**: Use borrow patterns for reading
- **Mutable operations**: Use borrow_mut for state modifications
- **Serialization**: Always serialize after state changes

### Governance Workflow
1. **Proposal Creation**: Add to `governance_proposals` vector
2. **Voting Period**: Update vote counts in proposal structure
3. **Execution**: Mark proposal as executed and update `eta`

## Performance Considerations

### Vector Operations
- **Growth patterns**: Vectors grow dynamically; monitor for reallocation
- **Search operations**: Use efficient lookup patterns for large vectors
- **Memory usage**: Large governance systems may require pagination

### Serialization Overhead
- **Borsh efficiency**: Optimized for blockchain storage
- **Compression**: Consider data compression for large structures
- **Caching**: Cache frequently accessed data off-chain

This comprehensive state structure provides a solid foundation for a sophisticated DeFi vault with multi-signature governance, yield farming, and advanced token management capabilities.
