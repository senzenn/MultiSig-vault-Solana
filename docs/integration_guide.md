# Vault Solana Program - Integration Guide

## Overview

This guide provides comprehensive instructions for integrating with the Vault Solana program. Whether you're building a frontend application, backend service, or integrating with other DeFi protocols, this guide will help you interact with the vault securely and efficiently.

## Prerequisites

### Development Environment
```bash
# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/v1.18.4/install)"

# Install Node.js and npm
# Install Rust and Cargo

# Clone the vault program
git clone <vault-program-repo>
cd vault-program

# Build the program
cargo build-bpf
```

### Dependencies
```json
// package.json for JavaScript/TypeScript integration
{
  "dependencies": {
    "@solana/web3.js": "^1.87.6",
    "@solana/spl-token": "^0.3.8",
    "@project-serum/anchor": "^0.26.0",
    "bn.js": "^5.2.1",
    "borsh": "^0.7.0"
  }
}
```

## Program Setup

### Program ID Configuration
```typescript
// Update with your deployed program ID
export const VAULT_PROGRAM_ID = new PublicKey("VAULT11111111111111111111111111111111111111");

// Update DeFi protocol IDs
export const DEF1_PROTOCOLS = {
  ORCA_WHIRLPOOL: new PublicKey("9bb45b8c3a8e8e4a1b6f8ea97a2b3d5f8c9e4b7d2b8c6e9e1b5f9c2d7a8e4b6f"),
  RAYDIUM_AMM: new PublicKey("9cb45b8c3a8e8e4a1b6f8ea97a2b3d5f8c9e4b7d2b8c6e9e1b5f9c2d7a8e4b70"),
  SABER_PROTOCOL: new PublicKey("9db45b8c3a8e8e4a1b6f8ea97a2b3d5f8c9e4b7d2b8c6e9e1b5f9c2d7a8e4b71"),
  JUPITER_AGGREGATOR: new PublicKey("9eb45b8c3a8e8e4a1b6f8ea97a2b3d5f8c9e4b7d2b8c6e9e1b5f9c2d7a8e4b72"),
};
```

### Connection Setup
```typescript
import { Connection, clusterApiUrl } from '@solana/web3.js';

// Connect to Solana network
const connection = new Connection(clusterApiUrl('mainnet-beta'), 'confirmed');

// For devnet
const devnetConnection = new Connection(clusterApiUrl('devnet'), 'confirmed');
```

## Core Integration Patterns

### Account Derivation
```typescript
import { PublicKey } from '@solana/web3.js';

// Derive vault PDA
export function getVaultPDA(authority: PublicKey, programId: PublicKey): [PublicKey, number] {
  return PublicKey.findProgramAddress(
    [Buffer.from("vault"), authority.toBuffer()],
    programId
  );
}

// Derive associated token account
export function getAssociatedTokenAddress(
  wallet: PublicKey,
  mint: PublicKey
): PublicKey {
  return getAssociatedTokenAddressSync(mint, wallet);
}
```

### Instruction Builders
```typescript
import { TransactionInstruction, PublicKey, SystemProgram } from '@solana/web3.js';
import { TOKEN_PROGRAM_ID } from '@solana/spl-token';

// Vault initialization instruction
export function createInitializeInstruction(
  vaultAccount: PublicKey,
  mintAccount: PublicKey,
  authority: PublicKey,
  vaultTokenAccount: PublicKey,
  tokenProgram: PublicKey,
  associatedTokenProgram: PublicKey,
  systemProgram: PublicKey,
  rentSysvar: PublicKey,
  bump: number
): TransactionInstruction {
  const data = Buffer.alloc(2);
  data.writeUInt8(0, 0); // Initialize instruction
  data.writeUInt8(bump, 1);

  return new TransactionInstruction({
    keys: [
      { pubkey: vaultAccount, isSigner: false, isWritable: true },
      { pubkey: mintAccount, isSigner: false, isWritable: false },
      { pubkey: authority, isSigner: true, isWritable: true },
      { pubkey: vaultTokenAccount, isSigner: false, isWritable: true },
      { pubkey: tokenProgram, isSigner: false, isWritable: false },
      { pubkey: associatedTokenProgram, isSigner: false, isWritable: false },
      { pubkey: systemProgram, isSigner: false, isWritable: false },
      { pubkey: rentSysvar, isSigner: false, isWritable: false },
    ],
    programId: VAULT_PROGRAM_ID,
    data,
  });
}
```

## Basic Operations

### Vault Initialization
```typescript
async function initializeVault(
  connection: Connection,
  payer: Keypair,
  mint: PublicKey
): Promise<PublicKey> {
  // Derive vault PDA
  const [vaultPDA, bump] = getVaultPDA(payer.publicKey, VAULT_PROGRAM_ID);

  // Get associated token account
  const vaultTokenAccount = getAssociatedTokenAddress(vaultPDA, mint);

  // Create instruction
  const instruction = createInitializeInstruction(
    vaultPDA,
    mint,
    payer.publicKey,
    vaultTokenAccount,
    TOKEN_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM_ID,
    SystemProgram.programId,
    SYSVAR_RENT_PUBKEY,
    bump
  );

  // Send transaction
  const transaction = new Transaction().add(instruction);
  const signature = await sendAndConfirmTransaction(connection, transaction, [payer]);

  console.log(`Vault initialized: ${vaultPDA.toBase58()}`);
  return vaultPDA;
}
```

### Token Deposit
```typescript
async function depositTokens(
  connection: Connection,
  vaultAccount: PublicKey,
  userTokenAccount: PublicKey,
  vaultTokenAccount: PublicKey,
  authority: PublicKey,
  amount: number,
  signer: Keypair
): Promise<string> {
  const instruction = createDepositInstruction(
    vaultAccount,
    userTokenAccount,
    vaultTokenAccount,
    authority,
    TOKEN_PROGRAM_ID,
    SYSVAR_CLOCK_PUBKEY,
    amount
  );

  const transaction = new Transaction().add(instruction);
  const signature = await sendAndConfirmTransaction(connection, transaction, [signer]);

  console.log(`Deposited ${amount} tokens`);
  return signature;
}
```

### Token Withdrawal
```typescript
async function withdrawTokens(
  connection: Connection,
  vaultAccount: PublicKey,
  vaultTokenAccount: PublicKey,
  userTokenAccount: PublicKey,
  authority: PublicKey,
  amount: number,
  signer: Keypair
): Promise<string> {
  const instruction = createWithdrawInstruction(
    vaultAccount,
    vaultTokenAccount,
    userTokenAccount,
    authority,
    TOKEN_PROGRAM_ID,
    SYSVAR_CLOCK_PUBKEY,
    amount
  );

  const transaction = new Transaction().add(instruction);
  const signature = await sendAndConfirmTransaction(connection, transaction, [signer]);

  console.log(`Withdrew ${amount} tokens`);
  return signature;
}
```

## Multi-Signature Integration

### Initialize Multi-Signature
```typescript
async function initializeMultiSig(
  connection: Connection,
  vaultAccount: PublicKey,
  authority: PublicKey,
  authorities: PublicKey[],
  threshold: number,
  bump: number,
  signer: Keypair
): Promise<string> {
  const instruction = createInitializeMultiSigInstruction(
    vaultAccount,
    authority,
    SYSVAR_CLOCK_PUBKEY,
    authorities,
    threshold,
    bump
  );

  const transaction = new Transaction().add(instruction);
  const signature = await sendAndConfirmTransaction(connection, transaction, [signer]);

  console.log(`Multi-sig initialized with ${authorities.length} authorities`);
  return signature;
}
```

### Create Proposal
```typescript
async function createProposal(
  connection: Connection,
  vaultAccount: PublicKey,
  proposer: PublicKey,
  instruction: VaultInstruction,
  signer: Keypair
): Promise<number> {
  const proposalInstruction = createCreateProposalInstruction(
    vaultAccount,
    proposer,
    SYSVAR_CLOCK_PUBKEY,
    instruction
  );

  const transaction = new Transaction().add(proposalInstruction);
  const signature = await sendAndConfirmTransaction(connection, transaction, [signer]);

  // Extract proposal ID from transaction logs or query vault state
  const proposalId = await getLatestProposalId(connection, vaultAccount);

  console.log(`Created proposal ${proposalId}`);
  return proposalId;
}
```

### Approve and Execute Proposal
```typescript
async function approveAndExecuteProposal(
  connection: Connection,
  vaultAccount: PublicKey,
  proposalId: number,
  approvers: Keypair[]
): Promise<string> {
  const signatures: string[] = [];

  // Each approver approves
  for (const approver of approvers) {
    const approveInstruction = createApproveProposalInstruction(
      vaultAccount,
      approver.publicKey,
      proposalId
    );

    const transaction = new Transaction().add(approveInstruction);
    const signature = await sendAndConfirmTransaction(connection, transaction, [approver]);
    signatures.push(signature);
  }

  // Execute proposal (can be done by any authorized party)
  const executeInstruction = createExecuteProposalInstruction(
    vaultAccount,
    approvers[0].publicKey,
    proposalId
  );

  const executeTransaction = new Transaction().add(executeInstruction);
  const executeSignature = await sendAndConfirmTransaction(
    connection,
    executeTransaction,
    [approvers[0]]
  );

  console.log(`Proposal ${proposalId} executed`);
  return executeSignature;
}
```

## Governance Integration

### Initialize Governance
```typescript
async function initializeGovernance(
  connection: Connection,
  vaultAccount: PublicKey,
  authority: PublicKey,
  votingTokenMint: PublicKey,
  config: GovernanceConfig,
  signer: Keypair
): Promise<string> {
  const instruction = createInitializeGovernanceInstruction(
    vaultAccount,
    authority,
    SYSVAR_CLOCK_PUBKEY,
    votingTokenMint,
    config.quorumThreshold,
    config.proposalThreshold,
    config.votingPeriod,
    config.timelockDelay,
    config.executionThreshold
  );

  const transaction = new Transaction().add(instruction);
  const signature = await sendAndConfirmTransaction(connection, transaction, [signer]);

  console.log('Governance initialized');
  return signature;
}
```

### Create Governance Proposal
```typescript
async function createGovernanceProposal(
  connection: Connection,
  vaultAccount: PublicKey,
  proposer: PublicKey,
  voterTokenAccount: PublicKey,
  title: string,
  description: string,
  instructions: GovernanceInstruction[],
  signer: Keypair
): Promise<number> {
  const instruction = createCreateGovernanceProposalInstruction(
    vaultAccount,
    proposer,
    voterTokenAccount,
    SYSVAR_CLOCK_PUBKEY,
    title,
    description,
    instructions
  );

  const transaction = new Transaction().add(instruction);
  const signature = await sendAndConfirmTransaction(connection, transaction, [signer]);

  const proposalId = await getLatestGovernanceProposalId(connection, vaultAccount);

  console.log(`Governance proposal ${proposalId} created`);
  return proposalId;
}
```

### Cast Vote
```typescript
async function castVote(
  connection: Connection,
  vaultAccount: PublicKey,
  voter: PublicKey,
  voterTokenAccount: PublicKey,
  proposalId: number,
  voteType: VoteType,
  signer: Keypair
): Promise<string> {
  const instruction = createCastVoteInstruction(
    vaultAccount,
    voter,
    voterTokenAccount,
    SYSVAR_CLOCK_PUBKEY,
    proposalId,
    voteType
  );

  const transaction = new Transaction().add(instruction);
  const signature = await sendAndConfirmTransaction(connection, transaction, [signer]);

  console.log(`Vote cast for proposal ${proposalId}`);
  return signature;
}
```

## DeFi Integration

### Set Yield Strategy
```typescript
async function setYieldStrategy(
  connection: Connection,
  vaultAccount: PublicKey,
  authority: PublicKey,
  tokenMint: PublicKey,
  strategyProgram: PublicKey,
  signer: Keypair
): Promise<string> {
  const instruction = createSetYieldStrategyInstruction(
    vaultAccount,
    authority,
    tokenMint,
    strategyProgram
  );

  const transaction = new Transaction().add(instruction);
  const signature = await sendAndConfirmTransaction(connection, transaction, [signer]);

  console.log(`Yield strategy set for ${tokenMint.toBase58()}`);
  return signature;
}
```

### Jupiter Token Swap
```typescript
async function performJupiterSwap(
  connection: Connection,
  vaultAccount: PublicKey,
  inputMint: PublicKey,
  outputMint: PublicKey,
  amount: number,
  slippageBps: number,
  authority: PublicKey,
  signer: Keypair
): Promise<string> {
  // Get swap instructions from Jupiter API
  const { swapInstruction } = await getJupiterSwapInstructions({
    inputMint: inputMint.toBase58(),
    outputMint: outputMint.toBase58(),
    amount,
    slippageBps,
    userPublicKey: vaultAccount.toBase58(),
  });

  // Create vault swap instruction
  const vaultSwapInstruction = createJupiterSwapInstruction(
    vaultAccount,
    authority,
    SYSVAR_CLOCK_PUBKEY,
    inputMint,
    outputMint,
    amount,
    slippageBps
  );

  // Combine instructions
  const transaction = new Transaction()
    .add(vaultSwapInstruction)
    .add(swapInstruction);

  const signature = await sendAndConfirmTransaction(connection, transaction, [signer]);

  console.log(`Swapped ${amount} tokens`);
  return signature;
}
```

## Advanced Integration Patterns

### Batch Operations
```typescript
async function batchDepositWithdraw(
  connection: Connection,
  vaultAccount: PublicKey,
  operations: BatchOperation[],
  signer: Keypair
): Promise<string[]> {
  const transaction = new Transaction();

  for (const op of operations) {
    if (op.type === 'deposit') {
      const instruction = createDepositInstruction(/* ... */);
      transaction.add(instruction);
    } else if (op.type === 'withdraw') {
      const instruction = createWithdrawInstruction(/* ... */);
      transaction.add(instruction);
    }
  }

  const signature = await sendAndConfirmTransaction(connection, transaction, [signer]);
  return [signature];
}
```

### Event Monitoring
```typescript
function setupEventListeners(connection: Connection, vaultAccount: PublicKey) {
  // Listen for deposit events
  connection.onLogs(vaultAccount, (logs, context) => {
    for (const log of logs.logs) {
      if (log.includes('EVENT:')) {
        const eventData = JSON.parse(log.replace('EVENT: ', ''));
        handleVaultEvent(eventData);
      }
    }
  });
}

function handleVaultEvent(event: any) {
  switch (event.event_type) {
    case 'deposit':
      console.log(`Deposit: ${event.amount} tokens by ${event.user}`);
      break;
    case 'withdraw':
      console.log(`Withdraw: ${event.amount} tokens by ${event.user}`);
      break;
    case 'proposal_created':
      console.log(`Proposal ${event.proposal_id} created`);
      break;
  }
}
```

### State Monitoring
```typescript
async function monitorVaultState(connection: Connection, vaultAccount: PublicKey) {
  // Fetch vault state
  const vaultInfo = await connection.getAccountInfo(vaultAccount);
  if (!vaultInfo) return null;

  // Deserialize vault state (requires borsh schema)
  const vaultState = deserializeVaultState(vaultInfo.data);

  return {
    authority: vaultState.authority,
    paused: vaultState.paused,
    totalValueLocked: vaultState.totalValueLocked,
    governanceProposals: vaultState.governanceProposals.length,
    // ... other state fields
  };
}
```

## Error Handling

### Comprehensive Error Handling
```typescript
async function safeExecuteInstruction(
  connection: Connection,
  instruction: TransactionInstruction,
  signers: Keypair[]
): Promise<{ success: boolean; signature?: string; error?: string }> {
  try {
    const transaction = new Transaction().add(instruction);
    const signature = await sendAndConfirmTransaction(connection, transaction, signers);

    return { success: true, signature };
  } catch (error) {
    console.error('Transaction failed:', error);

    // Parse program errors
    if (error.message.includes('0x')) {
      const programError = parseProgramError(error);
      return { success: false, error: programError };
    }

    return { success: false, error: error.message };
  }
}
```

### Custom Error Handling
```typescript
function parseProgramError(error: any): string {
  const errorCode = error.message.match(/0x([0-9a-f]+)/)?.[1];
  if (!errorCode) return 'Unknown error';

  const code = parseInt(errorCode, 16);

  switch (code) {
    case 1: return 'Unauthorized access';
    case 2: return 'Invalid signer';
    case 3: return 'Invalid amount';
    case 4: return 'Insufficient balance';
    // ... map other error codes
    default: return `Program error: ${code}`;
  }
}
```

## Testing Integration

### Unit Tests for Integration
```typescript
describe('Vault Integration', () => {
  let connection: Connection;
  let payer: Keypair;
  let vaultAccount: PublicKey;

  beforeAll(async () => {
    connection = new Connection(clusterApiUrl('devnet'), 'confirmed');
    payer = Keypair.generate();
    // Airdrop SOL for testing
    await connection.confirmTransaction(
      await connection.requestAirdrop(payer.publicKey, LAMPORTS_PER_SOL)
    );
  });

  test('should initialize vault successfully', async () => {
    const mint = await createMint(connection, payer, payer.publicKey, null, 9);

    vaultAccount = await initializeVault(connection, payer, mint);

    const vaultState = await monitorVaultState(connection, vaultAccount);
    expect(vaultState).toBeDefined();
    expect(vaultState?.authority.equals(payer.publicKey)).toBe(true);
  });

  test('should deposit tokens successfully', async () => {
    const userTokenAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      payer,
      mint,
      payer.publicKey
    );

    // Mint tokens to user
    await mintTo(connection, payer, mint, userTokenAccount.address, payer, 1000000);

    // Deposit tokens
    await depositTokens(
      connection,
      vaultAccount,
      userTokenAccount.address,
      getAssociatedTokenAddress(vaultAccount, mint),
      payer.publicKey,
      100000,
      payer
    );

    // Verify balance
    const vaultState = await monitorVaultState(connection, vaultAccount);
    expect(vaultState?.totalValueLocked).toBe(100000);
  });
});
```

## Production Considerations

### Connection Optimization
```typescript
// Use connection pooling for high throughput
const connection = new Connection(clusterApiUrl('mainnet-beta'), {
  commitment: 'confirmed',
  confirmTransactionInitialTimeout: 60000,
});

// Implement retry logic
async function sendTransactionWithRetry(
  connection: Connection,
  transaction: Transaction,
  signers: Keypair[],
  maxRetries: number = 3
): Promise<string> {
  for (let i = 0; i < maxRetries; i++) {
    try {
      const signature = await sendAndConfirmTransaction(connection, transaction, signers);
      return signature;
    } catch (error) {
      if (i === maxRetries - 1) throw error;
      await new Promise(resolve => setTimeout(resolve, 1000 * (i + 1)));
    }
  }
}
```

### Gas Optimization
```typescript
// Batch operations to reduce fees
async function batchOperations(
  connection: Connection,
  operations: TransactionInstruction[],
  signers: Keypair[]
): Promise<string> {
  const transaction = new Transaction();

  // Add all operations to single transaction
  for (const instruction of operations) {
    transaction.add(instruction);
  }

  return await sendAndConfirmTransaction(connection, transaction, signers);
}
```

### Monitoring and Alerting
```typescript
// Implement comprehensive monitoring
function setupMonitoring(connection: Connection, vaultAccount: PublicKey) {
  // Monitor transaction success rate
  // Monitor gas usage
  // Monitor error rates
  // Set up alerts for critical events

  connection.onLogs(vaultAccount, (logs) => {
    for (const log of logs.logs) {
      if (log.includes('PROGRAM ERROR')) {
        alert(`Program error detected: ${log}`);
      }
    }
  });
}
```

This integration guide provides everything needed to successfully integrate with the Vault Solana program, from basic operations to advanced DeFi functionality and governance features.
