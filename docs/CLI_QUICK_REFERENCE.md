# 🚀 Vault CLI Quick Reference

## 🎮 Interactive Mode (Recommended)
```bash
cargo run -- menu
```

## 💰 Essential Commands

### Get SOL
```bash
# Interactive airdrop
cargo run -- menu  # Select "💰 Airdrop SOL"

# Command line airdrop
cargo run -- airdrop --sol 1.0
```

### Check Balance
```bash
# Interactive
cargo run -- menu  # Select "□ Check Balance"

# Command line
cargo run -- balance --account 11111111111111111111111111111112
```

### Transfer SOL
```bash
# Interactive (recommended)
cargo run -- menu  # Select "💰 Transfer SOL"

# Valid inputs:
# Vault: 11111111111111111111111111111112
# Recipient: 11111111111111111111111111111112
# Amount: 0.1
```

## 🔧 Configuration

### Show Config
```bash
cargo run -- config
```

### Create Keypair
```bash
cargo run -- create-keypair --output-path keypair.json
```

### Manage Profiles
```bash
# List profiles
cargo run -- profile --action list

# Add profile
cargo run -- profile --action add --name devnet --rpc-url https://api.devnet.solana.com

# Use profile
cargo run -- profile --action use --name devnet
```

## 🌐 Network Info

### Get Blockhash
```bash
cargo run -- blockhash
```

### Check Transaction
```bash
cargo run -- tx-status --sig <signature>
```

### Program Accounts
```bash
cargo run -- program-accounts --program-id <program_id>
```

## 🏦 Vault Operations

### Vault Info
```bash
cargo run -- info --vault-account <vault_address>
```

### Run Tests
```bash
cargo run -- test --vault-account <vault_address>
```

## 🛠️ Development

### Build
```bash
cargo build
cargo check
```

### Run with Custom Settings
```bash
# Custom RPC
cargo run -- --rpc-url https://api.devnet.solana.com menu

# Custom keypair
cargo run -- --keypair keypair.json menu
```

## 📋 Valid Input Examples

### Solana Addresses
```
11111111111111111111111111111112
```

### SOL Amounts
```
0.1
1.5
10.0
```

## ⚠️ Common Issues

| Error | Solution |
|-------|----------|
| "String is the wrong size" | Use valid Solana public key |
| "Account not found" | Check if vault exists |
| "Insufficient funds" | Check balance, try smaller amount |

## 🎯 Menu Options

| Option | Action |
|--------|--------|
| ⚙️ Show Configuration | View settings |
| 💰 Airdrop SOL | Get SOL |
| 💰 Transfer SOL | Send SOL |
| → Latest Blockhash | Get blockhash |
| □ Check Balance | View balance |
| 🏦 Vault Info | Vault details |
| █ Program Accounts | List accounts |
| ▢ Transaction Status | Check tx |
| λ Quick Test | Run tests |
| ⚙️ Manage Profiles | Manage configs |
| 👤 Exit | Quit |

---

**For detailed documentation, see `CLI_COMMANDS_README.md`**
