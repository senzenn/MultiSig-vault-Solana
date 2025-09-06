# Vault Program Command Guide

## 🎯 Overview

This guide provides comprehensive instructions for using the Vault Program CLI. The vault is a complete DeFi vault implementation on Solana featuring token management, multisig security, governance, and advanced features.

## 🚀 Getting Started

### Prerequisites
- Rust and Cargo installed
- Solana CLI tools
- A Solana wallet/keypair

### Build the Project
```bash
cd vault-solana/vault_program
cargo build --release
```

### Run the CLI
```bash
./target/release/vault-cli --help
# or for development
./target/debug/vault-cli --help
```

## 📋 Available Commands

### 1. Interactive Menu (Recommended for Beginners)
```bash
vault-cli menu
```
Beautiful interactive UI with guided workflows for all vault operations.

### 2. Profile Management
```bash
# List all profiles
vault-cli profile list

# Add a new profile
vault-cli profile add myprofile --rpc-url https://api.devnet.solana.com --keypair ~/.config/solana/mykeypair.json

# Use a specific profile
vault-cli profile use myprofile

# Remove a profile
vault-cli profile remove myprofile
```

### 3. Keypair Management
```bash
# Create a new keypair
vault-cli create-keypair --output ~/.config/solana/newkeypair.json

# Check balance
vault-cli balance <PUBKEY>
```

### 4. Network Operations
```bash
# Request SOL airdrop (devnet/testnet only)
vault-cli airdrop 2  # Request 2 SOL

# Get latest blockhash
vault-cli blockhash

# Check transaction status
vault-cli tx-status <TRANSACTION_SIGNATURE>
```

## 🏦 Core Vault Operations

### Initialize Vault
```bash
vault-cli init-vault \
  --authority <AUTHORITY_PUBKEY> \
  --emergency-admin <EMERGENCY_ADMIN_PUBKEY> \
  --bump <VAULT_BUMP>
```

### Add Supported Token
```bash
vault-cli add-token \
  --vault <VAULT_PUBKEY> \
  --mint <TOKEN_MINT> \
  --bump <TOKEN_ACCOUNT_BUMP>
```

### Deposit Tokens
```bash
vault-cli deposit \
  --vault <VAULT_PUBKEY> \
  --user-token-account <USER_TOKEN_ACCOUNT> \
  --vault-token-account <VAULT_TOKEN_ACCOUNT> \
  --amount <AMOUNT_IN_SMALLEST_UNITS>
```

### Withdraw Tokens
```bash
vault-cli withdraw \
  --vault <VAULT_PUBKEY> \
  --vault-token-account <VAULT_TOKEN_ACCOUNT> \
  --user-token-account <USER_TOKEN_ACCOUNT> \
  --amount <AMOUNT_IN_SMALLEST_UNITS>
```

### Get Vault Information
```bash
vault-cli info --vault <VAULT_PUBKEY>
```

## 🔐 Multisig Operations

### Initialize Multisig
```bash
vault-cli init-multisig \
  --vault <VAULT_PUBKEY> \
  --owners <OWNER1_PUBKEY,OWNER2_PUBKEY,OWNER3_PUBKEY> \
  --threshold <REQUIRED_APPROVALS>
```

### Create Multisig Transaction
```bash
vault-cli create-multisig-tx \
  --vault <VAULT_PUBKEY> \
  --program-id <TARGET_PROGRAM_ID> \
  --accounts <ACCOUNT_METAS_JSON> \
  --data <INSTRUCTION_DATA_HEX>
```

### Approve Multisig Transaction
```bash
vault-cli approve-multisig-tx \
  --vault <VAULT_PUBKEY> \
  --transaction-id <TX_ID>
```

### Execute Multisig Transaction
```bash
vault-cli execute-multisig-tx \
  --vault <VAULT_PUBKEY> \
  --transaction-id <TX_ID>
```

### Update Multisig Owners
```bash
vault-cli update-multisig-owners \
  --vault <VAULT_PUBKEY> \
  --owners <NEW_OWNER1_PUBKEY,NEW_OWNER2_PUBKEY>
```

### Update Multisig Threshold
```bash
vault-cli update-multisig-threshold \
  --vault <VAULT_PUBKEY> \
  --threshold <NEW_THRESHOLD>
```

### List Multisig Transactions
```bash
vault-cli list-multisig-txs --vault <VAULT_PUBKEY>
```

## 🏛️ Governance Operations

### Initialize Governance
```bash
vault-cli init-governance \
  --vault <VAULT_PUBKEY> \
  --voting-token-mint <VOTING_TOKEN_MINT> \
  --quorum-threshold <QUORUM_PERCENTAGE> \
  --proposal-threshold <MIN_TOKENS_TO_PROPOSE> \
  --voting-period <VOTING_PERIOD_SECONDS> \
  --time-lock-delay <TIME_LOCK_SECONDS> \
  --execution-threshold <EXECUTION_THRESHOLD>
```

### Create Governance Proposal
```bash
vault-cli create-proposal \
  --vault <VAULT_PUBKEY> \
  --title "Proposal Title" \
  --description "Proposal Description" \
  --instructions <INSTRUCTIONS_JSON>
```

### Cast Vote
```bash
vault-cli cast-vote \
  --vault <VAULT_PUBKEY> \
  --proposal-id <PROPOSAL_ID> \
  --vote-type <for|against|abstain>
```

### Queue Proposal
```bash
vault-cli queue-proposal \
  --vault <VAULT_PUBKEY> \
  --proposal-id <PROPOSAL_ID>
```

### Execute Proposal
```bash
vault-cli execute-proposal \
  --vault <VAULT_PUBKEY> \
  --proposal-id <PROPOSAL_ID>
```

### Update Governance Config
```bash
vault-cli update-governance-config \
  --vault <VAULT_PUBKEY> \
  --quorum-threshold <NEW_QUORUM> \
  --proposal-threshold <NEW_PROPOSAL_THRESHOLD> \
  --voting-period <NEW_VOTING_PERIOD> \
  --time-lock-delay <NEW_TIME_LOCK_DELAY> \
  --execution-threshold <NEW_EXECUTION_THRESHOLD>
```

## ⚡ Advanced Features

### Emergency Controls
```bash
# Pause vault (emergency admin only)
vault-cli pause-vault --vault <VAULT_PUBKEY>

# Unpause vault (emergency admin only)
vault-cli unpause-vault --vault <VAULT_PUBKEY>

# Emergency withdraw (emergency admin only)
vault-cli emergency-withdraw \
  --vault <VAULT_PUBKEY> \
  --token-mint <TOKEN_MINT> \
  --amount <AMOUNT>
```

### Fee Management
```bash
# Collect accumulated fees
vault-cli collect-fees --vault <VAULT_PUBKEY>
```

### Authority Management
```bash
# Transfer vault authority
vault-cli transfer-authority \
  --vault <VAULT_PUBKEY> \
  --new-authority <NEW_AUTHORITY_PUBKEY>

# Update emergency admin
vault-cli update-emergency-admin \
  --vault <VAULT_PUBKEY> \
  --new-admin <NEW_ADMIN_PUBKEY>
```

## 🧪 Testing & Monitoring

### Run Tests
```bash
vault-cli test
```

### Monitor Program Accounts
```bash
vault-cli program-accounts --program-id <PROGRAM_ID>
```

### Configuration
```bash
vault-cli config
```

## 📋 Command Examples

### Complete Vault Setup Workflow
```bash
# 1. Create keypairs
vault-cli create-keypair --output authority.json
vault-cli create-keypair --output emergency.json

# 2. Initialize vault
vault-cli init-vault \
  --authority $(solana-keygen pubkey authority.json) \
  --emergency-admin $(solana-keygen pubkey emergency.json) \
  --bump 255

# 3. Add USDC token support
vault-cli add-token \
  --vault <VAULT_PUBKEY> \
  --mint EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v \
  --bump 254

# 4. Deposit tokens
vault-cli deposit \
  --vault <VAULT_PUBKEY> \
  --user-token-account <USER_USDC_ACCOUNT> \
  --vault-token-account <VAULT_USDC_ACCOUNT> \
  --amount 1000000  # 1 USDC (6 decimals)
```

### Multisig Transaction Workflow
```bash
# 1. Setup multisig
vault-cli init-multisig \
  --vault <VAULT_PUBKEY> \
  --owners "owner1.json,owner2.json,owner3.json" \
  --threshold 2

# 2. Create withdrawal transaction
vault-cli create-multisig-tx \
  --vault <VAULT_PUBKEY> \
  --program-id TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA \
  --accounts '[{"pubkey":"VAULT_TOKEN_ACCOUNT","is_signer":false,"is_writable":true},{"pubkey":"USER_TOKEN_ACCOUNT","is_signer":false,"is_writable":true},{"pubkey":"VAULT_PUBKEY","is_signer":true,"is_writable":false}]' \
  --data "0300000000000000"  # Transfer instruction data

# 3. Approve by multiple owners
vault-cli approve-multisig-tx --vault <VAULT_PUBKEY> --transaction-id 0
vault-cli approve-multisig-tx --vault <VAULT_PUBKEY> --transaction-id 0

# 4. Execute transaction
vault-cli execute-multisig-tx --vault <VAULT_PUBKEY> --transaction-id 0
```

## 🔧 Configuration Options

### Environment Variables
```bash
export SOLANA_RPC_URL=https://api.devnet.solana.com
export VAULT_KEYPAIR_PATH=~/.config/solana/id.json
```

### CLI Options
```bash
vault-cli --rpc-url https://api.mainnet.solana.com --keypair ~/.config/solana/mainnet.json <COMMAND>
```

## 🚨 Error Handling

### Common Issues & Solutions

1. **"Account not found"**
   - Ensure vault is initialized
   - Check account addresses are correct

2. **"Insufficient balance"**
   - Verify vault has enough tokens
   - Check token amounts and decimals

3. **"Invalid signature"**
   - Ensure correct keypair is loaded
   - Check signer permissions

4. **"Multisig threshold not met"**
   - Get more approvals from owners
   - Check current approval count

5. **"Proposal not executable"**
   - Ensure proposal passed voting
   - Wait for time-lock period if applicable

## 📊 Monitoring & Analytics

### Vault Statistics
```bash
vault-cli info --vault <VAULT_PUBKEY>
```

### Transaction History
```bash
vault-cli list-multisig-txs --vault <VAULT_PUBKEY>
```

### Balance Monitoring
```bash
vault-cli balance <VAULT_PUBKEY>
vault-cli balance <TOKEN_ACCOUNT_PUBKEY>
```

## 🛠️ Development & Testing

### Local Testing
```bash
# Run all tests
vault-cli test

# Test specific functionality
vault-cli test --filter deposit

# Test with custom RPC
vault-cli test --rpc-url http://localhost:8899
```

### Debug Mode
```bash
# Enable verbose logging
RUST_LOG=debug vault-cli <COMMAND>

# Show transaction details
vault-cli tx-status <SIGNATURE>
```

## 📚 API Reference

### Vault Account Structure
```rust
pub struct Vault {
    pub authority: Pubkey,
    pub bump: u8,
    pub emergency_admin: Pubkey,
    pub paused: bool,
    pub supported_tokens: Vec<SupportedToken>,
    pub token_balances: Vec<TokenBalance>,
    pub multi_sig: Option<MultiSig>,
    pub fee_config: FeeConfig,
    pub total_value_locked: u64,
}
```

### Transaction Types
- **Deposit**: Transfer tokens from user to vault
- **Withdraw**: Transfer tokens from vault to user
- **MultisigTx**: Multi-signature approved transaction
- **Proposal**: Governance proposal
- **Emergency**: Emergency admin operations

## 🔒 Security Best Practices

1. **Always backup keypairs**
2. **Use multisig for critical operations**
3. **Test on devnet before mainnet**
4. **Monitor vault balances regularly**
5. **Set appropriate governance thresholds**
6. **Use emergency admin carefully**

## 🎯 Quick Start

For new users:
```bash
# 1. Use interactive menu
vault-cli menu

# 2. Follow guided setup
# 3. Test basic operations
vault-cli test

# 4. Start with small amounts
vault-cli deposit --amount 1000000
```

---

## 📞 Support

If you encounter issues:
1. Check the error messages
2. Verify network connectivity
3. Ensure sufficient SOL balance
4. Use `--help` for command details
5. Check transaction status with `tx-status`

**Your vault is now ready for production use!** 🚀
