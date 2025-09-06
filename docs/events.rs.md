# Vault Solana Program - Events System Documentation (`events.rs`)

## Overview

The `events.rs` file implements a comprehensive event logging system for the Vault Solana program. Events provide transparency, auditability, and enable off-chain monitoring of vault activities. All significant state changes and operations emit structured events that can be indexed and queried.

## Event Architecture

### Base Event Structure

All events inherit from a common base structure for consistency:

```rust
#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct VaultEvent {
    pub event_type: String,      // Type of event (e.g., "deposit", "withdraw")
    pub vault: Pubkey,          // Vault account public key
    pub timestamp: i64,         // Unix timestamp of event
    pub authority: Pubkey,      // Account that triggered the event
}
```

### Event Creation Helper

A utility function for creating base events:

```rust
pub fn create_base_event(vault: Pubkey, authority: Pubkey, event_type: &str, clock: &Clock) -> VaultEvent {
    VaultEvent {
        event_type: event_type.to_string(),
        vault,
        timestamp: clock.unix_timestamp,
        authority,
    }
}
```

## Event Categories

### 1. Token Operation Events

#### Deposit Event
Emitted when tokens are deposited into the vault:

```rust
#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct DepositEvent {
    pub base: VaultEvent,
    pub token_mint: Pubkey,     // Mint of deposited token
    pub amount: u64,           // Amount deposited
    pub user: Pubkey,          // User who made the deposit
}
```

**Emission Trigger:** `process_deposit` function

#### Withdraw Event
Emitted when tokens are withdrawn from the vault:

```rust
#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct WithdrawEvent {
    pub base: VaultEvent,
    pub token_mint: Pubkey,    // Mint of withdrawn token
    pub amount: u64,          // Amount withdrawn
    pub user: Pubkey,         // User who made the withdrawal
}
```

**Emission Trigger:** `process_withdraw` function

### 2. Multi-Signature Events

#### Multi-Signature Initialization Event
Emitted when multi-signature is set up:

```rust
#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct MultiSigInitializedEvent {
    pub base: VaultEvent,
    pub authorities: Vec<Pubkey>,  // List of authorized signers
    pub threshold: u8,            // Required signature threshold
}
```

**Emission Trigger:** `process_initialize_multi_sig` function

#### Proposal Creation Event
Emitted when a new multi-signature proposal is created:

```rust
#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct ProposalCreatedEvent {
    pub base: VaultEvent,
    pub proposal_id: u64,           // Unique proposal identifier
    pub instruction_type: String,   // Type of instruction proposed
}
```

**Emission Trigger:** `process_create_proposal` function

#### Proposal Approval Event
Emitted when a proposal receives an approval:

```rust
#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct ProposalApprovedEvent {
    pub base: VaultEvent,
    pub proposal_id: u64,          // Proposal being approved
    pub approver: Pubkey,          // Account that approved
    pub total_approvals: usize,    // Total approvals received
}
```

**Emission Trigger:** `process_approve_proposal` function

#### Proposal Execution Event
Emitted when a proposal is successfully executed:

```rust
#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct ProposalExecutedEvent {
    pub base: VaultEvent,
    pub proposal_id: u64,  // Executed proposal ID
}
```

**Emission Trigger:** `process_execute_proposal` function

### 3. Emergency Control Events

#### Vault Pause Event
Emitted when the vault is paused:

```rust
#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct VaultPausedEvent {
    pub base: VaultEvent,
}
```

**Emission Trigger:** `process_pause_vault` function

#### Vault Unpause Event
Emitted when the vault is unpaused:

```rust
#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct VaultUnpausedEvent {
    pub base: VaultEvent,
}
```

**Emission Trigger:** `process_unpause_vault` function

#### Emergency Withdraw Event
Emitted during emergency withdrawals:

```rust
#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct EmergencyWithdrawEvent {
    pub base: VaultEvent,
    pub token_mint: Pubkey,    // Token being withdrawn
    pub amount: u64,          // Amount withdrawn
    pub recipient: Pubkey,    // Recipient of emergency withdrawal
}
```

**Emission Trigger:** `process_emergency_withdraw` function

### 4. Token Management Events

#### Token Addition Event
Emitted when a new token is added to the vault:

```rust
#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct TokenAddedEvent {
    pub base: VaultEvent,
    pub token_mint: Pubkey,  // Newly supported token mint
}
```

**Emission Trigger:** `process_add_supported_token` function

### 5. Time-Lock Events

#### Time-Lock Creation Event
Emitted when a new time-locked deposit is created:

```rust
#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct TimeLockCreatedEvent {
    pub base: VaultEvent,
    pub time_lock_index: usize,    // Index in time locks array
    pub beneficiary: Pubkey,       // Recipient of locked tokens
    pub amount: u64,              // Total locked amount
    pub duration: i64,            // Lock duration in seconds
    pub cliff_time: Option<i64>,  // Cliff period timestamp
    pub is_linear: bool,          // Linear release flag
}
```

**Emission Trigger:** `process_create_time_lock` function

#### Time-Lock Claim Event
Emitted when tokens are claimed from a time lock:

```rust
#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct TimeLockClaimedEvent {
    pub base: VaultEvent,
    pub time_lock_index: usize,   // Time lock index
    pub beneficiary: Pubkey,      // Beneficiary claiming tokens
    pub claimed_amount: u64,      // Amount claimed this time
    pub remaining_amount: u64,    // Remaining locked amount
}
```

**Emission Trigger:** `process_claim_time_lock` function

### 6. Yield Farming Events

#### Yield Strategy Set Event
Emitted when a yield strategy is configured:

```rust
#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct YieldStrategySetEvent {
    pub base: VaultEvent,
    pub token_mint: Pubkey,          // Token for yield farming
    pub strategy_program: Pubkey,    // Yield farming program
}
```

**Emission Trigger:** `process_set_yield_strategy` function

### 7. Fee Management Events

#### Fee Configuration Update Event
Emitted when fee settings are updated:

```rust
#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct FeeConfigUpdatedEvent {
    pub base: VaultEvent,
    pub deposit_fee_bps: u16,    // Deposit fee in basis points
    pub withdrawal_fee_bps: u16, // Withdrawal fee in basis points
    pub fee_recipient: Pubkey,   // Fee recipient address
}
```

**Emission Trigger:** `process_update_fee_config` function

### 8. Administrative Events

#### Authority Transfer Event
Emitted when vault authority is transferred:

```rust
#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct AuthorityTransferredEvent {
    pub base: VaultEvent,
    pub new_authority: Pubkey,  // New vault authority
}
```

**Emission Trigger:** `process_transfer_authority` function

#### Emergency Admin Update Event
Emitted when emergency admin is changed:

```rust
#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct EmergencyAdminUpdatedEvent {
    pub base: VaultEvent,
    pub new_admin: Pubkey,  // New emergency admin
}
```

**Emission Trigger:** `process_update_emergency_admin` function

### 9. Governance Events

#### Governance Initialization Event
Emitted when governance system is initialized:

```rust
#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct GovernanceInitializedEvent {
    pub base: VaultEvent,
    pub voting_token_mint: Pubkey,  // Token used for voting
    pub quorum_threshold: u16,      // Quorum requirement
    pub proposal_threshold: u64,    // Proposal creation threshold
}
```

**Emission Trigger:** `process_initialize_governance` function

#### Governance Proposal Creation Event
Emitted when a governance proposal is created:

```rust
#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct GovernanceProposalCreatedEvent {
    pub base: VaultEvent,
    pub proposal_id: u64,      // Unique proposal ID
    pub proposer: Pubkey,      // Proposal creator
    pub title: String,         // Proposal title
    pub end_time: i64,         // Voting end timestamp
}
```

**Emission Trigger:** `process_create_governance_proposal` function

#### Governance Vote Event
Emitted when a vote is cast:

```rust
#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct GovernanceVoteCastEvent {
    pub base: VaultEvent,
    pub proposal_id: u64,         // Proposal being voted on
    pub voter: Pubkey,            // Voter's address
    pub vote_type: VoteType,      // Type of vote cast
    pub voting_power: u64,        // Voting power used
}
```

**Emission Trigger:** `process_cast_vote` function

#### Governance Proposal Queue Event
Emitted when a proposal is queued for execution:

```rust
#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct GovernanceProposalQueuedEvent {
    pub base: VaultEvent,
    pub proposal_id: u64,  // Queued proposal ID
    pub eta: i64,          // Execution timestamp
}
```

**Emission Trigger:** `process_queue_proposal` function

#### Governance Proposal Execution Event
Emitted when a governance proposal is executed:

```rust
#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct GovernanceProposalExecutedEvent {
    pub base: VaultEvent,
    pub proposal_id: u64,  // Executed proposal ID
}
```

**Emission Trigger:** `process_execute_governance_proposal` function

#### Governance Proposal Cancellation Event
Emitted when a proposal is cancelled:

```rust
#[derive(BorshSerialize, BorshDeserialize, Debug, serde::Serialize)]
pub struct GovernanceProposalCancelledEvent {
    pub base: VaultEvent,
    pub proposal_id: u64,  // Cancelled proposal ID
}
```

**Emission Trigger:** `process_cancel_governance_proposal` function

## Event Emission Mechanism

### Event Emission Macro

Events are emitted using a convenient macro defined in `lib.rs`:

```rust
#[macro_export]
macro_rules! emit_event {
    ($event:expr) => {
        msg!("EVENT: {}", serde_json::to_string(&$event).unwrap_or_else(|_| "Event serialization failed".to_string()));
    };
}
```

### Usage Pattern

```rust
// Create and emit an event
let clock = Clock::from_account_info(clock_sysvar)?;
let deposit_event = DepositEvent {
    base: create_base_event(*vault_account.key, *authority.key, "deposit", &clock),
    token_mint,
    amount,
    user: *authority.key,
};
emit_event!(deposit_event);
```

## Event Data Structure

### Serialization
- **Borsh serialization** for on-chain efficiency
- **JSON serialization** for off-chain readability via `serde`
- **Structured data** with typed fields

### Common Fields
All events include:
- **event_type**: String identifier for the event type
- **vault**: Public key of the vault account
- **timestamp**: Unix timestamp when event occurred
- **authority**: Account that triggered the event

### Event-Specific Fields
Additional fields provide context for specific operations:
- **Amounts and balances**
- **Account addresses**
- **Proposal and voting data**
- **Token information**
- **Time-based data**

## Off-Chain Event Processing

### Event Parsing
Events can be parsed from transaction logs:

```javascript
// Example: Parse deposit event
const eventLog = "EVENT: {\"event_type\":\"deposit\",\"vault\":\"...\",\"timestamp\":1234567890,\"authority\":\"...\",\"token_mint\":\"...\",\"amount\":1000000,\"user\":\"...\"}";
const eventData = JSON.parse(eventLog.replace("EVENT: ", ""));
```

### Event Indexing
Events enable:
- **Transaction monitoring**
- **Portfolio tracking**
- **Governance participation tracking**
- **Yield farming analytics**
- **Security incident detection**

### Database Schema
Typical event storage structure:

```sql
CREATE TABLE vault_events (
    id SERIAL PRIMARY KEY,
    event_type VARCHAR(50) NOT NULL,
    vault_address VARCHAR(44) NOT NULL,
    timestamp BIGINT NOT NULL,
    authority VARCHAR(44) NOT NULL,
    event_data JSONB,
    transaction_signature VARCHAR(88),
    block_height BIGINT,
    created_at TIMESTAMP DEFAULT NOW()
);
```

## Event Categories and Use Cases

### User Activity Events
- **Deposit/Withdraw events** - Track user interactions
- **Time-lock events** - Monitor vesting activities
- **Governance events** - Track voting participation

### Administrative Events
- **Configuration changes** - Track parameter updates
- **Authority transfers** - Monitor access control changes
- **Emergency events** - Alert on security incidents

### Financial Events
- **Fee collection events** - Track revenue generation
- **Yield farming events** - Monitor DeFi strategy performance
- **Token management events** - Track supported assets

### Security Events
- **Multi-signature events** - Track approval processes
- **Emergency control events** - Monitor security measures
- **Authorization events** - Track permission changes

## Performance Considerations

### Event Size
- **Minimize data fields** to reduce transaction size
- **Use efficient data types** (u64 vs String where possible)
- **Compress repetitive data** using references

### Emission Frequency
- **Batch related events** when possible
- **Emit only essential events** to reduce costs
- **Consider event filtering** for high-frequency operations

### Storage Optimization
- **Off-chain indexing** for complex queries
- **Event aggregation** for summary statistics
- **Archival strategies** for old events

## Testing and Validation

### Event Emission Testing
```rust
#[test]
fn test_deposit_event_emission() {
    // Setup test environment
    // Execute deposit instruction
    // Verify event emission in logs
    // Validate event data structure
}
```

### Event Data Validation
- **Schema validation** for all event types
- **Data consistency** checks
- **Timestamp accuracy** verification
- **Address format** validation

### Integration Testing
- **End-to-end event flow** testing
- **Event indexer** compatibility testing
- **Off-chain consumer** validation

## Security Considerations

### Event Data Integrity
- **Cryptographic verification** of event authenticity
- **Tamper-proof logging** through Solana's transaction system
- **Event sequence validation** for temporal consistency

### Privacy Considerations
- **Public address exposure** in events
- **Amount disclosure** in transaction events
- **PII minimization** in event data

### Denial of Service Prevention
- **Event size limits** to prevent log spam
- **Rate limiting** for high-frequency operations
- **Cost considerations** for event emission

This comprehensive event system ensures complete transparency and auditability of all vault operations, enabling sophisticated off-chain analytics and monitoring capabilities.
