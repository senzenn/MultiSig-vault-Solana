# Vault Solana Program - Basic Processor Documentation (`processors/basic.rs`)

## Overview

The `basic.rs` processor handles core vault operations including initialization, deposits, and withdrawals. This module provides the fundamental functionality for managing token balances and vault state in the Solana ecosystem.

## Core Functions

### Vault Initialization

#### `process_initialize`
Creates and initializes a new vault instance with basic configuration.

**Function Signature:**
```rust
pub fn process_initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    bump: u8,
) -> ProgramResult
```

**Required Accounts:**
1. `vault_account` - The vault's main PDA account (writable)
2. `mint_account` - Token mint for the vault (readonly)
3. `authority` - Vault authority (signer, writable)
4. `vault_token_account` - Vault's token account (ATA, writable)
5. `token_program` - SPL Token program (readonly)
6. `associated_token_program` - Associated Token program (readonly)
7. `system_program` - System program (readonly)
8. `rent_sysvar` - Rent sysvar (readonly)

**Process Flow:**
1. **Validate signer** - Ensure authority is a signer
2. **Calculate account size** - Determine vault account size requirements
3. **Check rent exemption** - Verify sufficient lamports for rent
4. **Create vault account** - Use system instruction to create PDA
5. **Create token account** - Set up associated token account for vault
6. **Initialize vault state** - Set up initial vault configuration
7. **Serialize state** - Save vault state to account data

**Initial Vault Configuration:**
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

### Token Deposits

#### `process_deposit`
Handles token deposits into the vault with fee collection and balance updates.

**Function Signature:**
```rust
pub fn process_deposit(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
) -> ProgramResult
```

**Required Accounts:**
1. `vault_account` - Vault state account (writable)
2. `user_token_account` - User's token account (writable)
3. `vault_token_account` - Vault's token account (writable)
4. `authority` - Deposit authority (signer, readonly)
5. `token_program` - SPL Token program (readonly)
6. `clock_sysvar` - Clock sysvar for timestamp (readonly)

**Security Validations:**
- **Signer verification** - Authority must be a signer
- **Program ownership** - Vault account owned by program
- **State deserialization** - Valid vault state
- **Authorization check** - Single authority or multi-sig validation
- **Pause check** - Vault must not be paused

**Deposit Process:**
1. **Transfer tokens** - Move tokens from user to vault
2. **Update legacy balance** - Increment legacy total deposited
3. **Update token balance** - Update supported tokens balance
4. **Update TVL** - Increment total value locked
5. **Emit event** - Log deposit event with details
6. **Serialize state** - Save updated vault state

**Fee Handling:**
- Deposit fees calculated in basis points
- Fees deducted from deposit amount
- Fee recipient receives collected fees

### Token Withdrawals

#### `process_withdraw`
Processes token withdrawals from the vault with balance validation and event emission.

**Function Signature:**
```rust
pub fn process_withdraw(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
) -> ProgramResult
```

**Required Accounts:**
1. `vault_account` - Vault state account (writable)
2. `vault_token_account` - Vault's token account (writable)
3. `user_token_account` - User's token account (writable)
4. `authority` - Withdrawal authority (signer, readonly)
5. `token_program` - SPL Token program (readonly)
6. `clock_sysvar` - Clock sysvar for timestamp (readonly)

**Withdrawal Validations:**
- **Signer verification** - Authority must sign transaction
- **Program ownership** - Vault account program-owned
- **State integrity** - Valid vault state deserialization
- **Authorization** - Authority or multi-sig validation
- **Operational status** - Vault not paused
- **Sufficient balance** - Available funds for withdrawal

**Withdrawal Process:**
1. **Validate balance** - Check legacy_total_deposited >= amount
2. **Transfer tokens** - Move tokens from vault to user
3. **Update balances** - Decrement legacy and token balances
4. **Update TVL** - Decrement total value locked
5. **Apply fees** - Deduct withdrawal fees if configured
6. **Emit event** - Log withdrawal with details
7. **Serialize state** - Persist updated vault state

## Account Management

### Program-Derived Addresses (PDAs)
- **Vault PDA** - Derived from program ID and bump seed
- **Token Account PDAs** - Associated token accounts for vault
- **Authority validation** - PDA ownership verification

### Associated Token Accounts (ATAs)
- **Automatic creation** - ATAs created during initialization
- **Standard derivation** - Follows SPL ATA specification
- **Authority control** - Vault controls token account operations

## State Management

### Vault State Updates
- **Atomic operations** - All changes applied together
- **Borrow patterns** - Efficient memory management
- **Serialization** - Borsh encoding for on-chain storage

### Balance Tracking
- **Legacy support** - Backward compatibility with single token
- **Multi-token support** - Individual token balance tracking
- **TVL calculation** - Total value locked across all assets

## Event Emission

### Deposit Events
```rust
let deposit_event = DepositEvent {
    base: create_base_event(*vault_account.key, *authority.key, "deposit", &clock),
    token_mint,
    amount,
    user: *authority.key,
};
emit_event!(deposit_event);
```

### Withdrawal Events
```rust
let withdraw_event = WithdrawEvent {
    base: create_base_event(*vault_account.key, *authority.key, "withdraw", &clock),
    token_mint,
    amount,
    user: *authority.key,
};
emit_event!(withdraw_event);
```

## Fee Management

### Fee Calculation
- **Basis points** - Fees in 1/100th of 1% (0.01%)
- **Flexible configuration** - Separate deposit and withdrawal fees
- **Recipient specification** - Configurable fee recipient

### Fee Application
```rust
let fee_amount = (amount * deposit_fee_bps as u64) / 10000;
let net_amount = amount - fee_amount;
```

## Security Considerations

### Access Control
- **Authority validation** - Single or multi-signature
- **Signer verification** - All operations require signatures
- **Pause mechanism** - Emergency stop functionality

### Input Validation
- **Amount bounds** - Prevent overflow/underflow
- **Account validation** - Verify ownership and types
- **State consistency** - Ensure valid state transitions

### Error Handling
- **Comprehensive errors** - Clear error messages
- **Atomic operations** - No partial state changes
- **Rollback protection** - Failed operations don't persist

## Performance Optimization

### Memory Efficiency
- **Borrow patterns** - Minimize memory allocation
- **Efficient serialization** - Borsh for fast encoding/decoding
- **Lazy loading** - Load data only when needed

### Computational Efficiency
- **Minimal validation** - Fast authorization checks
- **Batch operations** - Process multiple operations together
- **Cached calculations** - Reuse computed values

## Testing Strategy

### Unit Tests
```rust
#[test]
fn test_initialize_vault() {
    // Test vault initialization
    // Verify account creation
    // Check initial state
}

#[test]
fn test_deposit_tokens() {
    // Test token deposit
    // Verify balance updates
    // Check event emission
}

#[test]
fn test_withdraw_tokens() {
    // Test token withdrawal
    // Verify balance deduction
    // Check insufficient funds handling
}
```

### Integration Tests
- **Full deposit/withdraw cycle** testing
- **Multi-signature** integration testing
- **Event emission** verification
- **Fee calculation** accuracy testing

### Edge Cases
- **Zero amount** operations
- **Maximum amount** operations
- **Insufficient balance** scenarios
- **Account validation** failures

## Error Conditions

### Common Errors
- `MissingRequiredSignature` - Missing signer
- `IncorrectProgramId` - Wrong program ownership
- `InvalidAccountData` - Invalid permissions
- `InsufficientFunds` - Insufficient balance
- `InvalidArgument` - Invalid parameters

### Recovery Mechanisms
- **State rollback** - Failed operations don't persist changes
- **Clear error messages** - Descriptive error information
- **Validation layers** - Multiple validation checkpoints

This basic processor provides the foundation for vault operations, ensuring secure, efficient, and reliable token management with comprehensive event logging and fee management capabilities.
