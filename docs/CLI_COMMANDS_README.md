# 🏦 Vault CLI Commands Reference

A comprehensive guide to all commands available in the Solana Vault CLI.

## 📋 Table of Contents

- [Quick Start](#quick-start)
- [Interactive Menu](#interactive-menu)
- [Command Line Interface](#command-line-interface)
- [Profile Management](#profile-management)
- [Network Operations](#network-operations)
- [Account Management](#account-management)
- [Vault Operations](#vault-operations)
- [Testing & Development](#testing--development)

## 🚀 Quick Start

### Interactive Mode (Recommended)
```bash
# Start interactive menu
cargo run -- menu

# With custom RPC
cargo run -- --rpc-url https://api.devnet.solana.com menu

# With custom keypair
cargo run -- --keypair /path/to/keypair.json menu
```

### Command Line Mode
```bash
# Check balance
cargo run -- balance --account 11111111111111111111111111111112

# Request airdrop
cargo run -- airdrop --sol 1.0

# Get latest blockhash
cargo run -- blockhash
```

## 🎮 Interactive Menu

The interactive menu provides a beautiful, user-friendly interface for all operations.

### Menu Options

| Option | Description | Command Equivalent |
|--------|-------------|-------------------|
| ⚙️ Show Configuration | Display current settings | `config` |
| 💰 Airdrop SOL | Request SOL from faucet | `airdrop` |
| 💰 Transfer SOL | Transfer SOL via vault | `transfer` |
| → Latest Blockhash | Get current blockhash | `blockhash` |
| □ Check Balance | Check account balance | `balance` |
| 🏦 Vault Info | Get vault details | `info` |
| █ Program Accounts | List program accounts | `program-accounts` |
| ▢ Transaction Status | Check tx status | `tx-status` |
| λ Quick Test | Run vault tests | `test` |
| ⚙️ Manage Profiles | Manage connections | `profile` |
| 👤 Exit | Quit application | - |

## 💻 Command Line Interface

### Basic Commands

#### `menu` - Interactive Mode
```bash
cargo run -- menu
```
Start the interactive menu interface with beautiful UI.

#### `config` - Show Configuration
```bash
cargo run -- config
cargo run -- config --show-secrets
```
Display current configuration and environment variables.

#### `create-keypair` - Create New Keypair
```bash
cargo run -- create-keypair --output-path /path/to/keypair.json
```
Generate a new Solana keypair for transactions.

### Network Operations

#### `airdrop` - Request SOL
```bash
# Airdrop to current signer
cargo run -- airdrop --sol 1.0

# Airdrop to specific account
cargo run -- airdrop --pubkey 11111111111111111111111111111112 --sol 2.0
```
Request SOL from the network faucet (max 2 SOL per request).

#### `blockhash` - Get Latest Blockhash
```bash
cargo run -- blockhash
```
Fetch the latest blockhash from the network.

#### `tx-status` - Transaction Status
```bash
cargo run -- tx-status --sig <transaction_signature>
```
Check transaction status and confirmation details.

### Account Management

#### `balance` - Check Balance
```bash
cargo run -- balance --account 11111111111111111111111111111112
```
Get SOL balance for any account address.

#### `program-accounts` - List Program Accounts
```bash
cargo run -- program-accounts --program-id <program_id> --limit 10
```
List all accounts owned by a program with balances.

### Vault Operations

#### `info` - Vault Information
```bash
cargo run -- info --vault-account <vault_address>
```
Get detailed vault account information.

#### `test` - Run Tests
```bash
cargo run -- test --vault-account <vault_address>
```
Run comprehensive vault functionality tests.

## 🔧 Profile Management

Manage connection profiles for different networks and configurations.

### Profile Commands

#### Add Profile
```bash
cargo run -- profile --action add --name devnet \
  --rpc-url https://api.devnet.solana.com \
  --ws-url wss://api.devnet.solana.com \
  --program-id <your_program_id> \
  --keypair /path/to/keypair.json
```

#### List Profiles
```bash
cargo run -- profile --action list
```

#### Use Profile
```bash
cargo run -- profile --action use --name devnet
```

#### Remove Profile
```bash
cargo run -- profile --action remove --name devnet
```

#### Set Default Profile
```bash
cargo run -- profile --action set-default --name devnet
```

## 🌐 Network Configuration

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `RPC_URL` | RPC endpoint URL | `https://api.devnet.solana.com` |
| `RPC_WS_URL` | WebSocket endpoint URL | `wss://api.devnet.solana.com` |
| `PROGRAM_ID` | Vault program ID | `VaUltPr0gr4mID1234567890abcdefghij1234567890abcdef` |
| `DEFAULT_KEYPAIR_PATH` | Default keypair path | None |

### Supported Networks

- **Devnet**: `https://api.devnet.solana.com`
- **Testnet**: `https://api.testnet.solana.com`
- **Mainnet**: `https://api.mainnet-beta.solana.com`
- **Local**: `http://localhost:8899`

## 💰 Transfer SOL Guide

### Interactive Transfer
1. Select "💰 Transfer SOL" from menu
2. Enter vault account address (e.g., `11111111111111111111111111111112`)
3. Enter recipient address
4. Enter SOL amount (e.g., `0.1`)
5. Confirm transfer

### Valid Input Examples

**Vault Address:**
```
11111111111111111111111111111112
```

**Recipient Address:**
```
11111111111111111111111111111112
```

**SOL Amount:**
```
0.1
1.5
10.0
```

## 🛠️ Development Commands

### Build and Run
```bash
# Build the project
cargo build

# Run with specific features
cargo run --release

# Check for errors
cargo check
```

### Testing
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture
```

## 🔍 Troubleshooting

### Common Issues

1. **"String is the wrong size" Error**
   - Ensure addresses are valid Solana public keys
   - Use base58-encoded addresses (~32-44 characters)

2. **"Account not found" Error**
   - Verify the vault account exists
   - Check if the vault has been initialized

3. **"Insufficient funds" Error**
   - Check vault balance
   - Try a smaller transfer amount

4. **Connection Issues**
   - Verify RPC URL is correct
   - Check network connectivity
   - Try different RPC endpoint

### Debug Mode
```bash
# Run with debug output
RUST_LOG=debug cargo run -- menu

# Check configuration
cargo run -- config --show-secrets
```

## 📚 Examples

### Complete Workflow
```bash
# 1. Start interactive mode
cargo run -- menu

# 2. Request airdrop (if needed)
# Select "💰 Airdrop SOL" from menu

# 3. Check balance
# Select "□ Check Balance" from menu

# 4. Transfer SOL
# Select "💰 Transfer SOL" from menu

# 5. Check transaction status
# Select "▢ Transaction Status" from menu
```

### Profile Setup
```bash
# Create devnet profile
cargo run -- profile --action add --name devnet \
  --rpc-url https://api.devnet.solana.com \
  --program-id 11111111111111111111111111111112

# Set as default
cargo run -- profile --action set-default --name devnet

# Use profile
cargo run -- profile --action use --name devnet
```

## 🎯 Quick Reference

| Command | Purpose | Example |
|---------|---------|---------|
| `menu` | Interactive interface | `cargo run -- menu` |
| `airdrop` | Get SOL | `cargo run -- airdrop --sol 1.0` |
| `balance` | Check balance | `cargo run -- balance --account <addr>` |
| `blockhash` | Get blockhash | `cargo run -- blockhash` |
| `config` | Show config | `cargo run -- config` |
| `profile` | Manage profiles | `cargo run -- profile --action list` |

## 📞 Support

For issues and questions:
- Check the troubleshooting section
- Verify your configuration
- Ensure valid Solana addresses
- Check network connectivity

---

**Happy Vaulting! 🚀**
