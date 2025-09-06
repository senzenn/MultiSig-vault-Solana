# Vault Program Architecture

## Overview

A Solana-based vault program with multisig governance and token management capabilities.

## System Architecture

```mermaid
graph TB
    A[CLI Interface] --> B[Solana Program]
    B --> C[Blockchain State]

    subgraph "CLI Layer"
        A1[Commands]
        A2[Keypair Mgmt]
        A3[Transaction Building]
    end

    subgraph "Program Layer"
        B1[Instruction Processor]
        B2[State Management]
        B3[Security Validation]
    end

    subgraph "Blockchain Layer"
        C1[Account Storage]
        C2[Transaction Execution]
        C3[Event Emission]
    end

    A --> A1
    A --> A2
    A --> A3

    B --> B1
    B --> B2
    B --> B3

    C --> C1
    C --> C2
    C --> C3
```

## Core Components

### 1. CLI Interface (`cli.rs`)
- Command parsing and validation
- Transaction building and signing
- RPC communication with Solana

### 2. Program Logic (`lib.rs`)
- Entry point definition
- Module organization
- Public API exports

### 3. State Management (`state.rs`)
- Vault account structures
- Multisig configurations
- Governance data models

### 4. Instruction Processing (`processor.rs`)
- Route instructions to handlers
- Account validation
- Business logic execution

## Data Flow

```mermaid
sequenceDiagram
    participant U as User
    participant C as CLI
    participant P as Program
    participant B as Blockchain

    U->>C: Command (e.g., deposit)
    C->>C: Build transaction
    C->>B: Send transaction
    B->>P: Execute instruction
    P->>P: Validate & process
    P->>B: Update state
    B->>C: Return result
    C->>U: Display response
```

## Security Model

### Authorization Levels
1. **Single Authority**: Basic owner control
2. **Multisig**: M-of-N signature requirements
3. **Governance**: Token-weighted voting

### Validation Layers
- Account ownership verification
- Signer authorization checks
- Balance and amount validation
- State consistency checks

## Multisig Workflow

```mermaid
stateDiagram-v2
    [*] --> CreateTx: User creates transaction
    CreateTx --> Pending: Store in vault
    Pending --> Approved: Owner approves
    Approved --> ThresholdCheck: Check if threshold met
    ThresholdCheck --> Pending: More approvals needed
    ThresholdCheck --> Executable: Threshold reached
    Executable --> Execute: User executes
    Execute --> [*]: Transaction complete
```

## Key Features

### ✅ Implemented
- CLI command interface
- Keypair generation and management
- Transaction creation and signing
- Multisig proposal system
- Balance checking
- Test transaction creation

### 🚧 Framework Ready
- Vault initialization
- Token deposit/withdraw
- Emergency controls
- Governance system
- Fee management

## Technology Stack

- **Language**: Rust
- **Framework**: Solana Program Library
- **Serialization**: Borsh
- **Testing**: Solana Program Test
- **CLI**: Clap
- **RPC**: Solana Client

## Configuration

### Environment Variables
```bash
RPC_URL=https://devnet.helius-rpc.com/?api-key=YOUR_KEY
PROGRAM_ID=11111111111111111111111111111112
```

### Network Support
- ✅ Solana Devnet (Primary)
- 🚧 Mainnet (Framework ready)
- 🚧 Testnet (Framework ready)

## Future Extensions

- Cross-program invocations
- DeFi protocol integration
- Advanced governance features
- Performance optimizations
- Multi-network support

---

*Built for Solana ecosystem with security and scalability in mind.*
