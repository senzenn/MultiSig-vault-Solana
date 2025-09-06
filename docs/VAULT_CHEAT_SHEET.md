# Vault Program Cheat Sheet

## 🚀 Quick Commands

### Setup & Basics
```bash
# Build project
cargo build --release

# Interactive menu (recommended)
./target/release/vault-cli menu

# Check help
./target/release/vault-cli --help
```

### Keypair & Network
```bash
# Create new keypair
vault-cli create-keypair --output mykey.json

# Check balance
vault-cli balance <PUBKEY>

# Get SOL airdrop (devnet)
vault-cli airdrop 2
```

---

## 🏦 Vault Operations

### Initialize Vault
```bash
vault-cli init-vault \
  --authority <AUTH_PUBKEY> \
  --emergency-admin <ADMIN_PUBKEY> \
  --bump 255
```

### Add Token Support
```bash
vault-cli add-token \
  --vault <VAULT_PUBKEY> \
  --mint <TOKEN_MINT> \
  --bump 254
```

### Deposit/Withdraw
```bash
# Deposit tokens
vault-cli deposit \
  --vault <VAULT_PUBKEY> \
  --user-token-account <USER_TOKEN_ACC> \
  --vault-token-account <VAULT_TOKEN_ACC> \
  --amount <AMOUNT>

# Withdraw tokens
vault-cli withdraw \
  --vault <VAULT_PUBKEY> \
  --vault-token-account <VAULT_TOKEN_ACC> \
  --user-token-account <USER_TOKEN_ACC> \
  --amount <AMOUNT>
```

---

## 🔐 Multisig Commands

### Setup Multisig
```bash
vault-cli init-multisig \
  --vault <VAULT_PUBKEY> \
  --owners "owner1,owner2,owner3" \
  --threshold 2
```

### Transaction Flow
```bash
# 1. Create transaction
vault-cli create-multisig-tx \
  --vault <VAULT_PUBKEY> \
  --program-id <TARGET_PROGRAM> \
  --accounts <ACCOUNTS_JSON> \
  --data <HEX_DATA>

# 2. Approve (multiple owners)
vault-cli approve-multisig-tx \
  --vault <VAULT_PUBKEY> \
  --transaction-id 0

# 3. Execute
vault-cli execute-multisig-tx \
  --vault <VAULT_PUBKEY> \
  --transaction-id 0
```

### Multisig Management
```bash
# Update owners
vault-cli update-multisig-owners \
  --vault <VAULT_PUBKEY> \
  --owners "new_owner1,new_owner2"

# Update threshold
vault-cli update-multisig-threshold \
  --vault <VAULT_PUBKEY> \
  --threshold 3

# List transactions
vault-cli list-multisig-txs --vault <VAULT_PUBKEY>
```

---

## 🏛️ Governance Commands

### Initialize Governance
```bash
vault-cli init-governance \
  --vault <VAULT_PUBKEY> \
  --voting-token-mint <VOTING_TOKEN> \
  --quorum-threshold 50 \
  --proposal-threshold 1000 \
  --voting-period 604800 \
  --time-lock-delay 86400 \
  --execution-threshold 75
```

### Proposal Workflow
```bash
# Create proposal
vault-cli create-proposal \
  --vault <VAULT_PUBKEY> \
  --title "Update Fee Structure" \
  --description "Reduce withdrawal fees" \
  --instructions <INSTRUCTIONS_JSON>

# Vote on proposal
vault-cli cast-vote \
  --vault <VAULT_PUBKEY> \
  --proposal-id 0 \
  --vote-type for

# Queue for execution
vault-cli queue-proposal \
  --vault <VAULT_PUBKEY> \
  --proposal-id 0

# Execute
vault-cli execute-proposal \
  --vault <VAULT_PUBKEY> \
  --proposal-id 0
```

---

## ⚡ Emergency & Admin

### Emergency Controls
```bash
# Pause vault
vault-cli pause-vault --vault <VAULT_PUBKEY>

# Unpause vault
vault-cli unpause-vault --vault <VAULT_PUBKEY>

# Emergency withdraw
vault-cli emergency-withdraw \
  --vault <VAULT_PUBKEY> \
  --token-mint <TOKEN_MINT> \
  --amount <AMOUNT>
```

### Authority Management
```bash
# Transfer authority
vault-cli transfer-authority \
  --vault <VAULT_PUBKEY> \
  --new-authority <NEW_AUTHORITY>

# Update emergency admin
vault-cli update-emergency-admin \
  --vault <VAULT_PUBKEY> \
  --new-admin <NEW_ADMIN>
```

---

## 📊 Monitoring & Info

### Get Information
```bash
# Vault info
vault-cli info --vault <VAULT_PUBKEY>

# Balance check
vault-cli balance <PUBKEY>

# Transaction status
vault-cli tx-status <TX_SIGNATURE>

# Program accounts
vault-cli program-accounts --program-id <PROGRAM_ID>
```

### Testing
```bash
# Run all tests
vault-cli test

# Run specific test
vault-cli test --filter deposit
```

---

## ⚙️ Configuration

### Environment Variables
```bash
export SOLANA_RPC_URL=https://api.devnet.solana.com
export VAULT_KEYPAIR_PATH=~/.config/solana/id.json
```

### Profile Management
```bash
# List profiles
vault-cli profile list

# Add profile
vault-cli profile add devnet \
  --rpc-url https://api.devnet.solana.com \
  --keypair ~/.config/solana/devnet.json

# Use profile
vault-cli profile use devnet
```

---

## 🔧 Common Token Mints (Devnet)

| Token | Mint Address |
|-------|--------------|
| USDC | EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v |
| USDT | Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB |
| SOL | So11111111111111111111111111111111111111112 |

---

## 🚨 Quick Fixes

### Common Errors & Solutions

1. **"Account not found"**
   ```bash
   # Check if vault exists
   vault-cli info --vault <VAULT_PUBKEY>
   ```

2. **"Insufficient balance"**
   ```bash
   # Check vault balance
   vault-cli balance <VAULT_PUBKEY>
   ```

3. **"Invalid signature"**
   ```bash
   # Verify keypair
   vault-cli balance $(solana-keygen pubkey ~/.config/solana/id.json)
   ```

4. **"Multisig threshold not met"**
   ```bash
   # Check transaction status
   vault-cli list-multisig-txs --vault <VAULT_PUBKEY>
   ```

---

## 🎯 Quick Start Workflow

```bash
# 1. Setup
vault-cli create-keypair --output vault.json
vault-cli airdrop 2

# 2. Initialize vault
VAULT_AUTH=$(solana-keygen pubkey vault.json)
vault-cli init-vault --authority $VAULT_AUTH --emergency-admin $VAULT_AUTH --bump 255

# 3. Add USDC support
vault-cli add-token --vault <VAULT_PUBKEY> --mint EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v --bump 254

# 4. Deposit tokens
vault-cli deposit --vault <VAULT_PUBKEY> --user-token-account <USER_USDC> --vault-token-account <VAULT_USDC> --amount 1000000

# 5. Setup multisig (optional)
vault-cli init-multisig --vault <VAULT_PUBKEY> --owners "$VAULT_AUTH" --threshold 1

# 6. Test everything
vault-cli test
```

---

## 📝 Notes

- **Always test on devnet first**
- **Backup your keypairs**
- **Use multisig for production**
- **Monitor balances regularly**
- **Keep emergency admin secure**

**Happy vaulting!** 🏦✨
