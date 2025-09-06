# Vault Solana Program - Instructions Documentation (`instruction.rs`)

## Overview

The `instruction.rs` file defines all the instruction types and data structures that users can invoke on the Vault Solana program. These instructions represent the complete API surface for interacting with the vault.

## Instruction Architecture

### Main Instruction Enum

The core instruction type that handles all program operations:

```rust
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum VaultInstruction {
    // Core vault operations
    Initialize { bump: u8 },
    Deposit { amount: u64 },
    Withdraw { amount: u64 },

    // Multi-signature operations
    InitializeMultiSig { authorities: Vec<Pubkey>, threshold: u8, bump: u8 },
    CreateProposal { instruction: Box<VaultInstruction> },
    ApproveProposal { proposal_id: u64 },
    ExecuteProposal { proposal_id: u64 },
    RejectProposal { proposal_id: u64 },

    // Emergency operations
    PauseVault,
    UnpauseVault,
    EmergencyWithdraw { token_mint: Pubkey, amount: u64 },

    // Multi-token operations
    AddSupportedToken { mint: Pubkey, bump: u8 },
    DepositMultiToken { mint: Pubkey, amount: u64 },
    WithdrawMultiToken { mint: Pubkey, amount: u64 },

    // Time-lock operations
    CreateTimeLock {
        beneficiary: Pubkey,
        amount: u64,
        duration: i64,
        cliff_duration: Option<i64>,
        is_linear: bool,
    },
    ClaimTimeLock { time_lock_index: usize },
    CancelTimeLock { time_lock_index: usize },

    // Yield farming operations
    SetYieldStrategy { token_mint: Pubkey, strategy_program: Pubkey },
    HarvestYield { token_mint: Pubkey },
    CompoundYield { token_mint: Pubkey },

    // Jupiter DEX operations
    JupiterSwap {
        input_mint: Pubkey,
        output_mint: Pubkey,
        amount: u64,
        slippage_bps: u16,
    },
    JupiterRoute {
        input_mint: Pubkey,
        output_mint: Pubkey,
        amount: u64,
        route: Vec<u8>,
    },

    // Fee management operations
    UpdateFeeConfig {
        deposit_fee_bps: u16,
        withdrawal_fee_bps: u16,
        fee_recipient: Pubkey,
    },
    CollectFees,

    // Administrative operations
    TransferAuthority { new_authority: Pubkey },
    UpdateEmergencyAdmin { new_admin: Pubkey },

    // Governance operations
    InitializeGovernance {
        voting_token_mint: Pubkey,
        quorum_threshold: u16,
        proposal_threshold: u64,
        voting_period: i64,
        timelock_delay: i64,
        execution_threshold: u16,
    },
    CreateGovernanceProposal {
        title: String,
        description: String,
        instructions: Vec<GovernanceInstruction>,
    },
    CastVote { proposal_id: u64, vote_type: VoteType },
    QueueProposal { proposal_id: u64 },
    ExecuteGovernanceProposal { proposal_id: u64 },
    CancelGovernanceProposal { proposal_id: u64 },
    UpdateGovernanceConfig {
        quorum_threshold: u16,
        proposal_threshold: u64,
        voting_period: i64,
        timelock_delay: i64,
        execution_threshold: u16,
    },
}
```

## Instruction Categories

### 1. Core Vault Operations

#### Initialize
Creates a new vault instance with basic configuration.

**Parameters:**
- `bump: u8` - PDA bump seed for vault account

**Required Accounts:**
- Vault account (PDA)
- Token mint account
- Authority account (signer)
- Vault token account (ATA)
- Token program
- Associated token program
- System program
- Rent sysvar

#### Deposit
Deposits tokens into the vault.

**Parameters:**
- `amount: u64` - Amount of tokens to deposit

**Required Accounts:**
- Vault account
- User token account
- Vault token account
- Authority account (signer)
- Token program
- Clock sysvar

#### Withdraw
Withdraws tokens from the vault.

**Parameters:**
- `amount: u64` - Amount of tokens to withdraw

**Required Accounts:**
- Vault account
- Vault token account
- User token account
- Authority account (signer)
- Token program
- Clock sysvar

### 2. Multi-Signature Operations

#### InitializeMultiSig
Sets up multi-signature authorization for the vault.

**Parameters:**
- `authorities: Vec<Pubkey>` - List of authorized signers
- `threshold: u8` - Minimum signatures required
- `bump: u8` - PDA bump seed

**Required Accounts:**
- Vault account
- Authority account (signer)

#### CreateProposal
Creates a new multi-signature proposal.

**Parameters:**
- `instruction: Box<VaultInstruction>` - Instruction to be executed

**Required Accounts:**
- Vault account
- Proposer account (signer)

#### ApproveProposal
Approves a pending multi-signature proposal.

**Parameters:**
- `proposal_id: u64` - ID of proposal to approve

**Required Accounts:**
- Vault account
- Approver account (signer)

#### ExecuteProposal
Executes an approved multi-signature proposal.

**Parameters:**
- `proposal_id: u64` - ID of proposal to execute

**Required Accounts:**
- Vault account
- Executor account (signer)
- Additional accounts based on the proposal instruction

#### RejectProposal
Rejects a pending multi-signature proposal.

**Parameters:**
- `proposal_id: u64` - ID of proposal to reject

**Required Accounts:**
- Vault account
- Rejector account (signer, must be original proposer)

### 3. Emergency Operations

#### PauseVault
Pauses all vault operations for emergency situations.

**Required Accounts:**
- Vault account
- Authority account (signer, must be emergency admin)

#### UnpauseVault
Resumes vault operations after emergency pause.

**Required Accounts:**
- Vault account
- Authority account (signer, must be emergency admin)

#### EmergencyWithdraw
Allows emergency withdrawal of specific tokens.

**Parameters:**
- `token_mint: Pubkey` - Token mint to withdraw
- `amount: u64` - Amount to withdraw

**Required Accounts:**
- Vault account
- Vault token account
- User token account
- Authority account (signer, must be emergency admin)
- Token program

### 4. Multi-Token Operations

#### AddSupportedToken
Adds support for a new token type.

**Parameters:**
- `mint: Pubkey` - Token mint to add
- `bump: u8` - PDA bump seed for token account

**Required Accounts:**
- Vault account
- Token mint account
- Vault token account (ATA)
- Authority account (signer)
- Token program
- Associated token program
- System program
- Rent sysvar

#### DepositMultiToken
Deposits a specific token type.

**Parameters:**
- `mint: Pubkey` - Token mint to deposit
- `amount: u64` - Amount to deposit

**Required Accounts:**
- Vault account
- User token account
- Vault token account
- Authority account (signer)
- Token program
- Clock sysvar

#### WithdrawMultiToken
Withdraws a specific token type.

**Parameters:**
- `mint: Pubkey` - Token mint to withdraw
- `amount: u64` - Amount to withdraw

**Required Accounts:**
- Vault account
- Vault token account
- User token account
- Authority account (signer)
- Token program
- Clock sysvar

### 5. Time-Lock Operations

#### CreateTimeLock
Creates a time-locked deposit with vesting schedule.

**Parameters:**
- `beneficiary: Pubkey` - Recipient of locked tokens
- `amount: u64` - Amount to lock
- `duration: i64` - Total lock duration in seconds
- `cliff_duration: Option<i64>` - Cliff period in seconds
- `is_linear: bool` - Linear vs cliff-based release

**Required Accounts:**
- Vault account
- Beneficiary token account
- Vault token account
- Authority account (signer)
- Token program
- Clock sysvar

#### ClaimTimeLock
Claims released tokens from a time lock.

**Parameters:**
- `time_lock_index: usize` - Index of time lock to claim

**Required Accounts:**
- Vault account
- Beneficiary token account
- Vault token account
- Beneficiary account (signer)
- Token program
- Clock sysvar

#### CancelTimeLock
Cancels a time lock (authority only).

**Parameters:**
- `time_lock_index: usize` - Index of time lock to cancel

**Required Accounts:**
- Vault account
- Authority account (signer)
- Clock sysvar

### 6. Yield Farming Operations

#### SetYieldStrategy
Sets a yield farming strategy for a token.

**Parameters:**
- `token_mint: Pubkey` - Token mint for strategy
- `strategy_program: Pubkey` - Yield farming program ID

**Required Accounts:**
- Vault account
- Strategy program account
- Authority account (signer)

#### HarvestYield
Harvests yield from a yield farming strategy.

**Parameters:**
- `token_mint: Pubkey` - Token mint to harvest

**Required Accounts:**
- Vault account
- Vault token account
- Reward token account
- Strategy account
- Authority account (signer)
- Strategy program
- Token program

#### CompoundYield
Compounds harvested yield back into strategy.

**Parameters:**
- `token_mint: Pubkey` - Token mint to compound

**Required Accounts:**
- Vault account
- Vault token account
- Strategy account
- Authority account (signer)
- Strategy program
- Token program

### 7. Jupiter DEX Operations

#### JupiterSwap
Performs a token swap via Jupiter aggregator.

**Parameters:**
- `input_mint: Pubkey` - Input token mint
- `output_mint: Pubkey` - Output token mint
- `amount: u64` - Amount to swap
- `slippage_bps: u16` - Slippage tolerance in basis points

**Required Accounts:**
- Vault account
- Input token account
- Output token account
- Authority account (signer)
- Jupiter program
- Token program
- Additional Jupiter-specific accounts

#### JupiterRoute
Performs a multi-hop token swap via Jupiter.

**Parameters:**
- `input_mint: Pubkey` - Input token mint
- `output_mint: Pubkey` - Output token mint
- `amount: u64` - Amount to swap
- `route: Vec<u8>` - Encoded swap route

**Required Accounts:**
- Similar to JupiterSwap with additional route accounts

### 8. Fee Management Operations

#### UpdateFeeConfig
Updates fee configuration for the vault.

**Parameters:**
- `deposit_fee_bps: u16` - Deposit fee in basis points
- `withdrawal_fee_bps: u16` - Withdrawal fee in basis points
- `fee_recipient: Pubkey` - Address to receive fees

**Required Accounts:**
- Vault account
- Authority account (signer)

#### CollectFees
Collects accumulated fees to fee recipient.

**Required Accounts:**
- Vault account
- Fee recipient token account
- Vault token account
- Authority account (signer)
- Token program

### 9. Administrative Operations

#### TransferAuthority
Transfers vault authority to new address.

**Parameters:**
- `new_authority: Pubkey` - New authority address

**Required Accounts:**
- Vault account
- Current authority account (signer)

#### UpdateEmergencyAdmin
Updates emergency administrator address.

**Parameters:**
- `new_admin: Pubkey` - New emergency admin address

**Required Accounts:**
- Vault account
- Current authority account (signer)

### 10. Governance Operations

#### InitializeGovernance
Initializes the governance system for the vault.

**Parameters:**
- `voting_token_mint: Pubkey` - Token used for voting
- `quorum_threshold: u16` - Minimum participation percentage
- `proposal_threshold: u64` - Minimum tokens to create proposal
- `voting_period: i64` - Voting duration in seconds
- `timelock_delay: i64` - Delay before execution
- `execution_threshold: u16` - Minimum approval percentage

**Required Accounts:**
- Vault account
- Voting token mint
- Authority account (signer)

#### CreateGovernanceProposal
Creates a new governance proposal.

**Parameters:**
- `title: String` - Proposal title
- `description: String` - Proposal description
- `instructions: Vec<GovernanceInstruction>` - Instructions to execute

**Required Accounts:**
- Vault account
- Proposer account (signer)
- Proposer token account

#### CastVote
Casts a vote on a governance proposal.

**Parameters:**
- `proposal_id: u64` - Proposal to vote on
- `vote_type: VoteType` - Type of vote (For/Against/Abstain)

**Required Accounts:**
- Vault account
- Voter account (signer)
- Voter token account

#### QueueProposal
Queues an approved proposal for execution.

**Parameters:**
- `proposal_id: u64` - Proposal to queue

**Required Accounts:**
- Vault account
- Authority account (signer)

#### ExecuteGovernanceProposal
Executes a queued governance proposal.

**Parameters:**
- `proposal_id: u64` - Proposal to execute

**Required Accounts:**
- Vault account
- Executor account (signer)
- Additional accounts based on proposal instructions

#### CancelGovernanceProposal
Cancels a pending governance proposal.

**Parameters:**
- `proposal_id: u64` - Proposal to cancel

**Required Accounts:**
- Vault account
- Canceler account (signer, must be proposer)

#### UpdateGovernanceConfig
Updates governance system configuration.

**Parameters:**
- Similar to InitializeGovernance

**Required Accounts:**
- Vault account
- Authority account (signer)

## Governance-Specific Types

### GovernanceInstruction
Represents instructions that can be executed via governance:

```rust
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct GovernanceInstruction {
    pub program_id: Pubkey,
    pub accounts: Vec<GovernanceAccountMeta>,
    pub data: Vec<u8>,
}
```

### GovernanceAccountMeta
Account metadata for governance instructions:

```rust
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct GovernanceAccountMeta {
    pub pubkey: Pubkey,
    pub is_signer: bool,
    pub is_writable: bool,
}
```

### VoteType
Enumeration of possible vote types:

```rust
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, serde::Serialize)]
pub enum VoteType {
    For,        // Vote in favor
    Against,    // Vote against
    Abstain,    // Abstain from voting
}
```

## Instruction Processing Flow

### 1. Instruction Deserialization
```rust
let instruction = VaultInstruction::try_from_slice(instruction_data)
    .map_err(|_| ProgramError::InvalidInstructionData)?;
```

### 2. Account Validation
- Verify account ownership
- Check signer requirements
- Validate account types and permissions

### 3. State Validation
- Check vault pause status
- Verify authorization (single or multi-sig)
- Validate amounts and balances

### 4. Business Logic Execution
- Update vault state
- Perform token transfers
- Create/update proposals
- Emit events

### 5. State Persistence
- Serialize updated vault state
- Update account data on-chain

## Error Handling

Instructions can fail with various `ProgramError` types:
- `InvalidInstructionData` - Malformed instruction data
- `MissingRequiredSignature` - Missing signer
- `IncorrectProgramId` - Wrong program ownership
- `InvalidAccountData` - Invalid account state
- `InsufficientFunds` - Insufficient balance
- `InvalidArgument` - Invalid parameter values

## Security Considerations

### Authorization
- **Signer verification**: All operations require proper signatures
- **Authority validation**: Check against vault authority or multi-sig
- **Permission levels**: Different operations have different permission requirements

### Input Validation
- **Amount validation**: Prevent overflow/underflow
- **Account validation**: Verify account ownership and types
- **Parameter bounds**: Check reasonable limits on all parameters

### State Consistency
- **Atomic operations**: All state changes happen atomically
- **Rollback protection**: Failed operations don't leave partial state
- **Balance invariants**: Maintain correct balance relationships

This instruction set provides a comprehensive API for a sophisticated DeFi vault with multi-signature governance, yield farming, and advanced token management capabilities.
