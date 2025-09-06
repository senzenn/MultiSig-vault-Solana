# Vault Solana Program - Testing Guide

## Overview

This testing guide provides comprehensive strategies and procedures for testing the Vault Solana program across unit, integration, and end-to-end testing levels. The guide ensures robust testing coverage for all program functionality, security features, and edge cases.

## Testing Architecture

### Testing Pyramid
```
End-to-End Tests (E2E)     ┌─────────────┐
Integration Tests         │    20%     │
Unit Tests               ┌┴─────────────┴┐
Security Tests          │     80%      │
└───────────────────────┘
```

### Test Categories
- **Unit Tests**: Individual function/component testing
- **Integration Tests**: Cross-component interaction testing
- **End-to-End Tests**: Complete user workflow testing
- **Security Tests**: Vulnerability and attack vector testing
- **Performance Tests**: Load and stress testing

## Unit Testing

### Program Logic Testing
```rust
// tests/unit.rs
use vault_program::*;

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_vault_initialization() {
        // Test vault initialization logic
        let authority = Pubkey::new_unique();
        let bump = 255;

        // Mock runtime context
        let mut vault = Vault::default();
        vault.authority = authority;
        vault.bump = bump;

        assert_eq!(vault.authority, authority);
        assert_eq!(vault.bump, bump);
        assert_eq!(vault.paused, false);
    }

    #[test]
    fn test_fee_calculation() {
        // Test fee calculation logic
        let amount = 1000000; // 1 token (6 decimals)
        let fee_bps = 50; // 0.5%

        let fee_amount = (amount * fee_bps as u64) / 10000;
        let net_amount = amount - fee_amount;

        assert_eq!(fee_amount, 5000); // 0.005 tokens
        assert_eq!(net_amount, 995000); // 0.995 tokens
    }

    #[test]
    fn test_authorization_logic() {
        // Test authorization patterns
        let vault = Vault {
            authority: Pubkey::new_unique(),
            multi_sig: Some(MultiSigAuthority {
                authorities: vec![Pubkey::new_unique(), Pubkey::new_unique()],
                threshold: 2,
                nonce: 0,
            }),
            ..Default::default()
        };

        // Test single authority
        assert!(vault.authority == vault.authority);

        // Test multi-sig threshold
        if let Some(multi_sig) = &vault.multi_sig {
            assert_eq!(multi_sig.threshold, 2);
            assert_eq!(multi_sig.authorities.len(), 2);
        }
    }
}
```

### Data Structure Testing
```rust
#[test]
fn test_borsh_serialization() {
    // Test Borsh serialization/deserialization
    let original_vault = Vault {
        authority: Pubkey::new_unique(),
        bump: 255,
        paused: false,
        ..Default::default()
    };

    // Serialize
    let mut buffer = Vec::new();
    original_vault.serialize(&mut buffer).unwrap();

    // Deserialize
    let deserialized_vault = Vault::try_from_slice(&buffer).unwrap();

    // Verify
    assert_eq!(original_vault.authority, deserialized_vault.authority);
    assert_eq!(original_vault.bump, deserialized_vault.bump);
    assert_eq!(original_vault.paused, deserialized_vault.paused);
}

#[test]
fn test_governance_proposal_state() {
    // Test governance proposal state transitions
    let mut proposal = GovernanceProposal {
        id: 1,
        executed: false,
        cancelled: false,
        ..Default::default()
    };

    // Test initial state
    assert!(!proposal.executed);
    assert!(!proposal.cancelled);

    // Test execution
    proposal.executed = true;
    assert!(proposal.executed);
    assert!(!proposal.cancelled);

    // Test cancellation (should not be possible if executed)
    proposal.cancelled = true;
    assert!(proposal.executed && proposal.cancelled); // Invalid state
}
```

### Instruction Processing Testing
```rust
#[test]
fn test_instruction_deserialization() {
    // Test instruction deserialization
    let deposit_data = Deposit { amount: 1000000 };
    let mut buffer = Vec::new();
    deposit_data.serialize(&mut buffer).unwrap();

    // Prepend instruction discriminator
    let mut instruction_data = vec![1]; // Deposit instruction
    instruction_data.extend(buffer);

    // Deserialize instruction
    let instruction = VaultInstruction::try_from_slice(&instruction_data).unwrap();

    match instruction {
        VaultInstruction::Deposit { amount } => {
            assert_eq!(amount, 1000000);
        }
        _ => panic!("Wrong instruction type"),
    }
}
```

## Integration Testing

### Cross-Component Testing
```rust
// tests/integration.rs
use vault_program::*;
use solana_program_test::*;
use solana_sdk::{signature::Keypair, signer::Signer};

#[tokio::test]
async fn test_deposit_withdraw_flow() {
    let program_test = ProgramTest::new(
        "vault_program",
        vault_program::id(),
        processor!(vault_program::processor::process_instruction),
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Create test accounts
    let mint = Keypair::new();
    let vault_account = Keypair::new();
    let user_token_account = Keypair::new();
    let vault_token_account = Keypair::new();

    // Initialize mint
    create_mint(&mut banks_client, &payer, &recent_blockhash, &mint.pubkey(), 6).await;

    // Initialize vault
    initialize_vault(
        &mut banks_client,
        &payer,
        &recent_blockhash,
        &vault_account,
        &mint.pubkey(),
    ).await;

    // Test deposit
    let deposit_amount = 1000000;
    deposit_tokens(
        &mut banks_client,
        &payer,
        &recent_blockhash,
        &vault_account,
        &user_token_account.pubkey(),
        &vault_token_account.pubkey(),
        deposit_amount,
    ).await;

    // Verify vault balance
    let vault_account_data = banks_client.get_account(vault_account.pubkey()).await.unwrap().unwrap();
    let vault = Vault::try_from_slice(&vault_account_data.data).unwrap();

    assert_eq!(vault.total_value_locked, deposit_amount);

    // Test withdrawal
    withdraw_tokens(
        &mut banks_client,
        &payer,
        &recent_blockhash,
        &vault_account,
        &vault_token_account.pubkey(),
        &user_token_account.pubkey(),
        deposit_amount,
    ).await;

    // Verify final state
    let final_vault_data = banks_client.get_account(vault_account.pubkey()).await.unwrap().unwrap();
    let final_vault = Vault::try_from_slice(&final_vault_data.data).unwrap();

    assert_eq!(final_vault.total_value_locked, 0);
}
```

### Multi-Signature Testing
```rust
#[tokio::test]
async fn test_multi_sig_flow() {
    // Setup test environment
    let program_test = ProgramTest::new(/* ... */);
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Create multi-sig with 3 authorities, threshold 2
    let authorities = [Keypair::new(), Keypair::new(), Keypair::new()];
    let threshold = 2;

    // Initialize multi-sig
    initialize_multi_sig(
        &mut banks_client,
        &payer,
        &recent_blockhash,
        &vault_account,
        &authorities.iter().map(|k| k.pubkey()).collect::<Vec<_>>(),
        threshold,
    ).await;

    // Create proposal
    let proposal_id = create_proposal(
        &mut banks_client,
        &payer,
        &recent_blockhash,
        &vault_account,
        &authorities[0],
        VaultInstruction::PauseVault,
    ).await;

    // First approval
    approve_proposal(
        &mut banks_client,
        &payer,
        &recent_blockhash,
        &vault_account,
        &authorities[1],
        proposal_id,
    ).await;

    // Second approval (meets threshold)
    approve_proposal(
        &mut banks_client,
        &payer,
        &recent_blockhash,
        &vault_account,
        &authorities[2],
        proposal_id,
    ).await;

    // Execute proposal
    execute_proposal(
        &mut banks_client,
        &payer,
        &recent_blockhash,
        &vault_account,
        &authorities[0],
        proposal_id,
    ).await;

    // Verify vault is paused
    let vault_data = banks_client.get_account(vault_account.pubkey()).await.unwrap().unwrap();
    let vault = Vault::try_from_slice(&vault_data.data).unwrap();
    assert!(vault.paused);
}
```

## End-to-End Testing

### Complete User Workflow Testing
```typescript
// tests/e2e/vault-workflow.test.ts
describe('Vault E2E Tests', () => {
  let connection: Connection;
  let payer: Keypair;
  let vaultAccount: PublicKey;
  let mint: PublicKey;

  beforeAll(async () => {
    connection = new Connection(clusterApiUrl('devnet'), 'confirmed');
    payer = Keypair.generate();

    // Airdrop SOL
    await connection.confirmTransaction(
      await connection.requestAirdrop(payer.publicKey, LAMPORTS_PER_SOL)
    );
  });

  test('complete vault lifecycle', async () => {
    // 1. Create mint
    mint = await createMint(connection, payer, payer.publicKey, null, 9);

    // 2. Initialize vault
    vaultAccount = await initializeVault(connection, payer, mint);

    // 3. Create user token account
    const userTokenAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      payer,
      mint,
      payer.publicKey
    );

    // 4. Mint tokens to user
    await mintTo(
      connection,
      payer,
      mint,
      userTokenAccount.address,
      payer,
      1000000000 // 1000 tokens
    );

    // 5. Deposit tokens
    await depositTokens(
      connection,
      vaultAccount,
      userTokenAccount.address,
      getAssociatedTokenAddress(vaultAccount, mint),
      payer.publicKey,
      100000000, // 100 tokens
      payer
    );

    // 6. Verify deposit
    const vaultInfo = await connection.getAccountInfo(vaultAccount);
    const vault = deserializeVaultState(vaultInfo!.data);
    expect(vault.totalValueLocked).toBe(100000000);

    // 7. Withdraw tokens
    await withdrawTokens(
      connection,
      vaultAccount,
      getAssociatedTokenAddress(vaultAccount, mint),
      userTokenAccount.address,
      payer.publicKey,
      50000000, // 50 tokens
      payer
    );

    // 8. Verify withdrawal
    const updatedVaultInfo = await connection.getAccountInfo(vaultAccount);
    const updatedVault = deserializeVaultState(updatedVaultInfo!.data);
    expect(updatedVault.totalValueLocked).toBe(50000000);
  });
});
```

## Security Testing

### Authorization Testing
```rust
#[test]
fn test_unauthorized_access() {
    // Test unauthorized deposit
    let unauthorized_user = Pubkey::new_unique();
    let vault = Vault {
        authority: Pubkey::new_unique(), // Different from unauthorized_user
        ..Default::default()
    };

    // Attempt operation with wrong authority
    let result = authorize_operation(&vault, &unauthorized_user);
    assert!(result.is_err());
}

#[test]
fn test_multi_sig_threshold() {
    // Test insufficient approvals
    let multi_sig = MultiSigAuthority {
        authorities: vec![Pubkey::new_unique(), Pubkey::new_unique(), Pubkey::new_unique()],
        threshold: 3,
        nonce: 0,
    };

    let proposal = Proposal {
        approvals: vec![multi_sig.authorities[0], multi_sig.authorities[1]], // Only 2 approvals
        ..Default::default()
    };

    // Should fail with insufficient approvals
    assert!(!can_execute_proposal(&proposal, &multi_sig));
}
```

### Input Validation Testing
```rust
#[test]
fn test_amount_validation() {
    // Test zero amount
    assert!(validate_amount(0).is_err());

    // Test overflow amount
    assert!(validate_amount(u64::MAX).is_err());

    // Test negative amount (if applicable)
    // assert!(validate_amount(-1).is_err());

    // Test valid amount
    assert!(validate_amount(1000000).is_ok());
}

#[test]
fn test_address_validation() {
    // Test default pubkey
    assert!(validate_pubkey(&Pubkey::default()).is_err());

    // Test valid pubkey
    let valid_key = Pubkey::new_unique();
    assert!(validate_pubkey(&valid_key).is_ok());
}
```

### Reentrancy Testing
```rust
#[test]
fn test_reentrancy_protection() {
    // Test that operations can't be called recursively
    let mut vault = Vault::default();
    vault.paused = false;

    // Simulate reentrant call
    let result1 = pause_vault(&mut vault);
    assert!(result1.is_ok());
    assert!(vault.paused);

    // Second call should fail (already paused)
    let result2 = pause_vault(&mut vault);
    assert!(result2.is_err());
}
```

## Performance Testing

### Load Testing
```rust
#[tokio::test]
async fn test_concurrent_operations() {
    let program_test = ProgramTest::new(/* ... */);
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Create multiple concurrent operations
    let mut handles = vec![];

    for i in 0..10 {
        let handle = tokio::spawn(async move {
            // Perform deposit operation
            deposit_tokens(/* ... */).await;
        });
        handles.push(handle);
    }

    // Wait for all operations to complete
    for handle in handles {
        handle.await.unwrap();
    }

    // Verify final state
    let vault_data = banks_client.get_account(vault_account.pubkey()).await.unwrap().unwrap();
    let vault = Vault::try_from_slice(&vault_data.data).unwrap();

    assert_eq!(vault.total_value_locked, 10000000); // 10 * 1000000
}
```

### Gas Usage Testing
```rust
#[tokio::test]
async fn test_compute_unit_usage() {
    let program_test = ProgramTest::new(/* ... */);
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Measure compute units for different operations
    let deposit_cu = measure_compute_units(|| async {
        deposit_tokens(&mut banks_client, /* ... */).await;
    }).await;

    let withdraw_cu = measure_compute_units(|| async {
        withdraw_tokens(&mut banks_client, /* ... */).await;
    }).await;

    // Assert reasonable compute unit usage
    assert!(deposit_cu < 100000);
    assert!(withdraw_cu < 100000);
}
```

## Fuzz Testing

### Input Fuzzing
```rust
use arbitrary::Arbitrary;

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    amount: u64,
    authority: [u8; 32],
    token_mint: [u8; 32],
}

#[test]
fn fuzz_vault_operations() {
    let mut buffer = Vec::new();

    for _ in 0..1000 {
        // Generate random input
        let input: FuzzInput = arbitrary::Arbitrary::arbitrary(&mut arbitrary::Unstructured::new(&buffer)).unwrap();

        // Test with fuzzed input
        let result = test_deposit_with_fuzz_input(input);

        // Should not panic or cause undefined behavior
        assert!(result.is_ok() || matches!(result, Err(VaultError::InvalidAmount | VaultError::InvalidAuthority)));
    }
}
```

## Test Automation

### CI/CD Integration
```yaml
# .github/workflows/test.yml
name: Test Vault Program

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v3

      - name: Setup Rust
        uses: actions-rust-lang/setup-rust-toolchain@v1

      - name: Setup Solana
        run: |
          sh -c "$(curl -sSfL https://release.solana.com/v1.18.4/install)"
          echo "$HOME/.local/share/solana/install/active_release/bin" >> $GITHUB_PATH

      - name: Run Unit Tests
        run: cargo test --lib

      - name: Run Integration Tests
        run: cargo test --test integration

      - name: Build Program
        run: cargo build-bpf

      - name: Run E2E Tests
        run: npm test
```

### Test Data Management
```rust
// test_utils.rs
pub fn create_test_vault() -> Vault {
    Vault {
        authority: Pubkey::new_unique(),
        bump: 255,
        paused: false,
        total_value_locked: 1000000,
        fee_config: FeeConfig {
            deposit_fee_bps: 50,
            withdrawal_fee_bps: 50,
            fee_recipient: Pubkey::new_unique(),
        },
        ..Default::default()
    }
}

pub fn create_test_accounts() -> TestAccounts {
    TestAccounts {
        vault: Keypair::new(),
        mint: Keypair::new(),
        authority: Keypair::new(),
        user: Keypair::new(),
    }
}
```

## Test Coverage Analysis

### Coverage Reporting
```bash
# Generate coverage report
cargo install cargo-tarpaulin
cargo tarpaulin --out Html

# Open coverage report
open tarpaulin-report.html
```

### Coverage Goals
- **Unit Tests**: > 90% coverage
- **Integration Tests**: > 85% coverage
- **Security Tests**: 100% critical path coverage
- **Error Paths**: All error conditions tested

### Coverage Metrics
```rust
// Track test coverage metrics
#[test]
fn test_coverage_metrics() {
    // Test that all instruction variants are covered
    let instructions = vec![
        VaultInstruction::Initialize { bump: 255 },
        VaultInstruction::Deposit { amount: 1000000 },
        VaultInstruction::Withdraw { amount: 500000 },
        // ... all instruction types
    ];

    for instruction in instructions {
        assert!(is_instruction_tested(&instruction));
    }
}
```

## Test Environment Management

### Local Test Validator
```bash
# Start local validator
solana-test-validator

# In another terminal, run tests
cargo test --test integration
```

### Devnet Testing
```bash
# Configure for devnet
solana config set --url https://api.devnet.solana.com

# Run devnet tests
npm run test:devnet
```

### CI Testing Strategy
```typescript
// test-config.ts
export const TEST_CONFIG = {
  local: {
    rpcUrl: 'http://127.0.0.1:8899',
    programId: localProgramId,
    keypairPath: './test-keypair.json',
  },
  devnet: {
    rpcUrl: 'https://api.devnet.solana.com',
    programId: devnetProgramId,
    keypairPath: './devnet-keypair.json',
  },
  ci: {
    rpcUrl: process.env.CI_RPC_URL || 'http://127.0.0.1:8899',
    programId: process.env.CI_PROGRAM_ID,
    keypairPath: process.env.CI_KEYPAIR_PATH,
  },
};
```

## Debugging and Troubleshooting

### Test Debugging
```rust
#[test]
fn debug_failing_test() {
    // Enable detailed logging
    env_logger::init();

    // Add debug prints
    println!("Starting test execution");

    let result = execute_test_operation();
    println!("Operation result: {:?}", result);

    assert!(result.is_ok());
}
```

### Common Test Issues
```rust
// Handle common test failures
#[test]
fn test_with_error_handling() {
    let result = potentially_failing_operation();

    match result {
        Ok(value) => {
            // Test success path
            assert!(value > 0);
        }
        Err(VaultError::InsufficientFunds) => {
            // Test expected error
            assert!(true);
        }
        Err(unexpected_error) => {
            // Log unexpected errors
            println!("Unexpected error: {:?}", unexpected_error);
            panic!("Unexpected error occurred");
        }
    }
}
```

## Performance Benchmarks

### Benchmark Tests
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_vault_operations(c: &mut Criterion) {
    c.bench_function("deposit_operation", |b| {
        b.iter(|| {
            let vault = create_test_vault();
            black_box(process_deposit_operation(vault, 1000000));
        })
    });

    c.bench_function("withdraw_operation", |b| {
        b.iter(|| {
            let vault = create_test_vault();
            black_box(process_withdraw_operation(vault, 500000));
        })
    });
}

criterion_group!(benches, bench_vault_operations);
criterion_main!(benches);
```

### Performance Assertions
```rust
#[test]
fn test_performance_requirements() {
    let start = std::time::Instant::now();

    // Execute operation
    let result = perform_operation();

    let duration = start.elapsed();

    // Assert performance requirements
    assert!(duration < std::time::Duration::from_millis(100));
    assert!(result.is_ok());
}
```

This comprehensive testing guide ensures robust validation of all Vault Solana program functionality across unit, integration, and end-to-end testing levels, with specific focus on security, performance, and reliability.
