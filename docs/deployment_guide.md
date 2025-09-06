# Vault Solana Program - Deployment Guide

## Overview

This deployment guide provides step-by-step instructions for deploying the Vault Solana program to various networks, including development, testing, and production environments.

## Prerequisites

### System Requirements
```bash
# Operating System
Ubuntu 20.04+ / macOS 12.0+

# Memory
8GB RAM minimum, 16GB recommended

# Storage
10GB free space for Solana toolchain and programs

# Network
Stable internet connection (50 Mbps minimum)
```

### Software Dependencies
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/v1.18.4/install)"

# Add Solana to PATH
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"

# Verify installations
rustc --version
solana --version
```

### Development Tools
```bash
# Install additional tools
cargo install cargo-build-bpf
npm install -g @project-serum/anchor-cli

# Verify BPF toolchain
cargo build-bpf --version
```

## Environment Setup

### 1. Configure Solana CLI
```bash
# Set Solana configuration
solana config set --url https://api.devnet.solana.com

# Generate or set keypair
solana-keygen new --outfile ~/.config/solana/id.json

# Check configuration
solana config get
```

### 2. Fund Development Account
```bash
# Get your public key
solana address

# Request airdrop (devnet/testnet only)
solana airdrop 2

# Check balance
solana balance
```

### 3. Clone and Setup Project
```bash
# Clone the repository
git clone <vault-program-repo>
cd vault-program

# Install dependencies
npm install

# Build the program
cargo build-bpf
```

## Build Process

### 1. Program Compilation
```bash
# Clean previous builds
cargo clean

# Build BPF program
cargo build-bpf

# Verify build artifacts
ls target/deploy/
# Should see: vault.so, vault-keypair.json
```

### 2. Program Size Optimization
```bash
# Check program size
ls -lh target/deploy/vault.so

# Program should be under 1MB for optimal performance
# If too large, consider:
# - Reducing string literals
# - Optimizing data structures
# - Removing unused code
```

### 3. Test Build
```bash
# Run tests before deployment
cargo test

# Run specific test suites
cargo test --lib -- --test-threads=1
cargo test --bins
```

## Deployment Strategies

### Development Deployment (Devnet)

#### 1. Configure for Devnet
```bash
# Set devnet as default
solana config set --url https://api.devnet.solana.com

# Verify configuration
solana config get
```

#### 2. Deploy to Devnet
```bash
# Deploy program
solana program deploy target/deploy/vault.so

# Expected output:
# Program Id: <PROGRAM_ID>
# Signature: <DEPLOYMENT_SIGNATURE>
```

#### 3. Verify Deployment
```bash
# Check program account
solana program show <PROGRAM_ID>

# Verify program is executable
solana program get <PROGRAM_ID>
```

### Testing Deployment (Testnet)

#### 1. Configure for Testnet
```bash
# Switch to testnet
solana config set --url https://api.testnet.solana.com

# Get testnet SOL
solana airdrop 5
```

#### 2. Deploy to Testnet
```bash
# Deploy with explicit keypair
solana program deploy \
  --program-id target/deploy/vault-keypair.json \
  target/deploy/vault.so
```

#### 3. Testnet Validation
```bash
# Run integration tests
npm test

# Monitor program logs
solana logs <PROGRAM_ID>
```

### Production Deployment (Mainnet)

#### 1. Pre-Deployment Checklist
```bash
# Security audit completed
# All tests passing
# Code review completed
# Emergency procedures documented
# Backup deployment key secured
```

#### 2. Mainnet Configuration
```bash
# Switch to mainnet
solana config set --url https://api.mainnet-beta.solana.com

# Use production keypair (NEVER dev keys)
solana config set --keypair ~/.config/solana/mainnet-keypair.json
```

#### 3. Mainnet Deployment
```bash
# Deploy with maximum priority and compute units
solana program deploy \
  --program-id target/deploy/vault-keypair.json \
  target/deploy/vault.so \
  --max-sign-attempts 100 \
  --with-compute-unit-price 100000
```

#### 4. Post-Deployment Verification
```bash
# Verify program deployment
solana program show <PROGRAM_ID>

# Check program balance
solana balance <PROGRAM_ID>

# Monitor initial transactions
solana logs <PROGRAM_ID> --follow
```

## Program ID Management

### 1. Program ID Generation
```bash
# Generate new program ID
solana-keygen new --outfile program-keypair.json

# Extract public key
solana-keygen pubkey program-keypair.json
```

### 2. Program ID Updates
```rust
// Update lib.rs with actual program ID
solana_program::declare_id!("YourActualProgramIdHere");
```

### 3. Program ID Verification
```bash
# Verify program ID matches deployment
solana program show <PROGRAM_ID>

# Check program executability
solana program dump <PROGRAM_ID> program.so
```

## Configuration Management

### Environment-Specific Configuration
```typescript
// config.ts
export const CONFIG = {
  devnet: {
    programId: new PublicKey("DEVNET_PROGRAM_ID"),
    cluster: "devnet",
    rpcUrl: "https://api.devnet.solana.com",
  },
  testnet: {
    programId: new PublicKey("TESTNET_PROGRAM_ID"),
    cluster: "testnet",
    rpcUrl: "https://api.testnet.solana.com",
  },
  mainnet: {
    programId: new PublicKey("MAINNET_PROGRAM_ID"),
    cluster: "mainnet-beta",
    rpcUrl: "https://api.mainnet-beta.solana.com",
  },
};
```

### DeFi Protocol Configuration
```typescript
// Update protocol IDs for production
export const DEF1_PROTOCOLS = {
  ORCA_WHIRLPOOL: new PublicKey("whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc"), // Mainnet
  RAYDIUM_AMM: new PublicKey("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8"), // Mainnet
  SABER_PROTOCOL: new PublicKey("SSwpkEEcbUqx4vtoEByFjSkhKdCT862DNVb52nZg1UZ"), // Mainnet
  JUPITER_AGGREGATOR: new PublicKey("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4"), // Mainnet
};
```

## Cost Optimization

### Deployment Cost Analysis
```bash
# Estimate deployment cost
solana program deploy --dry-run target/deploy/vault.so

# Monitor account costs
solana rent <ACCOUNT_SIZE>
```

### Program Size Optimization
```rust
// Use compact data structures
#[derive(BorshSerialize, BorshDeserialize)]
pub struct CompactVault {
    pub authority: Pubkey,      // 32 bytes
    pub bump: u8,              // 1 byte
    pub paused: bool,          // 1 byte
    // ... other fields
}

// Total: ~89 bytes vs ~200+ bytes for full structure
```

### Rent Optimization
```rust
// Calculate optimal rent-exempt balance
let vault_size = std::mem::size_of::<Vault>() as u64;
let rent = Rent::from_account_info(rent_sysvar)?;
let required_lamports = rent.minimum_balance(vault_size as usize);
```

## Monitoring and Maintenance

### Deployment Monitoring
```bash
# Monitor deployment logs
solana logs <PROGRAM_ID> --follow

# Check program health
solana program show <PROGRAM_ID>

# Monitor account balances
solana balance <PROGRAM_ID>
```

### Performance Monitoring
```typescript
// Monitor program performance
const connection = new Connection(clusterApiUrl('mainnet-beta'));

// Monitor transaction success rate
connection.onLogs(programId, (logs) => {
  const successRate = calculateSuccessRate(logs);
  if (successRate < 0.95) {
    alert(`Low success rate: ${successRate}`);
  }
});
```

### Update Procedures
```bash
# Create backup before updates
solana program dump <PROGRAM_ID> backup.so

# Deploy updated program
solana program deploy target/deploy/vault.so

# Verify update
solana program show <PROGRAM_ID>
```

## Troubleshooting

### Common Deployment Issues

#### Issue: Program Too Large
```bash
# Error: Program is too large
# Solution: Optimize code size
cargo build-bpf --release
# Remove unused dependencies
# Use smaller data structures
```

#### Issue: Insufficient Funds
```bash
# Error: Insufficient funds
# Solution: Fund deployment account
solana airdrop 10
# Or transfer from funded account
solana transfer <FUNDED_ACCOUNT> <DEPLOYMENT_ACCOUNT> 10
```

#### Issue: Program Verification Failed
```bash
# Error: Program verification failed
# Solution: Check build artifacts
ls -la target/deploy/
# Rebuild if necessary
cargo clean && cargo build-bpf
```

#### Issue: Network Congestion
```bash
# Solution: Use priority fees
solana program deploy \
  target/deploy/vault.so \
  --with-compute-unit-price 100000 \
  --max-sign-attempts 50
```

### Rollback Procedures
```bash
# Emergency rollback
# 1. Stop accepting new transactions
# 2. Deploy previous version
solana program deploy backup.so

# 3. Update client configurations
# 4. Notify users of rollback
# 5. Resume operations
```

## Security Considerations

### Pre-Deployment Security
```bash
# Audit checklist
- [ ] Code review completed
- [ ] Security audit passed
- [ ] All tests passing
- [ ] Emergency procedures documented
- [ ] Backup keys secured
- [ ] Deployment key isolated
```

### Post-Deployment Security
```bash
# Monitor for anomalies
solana logs <PROGRAM_ID> | grep -i error

# Set up alerts for suspicious activity
# Monitor program account changes
# Regular security assessments
```

## Multi-Environment Deployment

### CI/CD Pipeline Setup
```yaml
# .github/workflows/deploy.yml
name: Deploy Vault Program

on:
  push:
    branches: [main]
  workflow_dispatch:

jobs:
  deploy-devnet:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
      - name: Deploy to Devnet
        run: |
          solana config set --url https://api.devnet.solana.com
          solana program deploy target/deploy/vault.so

  deploy-mainnet:
    runs-on: ubuntu-latest
    if: github.ref == 'refs/heads/main'
    steps:
      - name: Deploy to Mainnet
        run: |
          solana config set --url https://api.mainnet-beta.solana.com
          solana program deploy target/deploy/vault.so
```

### Environment-Specific Configurations
```bash
# Devnet deployment
export SOLANA_CONFIG_URL="https://api.devnet.solana.com"
export PROGRAM_KEYPAIR="devnet-keypair.json"

# Testnet deployment
export SOLANA_CONFIG_URL="https://api.testnet.solana.com"
export PROGRAM_KEYPAIR="testnet-keypair.json"

# Mainnet deployment
export SOLANA_CONFIG_URL="https://api.mainnet-beta.solana.com"
export PROGRAM_KEYPAIR="mainnet-keypair.json"
```

## Performance Benchmarks

### Expected Performance Metrics
- **Deployment Time**: < 30 seconds (normal network conditions)
- **Transaction Confirmation**: < 10 seconds (average)
- **Program Size**: < 1MB (recommended)
- **Compute Units**: < 200,000 per instruction (target)

### Monitoring Scripts
```bash
#!/bin/bash
# monitor-deployment.sh

PROGRAM_ID=$1
LOG_FILE="deployment-$(date +%Y%m%d-%H%M%S).log"

echo "Monitoring program: $PROGRAM_ID" > $LOG_FILE

# Monitor logs
solana logs $PROGRAM_ID --follow >> $LOG_FILE &

# Monitor balance
while true; do
  BALANCE=$(solana balance $PROGRAM_ID 2>/dev/null || echo "error")
  echo "$(date): Balance $BALANCE" >> $LOG_FILE
  sleep 60
done
```

This deployment guide provides comprehensive procedures for successfully deploying the Vault Solana program across all environments while maintaining security, performance, and operational best practices.
